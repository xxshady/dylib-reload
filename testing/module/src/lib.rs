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
    while initial.elapsed() < Duration::from_secs(1) {
      vec![1];
    }
    println!("after");
  });
}

#[unsafe(no_mangle)]
pub extern "C" fn before_unload() {
  println!("before unload");
}
