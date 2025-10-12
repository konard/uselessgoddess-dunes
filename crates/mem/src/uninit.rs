use std::{
  mem,
  mem::MaybeUninit,
  ptr::{self, NonNull},
};

pub unsafe fn assume<T>(uninit: &mut [MaybeUninit<T>]) -> &mut [T] {
  unsafe { &mut *(uninit as *mut [MaybeUninit<T>] as *mut [T]) }
}

pub fn fill<T: Clone>(uninit: &mut [MaybeUninit<T>], val: T) -> &mut [T] {
  let mut guard = Guard { slice: uninit, init: 0 };

  if let Some((last, elems)) = guard.slice.split_last_mut() {
    for el in elems.iter_mut() {
      el.write(val.clone());
      guard.init += 1;
    }
    last.write(val);
  }

  mem::forget(guard);

  // SAFETY: slice was initialized by filling up
  unsafe { assume(uninit) }
}

pub fn fill_with<T>(
  uninit: &mut [MaybeUninit<T>],
  mut fill: impl FnMut() -> T,
) {
  let mut guard = Guard { slice: uninit, init: 0 };

  for el in guard.slice.iter_mut() {
    el.write(fill());
    guard.init += 1;
  }

  mem::forget(guard);
}

struct Guard<'a, T> {
  slice: &'a mut [MaybeUninit<T>],
  init: usize,
}

impl<T> Drop for Guard<'_, T> {
  fn drop(&mut self) {
    let slice = &mut self.slice[..self.init];
    // SAFETY: Valid elements have just been filled into `self` so it is initialized
    if !slice.is_empty() {
      unsafe { ptr::drop_in_place(slice as *mut [MaybeUninit<T>] as *mut [T]) }
    }
  }
}
