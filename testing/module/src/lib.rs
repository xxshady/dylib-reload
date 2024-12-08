#[allow(unused_imports)]
use std::{
  cell::Cell,
  thread::{self, sleep},
  time::{Duration, Instant},
};

use testing_define_module_export::define_module_export;
use dylib_reload_module as _;

dylib_interface::include_exports!();
dylib_interface::include_imports!();

use gen_exports::ModuleExportsImpl;
use testing_shared::exports::Exports;
impl Exports for ModuleExportsImpl {
  fn a() -> i32 {
    // panic!("awdawdadwadwdwddwdwdwddddddddddddddd");
    (unsafe { gen_imports::b() }) + 1
  }

  fn b() -> String {
    "qwawawada".to_owned()
  }
}

#[define_module_export]
fn main() {
  // dbg!();
  // vec![1_u8; 1024 * 1024 * 10];
  std::mem::forget(vec![1_u8; 1024 * 1024 * 50]);

  struct TestTlsDrop;
  impl Drop for TestTlsDrop {
    fn drop(&mut self) {
      // dbg!();
      vec![1];
    }
  }
  thread_local! {
    static V: TestTlsDrop = TestTlsDrop;
    static V2: Cell<Vec<u8>> = Vec::new().into();
  }
  V.with(|_| {});
  V2.with(|v| {
    let mut vec = v.take();
    vec.push(1);
    v.replace(vec);
  });

  // fn test_panic() {
  //   panic!();
  // }
  // test_panic();
  // 123
}

// #[unsafe(no_mangle)]
// pub extern "C" fn main() {
//   // println!("Hello, world!");
//   // std::mem::forget(vec![1_u8; 10000]);
//   // let _ = std::thread::spawn(|| {
//   //   std::thread::sleep_ms(1000);
//   //   dbg!();
//   //   std::thread::sleep_ms(1000);
//   // });

//   thread::spawn(|| {
//     println!("before");
//     let initial = Instant::now();
//     while initial.elapsed() < Duration::from_millis(750) {
//       // vec![1];
//     }
//     println!("after");
//   });

//   // thread_local! {
//   //   static V: Box<u8> = Box::new(1);
//   // }
//   // V.with(|_| {});
// }

#[define_module_export]
fn before_unload() {
  println!("before unload");
  // panic!();
  // thread::spawn(|| {
  //   println!("before");
  //   let initial = Instant::now();
  //   while initial.elapsed() < Duration::from_millis(750) {
  //     // vec![1];
  //   }
  //   println!("after");
  // });
}
