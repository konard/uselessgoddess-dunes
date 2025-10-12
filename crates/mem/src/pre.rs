use {
  crate::{Error, Page, RawMem, Result},
  std::{
    mem::{self, MaybeUninit},
    ops::{Deref, DerefMut},
  },
};

#[derive(Debug)]
pub struct PreAlloc<P> {
  place: P,
  used: usize,
}

impl<T, P: Deref<Target = [T]> + DerefMut> PreAlloc<P> {
  /// Constructs new `PreAlloc`
  pub fn new(place: P) -> Self {
    Self { place, used: 0 }
  }
}

use bytemuck::Pod;

impl<T: Pod, P: Deref<Target = [T]> + DerefMut> RawMem for PreAlloc<P> {
  type Item = T;

  fn as_slice(&self) -> &[Self::Item] {
    &self.place[..self.used]
  }

  fn as_mut_slice(&mut self) -> &mut [Self::Item] {
    &mut self.place[..self.used]
  }

  fn grow(&mut self, addition: usize) -> Result<Page<'_, Self::Item>> {
    let cap = self.used.checked_add(addition).ok_or(Error::CapacityOverflow)?;
    let available = self.place.len();

    if let Some(slice) = self.place.get_mut(self.used..cap) {
      // SAFETY: just transmute to less checked [MaybeUninit<T>]
      let uninit = unsafe {
        mem::transmute::<&mut [T], &mut [MaybeUninit<T>]>(&mut slice[..])
      };
      self.used = cap;

      Ok(Page { uninit, len: None })
    } else {
      Err(Error::OverGrow { available, to_grow: cap })
    }
  }

  fn shrink(&mut self, cap: usize) -> Result<()> {
    self.used = self.used.saturating_sub(cap);
    Ok(())
  }
}
