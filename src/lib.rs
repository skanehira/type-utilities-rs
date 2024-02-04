mod attribute;
mod refine;

use attribute::Attribute;
use proc_macro::TokenStream;
use quote::quote;
use refine::{into_optional, omit_or_pick};
use syn::{parse_macro_input, ItemStruct};

/// Omit the specified fields in the attribute
#[proc_macro_attribute]
pub fn omit(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as Attribute);
    let item = parse_macro_input!(item as ItemStruct);

    let item = omit_or_pick(attr, item, attribute::AttributeType::Omit);

    quote! {
        #item
    }
    .into()
}

/// Pick the specified fields in the attribute
#[proc_macro_attribute]
pub fn pick(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as Attribute);
    let item = parse_macro_input!(item as ItemStruct);

    let item = omit_or_pick(attr, item, attribute::AttributeType::Pick);

    quote! {
        #item
    }
    .into()
}

/// Make all fields in the struct optional
#[proc_macro_attribute]
pub fn partial(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as Attribute);
    let mut item = parse_macro_input!(item as ItemStruct);

    item.ident = syn::Ident::new(&attr.name.to_string(), item.ident.span());

    let fields = item.fields.into_iter().map(into_optional).collect();
    item.fields = syn::Fields::Named(syn::FieldsNamed {
        brace_token: syn::token::Brace::default(),
        named: fields,
    });

    quote! {
        #item
    }
    .into()
}
