use std::{
  cell::Cell,
  thread,
  time::{Duration, Instant},
};

use dylib_reload_module as _;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
  // println!("Hello, world!");
  // std::mem::forget(vec![1_u8; 10000]);
  // let _ = std::thread::spawn(|| {
  //   std::thread::sleep_ms(1000);
  //   dbg!();
  //   std::thread::sleep_ms(1000);
  // });

  thread::spawn(|| {
    println!("before");
    let initial = Instant::now();
    while initial.elapsed() < Duration::from_millis(10000) {
      // vec![1];
    }
    println!("after");
  });

  // thread_local! {
  //   static V: Box<u8> = Box::new(1);
  // }
  // V.with(|_| {});
}

#[unsafe(no_mangle)]
pub extern "C" fn before_unload() {
  println!("before unload");
}
