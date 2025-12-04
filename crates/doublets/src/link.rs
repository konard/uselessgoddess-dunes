use core::{
  fmt::{self, Debug, Formatter},
  num::{
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize,
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
  },
};

/// Trait for types that can be used as link identifiers
///
/// This trait is implemented for primitives (usize, u64, etc.)
/// The Repr type is used for memory-efficient tree storage using
/// NonZero variants.
pub trait Index:
  Copy + Clone + Eq + PartialEq + Ord + PartialOrd + Debug + Send + Sync
{
  /// The representation type for tree storage (typically NonZero variant)
  type Repr: Copy
    + Clone
    + Eq
    + PartialEq
    + Ord
    + PartialOrd
    + Debug
    + Send
    + Sync;

  /// Special constant values known at compile time
  const ZERO: Self;
  const ANY: Self;
  const ONE: Self;

  /// Check if the value is zero
  fn is_zero(&self) -> bool;

  /// Convert from usize
  fn from_usize(val: usize) -> Self;

  /// Convert to usize
  fn as_usize(&self) -> usize;

  /// Try to increment the value by one
  fn checked_add_one(&self) -> Option<Self>;

  /// Try to decrement the value by one
  fn checked_sub_one(&self) -> Option<Self>;

  /// Convert to Repr (NonZero representation)
  fn to_repr(self) -> Option<Self::Repr>;

  /// Convert from Repr (NonZero representation)
  fn from_repr(repr: Self::Repr) -> Self;
}

macro_rules! impl_index {
  ($prim:ty, $nonzero:ty) => {
    impl Index for $prim {
      type Repr = $nonzero;

      const ZERO: Self = 0;
      const ANY: Self = 0;
      const ONE: Self = 1;

      #[inline]
      fn is_zero(&self) -> bool {
        *self == 0
      }

      #[inline]
      fn from_usize(val: usize) -> Self {
        val as Self
      }

      #[inline]
      fn as_usize(&self) -> usize {
        *self as usize
      }

      #[inline]
      fn checked_add_one(&self) -> Option<Self> {
        self.checked_add(1)
      }

      #[inline]
      fn checked_sub_one(&self) -> Option<Self> {
        self.checked_sub(1)
      }

      #[inline]
      fn to_repr(self) -> Option<Self::Repr> {
        <$nonzero>::new(self)
      }

      #[inline]
      fn from_repr(repr: Self::Repr) -> Self {
        repr.get()
      }
    }
  };
}

impl_index!(usize, NonZeroUsize);
impl_index!(u8, NonZeroU8);
impl_index!(u16, NonZeroU16);
impl_index!(u32, NonZeroU32);
impl_index!(u64, NonZeroU64);
impl_index!(u128, NonZeroU128);
impl_index!(isize, NonZeroIsize);
impl_index!(i8, NonZeroI8);
impl_index!(i16, NonZeroI16);
impl_index!(i32, NonZeroI32);
impl_index!(i64, NonZeroI64);
impl_index!(i128, NonZeroI128);

/// Represents a link/edge in the doublets database
///
/// A link has an index (identifier), source, and target.
/// All three components use the same type L which implements Index.
#[derive(Default, Eq, PartialEq, Clone, Hash, Copy)]
#[repr(C)]
pub struct Link<L: Index> {
  pub index: L,
  pub source: L,
  pub target: L,
}

impl<L: Index> Link<L> {
  /// Create a new link
  #[inline]
  #[must_use]
  pub const fn new(index: L, source: L, target: L) -> Self {
    Self { index, source, target }
  }

  /// Create a "point" link where all components are the same
  #[inline]
  #[must_use]
  pub const fn point(val: L) -> Self {
    Self::new(val, val, val)
  }

  /// Create a null/default link
  #[inline]
  #[must_use]
  pub const fn nothing() -> Self {
    Self::new(L::ZERO, L::ZERO, L::ZERO)
  }

  /// Check if this is a null link
  #[inline]
  #[must_use]
  pub fn is_null(&self) -> bool {
    self.index.is_zero() && self.source.is_zero() && self.target.is_zero()
  }

  /// Check if this is a "full" point (all components equal)
  #[inline]
  #[must_use]
  pub fn is_full(&self) -> bool {
    self.index == self.source && self.index == self.target
  }

  /// Check if this is a "partial" point (index equals source OR target)
  #[inline]
  #[must_use]
  pub fn is_partial(&self) -> bool {
    self.index == self.source || self.index == self.target
  }
}

impl<L: Index> Debug for Link<L> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{:?}: {:?} {:?}", self.index, self.source, self.target)
  }
}

unsafe impl<L: Index> bytemuck::Pod for Link<L> where L: bytemuck::Pod {}
#[rustfmt::skip]
unsafe impl<L: Index> bytemuck::Zeroable for Link<L>
where L: bytemuck::Zeroable {}
