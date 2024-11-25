use std::{
  ffi::{CString, OsStr},
  sync::atomic::{AtomicU64, Ordering},
};

use libc::{RTLD_DEEPBIND, RTLD_LAZY, RTLD_LOCAL};
use shared::ModuleId;
use stabby::{libloading::StabbyLibrary, IStable};

pub fn unrecoverable(message: &str) -> ! {
  eprintln!("something unrecoverable happened: {message}");
  eprintln!("aborting");
  std::process::abort();
}

pub fn cstr_bytes(str: &str) -> Vec<u8> {
  [str.as_bytes(), &[0]].concat()
}

pub unsafe fn get_stabbied_fn<F>(library: &impl StabbyLibrary, name: &str) -> F
where
  F: IStable + Copy,
{
  let symbol = library.get_stabbied(name.as_bytes()).unwrap_or_else(|e| {
    panic!("Failed to get {name} symbol from module, reason: {e:#?}");
  });
  *symbol
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
