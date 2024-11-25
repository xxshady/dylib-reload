use std::ffi::OsStr;

use helpers::{cstr_bytes, get_stabbied_fn, next_module_id, open_library, unrecoverable};
use shared::{
  callbacks::{Init, Unrecoverable},
  ModuleId,
};
use stabby::str::Str;

mod error;
mod module;
mod module_allocs;
mod helpers;

pub use crate::{error::Error, module::Module};

pub unsafe fn load_module(path: impl AsRef<OsStr>) -> Result<Module, crate::Error> {
  let library = open_library(&path)?;

  let module_id = next_module_id();

  // TODO: use get_stabbied?
  unsafe {
    let unrecoverable_ptr: *mut Unrecoverable = *library
      .get(&cstr_bytes("__UNRECOVERABLE"))
      .unwrap_or_else(|e| {
        // TODO: crate name
        panic!(
          "Failed to get __UNRECOVERABLE from module, reason: {e}\n\
          note: consider adding `use <crate name>;` at the top of your lib.rs"
        );
      });

    *unrecoverable_ptr = unrecoverable_impl;

    extern "C" fn unrecoverable_impl(message: Str) -> ! {
      unrecoverable(&format!("{} (from module)", message));
    }

    // I could use `std::thread::current().id()`
    // but I'm not sure how safe it is for FFI (+ it needs to be stored in a static)
    // since it's an opaque object and as_u64() is unstable
    let owner_thread = libc::syscall(libc::SYS_gettid);

    let owner_thread_ptr: *mut i64 = *library
      .get(&cstr_bytes("__HOST_OWNER_THREAD"))
      .expect("Failed to get __HOST_OWNER_THREAD from module");
    *owner_thread_ptr = owner_thread;

    let module_id_ptr: *mut ModuleId = *library
      .get(&cstr_bytes("__MODULE_ID"))
      .expect("Failed to get __MODULE_ID from module");
    *module_id_ptr = module_id;
  }

  let init: Init = get_stabbied_fn(&library, "__init");

  let module = Module::new(module_id, library, (&path).into());
  module_allocs::add_module(&module);

  init();

  Ok(module)
}
