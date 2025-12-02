#![feature(test)]

extern crate test;

mod common;

use {
  common::{Store, TreapStore},
  std::hint::black_box,
  test::Bencher,
  trees::{Idx, Tree},
};

/// Trait for tree stores that can be reset and created
trait BenchStore<T: Idx>: Tree<T> {
  fn new(capacity: usize) -> Self;
  fn reset(&mut self);
}

impl<T: Idx> BenchStore<T> for Store<T> {
  fn new(capacity: usize) -> Self {
    Store::new(capacity)
  }

  fn reset(&mut self) {
    Store::reset(self)
  }
}

impl<T: Idx> BenchStore<T> for TreapStore<T> {
  fn new(capacity: usize) -> Self {
    TreapStore::new(capacity)
  }

  fn reset(&mut self) {
    TreapStore::reset(self)
  }
}

// Helper function for insert benchmarks
fn bench_insert_impl<S, T>(b: &mut Bencher, n: usize)
where
  S: BenchStore<T>,
  T: Idx + From<usize>,
{
  let mut store = S::new(n);
  b.iter(|| {
    let mut root = None;
    for i in 1..n {
      root = store.insert(root, T::from(i));
    }
    black_box(root);
    store.reset();
  });
}

// Helper function for insert and search benchmarks
fn bench_insert_and_search_impl<S, T>(b: &mut Bencher, n: usize)
where
  S: BenchStore<T>,
  T: Idx + From<usize>,
{
  let mut store = S::new(n);
  b.iter(|| {
    let mut root = None;
    for i in 1..n {
      root = store.insert(root, T::from(i));
    }
    for i in 1..n {
      black_box(store.contains(root.unwrap(), T::from(i)));
    }
    store.reset();
  });
}

// Helper function for full cycle benchmarks
fn bench_insert_remove_impl<S, T>(b: &mut Bencher, n: usize)
where
  S: BenchStore<T>,
  T: Idx + From<usize>,
{
  let mut store = S::new(n);
  b.iter(|| {
    let mut root = None;
    for i in 1..n {
      root = store.insert(root, T::from(i));
    }
    for i in 1..n {
      root = store.remove(root, T::from(i));
    }
    black_box(root);
    store.reset();
  });
}

// SBT Insert Benchmarks
#[bench]
fn sbt_insert_100(b: &mut Bencher) {
  bench_insert_impl::<Store<usize>, usize>(b, 100);
}

#[bench]
fn sbt_insert_1000(b: &mut Bencher) {
  bench_insert_impl::<Store<usize>, usize>(b, 1_000);
}

#[bench]
fn sbt_insert_10000(b: &mut Bencher) {
  bench_insert_impl::<Store<usize>, usize>(b, 10_000);
}

// SBT Insert + Search Benchmarks
#[bench]
fn sbt_insert_search_100(b: &mut Bencher) {
  bench_insert_and_search_impl::<Store<usize>, usize>(b, 100);
}

#[bench]
fn sbt_insert_search_1000(b: &mut Bencher) {
  bench_insert_and_search_impl::<Store<usize>, usize>(b, 1_000);
}

#[bench]
fn sbt_insert_search_10000(b: &mut Bencher) {
  bench_insert_and_search_impl::<Store<usize>, usize>(b, 10_000);
}

// SBT Full Cycle Benchmarks
#[bench]
fn sbt_full_cycle_100(b: &mut Bencher) {
  bench_insert_remove_impl::<Store<usize>, usize>(b, 100);
}

#[bench]
fn sbt_full_cycle_1000(b: &mut Bencher) {
  bench_insert_remove_impl::<Store<usize>, usize>(b, 1_000);
}

// Treap Insert Benchmarks
#[bench]
fn treap_insert_100(b: &mut Bencher) {
  bench_insert_impl::<TreapStore<usize>, usize>(b, 100);
}

#[bench]
fn treap_insert_1000(b: &mut Bencher) {
  bench_insert_impl::<TreapStore<usize>, usize>(b, 1_000);
}

#[bench]
fn treap_insert_10000(b: &mut Bencher) {
  bench_insert_impl::<TreapStore<usize>, usize>(b, 10_000);
}

// Treap Insert + Search Benchmarks
#[bench]
fn treap_insert_search_100(b: &mut Bencher) {
  bench_insert_and_search_impl::<TreapStore<usize>, usize>(b, 100);
}

#[bench]
fn treap_insert_search_1000(b: &mut Bencher) {
  bench_insert_and_search_impl::<TreapStore<usize>, usize>(b, 1_000);
}

#[bench]
fn treap_insert_search_10000(b: &mut Bencher) {
  bench_insert_and_search_impl::<TreapStore<usize>, usize>(b, 10_000);
}

// Treap Full Cycle Benchmarks
#[bench]
fn treap_full_cycle_100(b: &mut Bencher) {
  bench_insert_remove_impl::<TreapStore<usize>, usize>(b, 100);
}

#[bench]
fn treap_full_cycle_1000(b: &mut Bencher) {
  bench_insert_remove_impl::<TreapStore<usize>, usize>(b, 1_000);
}
