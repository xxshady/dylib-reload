use std::ffi::OsStr;

use libloading::Symbol;

use dylib_reload_shared::Str;

dylib_interface::include_exports!();
dylib_interface::include_imports!();
use gen_exports::ModuleExports as InternalModuleExports;
use gen_imports::init_imports;

mod errors;
mod module;
mod module_allocs;
mod helpers;
use helpers::{next_module_id, open_library};
mod imports_impl;
mod leak_library;
pub mod exports_types;
use exports_types::ModuleExportsForHost;

pub use crate::{errors::Error, module::Module};

pub fn load_module<E: ModuleExportsForHost>(
  path: impl AsRef<OsStr>,
) -> Result<Module<E>, crate::Error> {
  let library = open_library(&path)?;

  let compiled_with = unsafe {
    let compiled_with: Symbol<*const Str> = library.get(b"__CRATE_COMPILATION_INFO__\0")?;
    let compiled_with: &Str = &**compiled_with;
    compiled_with.to_string()
  };

  let expected = crate_compilation_info::get!();
  if compiled_with != expected {
    return Err(Error::ModuleCompilationMismatch(
      compiled_with,
      expected.to_owned(),
    ));
  }

  init_imports(&library);

  let module_id = next_module_id();

  module_allocs::add_module(module_id);

  let internal_exports = InternalModuleExports::new(&library);
  unsafe {
    internal_exports.init(thread_id::get(), module_id);
  }

  let pub_exports = E::new(&library);
  let module = Module::new(
    module_id,
    library,
    internal_exports,
    pub_exports,
    path.as_ref().into(),
  );
  Ok(module)
}

// TODO: fix it
#[cfg(target_os = "windows")]
#[expect(clippy::missing_safety_doc)]
pub unsafe fn __suppress_unused_warning_for_linux_only_exports(exports: InternalModuleExports) {
  exports.spawned_threads_count();
}
