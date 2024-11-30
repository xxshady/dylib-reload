fn main() {
  let module = unsafe { dylib_reload_host::load_module("target/debug/libtest_module.so") }.unwrap();

  dbg!();
  unsafe {
    module.main::<()>();
  }
  dbg!();
}
