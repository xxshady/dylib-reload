use std::{
  alloc::{GlobalAlloc, Layout, System},
  collections::HashMap,
  sync::{
    atomic::{AtomicBool, Ordering},
    LazyLock, Mutex, MutexGuard,
  },
};

use dylib_reload_shared::{Allocation, AllocatorOp, AllocatorPtr, StableLayout};

use crate::{
  exit_deallocation, gen_imports,
  helpers::{check_unloaded_in_allocator, unrecoverable},
  MODULE_ID,
};

#[derive(Default, Debug)]
pub struct Allocator {
  inner: System,
}

impl Allocator {
  pub const fn new() -> Self {
    Allocator { inner: System }
  }
}

unsafe impl GlobalAlloc for Allocator {
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    check_unloaded_in_allocator();

    let ptr = self.inner.alloc(layout);

    let c_layout = StableLayout {
      size: layout.size(),
      align: layout.align(),
    };

    if ALLOC_INIT.load(Ordering::SeqCst) {
      gen_imports::on_alloc(MODULE_ID, ptr, c_layout);
    } else {
      save_alloc_in_buffer(ptr, c_layout);
    }

    ptr
  }

  unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    check_unloaded_in_allocator();

    self.inner.dealloc(ptr, layout);

    if exit_deallocation() {
      return;
    }

    let c_layout = StableLayout {
      size: layout.size(),
      align: layout.align(),
    };

    save_dealloc_in_buffer(ptr, c_layout);
  }
}

const CACHE_SIZE: usize = 20_000;

type AllocsCache = HashMap<AllocatorPtr, AllocatorOp>;

static ALLOCS_CACHE: LazyLock<Mutex<AllocsCache>> = LazyLock::new(|| Mutex::new(HashMap::new()));
static ALLOC_INIT: AtomicBool = AtomicBool::new(false);

static TRANSPORT_BUFFER: Mutex<Vec<AllocatorOp>> = Mutex::new(Vec::new());

fn lock_allocs_cache() -> MutexGuard<'static, AllocsCache> {
  ALLOCS_CACHE.lock().unwrap_or_else(|_| {
    unrecoverable("failed to lock ALLOCS_CACHE");
  })
}

fn lock_transport_buffer() -> MutexGuard<'static, Vec<AllocatorOp>> {
  TRANSPORT_BUFFER.lock().unwrap_or_else(|_| {
    unrecoverable("failed to lock TRANSPORT_BUFFER");
  })
}

fn push_to_allocs_cache(op: AllocatorOp, cache: Option<&mut AllocsCache>) {
  let cache = if let Some(cache) = cache {
    cache
  } else {
    &mut lock_allocs_cache()
  };

  let ptr = match op {
    AllocatorOp::Alloc(Allocation(ptr, ..)) => ptr,
    AllocatorOp::Dealloc(Allocation(ptr, ..)) => ptr,
  };

  cache.insert(ptr, op);

  if cache.len() == CACHE_SIZE {
    send_cached_allocs(Some(cache));
  }
}

fn save_alloc_in_buffer(ptr: *mut u8, layout: StableLayout) {
  // unsafe { crate::PRINT("save_alloc_in_buffer"); }

  push_to_allocs_cache(
    AllocatorOp::Alloc(Allocation(AllocatorPtr(ptr), layout)),
    None,
  );
}

fn save_dealloc_in_buffer(ptr: *mut u8, layout: StableLayout) {
  // unsafe { crate::PRINT("save_dealloc_in_buffer"); }

  let mut cache = &mut lock_allocs_cache();

  let ptr = AllocatorPtr(ptr);
  push_to_allocs_cache(AllocatorOp::Dealloc(Allocation(ptr, layout)), Some(cache));
}

pub unsafe fn init() {
  ALLOC_INIT.swap(true, Ordering::SeqCst);

  let mut cache = lock_allocs_cache();
  cache.reserve(CACHE_SIZE);

  let mut transport = lock_transport_buffer();
  transport.reserve(CACHE_SIZE);

  ALLOC_INIT.swap(false, Ordering::SeqCst);
}

pub fn send_cached_allocs(cache: Option<&mut AllocsCache>) {
  let cache = if let Some(cache) = cache {
    cache
  } else {
    &mut lock_allocs_cache()
  };

  let mut transport = lock_transport_buffer();

  transport.extend(cache.drain().map(|(_, allocation)| allocation));

  unsafe {
    let slice: &[AllocatorOp] = &transport;
    gen_imports::on_cached_allocs(MODULE_ID, slice.into());
  }

  transport.clear();
}
