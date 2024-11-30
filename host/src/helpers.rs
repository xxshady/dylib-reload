use std::{
  ffi::{CString, OsStr},
  sync::atomic::{AtomicU64, Ordering},
};

use libc::{RTLD_DEEPBIND, RTLD_LAZY, RTLD_LOCAL};
use dylib_reload_shared::ModuleId;

pub fn unrecoverable(message: &str) -> ! {
  eprintln!("something unrecoverable happened: {message}");
  eprintln!("aborting");
  std::process::abort();
}

pub fn next_module_id() -> ModuleId {
  // module ids start from 1
  static ID_COUNTER: AtomicU64 = AtomicU64::new(1);

  let id = ID_COUNTER.fetch_add(1, Ordering::SeqCst);
  assert_ne!(id, 0, "this must never happen (integer overflow)");
  id
}

pub unsafe fn open_library(path: impl AsRef<OsStr>) -> Result<libloading::Library, crate::Error> {
  // RTLD_DEEPBIND allows replacing __cxa_thread_atexit_impl (it's needed to call destructors of thread-locals)
  // only for dynamic library without replacing it for the whole executable
  const FLAGS: i32 = RTLD_LAZY | RTLD_LOCAL | RTLD_DEEPBIND;

  use libloading::os::unix::Library;
  let library = Library::open(Some(path), FLAGS)?.into();
  Ok(library)
}
