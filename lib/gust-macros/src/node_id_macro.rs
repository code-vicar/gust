use proc_macro::TokenStream;
use quote::quote;
use syn;

fn is_gust_attr(attr: &syn::Attribute) -> bool {
    attr.style == syn::AttrStyle::Outer && attr.path.segments.len() >= 1 && attr.path.segments[0].ident == "gust"
}

fn get_id_type(attr: &syn::Attribute) -> Option<syn::Ident> {
    if !is_gust_attr(attr) {
        return None
    }
    let attr_body = attr.tts.clone();
    for token in attr_body.into_iter() {
        match token {
            // look for a group token
            quote::__rt::TokenTree::Group(group) => {
                if group.delimiter() != quote::__rt::Delimiter::Parenthesis {
                    return None;
                }
                let stream = group.stream();
                if stream.is_empty() {
                    return None;
                }
                let mut tokens = Vec::new();
                let mut idx = 0;
                for token in stream.into_iter() {
                    if idx == 3 {
                        break;
                    }
                    tokens.push(token);
                    idx = idx + 1;
                }
                if tokens.len() != 3 {
                    return None;
                }
                // ensure syntax of node_id=type
                match (tokens.get(0).unwrap(), tokens.get(1).unwrap(), tokens.get(2).unwrap()) {
                    (quote::__rt::TokenTree::Ident(ref lhs), quote::__rt::TokenTree::Punct(ref p), quote::__rt::TokenTree::Ident(ref i)) => {
                        if lhs.to_string() == "node_id" && p.as_char() == '=' {
                            return Some(i.to_owned());
                        }
                    }
                    _ => ()
                };
            }
            _ => ()
        }
    }
    None
}

fn get_id_type_in_attrs(attrs: &[syn::Attribute]) -> Option<syn::Ident> {
    for attr in attrs {
        let id_type = get_id_type(attr);
        if id_type.is_some() {
            return id_type
        }
    }
    None
}

pub fn impl_node_id_macro(ast: &syn::DeriveInput) -> TokenStream {
    let ident = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let id_type = get_id_type_in_attrs(&ast.attrs).unwrap_or_else(||
        panic!("Missing gust macro NodeID.  Expecting a type in attribute #[gust(node_id=...)]")
    );
    let gen = quote! {
        impl #impl_generics NodeID for #ident #ty_generics #where_clause {
            type ID_TYPE = #id_type;
        }
    };
    gen.into()
}
