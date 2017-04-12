#![crate_type = "proc-macro"]

#[macro_use]
extern crate proc_macro_hack;

#[macro_use]
extern crate quote;

extern crate syn;
extern crate proc_macro;

use syn::Item;
use syn::ItemKind::Trait;
use quote::Tokens;
use quote::ToTokens;

proc_macro_item_impl! {
    /// Generates a default blanket impl for a generic trait that gets passed into
    /// `generate_auto_impl_impl`, with the same type parameters and type bounds as its trait.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice of the trait source.
    pub fn generate_auto_impl_impl(input: &str) -> String {
        let item = syn::parse_item(input).unwrap();

        let name = &item.ident;
        match item.node {
            Trait(_, _, _, _) => {},
            _ => panic!("Cannot expand type {} since it's not a trait!", name)
        }

        let mut expanded = expand_trait(&item);
        expanded.append(expand_impl(&item).as_str());

        // Return the generated impl and its trait as a String
        expanded.parse().unwrap()
    }
}

/// Returns the Tokens of the trait that gets passed into `autoimpl!`
fn expand_trait(ast: &Item) -> Tokens {
    let mut tokens = Tokens::new();
    ast.to_tokens(&mut tokens);
    tokens
}

/// Generates a default impl of the trait that gets passed into `autoimpl!`, with the same type
/// arguments and generic bounds as its trait.
fn expand_impl(ast: &Item) -> Tokens {
    // Used in the quasi-quotation below as `#name`
    let name = &ast.ident;

    match ast.node {
        Trait(_, ref generics, _, _) => {
            let (impl_generics, _, where_clause) = generics.split_for_impl();

            if generics.ty_params.is_empty() {
                panic!("Cannot auto-implement {} since it does not have any type parameters!",
                       name)
            }

            quote! {
                impl #impl_generics #name #impl_generics for T #where_clause {}
            }
        }
        _ => {
            panic!("Cannot expand type {} implementation since it's not a trait!",
                   name)
        }
    }
}
