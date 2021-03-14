extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn;

#[proc_macro_attribute]
pub fn state_set_chainlink(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_state_set_chainlink_macro(&ast)
}

fn impl_state_set_chainlink_macro(ast: &syn::DeriveInput) -> TokenStream {
    let ident = &ast.ident;
    let mut functions = Vec::new();

    let enums = match &ast.data {
        syn::Data::Enum(e) => { e }
        _ => { return quote! { compile_error!("derive macro 'StateSetChainlink' only compatible with enums");}.into()}
    };

    for e in enums.variants.iter() {
        let val = &e.ident;
        let name = format_ident!("set_next_{}", e.ident.to_string().to_lowercase());
        functions.push(
            quote! {
                pub fn #name(
                    ::bevy::ecs::In(activate): ::bevy::ecs::In<bool>,
                    mut state: ::bevy::ecs::ResMut<::bevy::ecs::State<#ident>>
                ) {
                    if activate == true { state.set_next(#ident::#val).ok(); }
                }
            }
        );
    }
    return quote! {
        #ast
        impl #ident {
            #(#functions)*
        }
    }.into()
}