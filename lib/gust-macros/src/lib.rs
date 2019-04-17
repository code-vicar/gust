extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

mod has_id_macro;
use has_id_macro::*;

#[proc_macro_derive(HasID, attributes(gust))]
pub fn has_id_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_has_id_macro(&ast)
}
