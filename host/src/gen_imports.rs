// This file is generated, DO NOT edit manually
// ---------------------------------------------

use stabby::str::Str;
use dylib_reload_shared::{ModuleId, SliceAllocatorOp, StableLayout};
use dylib_reload_shared::imports::___Internal___Imports___ as Imports;
/// Struct for implementing your `Imports` trait
pub struct ModuleImportsImpl;
pub fn init_imports(library: &libloading::Library) {
    unsafe {
        let ptr: *mut extern "C" fn(
            module: ModuleId,
            ptr: *mut u8,
            layout: StableLayout,
        ) = *library
            .get(concat!("_____Internal___Imports____on_alloc", "\0").as_bytes())
            .expect(
                "Failed to get \"_____Internal___Imports____on_alloc\" symbol of static function pointer from module",
            );
        *ptr = impl_;
        extern "C" fn impl_(module: ModuleId, ptr: *mut u8, layout: StableLayout) {
            <ModuleImportsImpl as Imports>::on_alloc(module, ptr, layout)
        }
    }
    unsafe {
        let ptr: *mut extern "C" fn(
            module: ModuleId,
            ptr: *mut u8,
            layout: StableLayout,
        ) = *library
            .get(concat!("_____Internal___Imports____on_dealloc", "\0").as_bytes())
            .expect(
                "Failed to get \"_____Internal___Imports____on_dealloc\" symbol of static function pointer from module",
            );
        *ptr = impl_;
        extern "C" fn impl_(module: ModuleId, ptr: *mut u8, layout: StableLayout) {
            <ModuleImportsImpl as Imports>::on_dealloc(module, ptr, layout)
        }
    }
    unsafe {
        let ptr: *mut extern "C" fn(module: ModuleId, ops: SliceAllocatorOp) = *library
            .get(concat!("_____Internal___Imports____on_cached_allocs", "\0").as_bytes())
            .expect(
                "Failed to get \"_____Internal___Imports____on_cached_allocs\" symbol of static function pointer from module",
            );
        *ptr = impl_;
        extern "C" fn impl_(module: ModuleId, ops: SliceAllocatorOp) {
            <ModuleImportsImpl as Imports>::on_cached_allocs(module, ops)
        }
    }
    unsafe {
        let ptr: *mut extern "C" fn(message: Str) -> ! = *library
            .get(concat!("_____Internal___Imports____unrecoverable", "\0").as_bytes())
            .expect(
                "Failed to get \"_____Internal___Imports____unrecoverable\" symbol of static function pointer from module",
            );
        *ptr = impl_;
        extern "C" fn impl_(message: Str) -> ! {
            <ModuleImportsImpl as Imports>::unrecoverable(message)
        }
    }
}
