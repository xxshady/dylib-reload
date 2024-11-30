// This file is generated, DO NOT edit manually
// ---------------------------------------------

use stabby::str::Str;
use dylib_reload_shared::{ModuleId, SliceAllocatorOp, StableLayout};
use dylib_reload_shared::imports::___Internal___Imports___ as Imports;
/// Struct for implementing your `Imports` trait
pub struct ModuleImportsImpl;
pub fn init_imports(library: &libloading::Library) {
  unsafe {
    dbg!("init why");
    let ptr_to_static: *mut _ = *library
      .get(concat!("waaaaaaaaaaaaa", "\0").as_bytes())
      .expect("Failed to get \"on_alloc\" fn symbol from module (mangled name: \"_\")");
    *ptr_to_static = impl_;
    extern "C" fn impl_(module: ModuleId, ptr: *mut u8, layout: StableLayout) {
      <ModuleImportsImpl as Imports>::on_alloc(module, ptr, layout)
    }
  }
  unsafe {
    let ptr_to_static: *mut _ = *library
            .get(concat!("_____Internal___Imports____on_dealloc", "\0").as_bytes())
            .expect(
                "Failed to get \"on_dealloc\" fn symbol from module (mangled name: \"_____Internal___Imports____on_dealloc\")",
            );
    *ptr_to_static = impl_;
    extern "C" fn impl_(module: ModuleId, ptr: *mut u8, layout: StableLayout) {
      <ModuleImportsImpl as Imports>::on_dealloc(module, ptr, layout)
    }
  }
  unsafe {
    let ptr_to_static: *mut _ = *library
            .get(concat!("_____Internal___Imports____on_cached_allocs", "\0").as_bytes())
            .expect(
                "Failed to get \"on_cached_allocs\" fn symbol from module (mangled name: \"_____Internal___Imports____on_cached_allocs\")",
            );
    *ptr_to_static = impl_;
    extern "C" fn impl_(module: ModuleId, ops: SliceAllocatorOp) {
      <ModuleImportsImpl as Imports>::on_cached_allocs(module, ops)
    }
  }
  unsafe {
    let ptr_to_static: *mut _ = *library
            .get(concat!("_____Internal___Imports____unrecoverable", "\0").as_bytes())
            .expect(
                "Failed to get \"unrecoverable\" fn symbol from module (mangled name: \"_____Internal___Imports____unrecoverable\")",
            );
    *ptr_to_static = impl_;
    extern "C" fn impl_(message: Str) -> ! {
      <ModuleImportsImpl as Imports>::unrecoverable(message)
    }
  }
}
