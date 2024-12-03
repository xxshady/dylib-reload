fn main() {
  dylib_interface::module::generate(
    "../shared/src/exports.rs",
    "dylib_reload_shared::exports::___Internal___Exports___",
    "../shared/src/imports.rs",
    "dylib_reload_shared::imports::___Internal___Imports___",
  );
  crate_compilation_info::provide();
}
