
/// This crate should not be used on it's own, it implements the (IntoQuery trait)[https://docs.rs/into_query]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

fn get_types(data: &syn::Data) -> Vec<(proc_macro2::Ident, syn::Type)> {
    let mut map = vec![];
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
        ..
    }) = data.to_owned()
    {
        let mut iter = named.into_iter();
        while let Some(syn::Field {
            ident: Some(ident),
            ty,
            ..
        }) = iter.next()
        {
            map.push((ident, ty));
        }
    }

    map
}

fn is_vec(ty: &syn::Type) -> bool {
    match ty.to_owned() {
        syn::Type::Path(syn::TypePath {
            path: syn::Path { segments, .. },
            ..
        }) => match segments.iter().next() {
            Some(syn::PathSegment { ident, arguments })
                if ident == &syn::Ident::new("Option", proc_macro2::Span::call_site()) =>
            {
                match arguments {
                    syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                        args,
                        ..
                    }) => match args.iter().next() {
                        Some(syn::GenericArgument::Type(ty2)) => is_vec(ty2),
                        _ => false,
                    },
                    _ => unreachable!(),
                }
            }

            Some(syn::PathSegment { ident, .. })
                if ident == &syn::Ident::new("Vec", proc_macro2::Span::call_site()) =>
            {
                true
            }
            _ => false,
        },
        _ => false,
    }
}

fn attr_ident(attr: &syn::Attribute) -> Option<&syn::Ident> {
    let syn::Path { segments, .. } = &attr.path;
    if let Some(syn::PathSegment { ident, .. }) = segments.iter().next() { 
        Some(ident)
    } else {
        None
    }
}
fn attr_lit(attr: &syn::Attribute) -> Option<proc_macro2::Literal> {
    match attr.tokens.clone().into_iter().nth(1) {
        Some(proc_macro2::TokenTree::Literal(lit)) => Some(lit),
        _ => None,
    }
}

fn literal_to_ident(lit: proc_macro2::Literal) -> syn::Ident {
    let mut s = format!("{}", lit);
    s.pop();
    s.remove(0);
    syn::Ident::new(&s, proc_macro2::Span::call_site())
}

fn table_name(item: &syn::DeriveInput) -> Option<syn::Ident> {
    item.attrs
        .iter()
        .map(|attr| (attr_ident(attr).unwrap(), attr_lit(attr).unwrap()))
        .find(|(ident, _)| {
            ident == &&syn::Ident::new("table_name", proc_macro2::Span::call_site())
        })
        .map(|optional| literal_to_ident(optional.1))
}

#[proc_macro_derive(IntoQuery, attributes(table_name))]
pub fn derive_into_query(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let types = get_types(&input.data);
    let struct_ident = &input.ident;
    let table = table_name(&input).expect("table_name not specified");

    let body = types
        .iter()
        .map(|(ident, ty)| {
            if is_vec(ty) {
                quote! {
                    if let Some(container) = self.#ident {
                        let mut iter = container.into_iter();
                        if let Some(first) = iter.next() {
                            query = query.filter(#ident.eq(first));
                            for item in iter {
                                query = query.or_filter(#ident.eq(item));
                            }
                        }
                    }
                }
            } else {
                quote! {
                    if let Some(item) = self.#ident {
                        query = query.filter(#ident.eq(item));
                    }
                }
            }
        })
        .fold(TokenStream2::new(), |mut acc, cur| {
            acc.extend(cur.into_iter());
            acc
        });

    let gen = quote! {
        impl ::into_query::IntoQuery<crate::db::schema::#table::dsl::#table> for #struct_ident {
            fn into_query(
                self,
            ) -> diesel::helper_types::IntoBoxed<
                'static,
                crate::db::schema::#table::dsl::#table,
                diesel::mysql::Mysql,
            > {
                use crate::db::schema::#table::dsl::*;
                use diesel::prelude::*;
                let mut query = #table.into_boxed();
                #body
                query
            }
        }
    };
    gen.into()
}
