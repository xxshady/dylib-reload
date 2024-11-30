use std::{env::current_dir, fmt::Debug, fs, path::Path};

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
  punctuated::Punctuated, FnArg, Ident, Item, ItemTrait, ReturnType, Token, TraitItem, UseTree,
  ItemUse,
};

pub fn format_code(code: &str) -> String {
  let file = syn::parse_file(code).unwrap_or_else(|e| {
    panic!("Failed to parse code as syn File, reason: {e:#?}");
  });
  prettyplease::unparse(&file)
}

pub fn parse_trait_file(
  trait_name: &str,
  file_path: impl AsRef<Path> + Debug,
  trait_path: &str,
) -> (ItemTrait, TokenStream) {
  let file_path = file_path.as_ref();

  let Some((crate_name, _)) = trait_path.split_once("::") else {
    panic!("Failed to extract crate name from trait path: {trait_path}");
  };
  let crate_name = Ident::new(crate_name, Span::call_site());

  let module = fs::read_to_string(file_path).unwrap_or_else(|e| {
    panic!(
      "Failed to read module from: {file_path:?}, reason: {e:#?}, cwd: {:?}",
      current_dir().unwrap()
    );
  });

  let ast = syn::parse_file(&module).unwrap_or_else(|e| {
    panic!("Failed to parse Rust code from: {file_path:?}, reason: {e:#?}");
  });

  let items = ast.items;

  let module_use_items = items
    .iter()
    .filter_map(|item| {
      let Item::Use(item_use) = item else {
        return None;
      };

      Some(patch_item_use_if_needed(item_use, &crate_name))
    })
    .collect::<TokenStream>();

  let are_there_any_other_items = items
    .iter()
    .any(|item| !matches!(item, Item::Use(..) | Item::Trait(..)));
  if are_there_any_other_items {
    panic!(
      "Only `use` and `trait` items allowed in the {}",
      file_path.display()
    );
  }

  let trait_ = items.into_iter().find_map(|item| {
    let Item::Trait(trait_) = item else {
      return None;
    };
    Some(trait_)
  });

  let Some(trait_) = trait_ else {
    panic!("Expected trait item");
  };

  assert_eq!(
    trait_.ident, trait_name,
    r#"Trait must be named "{trait_name}""#
  );

  (trait_.clone(), module_use_items)
}

pub fn write_code_to_file(file: &str, code: TokenStream) {
  let code = code.to_string();
  let code = format_code(&code);
  let out = format!(
    "// This file is generated, DO NOT edit manually\n\
    // ---------------------------------------------\n\n\
    {code}"
  );

  fs::write(file, out).expect("Failed to create generated file in src directory");
}

pub struct TraitFn<'a> {
  pub ident: &'a Ident,
  pub unsafety: Option<Token![unsafe]>,
  pub inputs: &'a Punctuated<FnArg, Token![,]>,
  pub inputs_without_types: TokenStream,
  pub output: &'a ReturnType,
  pub mangled_name: String,
}

pub fn for_each_trait_item<'trait_>(
  trait_name: &str,
  trait_item: &'trait_ TraitItem,
) -> TraitFn<'trait_> {
  let TraitItem::Fn(fn_) = trait_item else {
    panic!("All trait items must be functions");
  };
  let fn_ = &fn_.sig;
  assert!(
    fn_.receiver().is_none(),
    "Functions in {trait_name} trait must not have `self` receiver"
  );

  let ident = &fn_.ident;

  let inputs_without_types = fn_
    .inputs
    .iter()
    .map(|arg| {
      let FnArg::Typed(arg) = arg else {
        unreachable!();
      };

      let ts = arg.pat.to_token_stream();
      quote! { #ts , }
    })
    .collect();

  TraitFn {
    ident,
    unsafety: fn_.unsafety,
    inputs: &fn_.inputs,
    inputs_without_types,
    output: &fn_.output,

    mangled_name: format!("__{trait_name}_{ident}"),
  }
}

pub fn extract_trait_name_from_path(trait_path: &str) -> &str {
  trait_path.split("::").last().unwrap_or_else(|| {
    panic!("Failed to extract trait name from path: {trait_path}");
  })
}

fn patch_item_use_if_needed(item_use: &ItemUse, crate_name: &Ident) -> TokenStream {
  match &item_use.tree {
    UseTree::Path(path) => {
      let ident = path.ident.to_string();
      let ident = ident.as_str();

      match ident {
        "super" => {
          let code = item_use.to_token_stream();
          panic!(
            "Failed to copy `{code}`\n\
            note: `use super::` syntax is not supported, use absolute imports, for example `use crate::something`"
          );
        }
        "crate" => {
          let tree = &path.tree;
          quote! {
            use #crate_name::#tree;
          }
        }
        _ => item_use.to_token_stream(),
      }
    }
    _ => {
      let code = item_use.to_token_stream();
      panic!("unexpected syntax: `{code}`");
    }
  }
}
