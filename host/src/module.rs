use std::{
  fmt::Debug,
  marker::PhantomData,
  path::{Path, PathBuf},
};

use dylib_reload_shared::ModuleId;
use libloading::Symbol;

use crate::{
  gen_exports::ModuleExports,
  helpers::{get_library_export, is_library_loaded, open_library},
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

    let main_fn = get_library_export(library, "main").unwrap_or_else(|e| {
      panic!("Failed to get main fn from module, reason: {e:#}");
    });
    let main_fn: Symbol<extern "C" fn() -> R> = main_fn;

    main_fn()
  }
}

impl Drop for Module {
  fn drop(&mut self) {
    // TEST
    println!("----------- unloading library");

    let library = self.library.take().unwrap_or_else(|| unreachable!());

    unsafe {
      let before_unload = get_library_export(&library, "before_unload");
      if let Ok(before_unload) = before_unload {
        println!(r#"Module exported "before_unload" function, calling it before unloading"#);

        let before_unload: extern "C" fn() = *before_unload;
        before_unload();
      } else {
        println!(
          r#"Module did not export "before_unload" function, unloading library without calling it"#
        );
      }
    }

    if self.exports.spawned_threads_count() > 0 {
      panic!(
        "Cannot unload module with running threads\n\
        note: module can export \"before_unload\" function to join spawned threads"
      );
    }

    dbg!();
    self.exports.lock_module_allocator();

    dbg!();
    unsafe {
      self.exports.run_thread_local_dtors();
    }

    dbg!();
    module_allocs::remove_module(self);

    dbg!();
    library.close().unwrap_or_else(|e| {
      panic!("Failed to unload module library, reason: {e}");
    });

    dbg!();
    let still_loaded = is_library_loaded(&self.library_path);
    if still_loaded {
      panic!("Failed to unload module: {}", self.library_path.display());
    }
  }
}

impl Debug for Module {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let id = self.id;
    write!(f, "Module {{ id: {id} }}")
  }
}
