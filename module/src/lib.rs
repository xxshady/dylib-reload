use std::{
  alloc::Layout,
  sync::atomic::{AtomicBool, Ordering},
};

use dylib_reload_shared::{Allocation, AllocatorPtr, ModuleId, SliceAllocation};

dylib_interface::include_generated!(gen_exports, "/generated_module_exports.rs");
dylib_interface::include_generated!(gen_imports, "/generated_module_imports.rs");

mod thread_locals;
mod helpers;
mod exports_impl;
mod allocator;
use allocator::Allocator;

#[global_allocator]
static GLOBAL: Allocator = Allocator::new();

static EXIT_DEALLOCATION: AtomicBool = AtomicBool::new(false);
fn exit_deallocation() -> bool {
  EXIT_DEALLOCATION.load(Ordering::SeqCst)
}

static UNLOADED: AtomicBool = AtomicBool::new(false);
fn unloaded() -> bool {
  UNLOADED.load(Ordering::SeqCst)
}

/// The id of the thread in which this module was loaded and in which it must be unloaded
///
/// SAFETY: will be initialized on one thread once and then never change
static mut HOST_OWNER_THREAD: i64 = 0;

/// SAFETY: will be initialized on one thread once and then never change
static mut MODULE_ID: ModuleId = 0;
