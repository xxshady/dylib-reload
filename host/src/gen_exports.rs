// This file is generated, DO NOT edit manually
// ---------------------------------------------

use dylib_reload_shared::{ModuleId, SliceAllocation};
pub struct ModuleExports {
    init: unsafe extern "C" fn(host_owner_thread: i64, module: ModuleId),
    exit: unsafe extern "C" fn(allocs: SliceAllocation),
    unloaded: extern "C" fn() -> bool,
    request_cached_allocs: extern "C" fn(),
    run_thread_local_dtors: unsafe extern "C" fn(),
}
impl ModuleExports {
    pub unsafe fn new(library: &libloading::Library) -> Self {
        Self {
            init: *library
                .get(concat!("_____Internal___Exports____init", "\0").as_bytes())
                .expect(
                    "Failed to get \"init\" fn symbol from module (mangled name: \"_____Internal___Exports____init\")",
                ),
            exit: *library
                .get(concat!("_____Internal___Exports____exit", "\0").as_bytes())
                .expect(
                    "Failed to get \"exit\" fn symbol from module (mangled name: \"_____Internal___Exports____exit\")",
                ),
            unloaded: *library
                .get(concat!("_____Internal___Exports____unloaded", "\0").as_bytes())
                .expect(
                    "Failed to get \"unloaded\" fn symbol from module (mangled name: \"_____Internal___Exports____unloaded\")",
                ),
            request_cached_allocs: *library
                .get(
                    concat!("_____Internal___Exports____request_cached_allocs", "\0")
                        .as_bytes(),
                )
                .expect(
                    "Failed to get \"request_cached_allocs\" fn symbol from module (mangled name: \"_____Internal___Exports____request_cached_allocs\")",
                ),
            run_thread_local_dtors: *library
                .get(
                    concat!("_____Internal___Exports____run_thread_local_dtors", "\0")
                        .as_bytes(),
                )
                .expect(
                    "Failed to get \"run_thread_local_dtors\" fn symbol from module (mangled name: \"_____Internal___Exports____run_thread_local_dtors\")",
                ),
        }
    }
    pub unsafe fn init(&self, host_owner_thread: i64, module: ModuleId) {
        (self.init)(host_owner_thread, module)
    }
    pub unsafe fn exit(&self, allocs: SliceAllocation) {
        (self.exit)(allocs)
    }
    pub fn unloaded(&self) -> bool {
        (self.unloaded)()
    }
    pub fn request_cached_allocs(&self) {
        (self.request_cached_allocs)()
    }
    pub unsafe fn run_thread_local_dtors(&self) {
        (self.run_thread_local_dtors)()
    }
}
