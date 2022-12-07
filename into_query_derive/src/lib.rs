/// This crate should n ot be used on it's own, it implements the (IntoQuery trait)[https://docs.rs/into_query]
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::collections::HashMap;
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
    syn::Ident::new(&literal_to_string(lit), proc_macro2::Span::call_site())
}
fn literal_to_string(lit: proc_macro2::Literal) -> String {
    let mut s = format!("{}", lit);
    s.pop();
    s.remove(0);
    s
}

fn str_to_segments<S: ToString>(
    s: S,
) -> syn::punctuated::Punctuated<syn::PathSegment, syn::Token![::]> {
    let s = s.to_string();
    s.split("::")
        .fold(syn::punctuated::Punctuated::new(), |mut segs, cur| {
            segs.push(str_to_seg(cur));
            segs
        })
}

fn str_to_seg<S: ToString>(s: S) -> syn::PathSegment {
    syn::PathSegment {
        ident: syn::Ident::new(&s.to_string(), proc_macro2::Span::call_site()),
        arguments: syn::PathArguments::None,
    }
}

fn table_name(item: &syn::DeriveInput) -> Option<syn::Ident> {
    attrs(item)
        .get(&syn::Ident::new(
            "table_name",
            proc_macro2::Span::call_site(),
        ))
        .map(|lit| literal_to_ident(lit.clone()))
}
fn schema_prefix(item: &syn::DeriveInput) -> Option<String> {
    attrs(item)
        .get(&syn::Ident::new(
            "schema_prefix",
            proc_macro2::Span::call_site(),
        ))
        .map(|lit| literal_to_string(lit.clone()))
}
fn attrs(item: &syn::DeriveInput) -> HashMap<syn::Ident, proc_macro2::Literal> {
    item.attrs
        .iter()
        .filter(|attr| attr_lit(attr).is_some() && attr_ident(attr).is_some())
        .map(|attr| (attr_ident(attr).unwrap(), attr_lit(attr).unwrap()))
        .fold(HashMap::new(), |mut map, (label, ident)| {
            map.insert(label.clone(), ident);
            map
        })
}

#[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
#[proc_macro_derive(IntoQuery, attributes(table_name, schema_prefix))]
pub fn derive_into_query(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let types = get_types(&input.data);
    let struct_ident = &input.ident;
    let table = table_name(&input).expect("table_name not specified");
    let schema_prefix = match schema_prefix(&input) {
        Some(prefix) => str_to_segments(format!("{}::schema", prefix.to_string())),
        None => str_to_segments("crate::schema"),
    };

    let body = types
        .iter()
        .map(|(ident, ty)| {
            if is_vec(ty) {
                quote! {
                    if let Some(container) = self.#ident {
                        let mut iter = container.into_iter();
                        let values = iter.collect::<Vec<_>>();
                        query = query.filter(#ident.eq_any(values));
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

    #[cfg(feature = "mysql")]
    let db = quote! { diesel::mysql::Mysql };
    #[cfg(feature = "postgres")]
    let db = quote! { diesel::pg::Pg };
    #[cfg(feature = "sqlite")]
    let db = quote! { diesel::sqlite::Sqlite };

    let gen = quote! {
        impl ::into_query::IntoQuery<#schema_prefix::#table::dsl::#table, #db> for #struct_ident {
            fn into_query(
                self,
            ) -> diesel::helper_types::IntoBoxed<
                'static,
                #schema_prefix::#table::dsl::#table,
                #db,
            > {
                use #schema_prefix::#table::dsl::*;
                use diesel::prelude::*;
                let mut query = #table.into_boxed();
                #body
                query
            }
        }
    };
    gen.into()
}
