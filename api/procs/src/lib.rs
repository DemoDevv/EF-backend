use proc_macro::TokenStream;

extern crate proc_macro2;
extern crate quote;
extern crate syn;

extern crate proc_macro;

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    let expanded = quote::quote! {
        impl Struct {
            fn update(&self, updatable_data: i32) -> Self {
                Self {
                    0: updatable_data
                }
            }
        }
    };
    TokenStream::from(expanded)
}
