use std::{fmt::Debug, path::Path};

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{FnArg, Ident};

use crate::shared::{
  extract_trait_name_from_path, fn_output_to_type, for_each_trait_item, parse_trait_file,
  write_code_to_file, TraitFn,
};

/// Will generate `generated_module_exports.rs` and `generated_module_imports.rs` in the OUT_DIR which you can include
/// using `include!(concat!(env!("OUT_DIR"), "/<file>"));` in your `lib.rs` or `main.rs`
/// and then use `ModuleExportsImpl` struct to implement your `Exports` trait:
/// ```
/// impl Exports for ModuleExportsImpl {
///   // ...
/// }
/// ```
pub fn generate(
  exports_file_path: impl AsRef<Path> + Debug,
  exports_trait_path: &str,
  imports_file_path: impl AsRef<Path> + Debug,
  imports_trait_path: &str,
) {
  generate_exports(exports_file_path, exports_trait_path, false);
  generate_imports(imports_file_path, imports_trait_path);
}

/// Will generate `generated_module_exports.rs` and `generated_module_imports.rs` in the OUT_DIR which you can include
/// using `include!(concat!(env!("OUT_DIR"), "/<file>"));` in your `lib.rs` or `main.rs`
/// and then use `ModuleExportsImpl` struct to implement your `Exports` trait:
/// ```
/// impl Exports for ModuleExportsImpl {
///   // ...
/// }
/// ```
pub fn generate_pub(
  exports_file_path: impl AsRef<Path> + Debug,
  exports_trait_path: &str,
  imports_file_path: impl AsRef<Path> + Debug,
  imports_trait_path: &str,
) {
  generate_exports(exports_file_path, exports_trait_path, true);
  generate_imports(imports_file_path, imports_trait_path);
}

fn generate_exports(
  exports_file_path: impl AsRef<Path> + Debug,
  exports_trait_path: &str,
  panic_handling: bool,
) {
  let trait_name = extract_trait_name_from_path(exports_trait_path);

  let (exports_trait, module_use_items) =
    parse_trait_file(trait_name, exports_file_path, exports_trait_path);

  let exports_trait_path: syn::Path =
    syn::parse_str(exports_trait_path).expect("Failed to parse exports_trait_path as syn::Path");

  let mut exports = Vec::<TokenStream2>::new();

  for item in exports_trait.items {
    let TraitFn {
      ident,
      unsafety,
      inputs,
      inputs_without_types,
      output,
      mangled_name,
    } = for_each_trait_item(trait_name, &item);

    let mangled_name = Ident::new(&mangled_name, Span::call_site());

    let code = if panic_handling {
      let return_type = fn_output_to_type(output);

      quote! {
        #[unsafe(no_mangle)]
        pub #unsafety extern "C" fn #mangled_name(
          ____return_value____: *mut std::mem::MaybeUninit<#return_type>, // will be initialized if function won't panic
          #inputs
        ) -> bool // returns false if function panicked
        {
          let result = std::panic::catch_unwind(move || {
            <ModuleExportsImpl as Exports>::#ident( #inputs_without_types )
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
    } else {
      quote! {
        #[unsafe(no_mangle)]
        pub #unsafety extern "C" fn #mangled_name( #inputs ) #output {
          <ModuleExportsImpl as Exports>::#ident( #inputs_without_types )
        }
      }
    };

    exports.push(code);
  }

  write_code_to_file(
    "generated_module_exports.rs",
    quote! {
      #module_use_items

      use #exports_trait_path as Exports;

      /// Struct for implementing your `Exports` trait
      pub struct ModuleExportsImpl;

      #( #exports )*
    },
  );
}

fn generate_imports(imports_file_path: impl AsRef<Path> + Debug, imports_trait_path: &str) {
  let trait_name = extract_trait_name_from_path(imports_trait_path);
  let (imports_trait, module_use_items) =
    parse_trait_file(trait_name, imports_file_path, imports_trait_path);

  let mut imports = Vec::<TokenStream2>::new();

  for item in imports_trait.items {
    let TraitFn {
      ident,
      unsafety,
      inputs,
      inputs_without_types,
      output,
      mangled_name,
    } = for_each_trait_item(trait_name, &item);

    let mangled_name = Ident::new(&mangled_name, Span::call_site());

    let placeholder_inputs: TokenStream2 = inputs
      .iter()
      .map(|arg| {
        let FnArg::Typed(arg) = arg else {
          unreachable!();
        };

        // let ts = arg.pat.to_token_stream();
        let ty = &arg.ty;
        quote! { _: #ty , }
      })
      .collect();

    imports.push(quote! {
      pub #unsafety fn #ident( #inputs ) #output {
        #[allow(non_upper_case_globals)]
        #[unsafe(no_mangle)]
        static mut #mangled_name: #unsafety extern "C" fn( #inputs ) #output = placeholder;

        #unsafety extern "C" fn placeholder( #placeholder_inputs ) #output {
          unreachable!();
        }

        unsafe {
          #mangled_name( #inputs_without_types )
        }
      }
    });
  }

  write_code_to_file(
    "generated_module_imports.rs",
    quote! {
      #module_use_items

      #( #imports )*
    },
  );
}
