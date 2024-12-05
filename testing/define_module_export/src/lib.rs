use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn define_module_export(_args: TokenStream, input: TokenStream) -> TokenStream {
  define_module_export::define_module_export(input.into()).into()
}
