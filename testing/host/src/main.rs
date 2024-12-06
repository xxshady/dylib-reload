#[allow(unused_imports)]
use std::{io::stdin, thread, time::Duration};

use dylib_reload_host::Module;

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
  loop {
    let module = load();
    println!("----------------------------");

    let mut message = String::new();
    stdin().read_line(&mut message).unwrap();

    println!("unloading");
    module.unload().unwrap_or_else(|e| {
      panic!("{e:#}");
    });

    message.clear();

    println!("unloaded");
    stdin().read_line(&mut message).unwrap();

    if message == "q\n" {
      return;
    }
  }
}

fn load() -> Module {
  let path = if cfg!(target_os = "linux") {
    "target/debug/libtest_module.so"
  } else {
    "target/debug/test_module.dll"
  };
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

  // module.unload().unwrap_or_else(|e| {
  //   panic!("{e:#}");
  // });
  module
}
