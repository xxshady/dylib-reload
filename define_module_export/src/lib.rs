use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{FnArg, ItemFn, ReturnType};

#[proc_macro_attribute]
pub fn define_module_export(_args: TokenStream, input: TokenStream) -> TokenStream {
  let input = syn::parse_macro_input!(input as ItemFn);
  let ItemFn {
    attrs,
    vis: _,
    sig,
    block,
  } = input;

  let ident = sig.ident;
  let mangled_name = format!("__{ident}");

  let output = sig.output;

  // TODO: move it to shared crate since it's also needed in dylib_interface crate
  let return_type = match &output {
    ReturnType::Default => {
      quote! { () }
    }
    ReturnType::Type(_, ty) => ty.to_token_stream(),
  };

  let inputs = sig.inputs;

  // TODO: move it to shared crate since it's also needed in dylib_interface crate
  let inputs_without_types: TokenStream2 = inputs
    .iter()
    .map(|arg| {
      let FnArg::Typed(arg) = arg else {
        unreachable!();
      };

      let ts = arg.pat.to_token_stream();
      quote! { #ts , }
    })
    .collect();

  quote! {
    #[unsafe(export_name = #mangled_name)]
    #( #attrs )*
    extern "C" fn #ident(
      ____return_value____: *mut std::mem::MaybeUninit<#return_type>, // will be initialized if function won't panic
      #inputs
    ) -> bool // returns false if function panicked
    {
      fn ___do_call___( #inputs ) #output {
        #block
      }

      let result = std::panic::catch_unwind(move || {
        ___do_call___( #inputs_without_types )
      });

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
  .into()
}
