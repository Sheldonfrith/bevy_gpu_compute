use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::ItemMod;

/// make a module that is not intended to be viewed or accessed just to allow the rust compiler to run and find any potential errors in the original code that might be missed elsewhere in our macro if we remove or alter parts of the original code.
pub fn generate_unaltered_module(original_module: &ItemMod) -> TokenStream {
    let original_ident = &original_module.ident;
    let content: Vec<TokenStream> = if let Some(content) = &original_module.content {
        content
            .1
            .iter()
            .map(|item| {
                let item = item.to_token_stream();
                quote! {
                    #item
                }
            })
            .collect()
    } else {
        vec![quote! {}]
    };
    let content_combined: TokenStream = content.into_iter().collect();
    let new_ident = format_ident!("_internal_{}", original_ident);
    quote! {
        #[allow(dead_code, unused_variables, unused_imports)]
        mod #new_ident {
            use super::*;
            #content_combined
        }
    }
}
