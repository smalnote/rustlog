extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(AddLengths)]
pub fn derive_add_lengths(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        // Implement Add for other types that can be converted into the same type
        impl<T> Add<T> for #name
        where
            T: Into<#name>,
        {
            type Output = #name;

            fn add(self, rhs: T) -> Self::Output {
                let rhs_converted: #name = rhs.into();
                #name(self.0 + rhs_converted.0)
            }
        }
    };

    TokenStream::from(expanded)
}
