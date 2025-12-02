use std::num::*;

/// Index type for tree nodes - can be u8, u16, u32, u64, or NonZero variants
pub trait Idx: Copy + Eq {
  fn as_usize(self) -> usize;
  fn is_null(self) -> bool;
  fn null() -> Self;
}

macro_rules! impl_idx {
  ($($ty:ty)*) => {$(
    impl Idx for $ty {
      #[inline]
      fn as_usize(self) -> usize {
        self as usize
      }

      #[inline]
      fn is_null(self) -> bool {
        self == 0
      }

      #[inline]
      fn null() -> Self {
        0
      }
    }
  )*};
}

macro_rules! impl_idx_nonzero {
  ($($ty:ty)*) => {$(
    impl Idx for $ty {
      #[inline]
      fn as_usize(self) -> usize {
        self.get() as usize
      }

      #[inline]
      fn is_null(self) -> bool {
        false
      }

      #[inline]
      fn null() -> Self {
        unreachable!("NonZero types use Option<T> for null")
      }
    }
  )*};
}

impl_idx! { u8 u16 u32 u64 usize }
impl_idx_nonzero! {
  NonZeroU8 NonZeroU16 NonZeroU32 NonZeroU64 NonZeroUsize
}

/// Tree node structure - stores size and children
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Node<T> {
  pub size: usize,
  pub left: Option<T>,
  pub right: Option<T>,
}

impl<T> Default for Node<T> {
  fn default() -> Self {
    Self { size: 0, left: None, right: None }
  }
}

impl<T> Node<T> {
  #[inline]
  pub fn new() -> Self {
    Self::default()
  }

  #[inline]
  pub fn with_size(size: usize) -> Self {
    Self { size, left: None, right: None }
  }
}
