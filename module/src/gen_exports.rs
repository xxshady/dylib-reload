// This file is generated, DO NOT edit manually
// ---------------------------------------------

use dylib_reload_shared::{ModuleId, SliceAllocation};
use dylib_reload_shared::exports::___Internal___Exports___ as Exports;
/// Struct for implementing your `Exports` trait
pub struct ModuleExportsImpl;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _____Internal___Exports____init(
    host_owner_thread: i64,
    module: ModuleId,
) {
    <ModuleExportsImpl as Exports>::init(host_owner_thread, module)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _____Internal___Exports____exit(allocs: SliceAllocation) {
    <ModuleExportsImpl as Exports>::exit(allocs)
}
#[unsafe(no_mangle)]
pub extern "C" fn _____Internal___Exports____unloaded() -> bool {
    <ModuleExportsImpl as Exports>::unloaded()
}
#[unsafe(no_mangle)]
pub extern "C" fn _____Internal___Exports____request_cached_allocs() {
    <ModuleExportsImpl as Exports>::request_cached_allocs()
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _____Internal___Exports____run_thread_local_dtors() {
    <ModuleExportsImpl as Exports>::run_thread_local_dtors()
}
