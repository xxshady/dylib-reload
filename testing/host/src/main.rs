#[allow(unused_imports)]
use std::{thread, time::Duration};

dylib_interface::include_exports!();
dylib_interface::include_imports!();
use gen_exports::ModuleExports;
use gen_imports::{init_imports, ModuleImportsImpl};

use testing_shared::imports::Imports;

impl Imports for ModuleImportsImpl {
  fn b() -> i32 {
    1222
  }
}

fn main() {
  for _ in 1..=3 {
    load_and_unload();
  }
}

fn load_and_unload() {
  let path = "target/debug/libtest_module.so";
  // let path = "target/release/libtest_module.so";
  // let path = "./libtest_moduledddddd.so";

  let module = unsafe { dylib_reload_host::load_module(path) }.unwrap();

  init_imports(module.library());
  let exports = ModuleExports::new(module.library());

  // dbg!();
  unsafe {
    let out: Option<()> = module.call_main();
    dbg!(out);
  }
  // dbg!();

  let a = exports.a();
  dbg!(a);

  // thread::sleep(Duration::from_millis(200));
  module.unload().unwrap_or_else(|e| {
    panic!("{e:#}");
  });
  // thread::sleep(Duration::from_millis(1000));
}
