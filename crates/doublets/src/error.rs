use {crate::Index, core::fmt::Debug, thiserror::Error};
/// Errors that can occur during doublets operations
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum Error<L: Index> {
  #[error("Link {0:?} does not exist")]
  NotExists(L),
  #[error("Link {0:?} already exists with source {1:?} and target {2:?}")]
  AlreadyExists(L, L, L),
  #[error("Link {0:?} has usages and cannot be deleted")]
  HasUsages(L),
  #[error("Memory allocation failed")]
  AllocationFailed,
  #[error("Operation would overflow capacity")]
  Overflow,
  #[error("Invalid query parameters")]
  InvalidQuery,
}
pub type Result<T, L> = core::result::Result<T, Error<L>>;
