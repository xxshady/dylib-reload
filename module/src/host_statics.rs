use std::sync::atomic::{AtomicBool, Ordering};

use shared::{
  callbacks::{OnAllocDealloc, OnCachedAllocs, Unrecoverable},
  ModuleId, SliceAllocatorOp, StableLayout,
};
use stabby::str::Str;

// SAFETY: all these statics will be initialized on one thread when
// this dynamic library is loaded and then never change

#[unsafe(no_mangle)]
pub static mut __ON_ALLOC: OnAllocDealloc = on_alloc_dealloc_placeholder;

#[unsafe(no_mangle)]
pub static mut __ON_DEALLOC: OnAllocDealloc = on_alloc_dealloc_placeholder;

#[unsafe(no_mangle)]
pub static mut __ON_CACHED_ALLOCS: OnCachedAllocs = on_cached_allocs_placeholder;

extern "C" fn on_cached_allocs_placeholder(_: ModuleId, _: SliceAllocatorOp) {
  unreachable!();
}

extern "C" fn on_alloc_dealloc_placeholder(_: ModuleId, _: *mut u8, _: StableLayout) {
  unreachable!()
}

#[unsafe(no_mangle)]
#[used]
pub static mut __UNRECOVERABLE: Unrecoverable = unrecoverable_placeholder;

extern "C" fn unrecoverable_placeholder(_: Str) -> ! {
  unreachable!();
}

/// The id of the thread in which this module was loaded and in which it must be unloaded
///
/// SAFETY: will be initialized on one thread once and then never change
#[unsafe(no_mangle)]
pub static mut __HOST_OWNER_THREAD: i64 = 0;

/// SAFETY: will be initialized on one thread once and then never change
#[unsafe(no_mangle)]
pub static mut __MODULE_ID: ModuleId = 0;

pub static UNLOADED: AtomicBool = AtomicBool::new(false);

#[stabby::export]
pub extern "C" fn __unloaded() -> bool {
  UNLOADED.load(Ordering::SeqCst)
}
