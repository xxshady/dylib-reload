use std::{
  collections::HashMap,
  sync::{LazyLock, Mutex, MutexGuard},
};

use shared::{
  callbacks::{OnAllocDealloc, OnCachedAllocs},
  Allocation, AllocatorOp, AllocatorPtr, ModuleId, SliceAllocatorOp, StableLayout,
};

use crate::{
  helpers::{cstr_bytes, get_stabbied_fn, unrecoverable},
  Module,
};

type Allocs = HashMap<ModuleId, HashMap<AllocatorPtr, Allocation>>;

static ALLOCS: LazyLock<Mutex<Allocs>> = LazyLock::new(|| Mutex::new(HashMap::new()));

fn lock_allocs() -> MutexGuard<'static, Allocs> {
  let Ok(allocs) = ALLOCS.lock() else {
    unrecoverable("failed to lock ALLOCS");
  };

  allocs
}

pub fn add_module(module: &Module) {
  let library = module.library.as_ref().unwrap_or_else(|| unreachable!());

  unsafe {
    let on_cached_allocs_ptr: *mut OnCachedAllocs = *library
      .get(&cstr_bytes("__ON_CACHED_ALLOCS"))
      .expect("Failed to get __ON_CACHED_ALLOCS from module");
    *on_cached_allocs_ptr = on_cached_allocs;

    let on_alloc_ptr: *mut OnAllocDealloc = *library
      .get(&cstr_bytes("__ON_ALLOC"))
      .expect("Failed to get __ON_ALLOC from module");
    *on_alloc_ptr = on_alloc;

    let on_dealloc_ptr: *mut OnAllocDealloc = *library
      .get(&cstr_bytes("__ON_DEALLOC"))
      .expect("Failed to get __ON_DEALLOC from module");
    *on_dealloc_ptr = on_dealloc;
  }

  let mut allocs = lock_allocs();
  allocs.insert(module.id, Default::default());
}

pub fn remove_module(module: &Module) {
  (module.request_cached_allocs)();

  let mut allocs = lock_allocs();
  let Some(allocs) = allocs.remove(&module.id) else {
    panic!("Failed to take allocs of module with id: {}", module.id);
  };

  let allocs: Box<[Allocation]> = allocs.into_values().collect();
  let allocs: &[Allocation] = &allocs;

  (module.exit)(allocs.into());
}

extern "C" fn on_cached_allocs(module_id: ModuleId, ops: SliceAllocatorOp) {
  let ops = unsafe { ops.into_slice() };

  // TEST
  println!("received cached alloc ops: {}", ops.len());

  let mut allocs = lock_allocs();
  let allocs = allocs.get_mut(&module_id).unwrap_or_else(|| unreachable!());

  for op in ops {
    match op {
      AllocatorOp::Alloc(allocation) => {
        let Allocation(ptr, ..) = allocation;
        allocs.insert(*ptr, allocation.clone());
      }
      AllocatorOp::Dealloc(Allocation(ptr, ..)) => {
        // doesnt matter if allocs didnt have it
        let _ = allocs.remove(ptr);
      }
    }
  }
}

extern "C" fn on_alloc(module_id: ModuleId, ptr: *mut u8, layout: StableLayout) {
  let mut allocs = lock_allocs();
  let allocs = allocs
    .get_mut(&module_id)
    .unwrap_or_else(|| unrecoverable("on_alloc unreachable"));

  let ptr = AllocatorPtr(ptr);
  allocs.insert(ptr, Allocation(ptr, layout));
}

extern "C" fn on_dealloc(module_id: ModuleId, ptr: *mut u8, layout: StableLayout) {
  let mut allocs = lock_allocs();
  let allocs = allocs
    .get_mut(&module_id)
    .unwrap_or_else(|| unrecoverable("on_dealloc unreachable"));

  allocs.remove(&AllocatorPtr(ptr)).unwrap_or_else(|| {
    unrecoverable("did not found allocation");
  });
}
