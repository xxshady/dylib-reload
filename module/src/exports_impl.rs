use std::{
  alloc::{GlobalAlloc, Layout, System},
  sync::atomic::Ordering,
};

use dylib_reload_shared::{
  exports::___Internal___Exports___ as Exports, Allocation, AllocatorPtr, ModuleId,
};
use crate::{
  allocator, gen_exports::ModuleExportsImpl, thread_locals, thread_spawn_hook, ALLOCATOR_LOCK,
  HOST_OWNER_THREAD, IS_IT_HOST_OWNER_THREAD, MODULE_ID,
};

impl Exports for ModuleExportsImpl {
  unsafe fn init(host_owner_thread: i64, module: ModuleId) {
    HOST_OWNER_THREAD = host_owner_thread;
    MODULE_ID = module;

    allocator::init();
  }

  unsafe fn exit(allocs: dylib_reload_shared::SliceAllocation) {
    let allocs = allocs.into_slice();
    let system = System;

    for Allocation(AllocatorPtr(ptr), layout, ..) in allocs {
      system.dealloc(
        *ptr,
        Layout::from_size_align(layout.size, layout.align).unwrap(),
      );
    }
  }

  fn take_cached_allocs_before_exit() {
    allocator::send_cached_allocs(None);
  }

  unsafe fn run_thread_local_dtors() {
    IS_IT_HOST_OWNER_THREAD.set(true);
    thread_locals::dtors::run();
  }

  fn lock_module_allocator() {
    ALLOCATOR_LOCK.store(true, Ordering::SeqCst);
  }

  fn spawned_threads_count() -> u64 {
    thread_spawn_hook::spawned_threads_count()
  }
}
