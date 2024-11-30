use std::{
  cell::Cell,
  fmt::Debug,
  marker::PhantomData,
  path::PathBuf,
  sync::atomic::{AtomicU64, Ordering},
  thread::{self, ThreadId},
};

use dylib_reload_shared::ModuleId;

use crate::{
  gen_exports::ModuleExports,
  helpers::{next_module_id, open_library, unrecoverable},
  module_allocs,
};

pub struct Module {
  pub id: ModuleId,

  /// it's Option because of Drop impl
  pub(crate) library: Option<libloading::Library>,
  library_path: PathBuf,

  pub(crate) exports: ModuleExports,

  /// Module must be loaded and unloaded from the same thread
  _not_thread_safe: PhantomData<*const ()>,
}

impl Module {
  pub(crate) fn new(
    id: ModuleId,
    library: libloading::Library,
    exports: ModuleExports,
    library_path: PathBuf,
  ) -> Self {
    Self {
      id,
      library: Some(library),
      library_path,
      exports,
      _not_thread_safe: PhantomData,
    }
  }

  pub unsafe fn main<R>(&self) -> R {
    let library = self.library.as_ref().unwrap_or_else(|| unreachable!());

    let main_fn: extern "C" fn() -> R = *library.get(b"main\0").unwrap_or_else(|e| {
      panic!("Failed to get main fn from module, reason: {e:#}");
    });

    main_fn()
  }
}

impl Drop for Module {
  fn drop(&mut self) {
    unsafe {
      self.exports.run_thread_local_dtors();
    }

    module_allocs::remove_module(self);

    let library = self.library.take().unwrap_or_else(|| unreachable!());

    library.close().unwrap_or_else(|e| {
      panic!("Failed to unloaded module library, reason: {e}");
    });

    unsafe {
      let library = open_library(&self.library_path).unwrap_or_else(|e| {
        panic!("Failed to load module library, reason: {e}");
      });

      let exports = ModuleExports::new(&library);
      if exports.unloaded() {
        panic!(
          "Failed to unload module\n\
          note: before unloading the module, make sure that all threads are joined if any were spawned by it"
        );
      }
    }
  }
}

impl Debug for Module {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let id = self.id;
    write!(f, "Module {{ id: {id} }}")
  }
}
