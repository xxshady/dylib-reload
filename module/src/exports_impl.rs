use std::{alloc::Layout, sync::atomic::Ordering};

use dylib_reload_shared::{
  exports::___Internal___Exports___ as Exports, Allocation, AllocatorPtr, ModuleId,
};
use crate::{
  allocator, gen_exports::ModuleExportsImpl, thread_locals, unloaded, EXIT_DEALLOCATION,
  HOST_OWNER_THREAD, MODULE_ID, UNLOADED,
};

impl Exports for ModuleExportsImpl {
  unsafe fn init(host_owner_thread: i64, module: ModuleId) {
    HOST_OWNER_THREAD = host_owner_thread;
    MODULE_ID = module;

    allocator::init();
  }

  unsafe fn exit(allocs: dylib_reload_shared::SliceAllocation) {
    let allocs = allocs.into_slice();

    EXIT_DEALLOCATION.store(true, Ordering::SeqCst);

    // TODO: use System allocator here and lock module allocator in take_cached_allocs_before_exit?
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

  fn unloaded() -> bool {
    unloaded()
  }

  fn take_cached_allocs_before_exit() {
    allocator::send_cached_allocs(None);
  }

  unsafe fn run_thread_local_dtors() {
    thread_locals::dtors::run();
  }
}
