use mem::{Alloc, RawMem, Result};

#[test]
fn test_new() {
  let alloc = Alloc::<u64>::new();
  assert_eq!(alloc.len(), 0);
  assert_eq!(alloc.capacity(), 0);
  assert!(alloc.is_empty());
}

#[test]
fn test_grow_and_shrink() -> Result<()> {
  let mut alloc = Alloc::<u64>::new();

  alloc.grow(10)?.zeroed();
  assert_eq!(alloc.len(), 10);
  assert_eq!(alloc.capacity(), 10);

  alloc.shrink(5)?;
  assert_eq!(alloc.capacity(), 5);

  Ok(())
}

#[test]
fn test_zeroed() -> Result<()> {
  let mut alloc = Alloc::<u64>::new();
  let data = alloc.grow(5)?.zeroed();
  assert_eq!(data, &[0u64; 5]);
  Ok(())
}

#[test]
fn test_filled() -> Result<()> {
  let mut alloc = Alloc::<i32>::new();
  let data = alloc.grow(3)?.filled(42);
  assert_eq!(data, &[42, 42, 42]);
  Ok(())
}
