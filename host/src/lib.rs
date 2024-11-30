use std::ffi::OsStr;

use gen_exports::ModuleExports;
use gen_imports::init_imports;
use helpers::{next_module_id, open_library, unrecoverable};
use dylib_reload_shared::ModuleId;
use stabby::str::Str;

mod error;
mod module;
mod module_allocs;
mod helpers;
mod gen_exports;
mod gen_imports;
mod imports_impl;

pub use crate::{error::Error, module::Module};

pub unsafe fn load_module(path: impl AsRef<OsStr>) -> Result<Module, crate::Error> {
  let library = open_library(&path)?;

  dbg!();
  init_imports(&library);
  dbg!();

  let owner_thread = unsafe {
    // I could use `std::thread::current().id()`
    // but I'm not sure how safe it is for FFI + it needs to be stored in a static
    // since it's an opaque object and as_u64() is unstable
    libc::syscall(libc::SYS_gettid)
  };

  let module_id = next_module_id();

  module_allocs::add_module(module_id);

  let exports = ModuleExports::new(&library);

  dbg!();
  exports.init(owner_thread, module_id);
  dbg!();

  let module = Module::new(module_id, library, exports, path.as_ref().into());

  Ok(module)
}
