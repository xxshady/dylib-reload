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

fn load() -> Module<ModuleExports> {
  let directory = if cfg!(debug_assertions) {
    "debug"
  } else {
    "release"
  };

  let path = if cfg!(target_os = "linux") {
    format!("target/{directory}/libtest_module.so")
  } else {
    format!("target/{directory}/test_module.dll")
  };

  let module = unsafe { dylib_reload_host::load_module::<ModuleExports>(path) }.unwrap();

  init_imports(module.library());

  // dbg!();
  unsafe {
    module.call_main::<()>().unwrap();
  }
  // dbg!();

  // let a = module.exports().a().unwrap();
  // dbg!(&a);
  // let a = *a;

  let b = unsafe { module.exports().b() }.unwrap();
  dbg!(b);

  // module.unload().unwrap_or_else(|e| {
  //   panic!("{e:#}");
  // });

  // todo!()
  module
}
