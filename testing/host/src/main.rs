fn main() {
  let module = unsafe { host::load_module("target/debug/libtest_module.so") }.unwrap();
  unsafe {
    // TODO: fix compiler overflow error if turbo fish is removed
    module.main::<()>();
  }
}
