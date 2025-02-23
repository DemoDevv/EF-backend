use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{parse_macro_input, DeriveInput, Ident};

extern crate proc_macro2;
extern crate quote;
extern crate syn;

extern crate proc_macro;

#[proc_macro_derive(Updatable, attributes(updatable))]
pub fn derive_updatable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let updatable_struct_name = Ident::new(&format!("Updatable{}", struct_name), Span::call_site());

    let struct_input = match input.data {
        syn::Data::Struct(data) => data,
        _ => panic!("Only structs are supported"),
    };

    let struct_fields = match struct_input.fields {
        syn::Fields::Named(fields) => fields,
        _ => panic!("Only named fields are supported"),
    };

    let struct_fields = struct_fields
        .named
        .iter()
        .filter(|field| {
            field
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("updatable"))
        })
        .map(|field| &field.ident)
        .collect::<Vec<_>>();

    let expanded = quote::quote! {
        impl ::api_model_traits::update::Updatable<#updatable_struct_name, #struct_name> for #struct_name {
            fn perform_update(&self, updatable_data: #updatable_struct_name) -> ::api_model_traits::update::UpdateResult<Self> {
                Ok(Self {
                    #(#struct_fields: updatable_data.#struct_fields.unwrap_or_else(|| self.#struct_fields.clone())),*,
                    ..self.clone()
                })
            }
        }
    };

    TokenStream::from(expanded)
}
