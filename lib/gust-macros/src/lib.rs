extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

mod node_id_macro;
use node_id_macro::*;

#[proc_macro_derive(NodeID, attributes(gust))]
pub fn node_id_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_node_id_macro(&ast)
}
