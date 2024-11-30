// This file is generated, DO NOT edit manually
// ---------------------------------------------

#[allow(non_upper_case_globals)]
#[unsafe(no_mangle)]
static mut waaaaaaaaaaaaa: extern "C" fn(module: ModuleId, ptr: *mut u8, layout: StableLayout) =
  placeholder____;
extern "C" fn placeholder____(module: ModuleId, ptr: *mut u8, layout: StableLayout) {
  unreachable!();
}

use stabby::str::Str;
use dylib_reload_shared::{ModuleId, SliceAllocatorOp, StableLayout};

pub fn on_alloc(module: ModuleId, ptr: *mut u8, layout: StableLayout) {
  unsafe { waaaaaaaaaaaaa(module, ptr, layout) }
}
pub fn on_dealloc(module: ModuleId, ptr: *mut u8, layout: StableLayout) {
  #[allow(non_upper_case_globals)]
  #[unsafe(no_mangle)]
  static mut _____Internal___Imports____on_dealloc: extern "C" fn(
    module: ModuleId,
    ptr: *mut u8,
    layout: StableLayout,
  ) = placeholder;
  extern "C" fn placeholder(module: ModuleId, ptr: *mut u8, layout: StableLayout) {
    unreachable!();
  }
  unsafe { _____Internal___Imports____on_dealloc(module, ptr, layout) }
}
pub fn on_cached_allocs(module: ModuleId, ops: SliceAllocatorOp) {
  #[allow(non_upper_case_globals)]
  #[unsafe(no_mangle)]
  static mut _____Internal___Imports____on_cached_allocs: extern "C" fn(
    module: ModuleId,
    ops: SliceAllocatorOp,
  ) = placeholder;
  extern "C" fn placeholder(module: ModuleId, ops: SliceAllocatorOp) {
    unreachable!();
  }
  unsafe { _____Internal___Imports____on_cached_allocs(module, ops) }
}
pub fn unrecoverable(message: Str) -> ! {
  #[allow(non_upper_case_globals)]
  #[unsafe(no_mangle)]
  static mut _____Internal___Imports____unrecoverable: extern "C" fn(message: Str) -> ! =
    placeholder;
  extern "C" fn placeholder(message: Str) -> ! {
    unreachable!();
  }
  unsafe { _____Internal___Imports____unrecoverable(message) }
}
