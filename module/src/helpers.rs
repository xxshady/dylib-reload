use crate::host_statics::{__unloaded, __UNRECOVERABLE};

pub fn unrecoverable(message: &str) -> ! {
  unsafe {
    __UNRECOVERABLE(message.into());
  }
}

pub fn check_unloaded_in_allocator() {
  if __unloaded() {
    unrecoverable(
      "Module is unloaded but it's allocator has been invoked\n\
      note: before unloading the module, make sure that all threads are joined if any were spawned by it"
    );
  }
}
