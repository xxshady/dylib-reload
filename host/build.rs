fn main() {
  dylib_interface::host::generate(
    "../shared/src/exports.rs",
    "dylib_reload_shared::exports::___Internal___Exports___",
    "../shared/src/imports.rs",
    "dylib_reload_shared::imports::___Internal___Imports___",
  );
}
