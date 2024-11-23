use std::ffi::CStr;

pub fn unrecoverable(message: &'static str) -> ! {
  eprintln!("something unrecoverable happened: {message}");
  eprintln!("aborting");
  std::process::abort();
}
