use {
  crate::{Result, uninit},
  bytemuck::{Pod, Zeroable},
  std::{alloc::Layout, mem::MaybeUninit},
};

/// Error of memory allocation
// fixme: maybe we should add `(X bytes)` after `cannot allocate/occupy`
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
  /// Error due to the computed capacity exceeding the maximum
  /// (usually `isize::MAX` bytes).
  ///
  /// ## Examples
  ///
  /// grow more than `isize::MAX` bytes:
  ///
  /// ```
  /// # #![feature(allocator_api)]
  /// # #![feature(assert_matches)]
  /// # use std::alloc::Global;
  /// # use std::assert_matches::assert_matches;
  /// # use mem::{Error, Alloc, RawMem};
  /// let mut mem = Alloc::new(Global);
  /// assert_matches!(mem.grow_filled(usize::MAX, 0u64), Err(Error::CapacityOverflow));
  /// ```
  #[error("exceeding the capacity maximum")]
  CapacityOverflow,
  #[error("can't grow {to_grow} elements, only available {available}")]
  OverGrow { to_grow: usize, available: usize },
  /// The memory allocator returned an error
  #[error("memory allocation of {layout:?} failed")]
  AllocError {
    /// The layout of allocation request that failed
    layout: Layout,
    #[doc(hidden)]
    non_exhaustive: (),
  },
  /// System error memory allocation occurred
  #[error(transparent)]
  System(#[from] std::io::Error),
}

#[derive(Debug)]
pub struct Page<'a, T> {
  pub(crate) len: Option<&'a mut usize>,
  pub uninit: &'a mut [MaybeUninit<T>],
}

impl<'a, T> Page<'a, T> {
  fn advance<F>(self, fill: F) -> &'a mut [T]
  where
    F: FnOnce(&mut [MaybeUninit<T>]) -> &mut [T],
  {
    let slice = fill(self.uninit);
    if let Some(len) = self.len {
      *len += slice.len();
    }
    slice
  }
}

impl<'a, T: Pod> Page<'a, T> {
  /// [`grow`] which assumes that the memory is already initialized
  ///
  /// # Safety
  ///
  /// When calling this method, you have to ensure that one of the following is true:
  ///
  /// * memory already initialized as [`Item`]
  ///
  /// * memory is initialized bytes and [`Item`] can be represented as bytes
  ///
  /// # Examples
  ///
  /// ```no_run
  /// # use mem::Result;
  /// use mem::{FileMapped, RawMem};
  ///
  /// let mut file = FileMapped::from_path("file.bin")?;
  /// // file is always represents as initialized bytes
  /// // and usize is transparent as bytes
  /// let _: &mut [usize] = unsafe { file.grow_assumed(10)? };
  /// # Result::Ok(())
  /// ```
  ///
  /// [`grow`]: Self::grow
  /// [`Item`]: Self::Item
  pub unsafe fn assumed(self) -> &'a mut [T] {
    self.advance(|uninit| uninit::assume(uninit))
  }
}

impl<'a, T: Clone> Page<'a, T> {
  pub fn filled(self, value: T) -> &'a mut [T] {
    self.advance(|uninit| uninit::fill(uninit, value))
  }
}

impl<'a, T: Zeroable> Page<'a, T> {
  /// # Examples
  /// Correct usage of this function: initializing an [`Zeroable`](Zeroable) types with zeroes:
  /// ```
  /// # #![feature(allocator_api)]
  /// # use mem::Error;
  /// use mem::{Global, RawMem};
  ///
  /// let mut alloc = Global::new();
  /// let zeroes: &mut [(u8, u16)] = unsafe {
  ///     alloc.grow_zeroed(10)?
  /// };
  ///
  /// assert_eq!(zeroes, [(0, 0); 10]);
  /// # Ok::<_, Error>(())
  /// ```
  pub fn zeroed(self) -> &'a mut [T] {
    self.advance(|uninit| {
      // SAFETY: zeroable types must be valid when filled by zeros
      unsafe {
        uninit.as_mut_ptr().write_bytes(0u8, uninit.len());
        uninit::assume(uninit)
      }
    })
  }
}

pub trait RawMem {
  type Item: Pod;

  fn as_slice(&self) -> &[Self::Item];
  fn as_mut_slice(&mut self) -> &mut [Self::Item];

  /// # Safety
  /// Caller must guarantee that `fill` initialize memory
  /// [`MaybeUninit::slice_assume_init_mut`]
  ///
  /// ### Incorrect usage
  /// ```no_run
  /// # #![feature(allocator_api)]
  /// # use std::alloc::Global;
  /// # use std::mem::MaybeUninit;
  /// # use mem::Result;
  /// use mem::{Alloc, RawMem};
  ///
  /// let mut alloc = Alloc::new(Global);
  /// unsafe {
  ///     alloc.grow(10, |_uninit: &mut [MaybeUninit<u64>]| {
  ///         // `RawMem` relies on the fact that we initialize memory
  ///         // even if they are primitives
  ///     })?;
  /// }
  /// # Result::Ok(())
  /// ```
  fn grow(&mut self, cap: usize) -> Result<Page<'_, Self::Item>>;

  fn shrink(&mut self, cap: usize) -> Result<()>;
}
