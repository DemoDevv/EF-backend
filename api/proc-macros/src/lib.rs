use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput};

extern crate proc_macro2;
extern crate quote;
extern crate syn;

extern crate proc_macro;

#[proc_macro_derive(Updatable)]
pub fn derive_updatable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_input = match input.data {
        syn::Data::Struct(data) => data,
        _ => panic!("Only structs are supported"),
    };

    let struct_fields = match struct_input.fields {
        syn::Fields::Named(fields) => fields,
        _ => panic!("Only named fields are supported"),
    };

    // print type of each field

    struct_fields.named.iter().for_each(|field| {
        println!("{:?}", field.ty.to_token_stream().to_string());
    });

    struct_fields.named.iter().for_each(|field| {
        println!("{:?}", field.ident.as_ref().unwrap().to_string());
    });

    let expanded = quote::quote! {
        impl Structe {
            fn update(&self, updatable_data: i32) -> Self {
                Self {
                    data: updatable_data
                }
            }
        }
    };
    TokenStream::from(expanded)
}
