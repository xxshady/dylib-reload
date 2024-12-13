use proc_macro::TokenStream;

/// Use it for exporting `main` or `before_unload` functions from the module.
///
/// **note:** see documentation of `relib_exportify::exportify` for implementation details.
///
/// # Examples
/// ```
/// #[relib_module::export]
/// fn foo() {
///   // ...
/// }
/// ```
#[proc_macro_attribute]
pub fn export(_args: TokenStream, input: TokenStream) -> TokenStream {
  relib_exportify::exportify(input.into()).into()
}
