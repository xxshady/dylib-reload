use std::{
  cell::Cell,
  sync::atomic::{AtomicU64, Ordering},
};

use stabby::libloading::StabbyLibrary;

use crate::module_allocs;

pub type ModuleId = u64;

pub struct Module {
  pub id: ModuleId,
  library: libloading::Library,

  init_fn: extern "C" fn(host_thread_id: i64),
}

impl Module {
  pub(crate) fn new(library: libloading::Library) -> Self {
    // module ids start from 1
    static ID_COUNTER: AtomicU64 = AtomicU64::new(1);

    let id = ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    assert_ne!(id, 0, "this must never happen (integer overflow)");

    //////////////////////////////////////////////////////////// TODO: move it to macro
    let str = "kek";
    let str = str.to_owned();
    let str = str + "\0";

    let cstr = CStr::from_bytes_until_nul(str.as_bytes()).unwrap_or_else(|_| {
      panic!("Failed to create CStr from: {str:?}");
    });

    let bytes = cstr.to_bytes_with_nul();
    /// ////////////////////////////////////////////////////////////////////////
    let instance = Self {
      id,
      library,
      init_fn: unsafe { library.get_stabbied(bytes) },
    };

    module_allocs::add_module(&instance);

    instance
  }
}

impl Drop for Module {
  fn drop(&mut self) {
    module_allocs::remove_module(self);
  }
}
