use std::{
  alloc::{GlobalAlloc, Layout, System},
  sync::atomic::Ordering,
};

use dylib_reload_shared::{
  exports::___Internal___Exports___ as Exports, Allocation, AllocatorPtr, ModuleId,
};
use crate::{
  allocator, gen_exports::ModuleExportsImpl, helpers::is_it_host_owner_thread, panic_hook,
  ALLOCATOR_LOCK, HOST_OWNER_THREAD, MODULE_ID,
};

impl Exports for ModuleExportsImpl {
  fn init(host_owner_thread: usize, module: ModuleId) {
    unsafe {
      HOST_OWNER_THREAD = host_owner_thread;
      MODULE_ID = module;

      allocator::init();
    }

    panic_hook::init();

    dbg!(host_owner_thread, is_it_host_owner_thread());
  }

  fn exit(allocs: dylib_reload_shared::SliceAllocation) {
    let allocs = unsafe { allocs.into_slice() };
    let system = System;

    for Allocation(AllocatorPtr(ptr), layout, ..) in allocs {
      unsafe {
        system.dealloc(
          *ptr,
          Layout::from_size_align(layout.size, layout.align).unwrap(),
        );
      }
    }
  }

  fn take_cached_allocs_before_exit() {
    allocator::send_cached_allocs(None);
  }

  fn lock_module_allocator() {
    ALLOCATOR_LOCK.store(true, Ordering::SeqCst);
  }

  fn run_thread_local_dtors() {
    #[cfg(target_os = "linux")]
    {
      use crate::thread_locals;
      unsafe {
        thread_locals::dtors::run();
      }
    }
  }

  fn spawned_threads_count() -> u64 {
    #[cfg(target_os = "linux")]
    {
      crate::thread_spawn_hook::spawned_threads_count()
    }
    #[cfg(target_os = "windows")]
    {
      Default::default()
    }
  }
}
