use doublets::{Flow, Links, create_heap_store};

#[test]
fn test_basic_create() {
  let mut store = create_heap_store::<usize>().expect("Failed to create store");

  // Create a link with empty query
  let mut index = 0;
  store
    .create(&[], &mut |_before, after| {
      index = after.index;
      println!("Created link with index: {}", index);
      Flow::Continue
    })
    .expect("Failed to create link");

  println!("Checking if link {} exists...", index);
  let link = store.get(index);
  println!("Result: {:?}", link);
  assert!(link.is_some(), "Link should exist");
}
