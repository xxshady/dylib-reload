use std::{fmt::Debug, path::Path};

use proc_macro2::TokenStream;
use quote::quote;

use crate::shared::{
  extract_trait_name_from_path, for_each_trait_item, parse_trait_file, write_code_to_file, TraitFn,
};

/// Will generate src/gen_exports.rs module which you can include using `mod gen_exports;` in your lib.rs or main.rs
/// and then use `ModuleExports` struct:
/// ```
/// let exports = ModuleExports::new(library);
/// exports.something();
/// ```
pub fn generate(
  exports_file_path: impl AsRef<Path> + Debug,
  exports_trait_path: &str,
  imports_file_path: impl AsRef<Path> + Debug,
  imports_trait_path: &str,
) {
  generate_exports(exports_file_path, exports_trait_path);
  generate_imports(imports_file_path, imports_trait_path);
}

fn generate_exports(exports_file_path: impl AsRef<Path> + Debug, exports_trait_path: &str) {
  let trait_name = extract_trait_name_from_path(exports_trait_path);
  let (exports_trait, module_use_items) =
    parse_trait_file(trait_name, exports_file_path, exports_trait_path);

  let mut export_decls = Vec::<TokenStream>::new();
  let mut export_inits = Vec::<TokenStream>::new();
  let mut export_impls = Vec::<TokenStream>::new();

  for item in &exports_trait.items {
    let TraitFn {
      ident,
      unsafety,
      inputs,
      inputs_without_types,
      output,
      mangled_name,
    } = for_each_trait_item(trait_name, item);

    export_decls.push(quote! {
      #ident: #unsafety extern "C" fn( #inputs ) #output
    });

    let panic_message =
      format!(r#"Failed to get "{ident}" fn symbol from module (mangled name: "{mangled_name}")"#);

    export_inits.push(quote! {
      #ident: *library.get(concat!(#mangled_name, "\0").as_bytes()).expect(#panic_message)
    });

    export_impls.push(quote! {
      pub #unsafety fn #ident(&self, #inputs ) #output {
        (self.#ident)( #inputs_without_types )
      }
    });
  }

  write_code_to_file(
    "src/gen_exports.rs",
    quote! {
      #module_use_items

      pub struct ModuleExports {
        #( #export_decls, )*
      }

      impl ModuleExports {
        pub unsafe fn new(library: &libloading::Library) -> Self {
          Self {
            #( #export_inits, )*
          }
        }

        #( #export_impls )*
      }
    },
  );
}

fn generate_imports(imports_file_path: impl AsRef<Path> + Debug, imports_trait_path: &str) {
  let trait_name = extract_trait_name_from_path(imports_trait_path);
  let (imports_trait, module_use_items) =
    parse_trait_file(trait_name, imports_file_path, imports_trait_path);

  let imports_trait_path: syn::Path =
    syn::parse_str(imports_trait_path).expect("Failed to parse imports_trait_path as syn::Path");

  let mut imports = Vec::<TokenStream>::new();

  for item in imports_trait.items {
    let TraitFn {
      ident,
      unsafety,
      inputs,
      inputs_without_types,
      output,
      mangled_name,
    } = for_each_trait_item(trait_name, &item);

    let panic_message =
      format!(r#"Failed to get "{mangled_name}" symbol of static function pointer from module"#);

    imports.push(quote! {
      unsafe {
        let ptr: *mut #unsafety extern "C" fn( #inputs ) #output
          = *library.get(concat!(#mangled_name, "\0").as_bytes()).expect(#panic_message);

        *ptr = impl_;

        #unsafety extern "C" fn impl_( #inputs ) #output {
          <ModuleImportsImpl as Imports>::#ident( #inputs_without_types )
        }
      }
    });
  }

  write_code_to_file(
    "src/gen_imports.rs",
    quote! {
      #module_use_items

      use #imports_trait_path as Imports;

      /// Struct for implementing your `Imports` trait
      pub struct ModuleImportsImpl;

      pub fn init_imports(library: &libloading::Library) {
        #( #imports )*
      }
    },
  );
}
