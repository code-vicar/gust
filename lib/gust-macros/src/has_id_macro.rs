use proc_macro::TokenStream;
use quote::quote;
use syn;

fn get_id_field(body: &syn::Data) -> Option<&syn::Field> {
    match body {
        &syn::Data::Enum(_) => panic!("HasID can not be implemented for enums"),
        &syn::Data::Union(_) => panic!("HasID can not be implemented for unions"),
        &syn::Data::Struct(ref s) => {
            if let syn::Fields::Named(ref named_fields) = s.fields {
                for field in &named_fields.named {
                    if field.attrs.iter().find(|a| is_gust_id_attr(a)).is_some() {
                        return Some(field);
                    }
                }
            }
            None
        }
    }
}

fn is_gust_id_attr(attr: &syn::Attribute) -> bool {
    is_gust_attr(attr) && is_id_attr(attr)
}

fn is_gust_attr(attr: &syn::Attribute) -> bool {
    attr.style == syn::AttrStyle::Outer && attr.path.segments.len() >= 1 && attr.path.segments[0].ident == "gust"
}

fn is_id_attr(attr: &syn::Attribute) -> bool {
    let attr_body = attr.tts.clone();
    attr_body.into_iter().any(|token|
        match token {
            // look for a group token
            quote::__rt::TokenTree::Group(group) => {
                if group.delimiter() != quote::__rt::Delimiter::Parenthesis {
                    return false;
                }
                let stream = group.stream();
                if stream.is_empty() {
                    return false;
                }
                stream.into_iter().any(|token|
                    match token {
                        // look for an Ident token within the group
                        quote::__rt::TokenTree::Ident(i) => {
                            i.to_string() == "id"
                        }
                        _ => false
                    }
                )
            }
            _ => false
        }
    )
}

pub fn impl_has_id_macro(ast: &syn::DeriveInput) -> TokenStream {
    let ident = &ast.ident;
    let generics = &ast.generics;
    let where_clause = &ast.generics.where_clause;
    let id_field = get_id_field(&ast.data).unwrap_or_else(||
        panic!("HasID is expecting an attribute marked with #[gust(id)]")
    );
    let id_name = id_field.ident.as_ref().unwrap_or_else(||
        panic!("HasID id attribute must have an identifier")
    );
    let id_type = &id_field.ty;
    let gen = quote! {
        impl HasID for #ident #generics #where_clause {
            type ID_TYPE = #id_type;

            fn get_id(&self) -> &Self::ID_TYPE {
                &self.#id_name
            }
        }
    };
    gen.into()
}
