use {
  crate::{Error::CapacityOverflow, Page, RawMem, Result},
  memmap2::{MmapMut, MmapOptions},
  std::{
    alloc::Layout,
    fmt::{self, Formatter},
    fs::{File, OpenOptions},
    io,
    marker::PhantomData,
    path::Path,
    ptr::{self, NonNull},
    slice,
  },
};

pub struct FileMapped<T> {
  pub(crate) file: File,
  len: usize,
  pub map: Option<MmapMut>,
  _marker: PhantomData<T>,
}

impl<T> FileMapped<T> {
  // todo: say about mapping, read-write guarantees, and `MIN_PAGE_SIZE`
  pub fn new(file: File) -> io::Result<Self> {
    const MIN_PAGE_SIZE: u64 = 8 * 1024;

    if file.metadata()?.len() < MIN_PAGE_SIZE {
      file.set_len(MIN_PAGE_SIZE)?;
    }

    Ok(Self { file, len: 0, map: None, _marker: PhantomData })
  }

  fn options() -> OpenOptions {
    let mut options = OpenOptions::new();
    options.create(true).read(true).write(true);
    options
  }

  pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
    Self::options().open(path).and_then(Self::new)
  }

  fn map_mut(&mut self, cap: u64) -> io::Result<MmapMut> {
    unsafe { MmapOptions::new().len(cap as usize).map_mut(&self.file) }
  }

  fn map_replace(&mut self, cap: u64) -> io::Result<&mut MmapMut> {
    let map = self.map_mut(cap)?;
    let _ = self.map.replace(map);
    Ok(self.map.as_mut().unwrap())
  }

  fn as_slice_ptr(&self) -> NonNull<[T]> {
    let slice = if let Some(map) = self.map.as_ref() {
      NonNull::from(&map[..]).cast::<T>()
    } else {
      debug_assert_eq!(0, self.len, "non-empty data without mapping");
      NonNull::dangling()
    };
    NonNull::slice_from_raw_parts(slice.cast(), self.len)
  }

  fn capacity(&self) -> Option<usize> {
    self.map.as_ref().map(|map| map.len() / size_of::<T>())
  }
}

use bytemuck::Pod;

impl<T: Pod> RawMem for FileMapped<T> {
  type Item = T;

  fn as_slice(&self) -> &[Self::Item] {
    unsafe { self.as_slice_ptr().as_ref() }
  }

  fn as_mut_slice(&mut self) -> &mut [Self::Item] {
    unsafe { self.as_slice_ptr().as_mut() }
  }

  fn grow(&mut self, addition: usize) -> Result<Page<'_, Self::Item>> {
    // grow from initialized part that means `len`
    let cap = self.len.checked_add(addition).ok_or(CapacityOverflow)?;

    // use layout to prevent all capacity bugs
    let layout = Layout::array::<T>(cap).map_err(|_| CapacityOverflow)?;

    // unmap the file by dropping
    let _ = self.map.take();

    let new = layout.size() as u64;
    let old = self.file.metadata()?.len();

    if new > old {
      self.file.set_len(new)?;
    }

    let ptr = NonNull::from(self.map_replace(new)?.as_mut());
    // SAFETY: provide valid lifetime inferred from inner `buf`
    let uninit = unsafe { slice::from_raw_parts_mut(ptr.cast().as_ptr(), cap) };
    Ok(Page { uninit: &mut uninit[self.len..], len: Some(&mut self.len) })
  }

  fn shrink(&mut self, shrink: usize) -> Result<()> {
    let Some(cap) = self.capacity() else {
      return Ok(());
    };
    let cap = cap.saturating_sub(shrink);

    let _ = self.map.take();
    // SAFETY: avoid checked mul because memory layout is valid
    //  then smaller layout will also be valid
    let new = unsafe { size_of::<T>().unchecked_mul(cap) as u64 };

    self.file.set_len(new)?;

    let _ = self.map_replace(new)?;
    self.len = cap;

    Ok(())
  }
}

impl<T> Drop for FileMapped<T> {
  fn drop(&mut self) {
    let _ = self.file.sync_all();
  }
}

impl<T> fmt::Debug for FileMapped<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_struct("FileMapped")
      .field("mmap", &self.map)
      .field("file", &self.file)
      .finish()
  }
}
