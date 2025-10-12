use {mem::RawMem, std::error::Error};

type Result = std::result::Result<(), Box<dyn Error>>;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct Pod {
  i: i32,
  a: [i32; 3],
  t: (u8, i32),
}

unsafe impl bytemuck::Zeroable for Pod {}
unsafe impl bytemuck::Pod for Pod {}

pub fn miri(mut mem: impl RawMem<Item = Pod>) -> Result {
  const GROW: usize = if cfg!(miri) { 100 } else { 10_000 };

  let val = Pod { i: 1, a: [2; 3], t: (3, 4) };

  for _ in 0..10 {
    mem.grow(GROW)?.filled(val);
  }
  assert_eq!(mem.as_slice(), &vec![val; GROW * 10][..]);

  for _ in 0..10 {
    mem.shrink(GROW)?;
  }
  assert_eq!(mem.as_slice().len(), 0);

  Ok(())
}
