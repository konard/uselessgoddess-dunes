use crate::{
  Error, Flow, Index, Link, Links, ReadHandler, Result, WriteHandler,
};

use mem::{Alloc, RawMem};

/// Raw link data stored in memory
///
/// Stores source and target for a link. Tree navigation is handled
/// separately. We use a special marker in source to indicate if this
/// link is in the free list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
pub struct RawLink {
  source: usize,
  target: usize,
  is_free: usize,
}

unsafe impl bytemuck::Pod for RawLink {}
unsafe impl bytemuck::Zeroable for RawLink {}

/// Doublets store implementation using modern memory management
///
/// This is a simplified implementation that stores links in a flat
/// array. More sophisticated indexing (like the old tree-based
/// approach) can be added later.
pub struct Store<L, M = Alloc<RawLink>>
where
  L: Index,
  M: RawMem<Item = RawLink> + Send + Sync,
{
  mem: M,
  allocated: usize,
  free_count: usize,
  first_free: Option<usize>,
  _phantom: core::marker::PhantomData<L>,
}

impl<L, M> Store<L, M>
where
  L: Index,
  M: RawMem<Item = RawLink> + Send + Sync,
{
  /// Create a new doublets store with default capacity
  pub fn new(mut mem: M) -> Result<Self, L> {
    mem.grow(1024).map_err(|_| Error::AllocationFailed)?.zeroed();

    Ok(Self {
      mem,
      allocated: 1,
      free_count: 0,
      first_free: None,
      _phantom: core::marker::PhantomData,
    })
  }

  /// Get a raw link from memory
  #[inline]
  fn repr_at(&self, index: usize) -> Option<&RawLink> {
    let slice = self.mem.as_slice();
    slice.get(index)
  }

  /// Get a mutable raw link from memory
  #[inline]
  fn repr_mut_at(&mut self, index: usize) -> Option<&mut RawLink> {
    let slice = self.mem.as_mut_slice();
    slice.get_mut(index)
  }

  /// Check if a link exists and is not in free list
  fn exists(&self, index: L) -> bool {
    let idx = index.as_usize();
    if index.is_zero() || idx >= self.allocated {
      return false;
    }

    if let Some(raw) = self.repr_at(idx) {
      raw.is_free != usize::MAX
    } else {
      false
    }
  }

  /// Allocate a new link index
  fn allocate_index(&mut self) -> Result<L, L> {
    if let Some(free_index) = self.first_free {
      let next_free = if let Some(raw) = self.repr_at(free_index) {
        if raw.source == 0 { None } else { Some(raw.source) }
      } else {
        None
      };

      if let Some(raw) = self.repr_mut_at(free_index) {
        raw.is_free = 0;
      }

      self.first_free = next_free;
      self.free_count -= 1;
      return Ok(L::from_usize(free_index));
    }

    let index = self.allocated;
    self.allocated += 1;

    if self.allocated >= self.mem.as_slice().len() {
      let current_len = self.mem.as_slice().len();
      let addition = current_len;
      self.mem.grow(addition).map_err(|_| Error::AllocationFailed)?.zeroed();
    }

    if let Some(raw) = self.repr_mut_at(index) {
      raw.source = 0;
      raw.target = 0;
      raw.is_free = 0;
    }

    Ok(L::from_usize(index))
  }

  /// Free a link index
  fn free_index(&mut self, index: L) {
    let idx = index.as_usize();
    let next_free = self.first_free.unwrap_or(0);

    if let Some(raw) = self.repr_mut_at(idx) {
      raw.source = next_free;
      raw.target = 0;
      raw.is_free = usize::MAX;
    }

    self.first_free = Some(idx);
    self.free_count += 1;
  }

  /// Count all non-free links
  fn count_total(&self) -> usize {
    self.allocated - self.free_count - 1
  }
}

impl<L, M> Links<L> for Store<L, M>
where
  L: Index,
  M: RawMem<Item = RawLink> + Send + Sync,
{
  fn count<const N: usize>(&self, query: [L; N]) -> L {
    match N {
      0 => L::from_usize(self.count_total()),
      1 => {
        let index = query[0];
        if index == L::ANY {
          L::from_usize(self.count_total())
        } else if self.exists(index) {
          L::ONE
        } else {
          L::ZERO
        }
      }
      _ => {
        let mut count = 0;
        self.each(query, &mut |_| {
          count += 1;
          Flow::Continue
        });
        L::from_usize(count)
      }
    }
  }

  fn create<const N: usize, H: WriteHandler<L>>(
    &mut self,
    query: [L; N],
    handler: &mut H,
  ) -> Result<Flow, L> {
    let index = self.allocate_index()?;
    let before = Link::nothing();

    let (source, target) = match N {
      0 => (L::ZERO, L::ZERO),
      1 => (query[0], query[0]),
      _ => (query[0], query[1]),
    };

    if let Some(raw) = self.repr_mut_at(index.as_usize()) {
      raw.source = source.as_usize();
      raw.target = target.as_usize();
      raw.is_free = 0;
    }

    let after = Link::new(index, source, target);
    Ok(handler.handle(before, after))
  }

  fn each<const N: usize, H: ReadHandler<L>>(
    &self,
    query: [L; N],
    handler: &mut H,
  ) -> Flow {
    if N == 0 {
      for i in 1..self.allocated {
        let index = L::from_usize(i);
        if self.exists(index)
          && let Some(raw) = self.repr_at(i)
        {
          let source = L::from_usize(raw.source);
          let target = L::from_usize(raw.target);
          let link = Link::new(index, source, target);
          if handler.handle(link) == Flow::Break {
            return Flow::Break;
          }
        }
      }
      return Flow::Continue;
    }

    let index_query = query[0];

    if N == 1 {
      if index_query == L::ANY {
        return self.each([], handler);
      } else if self.exists(index_query)
        && let Some(raw) = self.repr_at(index_query.as_usize())
      {
        let source = L::from_usize(raw.source);
        let target = L::from_usize(raw.target);
        return handler.handle(Link::new(index_query, source, target));
      }
      return Flow::Continue;
    }

    let source = if N >= 2 { query[1] } else { L::ANY };
    let target = if N >= 3 { query[2] } else { L::ANY };

    for i in 1..self.allocated {
      let index = L::from_usize(i);
      if !self.exists(index) {
        continue;
      }

      let raw = match self.repr_at(i) {
        Some(r) => r,
        None => continue,
      };

      let raw_source = L::from_usize(raw.source);
      let raw_target = L::from_usize(raw.target);

      let matches = (index_query == L::ANY || index_query == index)
        && (source == L::ANY || source == raw_source)
        && (target == L::ANY || target == raw_target);

      if matches {
        let link = Link::new(index, raw_source, raw_target);
        if handler.handle(link) == Flow::Break {
          return Flow::Break;
        }
      }
    }

    Flow::Continue
  }

  fn update<const N1: usize, const N2: usize, H: WriteHandler<L>>(
    &mut self,
    query: [L; N1],
    change: [L; N2],
    handler: &mut H,
  ) -> Result<Flow, L> {
    if N1 == 0 || N2 == 0 {
      return Err(Error::InvalidQuery);
    }

    let index = query[0];
    if !self.exists(index) {
      return Err(Error::NotExists(index));
    }

    let before = self.get(index).ok_or(Error::NotExists(index))?;

    let new_source = if N2 >= 2 { change[1] } else { before.source };
    let new_target = if N2 >= 3 { change[2] } else { before.target };

    if let Some(raw) = self.repr_mut_at(index.as_usize()) {
      raw.source = new_source.as_usize();
      raw.target = new_target.as_usize();
    }

    let after = Link::new(index, new_source, new_target);
    Ok(handler.handle(before, after))
  }

  fn delete<const N: usize, H: WriteHandler<L>>(
    &mut self,
    query: [L; N],
    handler: &mut H,
  ) -> Result<Flow, L> {
    if N == 0 {
      return Err(Error::InvalidQuery);
    }

    let index = query[0];
    if !self.exists(index) {
      return Err(Error::NotExists(index));
    }

    let before = self.get(index).ok_or(Error::NotExists(index))?;

    self.free_index(index);

    let after = Link::nothing();
    Ok(handler.handle(before, after))
  }

  fn get(&self, index: L) -> Option<Link<L>> {
    if !self.exists(index) {
      return None;
    }

    let raw = self.repr_at(index.as_usize())?;
    let source = L::from_usize(raw.source);
    let target = L::from_usize(raw.target);
    Some(Link::new(index, source, target))
  }
}

/// Create a doublets store with heap allocation
pub fn create_heap_store<L>() -> Result<Store<L, Alloc<RawLink>>, L>
where
  L: Index,
{
  Store::new(Alloc::new())
}
