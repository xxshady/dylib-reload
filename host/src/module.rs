use std::{
  fmt::Debug,
  marker::PhantomData,
  mem::MaybeUninit,
  path::{Path, PathBuf},
};

use dylib_reload_shared::ModuleId;
use libloading::Symbol;

use crate::{
  gen_exports::ModuleExports,
  helpers::{call_module_pub_export, get_library_export, is_library_loaded, open_library},
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

  pub fn library(&self) -> &libloading::Library {
    self.library.as_ref().unwrap_or_else(|| unreachable!())
  }

  /// Returns `None` if module panics.
  /// Note: not all panics are handled, see a ["double panic"](https://doc.rust-lang.org/std/ops/trait.Drop.html#panics)
  /// ```
  /// struct Bomb;
  ///   impl Drop for Bomb {
  ///     fn drop(&mut self) {
  ///         panic!("boom"); // will abort the program
  ///     }
  /// }
  /// let _bomb = Bomb;
  /// panic!();
  /// ```
  pub unsafe fn call_main<R>(&self) -> Option<R> {
    call_module_pub_export(self.library(), "__main").unwrap_or_else(|e| {
      panic!("Failed to get main fn from module, reason: {e:#}");
    })
  }
}

impl Drop for Module {
  fn drop(&mut self) {
    // TEST
    println!("----------- unloading library");

    let library = self.library.take().unwrap_or_else(|| unreachable!());

    unsafe {
      println!(r#"Trying to call "before_unload"#);

      let result = call_module_pub_export(&library, "__before_unload");
      match result {
        Ok(Some(())) => {}
        Err(e) => {
          println!("Failed to get \"before_unload\" from module: {e:#}, ignoring it");
        }
        Ok(None) => {
          panic!(r#"Failed to call "before_unload", module panicked"#);
        }
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
