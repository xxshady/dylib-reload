use crate::{allocator_lock, gen_imports, IS_IT_HOST_OWNER_THREAD};

pub fn unrecoverable(message: &str) -> ! {
  gen_imports::unrecoverable(message.into())
}

pub fn assert_allocator_is_still_accessible() {
  if allocator_lock() && !IS_IT_HOST_OWNER_THREAD.get() {
    unrecoverable(
      "module allocator was invoked while module was in the process of unloading\n\
      note: before unloading the module, make sure that all threads are joined if any were spawned by it\n\
      note: you can register \"before_unload\" callback for it",
    );
  }
}
