use dylib_reload_module;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
  println!("Hello, world!");
  std::mem::forget(vec![1_u8; 10000]);
}
