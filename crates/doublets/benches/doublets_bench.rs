use {
  criterion::{Criterion, black_box, criterion_group, criterion_main},
  doublets::{Doublets, create_heap_store},
};

fn bench_create_point(c: &mut Criterion) {
  c.bench_function("create_point", |b| {
    b.iter(|| {
      let mut store = create_heap_store::<usize>().unwrap();
      for _ in 0..100 {
        black_box(store.create_point().unwrap());
      }
    });
  });
}

fn bench_create_link(c: &mut Criterion) {
  c.bench_function("create_link", |b| {
    b.iter(|| {
      let mut store = create_heap_store::<usize>().unwrap();
      let a = store.create_point().unwrap();
      let b = store.create_point().unwrap();
      for _ in 0..100 {
        black_box(store.create_link(a, b).unwrap());
      }
    });
  });
}

fn bench_search(c: &mut Criterion) {
  c.bench_function("search", |b| {
    let mut store = create_heap_store::<usize>().unwrap();
    let a = store.create_point().unwrap();
    let b = store.create_point().unwrap();
    store.create_link(a, b).unwrap();

    b.iter(|| {
      black_box(store.search(a, b));
    });
  });
}

fn bench_iteration(c: &mut Criterion) {
  c.bench_function("iterate_all", |b| {
    let mut store = create_heap_store::<usize>().unwrap();
    for _ in 0..100 {
      store.create_point().unwrap();
    }

    b.iter(|| {
      let links: Vec<_> = store.iter().collect();
      black_box(links);
    });
  });
}

criterion_group!(
  benches,
  bench_create_point,
  bench_create_link,
  bench_search,
  bench_iteration
);
criterion_main!(benches);
