use std::thread::sleep;
#[allow(unused_imports)]
use std::{io::stdin, thread, time::Duration};

use dylib_reload_host::Module;

dylib_interface::include_exports!();
dylib_interface::include_imports!();
use gen_exports::ModuleExports;
use gen_imports::{init_imports, ModuleImportsImpl};

use testing_shared::imports::Imports;

impl Imports for ModuleImportsImpl {}

pub fn main() {
  for _ in 1..=100 {
    println!();
  }
  
  module_load_loop();
}

fn module_load_loop() {

  println!("[host] loading module");
    let module = load();

      println!("[host] unloading module");
      module.unload().unwrap_or_else(|e| {
        panic!("{e:#}");
      });

  // let mut unload_immediately = true;
  // loop {
  //   println!("[host] loading module");
  //   let module = load();

  //   if unload_immediately {
  //     println!("[host] unloading module");
  //     module.unload().unwrap_or_else(|e| {
  //       panic!("{e:#}");
  //     });
  //   }
    
  //   let mut message = String::new();
  //   stdin().read_line(&mut message).unwrap();

  //   if message == "q\n" {
  //     return;
  //   }
  //   else if message == "stop\n" {
  //     unload_immediately = !unload_immediately;
  //     if unload_immediately {
  //       println!("[host] module will be unloaded immediately");
  //     } else {
  //       println!("[host] module won't be unloaded immediately");
  //     }
  //   }
  // }
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

  let module = dylib_reload_host::load_module::<ModuleExports>(path).unwrap();

  init_imports(module.library());

  // dbg!();
  unsafe {
    module.call_main::<()>().unwrap();
  }
  // dbg!();

  // let a = module.exports().a().unwrap();
  // dbg!(&a);
  // let a = *a;

  // let b = unsafe { module.exports().b() }.unwrap();
  // dbg!(b);

  // module.unload().unwrap_or_else(|e| {
  //   panic!("{e:#}");
  // });

  // todo!()
  module
}

fn leak_100mb() {
  std::mem::forget(vec![1_u8; 1024 * 1024 * 100]);
}
