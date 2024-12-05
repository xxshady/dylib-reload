use std::{fmt::Debug, marker::PhantomData, path::PathBuf};

use dylib_reload_shared::ModuleId;

use crate::{
  errors::UnloadError,
  gen_exports::ModuleExports,
  helpers::{call_module_pub_export, is_library_loaded},
  leak_library::LeakLibrary,
  module_allocs,
};

#[must_use = "module will be leaked if dropped, if you don't want that consider using `unload` method"]
pub struct Module {
  pub id: ModuleId,

  pub(crate) library: LeakLibrary,
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
      library: LeakLibrary::new(library),
      library_path,
      exports,
      _not_thread_safe: PhantomData,
    }
  }

  pub fn library(&self) -> &libloading::Library {
    self.library.get_ref()
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

  pub fn unload(self) -> Result<(), UnloadError> {
    // TEST
    println!("----------- unloading library");

    let library = self.library();
    let library_path = self.library_path.to_string_lossy().into_owned();

    unsafe {
      println!(r#"Trying to call "before_unload"#);

      let result = call_module_pub_export(library, "__before_unload");
      match result {
        Ok(Some(())) => {}
        Err(e) => {
          println!("Failed to get \"before_unload\" from module: {e:#}, ignoring it");
        }
        Ok(None) => {
          return Err(UnloadError::BeforeUnloadPanicked(library_path));
        }
      }
    }

    if self.exports.spawned_threads_count() > 0 {
      return Err(UnloadError::ThreadsStillRunning(library_path));
    }

    self.exports.lock_module_allocator();

    unsafe {
      self.exports.run_thread_local_dtors();
    }

    module_allocs::remove_module(&self);

    self.library.take().close()?;

    let still_loaded = is_library_loaded(&self.library_path);
    if still_loaded {
      return Err(UnloadError::UnloadingFail(library_path));
    }

    Ok(())
  }
}

impl Debug for Module {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let id = self.id;
    write!(f, "Module {{ id: {id} }}")
  }
}
