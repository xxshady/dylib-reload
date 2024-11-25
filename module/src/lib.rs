use std::{
  alloc::Layout,
  sync::atomic::{AtomicBool, Ordering},
};

use host_statics::UNLOADED;
use shared::{Allocation, AllocatorPtr, SliceAllocation};

mod host_statics;
mod thread_locals;
mod allocator;
mod helpers;

static EXIT_DEALLOCATION: AtomicBool = AtomicBool::new(false);

#[stabby::export]
pub unsafe extern "C" fn __init() {
  allocator::init();
}

#[stabby::export]
pub extern "C" fn __exit(allocs: SliceAllocation) {
  let allocs = unsafe { allocs.into_slice() };

  EXIT_DEALLOCATION.store(true, Ordering::SeqCst);

  for Allocation(AllocatorPtr(ptr), layout, ..) in allocs {
    unsafe {
      std::alloc::dealloc(
        *ptr,
        Layout::from_size_align(layout.size, layout.align).unwrap(),
      );
    }
  }

  UNLOADED.store(true, Ordering::SeqCst);
}
