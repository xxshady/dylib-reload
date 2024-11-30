use crate::{gen_imports, unloaded};

pub fn unrecoverable(message: &str) -> ! {
  gen_imports::unrecoverable(message.into())
}

pub fn check_unloaded_in_allocator() {
  if unloaded() {
    unrecoverable(
      "Module is unloaded but it's allocator has been invoked\n\
      note: before unloading the module, make sure that all threads are joined if any were spawned by it"
    );
  }
}
