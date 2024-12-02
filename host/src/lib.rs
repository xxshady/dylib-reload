use std::ffi::OsStr;

dylib_interface::include_generated!(gen_exports, "/generated_module_exports.rs");
use gen_exports::ModuleExports;

dylib_interface::include_generated!(gen_imports, "/generated_module_imports.rs");
use gen_imports::init_imports;

mod error;
mod module;
mod module_allocs;
mod helpers;
use helpers::{next_module_id, open_library};
mod imports_impl;

pub use crate::{error::Error, module::Module};

pub unsafe fn load_module(path: impl AsRef<OsStr>) -> Result<Module, crate::Error> {
  let library = open_library(&path)?;
  init_imports(&library);

  let owner_thread = unsafe {
    // I could use `std::thread::current().id()`
    // but I'm not sure how safe it is for FFI + it needs to be stored in a static
    // since it's an opaque object and as_u64() is unstable
    libc::syscall(libc::SYS_gettid)
  };
  let module_id = next_module_id();

  module_allocs::add_module(module_id);

  let exports = ModuleExports::new(&library);
  exports.init(owner_thread, module_id);

  let module = Module::new(module_id, library, exports, path.as_ref().into());
  Ok(module)
}
