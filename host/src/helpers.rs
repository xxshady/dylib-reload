use std::{
  ffi::OsStr,
  fs::File,
  io::{BufRead, BufReader},
  path::Path,
  sync::atomic::{AtomicU64, Ordering},
};

use libc::{RTLD_DEEPBIND, RTLD_LAZY, RTLD_LOCAL};
use dylib_reload_shared::ModuleId;
use libloading::{Library, Symbol};

pub fn unrecoverable(message: &str) -> ! {
  let message = format!("something unrecoverable happened: {message}");
  unrecoverable_impl(&message);
}

pub fn unrecoverable_with_prefix(message: &str, prefix: &str) -> ! {
  let message = format!("[{prefix}] something unrecoverable happened: {message}");
  unrecoverable_impl(&message);
}

fn unrecoverable_impl(message: &str) -> ! {
  eprintln!("{message}");
  eprintln!("aborting");
  std::process::abort();
}

pub fn cstr_bytes(str: &str) -> Vec<u8> {
  [str.as_bytes(), &[0]].concat()
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

pub unsafe fn get_library_export<'lib, F>(
  library: &'lib Library,
  name: &str,
) -> Result<Symbol<'lib, F>, libloading::Error> {
  let fn_ = library.get(&cstr_bytes(name))?;
  Ok(fn_)
}

#[cfg(target_os = "linux")]
pub fn is_library_loaded(library_path: &Path) -> bool {
  let library_path = library_path
    .to_str()
    .expect("library path must be UTF-8 string");

  let file = File::open("/proc/self/maps").expect("Failed to open /proc/self/maps");
  let reader = BufReader::new(file);

  reader.lines().any(|line_result| {
    if let Ok(line) = line_result {
      line.contains(library_path)
    } else {
      false
    }
  })
}
