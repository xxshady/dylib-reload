fn main() {
  relib_interface::host::generate(
    "../shared/src/exports.rs",
    "relib_internal_shared::exports::___Internal___Exports___",
    "../shared/src/imports.rs",
    "relib_internal_shared::imports::___Internal___Imports___",
  );

  crate_compilation_info::provide();
}
