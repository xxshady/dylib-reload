use std::{thread, time::Duration};

fn main() {
  let path = "target/debug/libtest_module.so";
  // let path = "./libtest_moduledddddd.so";

  let module = unsafe { dylib_reload_host::load_module(path) }.unwrap();

  dbg!();
  unsafe {
    let out: () = module.main();
  }
  dbg!();

  thread::sleep(Duration::from_millis(200));
  drop(module);
  thread::sleep(Duration::from_millis(1000));
}
