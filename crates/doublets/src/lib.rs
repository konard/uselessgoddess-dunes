#![doc = include_str!("../README.md")]

mod error;
mod handler;
mod link;
mod store;
mod traits;

pub use {
  error::{DoubletsError, Result},
  handler::{Constants, Flow, ReadHandler, WriteHandler},
  link::{Link, LinkIndex},
  store::{DoubletsStore, create_heap_store},
  traits::{Doublets, Links},
};
