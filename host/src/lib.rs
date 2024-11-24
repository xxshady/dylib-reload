use std::ffi::OsStr;

use libc::{RTLD_DEEPBIND, RTLD_LAZY, RTLD_LOCAL};

mod error;
mod module;
mod module_allocs;
mod helpers;

pub use crate::{error::Error, module::Module};

pub unsafe fn load_module(path: impl AsRef<OsStr>) -> Result<Module, crate::Error> {
  let library = open_library(path)?;
  let module = Module::new(library);

  // I could use `std::thread::current().id()`
  // but I'm not sure how safe it is for FFI (+ it needs to be stored in a static)
  // since it's an opaque object and as_u64() is unstable
  let host_thread_id = libc::syscall(libc::SYS_gettid);

  Ok(module)
}

unsafe fn open_library(path: impl AsRef<OsStr>) -> Result<libloading::Library, crate::Error> {
  // RTLD_DEEPBIND allows replacing __cxa_thread_atexit_impl (it's needed to call destructors of thread-locals)
  // only for dynamic library without replacing it for the whole executable
  const FLAGS: i32 = RTLD_LAZY | RTLD_LOCAL | RTLD_DEEPBIND;

  use libloading::os::unix::Library;
  let library = Library::open(Some(path), FLAGS)?.into();
  Ok(library)
}
