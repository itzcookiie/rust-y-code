use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, DataStruct, Fields};

#[proc_macro_derive(PrintStruct)]
pub fn PrintStruct(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let id = input.ident.clone();
    let id_string = input.ident.to_string();

    let expanded = quote! {
        impl #id {
            pub fn print_struct_name() {
                println!("{:?}", #id_string);
            }
        }
    };

    TokenStream::from(expanded)
}