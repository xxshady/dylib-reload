use std::{
  cell::Cell,
  sync::atomic::{AtomicBool, Ordering},
};

use dylib_reload_shared::ModuleId;

dylib_interface::include_generated!(gen_exports, "/generated_module_exports.rs");
dylib_interface::include_generated!(gen_imports, "/generated_module_imports.rs");

mod thread_locals;
mod helpers;
mod exports_impl;
mod allocator;
use allocator::Allocator;
mod thread_spawn_hook;
mod compilation_info;
mod panic_hook;

#[global_allocator]
static GLOBAL: Allocator = Allocator::new();

static ALLOCATOR_LOCK: AtomicBool = AtomicBool::new(false);
fn allocator_lock() -> bool {
  ALLOCATOR_LOCK.load(Ordering::SeqCst)
}

/// The id of the thread in which this module was loaded and in which it must be unloaded
///
/// SAFETY: will be initialized on one thread once and then never change
static mut HOST_OWNER_THREAD: i64 = 0;

/// SAFETY: will be initialized on one thread once and then never change
static mut MODULE_ID: ModuleId = 0;

thread_local! {
  static IS_IT_HOST_OWNER_THREAD: Cell<bool> = const { Cell::new(false) };
}
