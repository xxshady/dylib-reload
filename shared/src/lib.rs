use std::fmt::{Debug, Formatter, Result as FmtResult};

pub mod exports;
pub mod imports;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct AllocatorPtr(pub *mut u8);

// SAFETY: `*mut u8` won't be touched anywhere except in the dynamic library in the main thread for deallocation
unsafe impl Send for AllocatorPtr {}
unsafe impl Sync for AllocatorPtr {}

#[repr(C)]
#[derive(Clone, PartialEq)]
pub struct Allocation(pub AllocatorPtr, pub StableLayout);

impl Debug for Allocation {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let Self(AllocatorPtr(ptr), StableLayout { size, .. }) = self;
    write!(f, "({:?}, {:?})", ptr, size)
  }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct StableLayout {
  pub size: usize,
  pub align: usize,
}

#[repr(C)]
#[derive(Clone, PartialEq, Debug)]
pub enum AllocatorOp {
  Alloc(Allocation),
  Dealloc(Allocation),
}

pub type SliceAllocatorOp = RawSlice<AllocatorOp>;
pub type SliceAllocation = RawSlice<Allocation>;

/// FFI-safe `&[T]`
#[repr(C)]
pub struct RawSlice<T> {
  ptr: *const T,
  len: usize,
}

impl<T> RawSlice<T> {
  /// # Safety
  /// See `Safety` of [`std::slice::from_raw_parts`]
  pub unsafe fn into_slice<'a>(self) -> &'a [T] {
    std::slice::from_raw_parts(self.ptr, self.len)
  }
}

impl<T> From<&[T]> for RawSlice<T> {
  fn from(value: &[T]) -> Self {
    RawSlice {
      ptr: value.as_ptr(),
      len: value.len(),
    }
  }
}

/// FFI-safe `&str`
#[repr(C)]
pub struct Str(RawSlice<u8>);

impl Str {
  /// # Safety
  /// See `Safety` of [`std::slice::from_raw_parts`]
  pub unsafe fn into_str<'a>(self) -> &'a str {
    let bytes = self.0.into_slice();
    std::str::from_utf8(bytes).expect("Failed to get valid UTF-8 string slice back")
  }
}

impl From<&str> for Str {
  fn from(value: &str) -> Self {
    Self(value.as_bytes().into())
  }
}

pub type ModuleId = u64;
