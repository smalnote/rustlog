use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(HelloMarco)]
pub fn hello_marco_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_hello_marco(&ast)
}

fn impl_hello_marco(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // As of edition 2024, `gen` is a reserved keyword
    let r#gen = quote! {
        impl HelloMarco for #name {
            fn hello_marco() {
                println!("Hello, Marco! My type name is {}!", stringify!(#name));
            }
        }
    };
    r#gen.into()
}
