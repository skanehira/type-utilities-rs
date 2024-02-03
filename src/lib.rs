mod attribute;
mod refine;

use attribute::Attribute;
use proc_macro::TokenStream;
use quote::quote;
use refine::omit_or_pick;
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
