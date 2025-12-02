mod common;

use {common::Store, dunes_trees::Tree};

#[test]
fn test_single_insert() {
  let mut store: Store<usize> = Store::new(10);
  let root = store.insert(None, 5);
  assert_eq!(root, Some(5));

  // Check the node was initialized correctly
  let node = store.get(5).unwrap();
  assert_eq!(node.size, 1);
  assert_eq!(node.left, None);
  assert_eq!(node.right, None);

  // Search for it
  assert!(store.contains(5, 5));
}

#[test]
fn test_two_inserts() {
  let mut store: Store<usize> = Store::new(10);

  let mut root = store.insert(None, 5);
  println!("After inserting 5, root = {:?}", root);

  root = store.insert(root, 3);
  println!("After inserting 3, root = {:?}", root);

  if let Some(r) = root {
    println!("Root node {:?}: {:?}", r, store.get(r));
    println!("Node 3: {:?}", store.get(3));
    println!("Node 5: {:?}", store.get(5));

    assert!(store.contains(r, 3), "Should contain 3");
    assert!(store.contains(r, 5), "Should contain 5");
  } else {
    panic!("Root should not be None");
  }
}
