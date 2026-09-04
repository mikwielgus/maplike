// SPDX-FileCopyrightText: 2026 maplike contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Keyed)]
pub fn derive_keyed(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_keyed(input).unwrap_or_else(|err| err.to_compile_error().into())
}

fn expand_keyed(input: DeriveInput) -> syn::Result<TokenStream> {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let name = &input.ident;

    let output = quote! {
        impl #impl_generics ::maplike::abc::Keyed for #name #ty_generics
        #where_clause
        {
            type Key = usize;
            type Value = Self;
        }
    };
    Ok(output.into())
}
