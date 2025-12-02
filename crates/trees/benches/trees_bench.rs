mod common;

use {
  common::{Store, TreapStore},
  criterion::{
    BenchmarkId, Criterion, Throughput, criterion_group, criterion_main,
  },
  dunes_trees::{Idx, Tree},
  std::time::{Duration, Instant},
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

fn bench_insert<S, T>(c: &mut Criterion, tree_name: &str, n: usize)
where
  S: BenchStore<T>,
  T: Idx + From<usize>,
{
  let mut group = c.benchmark_group(format!("{}_insert", tree_name));
  group.throughput(Throughput::Elements(n as u64));

  group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
    let mut store = S::new(n);
    b.iter_custom(|iters| {
      let mut elapsed = Duration::ZERO;
      for _ in 0..iters {
        let instant = Instant::now();
        let mut root = None;
        for i in 1..n {
          root = store.insert(root, T::from(i));
        }
        elapsed += instant.elapsed();
        store.reset();
      }
      elapsed
    });
  });

  group.finish();
}

fn bench_insert_and_search<S, T>(c: &mut Criterion, tree_name: &str, n: usize)
where
  S: BenchStore<T>,
  T: Idx + From<usize>,
{
  let mut group = c.benchmark_group(format!("{}_insert_search", tree_name));
  group.throughput(Throughput::Elements(n as u64 * 2));

  group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
    let mut store = S::new(n);
    b.iter_custom(|iters| {
      let mut elapsed = Duration::ZERO;
      for _ in 0..iters {
        let instant = Instant::now();
        let mut root = None;
        for i in 1..n {
          root = store.insert(root, T::from(i));
        }
        for i in 1..n {
          assert!(store.contains(root.unwrap(), T::from(i)));
        }
        elapsed += instant.elapsed();
        store.reset();
      }
      elapsed
    });
  });

  group.finish();
}

fn bench_insert_remove<S, T>(c: &mut Criterion, tree_name: &str, n: usize)
where
  S: BenchStore<T>,
  T: Idx + From<usize>,
{
  let mut group = c.benchmark_group(format!("{}_full_cycle", tree_name));
  group.throughput(Throughput::Elements(n as u64 * 2));

  group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
    let mut store = S::new(n);
    b.iter_custom(|iters| {
      let mut elapsed = Duration::ZERO;
      for _ in 0..iters {
        let instant = Instant::now();
        let mut root = None;
        for i in 1..n {
          root = store.insert(root, T::from(i));
        }
        for i in 1..n {
          root = store.remove(root, T::from(i));
        }
        elapsed += instant.elapsed();
        store.reset();
      }
      elapsed
    });
  });

  group.finish();
}

fn benchmark_trees(c: &mut Criterion) {
  let sizes = [100, 1_000, 10_000];

  // Benchmark SBT (Size-Balanced Tree)
  for &n in &sizes {
    bench_insert::<Store<usize>, usize>(c, "sbt", n);
  }

  for &n in &sizes {
    bench_insert_and_search::<Store<usize>, usize>(c, "sbt", n);
  }

  for &n in &[100, 1_000] {
    bench_insert_remove::<Store<usize>, usize>(c, "sbt", n);
  }

  // Benchmark Treap
  for &n in &sizes {
    bench_insert::<TreapStore<usize>, usize>(c, "treap", n);
  }

  for &n in &sizes {
    bench_insert_and_search::<TreapStore<usize>, usize>(c, "treap", n);
  }

  for &n in &[100, 1_000] {
    bench_insert_remove::<TreapStore<usize>, usize>(c, "treap", n);
  }
}

criterion_group!(benches, benchmark_trees);
criterion_main!(benches);
