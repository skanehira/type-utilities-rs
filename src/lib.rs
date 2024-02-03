mod attribute;
mod omit;

use attribute::Attribute;
use proc_macro::TokenStream;
use quote::quote;
use structure::Struct;
use syn::{parse_macro_input, punctuated::Punctuated, token, Field, ItemStruct};

#[proc_macro_attribute]
pub fn omit(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as Attribute);
    let mut item = parse_macro_input!(item as ItemStruct);

    item.ident = syn::Ident::new(&attr.name.to_string(), item.ident.span());

    let is_tuple_or_unit = matches!(item.fields, syn::Fields::Unnamed(_) | syn::Fields::Unit);

    if !is_tuple_or_unit {
        let fields: Punctuated<Field, token::Comma> = item
            .fields
            .into_iter()
            .filter(|field| {
                if let Some(ref ident) = field.ident {
                    if attr.fields.contains_key(ident) {
                        return false;
                    }
                };
                true
            })
            .collect();

        item.fields = syn::Fields::Named(syn::FieldsNamed {
            brace_token: syn::token::Brace::default(),
            named: fields,
        });
    }

    quote! {
        #item
    }
    .into()
}
