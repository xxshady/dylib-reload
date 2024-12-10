use testing_define_module_export::define_module_export;
use dylib_reload_module as _;

dylib_interface::include_imports!();

// includes `mod gen_exports`
dylib_interface::include_exports!();
use gen_exports::ModuleExportsImpl;

use testing_shared::exports::Exports;

impl Exports for ModuleExportsImpl {}














#[define_module_export]
fn main() {
  println!("[module] hello");

  // all leaked module memory which will be deallocated 
  // automatically on module unload
  leak_100mb();

  fn leak_100mb() {
    std::mem::forget(vec![1_u8; 1024 * 1024 * 100]);
  }
}
