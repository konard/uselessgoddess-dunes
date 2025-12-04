use doublets::{Doublets, Flow, Link, Links, create_heap_store};

#[test]
fn test_create_point() {
  let mut store = create_heap_store::<usize>().expect("Failed to create store");

  let a = store.create_point().expect("Failed to create point");
  assert_eq!(a, 1);

  let link = store.get(a).expect("Link should exist");
  assert_eq!(link, Link::new(a, a, a));
}

#[test]
fn test_create_link() {
  let mut store = create_heap_store::<usize>().expect("Failed to create store");

  let a = store.create_point().expect("Failed to create point a");
  let b = store.create_point().expect("Failed to create point b");

  let c = store.create_link(a, b).expect("Failed to create link");

  let link = store.get(c).expect("Link should exist");
  assert_eq!(link, Link::new(c, a, b));
}

#[test]
fn test_update_link() {
  let mut store = create_heap_store::<usize>().expect("Failed to create store");

  let a = store.create_point().expect("Failed to create point a");
  let b = store.create_point().expect("Failed to create point b");
  let c = store.create_point().expect("Failed to create point c");

  store.update_link(c, a, b).expect("Failed to update link");

  let link = store.get(c).expect("Link should exist");
  assert_eq!(link, Link::new(c, a, b));
}

#[test]
fn test_delete_link() {
  let mut store = create_heap_store::<usize>().expect("Failed to create store");

  let a = store.create_point().expect("Failed to create point");

  store.delete_link(a).expect("Failed to delete link");

  assert!(store.get(a).is_none());
}

#[test]
fn test_search() {
  let mut store = create_heap_store::<usize>().expect("Failed to create store");

  let a = store.create_point().expect("Failed to create point a");
  let b = store.create_point().expect("Failed to create point b");
  let c = store.create_link(a, b).expect("Failed to create link");

  let found = store.search(a, b);
  assert_eq!(found, Some(c));

  let not_found = store.search(b, a);
  assert_eq!(not_found, None);
}

#[test]
fn test_count() {
  let mut store = create_heap_store::<usize>().expect("Failed to create store");

  assert_eq!(store.count_all(), 0);

  let _a = store.create_point().expect("Failed to create point a");
  assert_eq!(store.count_all(), 1);

  let _b = store.create_point().expect("Failed to create point b");
  assert_eq!(store.count_all(), 2);

  let _c = store.create_link(1, 2).expect("Failed to create link");
  assert_eq!(store.count_all(), 3);
}

#[test]
fn test_rebase() {
  let mut store = create_heap_store::<usize>().expect("Failed to create store");

  let a = store.create_point().expect("Failed to create point a");
  let b = store.create_point().expect("Failed to create point b");

  let c = store.create_point().expect("Failed to create point c");
  store.update_link(c, c, a).expect("Failed to update link c");

  let d = store.create_point().expect("Failed to create point d");
  store.update_link(d, a, d).expect("Failed to update link d");

  // Verify initial state
  let links: Vec<_> = store.iter().collect();
  assert_eq!(
    links,
    vec![
      Link::new(a, a, a),
      Link::new(b, b, b),
      Link::new(c, c, a),
      Link::new(d, a, d)
    ]
  );

  // Rebase a to b
  store.rebase(a, b).expect("Failed to rebase");

  // Verify after rebase
  let links: Vec<_> = store.iter().collect();
  assert_eq!(
    links,
    vec![
      Link::new(a, a, a),
      Link::new(b, b, b),
      Link::new(c, c, b),
      Link::new(d, b, d)
    ]
  );
}

#[test]
fn test_each() {
  let mut store = create_heap_store::<usize>().expect("Failed to create store");

  let a = store.create_point().expect("Failed to create point a");
  let b = store.create_point().expect("Failed to create point b");
  let _c = store.create_link(a, b).expect("Failed to create link");

  let mut count = 0;
  store.each(&[], &mut |_link| {
    count += 1;
    Flow::Continue
  });

  assert_eq!(count, 3);
}

#[test]
fn test_each_with_query() {
  let mut store = create_heap_store::<usize>().expect("Failed to create store");

  let a = store.create_point().expect("Failed to create point a");
  let b = store.create_point().expect("Failed to create point b");
  let _c = store.create_link(a, b).expect("Failed to create link");

  let any = store.constants().any;
  let mut found_links = Vec::new();

  store.each(&[any, a, any], &mut |link| {
    found_links.push(link);
    Flow::Continue
  });

  // Should find link c (which has source = a)
  assert_eq!(found_links.len(), 2); // link a and link c
}

#[test]
fn test_get_or_create() {
  let mut store = create_heap_store::<usize>().expect("Failed to create store");

  let a = store.create_point().expect("Failed to create point a");
  let b = store.create_point().expect("Failed to create point b");

  let c1 = store.get_or_create(a, b).expect("Failed to get or create");
  let c2 = store.get_or_create(a, b).expect("Failed to get or create");

  assert_eq!(c1, c2);
  assert_eq!(store.count_all(), 3); // Only a, b, and c (not duplicated)
}
