use std::{
  cell::Cell,
  sync::atomic::{AtomicU64, Ordering},
};

use stabby::libloading::StabbyLibrary;

use crate::{
  helpers::{cstr_bytes, get_stabbied_fn},
  module_allocs,
};

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

    let instance = Self {
      id,
      init_fn: unsafe { get_stabbied_fn(&library, "__init") },
      library,
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
