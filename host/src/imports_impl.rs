use dylib_reload_shared::{
  imports::___Internal___Imports___ as Imports, ModuleId, SliceAllocatorOp, StableLayout, Str,
};
use crate::{gen_imports::ModuleImportsImpl, helpers, module_allocs};

impl Imports for ModuleImportsImpl {
  fn on_alloc(module: ModuleId, ptr: *mut u8, layout: StableLayout) {
    module_allocs::on_alloc(module, ptr, layout);
  }

  fn on_dealloc(module: ModuleId, ptr: *mut u8, layout: StableLayout) {
    module_allocs::on_dealloc(module, ptr, layout);
  }

  fn on_cached_allocs(module: ModuleId, ops: SliceAllocatorOp) {
    module_allocs::on_cached_allocs(module, ops);
  }

  fn unrecoverable(message: Str) -> ! {
    let message = unsafe { message.into_str() };
    helpers::unrecoverable(&format!("{message} (from module)"));
  }
}
