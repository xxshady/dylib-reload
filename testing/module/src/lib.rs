// use stabby::str::Str;

use dylib_reload_module;

#[stabby::export]
pub extern "C" fn main() {
  println!("Hello, world!");
  std::mem::forget(vec![1_u8; 10000]);
}

// #[unsafe(no_mangle)]
// pub static mut __UNRECOVERABLE: extern "C" fn(Str) -> ! = unrecoverable_placeholder;

// extern "C" fn unrecoverable_placeholder(_: Str) -> ! {
//   unreachable!();
// }
