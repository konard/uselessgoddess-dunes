use crate::{Index, Link};

/// Flow control for iteration handlers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flow {
  /// Continue iterating
  Continue,
  /// Stop iterating
  Break,
}

impl From<bool> for Flow {
  fn from(value: bool) -> Self {
    if value { Flow::Continue } else { Flow::Break }
  }
}

/// Handler function for read operations (iteration)
///
/// Called for each link during iteration. Returns Flow to control
/// whether to continue.
pub trait ReadHandler<L: Index> {
  fn handle(&mut self, link: Link<L>) -> Flow;
}

impl<L, F> ReadHandler<L> for F
where
  L: Index,
  F: FnMut(Link<L>) -> Flow,
{
  fn handle(&mut self, link: Link<L>) -> Flow {
    self(link)
  }
}

/// Handler function for write operations (create, update, delete)
///
/// Called with before and after states. Returns Flow to control
/// whether to continue.
pub trait WriteHandler<L: Index> {
  fn handle(&mut self, before: Link<L>, after: Link<L>) -> Flow;
}

impl<L, F> WriteHandler<L> for F
where
  L: Index,
  F: FnMut(Link<L>, Link<L>) -> Flow,
{
  fn handle(&mut self, before: Link<L>, after: Link<L>) -> Flow {
    self(before, after)
  }
}
