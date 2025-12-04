use crate::{
  Constants, DoubletsError, Flow, Link, LinkIndex, ReadHandler, Result,
  WriteHandler,
};

/// Core trait for doublets storage operations
///
/// Provides low-level CRUD operations on links
pub trait Links<T: LinkIndex>: Send + Sync {
  /// Get the constants for this store
  fn constants(&self) -> &Constants<T>;

  /// Count links matching a query
  ///
  /// Query format: [index?, source?, target?]
  /// Use constants().any for wildcards
  fn count(&self, query: &[T]) -> T;

  /// Create a new link
  ///
  /// Query format: [source?, target?] or []
  /// Empty query creates a point link
  fn create(
    &mut self,
    query: &[T],
    handler: WriteHandler<'_, T>,
  ) -> Result<Flow, T>;

  /// Iterate over links matching a query
  fn each(&self, query: &[T], handler: ReadHandler<'_, T>) -> Flow;

  /// Update links matching a query
  ///
  /// Query identifies which links to update
  /// Change specifies new values
  fn update(
    &mut self,
    query: &[T],
    change: &[T],
    handler: WriteHandler<'_, T>,
  ) -> Result<Flow, T>;

  /// Delete links matching a query
  fn delete(
    &mut self,
    query: &[T],
    handler: WriteHandler<'_, T>,
  ) -> Result<Flow, T>;

  /// Get a specific link by index
  fn get(&self, index: T) -> Option<Link<T>>;
}

/// High-level doublets operations
///
/// Extends Links with convenient methods for common operations
pub trait Doublets<T: LinkIndex>: Links<T> {
  /// Count all links in the store
  #[inline]
  fn count_all(&self) -> T {
    self.count(&[])
  }

  /// Count links matching a specific query
  #[inline]
  fn count_by(&self, query: &[T]) -> T {
    self.count(query)
  }

  /// Create a new link and return its index
  fn create_link(&mut self, source: T, target: T) -> Result<T, T> {
    let mut result = T::zero();
    self.create(&[source, target], &mut |_before, after| {
      result = after.index;
      Flow::Continue
    })?;
    Ok(result)
  }

  /// Create a point link (source = target = index)
  fn create_point(&mut self) -> Result<T, T> {
    let mut index = T::zero();
    // First create empty link
    self.create(&[], &mut |_before, after| {
      index = after.index;
      Flow::Continue
    })?;
    // Then update it to point to itself
    self.update(&[index], &[index, index, index], &mut |_before, _after| {
      Flow::Continue
    })?;
    Ok(index)
  }

  /// Update a specific link
  fn update_link(&mut self, index: T, source: T, target: T) -> Result<T, T> {
    let mut result = T::zero();
    self.update(
      &[index],
      &[index, source, target],
      &mut |_before, after| {
        result = after.index;
        Flow::Continue
      },
    )?;
    Ok(result)
  }

  /// Delete a specific link
  fn delete_link(&mut self, index: T) -> Result<T, T> {
    let mut result = T::zero();
    self.delete(&[index], &mut |before, _after| {
      result = before.index;
      Flow::Continue
    })?;
    Ok(result)
  }

  /// Search for a link with specific source and target
  fn search(&self, source: T, target: T) -> Option<T> {
    let any = self.constants().any;
    let mut result = None;
    self.each(&[any, source, target], &mut |link| {
      result = Some(link.index);
      Flow::Break
    });
    result
  }

  /// Get or create a link with specific source and target
  fn get_or_create(&mut self, source: T, target: T) -> Result<T, T> {
    if let Some(existing) = self.search(source, target) {
      Ok(existing)
    } else {
      self.create_link(source, target)
    }
  }

  /// Count usages of a link (as source or target)
  fn count_usages(&self, index: T) -> Result<T, T> {
    let any = self.constants().any;
    let link = self.get(index).ok_or(DoubletsError::NotExists(index))?;

    let mut usage_source = self.count(&[any, index, any]);
    if index == link.source {
      usage_source = usage_source.checked_sub_one().unwrap_or(usage_source);
    }

    let mut usage_target = self.count(&[any, any, index]);
    if index == link.target {
      usage_target = usage_target.checked_sub_one().unwrap_or(usage_target);
    }

    // Simple addition - in production might need checked arithmetic
    Ok(T::from_usize(usage_source.as_usize() + usage_target.as_usize()))
  }

  /// Check if a link has any usages
  fn has_usages(&self, index: T) -> bool {
    self.count_usages(index).map(|count| !count.is_zero()).unwrap_or(false)
  }

  /// Rebase: replace all occurrences of old with new
  fn rebase(&mut self, old: T, new: T) -> Result<T, T> {
    if old == new {
      return Ok(new);
    }

    // Verify old link exists
    let _ = self.get(old).ok_or(DoubletsError::NotExists(old))?;

    let any = self.constants().any;

    // Collect all links that use 'old' as source or target
    let mut to_update = Vec::new();
    self.each(&[any, old, any], &mut |link| {
      if link.index != old {
        to_update.push((link.index, new, link.target));
      }
      Flow::Continue
    });
    self.each(&[any, any, old], &mut |link| {
      if link.index != old {
        to_update.push((link.index, link.source, new));
      }
      Flow::Continue
    });

    // Update all collected links
    for (index, source, target) in to_update {
      self.update_link(index, source, target)?;
    }

    Ok(new)
  }

  /// Rebase and then delete the old link
  fn rebase_and_delete(&mut self, old: T, new: T) -> Result<T, T> {
    if old == new {
      Ok(new)
    } else {
      self.rebase(old, new)?;
      self.delete_link(old)
    }
  }

  /// Collect all links into a vector
  fn collect_all(&self) -> Vec<Link<T>> {
    let count = self.count_all().as_usize();
    let mut result = Vec::with_capacity(count);
    self.each(&[], &mut |link| {
      result.push(link);
      Flow::Continue
    });
    result
  }

  /// Iterate over all links
  fn iter(&self) -> impl Iterator<Item = Link<T>> {
    self.collect_all().into_iter()
  }
}

// Automatic implementation of Doublets for any type implementing Links
impl<T: LinkIndex, L: Links<T> + ?Sized> Doublets<T> for L {}
