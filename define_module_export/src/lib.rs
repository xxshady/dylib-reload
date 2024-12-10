use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{ItemFn, ReturnType};

pub fn define_module_export(input: TokenStream2) -> TokenStream2 {
  let input = syn::parse2(input);

  let input: ItemFn = match input {
    Ok(input) => input,
    Err(e) => return e.to_compile_error(),
  };

  let ItemFn {
    attrs,
    vis: _,
    sig,
    block,
  } = input;

  let output = sig.output;
  let inputs = sig.inputs;
  let ident = sig.ident;
  let mangled_name = format!("__{ident}");

  // TODO: move it to shared crate since it's also needed in dylib_interface crate
  let return_type = match &output {
    ReturnType::Default => {
      quote! { () }
    }
    ReturnType::Type(_, ty) => ty.to_token_stream(),
  };

  quote! {
    #[unsafe(export_name = #mangled_name)]
    #( #attrs )*
    extern "C" fn #ident(
      ____return_value____: *mut std::mem::MaybeUninit<#return_type>, // will be initialized if function won't panic
      #inputs
    ) -> bool // returns false if function panicked
    {
      let result = std::panic::catch_unwind(move || #block);
      match result {
        Ok(value) => {
          unsafe {
            (*____return_value____).write(value);
          }
          true
        }
        // ignoring content since it's handled in our panic hook
        Err(_) => { false }
      }
    }
  }
}
