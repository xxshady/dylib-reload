use std::{
  cell::Cell,
  marker::PhantomData,
  path::PathBuf,
  sync::atomic::{AtomicU64, Ordering},
  thread::{self, ThreadId},
};

use shared::{
  callbacks::{Exit, RunThreadLocalDtors, Unloaded, Unrecoverable},
  ModuleId,
};
use stabby::{libloading::StabbyLibrary, str::Str, IStable};

use crate::{
  helpers::{cstr_bytes, get_stabbied_fn, next_module_id, open_library, unrecoverable},
  module_allocs,
};

#[derive(Debug)]
pub struct Module {
  pub id: ModuleId,

  /// it's Option because of Drop impl
  pub(crate) library: Option<libloading::Library>,
  library_path: PathBuf,

  run_thread_local_dtors: RunThreadLocalDtors,
  pub(crate) request_cached_allocs: extern "C" fn(),
  pub(crate) exit: Exit,

  /// Module must be loaded and unloaded from the same thread
  _not_thread_safe: PhantomData<*const ()>,
}

impl Module {
  pub(crate) fn new(id: ModuleId, library: libloading::Library, library_path: PathBuf) -> Self {
    Self {
      id,
      exit: unsafe { get_stabbied_fn(&library, "__exit") },
      request_cached_allocs: unsafe { get_stabbied_fn(&library, "__request_cached_allocs") },
      run_thread_local_dtors: unsafe { get_stabbied_fn(&library, "__run_thread_local_dtors") },
      library: Some(library),
      library_path,

      _not_thread_safe: PhantomData,
    }
  }

  pub unsafe fn main<R: IStable>(&self) -> R {
    let library = self.library.as_ref().unwrap_or_else(|| unreachable!());

    let main: extern "C" fn() -> R = unsafe { get_stabbied_fn(library, "main") };
    main()
  }
}

impl Drop for Module {
  fn drop(&mut self) {
    unsafe {
      (self.run_thread_local_dtors)();
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

      let unloaded: Unloaded = get_stabbied_fn(&library, "__unloaded");

      if unloaded() {
        panic!(
          "Failed to unload module\n\
          note: before unloading the module, make sure that all threads are joined if any were spawned by it"
        );
      }
    }
  }
}
