use std::sync::atomic::{AtomicBool, Ordering};

use dylib_reload_shared::ModuleId;

dylib_interface::include_exports!();
dylib_interface::include_imports!();

#[cfg(target_os = "linux")]
mod thread_locals;
#[cfg(target_os = "linux")]
mod thread_spawn_hook;

mod helpers;
mod exports_impl;
mod allocator;
use allocator::Allocator;
mod compilation_info;
mod panic_hook;

#[global_allocator]
static GLOBAL: Allocator = Allocator::new();

static ALLOCATOR_LOCK: AtomicBool = AtomicBool::new(false);
fn allocator_lock() -> bool {
  ALLOCATOR_LOCK.load(Ordering::SeqCst)
}

// SAFETY: will be initialized on one thread once and then never change
static mut MODULE_ID: ModuleId = 0;

// The id of the thread in which this module was loaded and in which it must be unloaded
//
// SAFETY: will be initialized on one thread once and then never change
pub static mut HOST_OWNER_THREAD: usize = 0;
