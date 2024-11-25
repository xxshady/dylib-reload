use std::fmt::{Debug, Formatter, Result as FmtResult};

use stabby::{stabby, str::Str};

#[stabby]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct AllocatorPtr(pub *mut u8);

// SAFETY: `*mut u8` won't be touched anywhere except in the dynamic library in the main thread for deallocation
unsafe impl Send for AllocatorPtr {}
unsafe impl Sync for AllocatorPtr {}

#[stabby]
#[derive(Clone, PartialEq)]
pub struct Allocation(pub AllocatorPtr, pub StableLayout);

impl Debug for Allocation {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let Self(AllocatorPtr(ptr), StableLayout { size, .. }) = self;
    write!(f, "({:?}, {:?})", ptr, size)
  }
}

// TODO: use Layout from stabby?
#[stabby]
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

// had to make my own type because stabby's one didn't work with get_stabbied
// https://github.com/ZettaScaleLabs/stabby/issues/95
#[stabby]
pub struct RawSlice<T> {
  ptr: *const T,
  len: usize,
}

impl<T> RawSlice<T> {
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

pub mod callbacks {
  use super::*;
  pub type Unrecoverable = extern "C" fn(Str) -> !;
  pub type OnCachedAllocs = extern "C" fn(ModuleId, SliceAllocatorOp);
  pub type OnAllocDealloc = extern "C" fn(ModuleId, *mut u8, StableLayout);
  pub type Unloaded = extern "C" fn() -> bool;
  pub type RunThreadLocalDtors = unsafe extern "C" fn();
  pub type Exit = extern "C" fn(SliceAllocation);
  pub type Init = unsafe extern "C" fn();
}

pub type ModuleId = u64;
