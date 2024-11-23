use std::{
  collections::HashMap,
  sync::{LazyLock, Mutex, MutexGuard},
};

use shared::{Allocation, AllocatorPtr};

use crate::{helpers::unrecoverable, module::ModuleId, Module};

type Allocs = HashMap<ModuleId, HashMap<AllocatorPtr, Allocation>>;

static ALLOCS: LazyLock<Mutex<Allocs>> = LazyLock::new(|| Mutex::new(HashMap::new()));

fn lock_allocs() -> MutexGuard<'static, Allocs> {
  let Ok(allocs) = ALLOCS.lock() else {
    unrecoverable("failed to lock ALLOCS");
  };

  allocs
}

pub fn add_module(module: &Module) {
  let mut allocs = lock_allocs();
  allocs.insert(module.id, Default::default());
}

pub fn remove_module(module: &Module) {
  let mut allocs = lock_allocs();
  allocs.remove(&module.id).unwrap_or_else(|| {
    unrecoverable("failed to remove module from ALLOCS");
  });
}
