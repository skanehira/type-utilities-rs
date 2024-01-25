use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, ItemStruct, Token,
};

#[allow(dead_code)]
#[derive(Debug)]
struct Omit {
    name: Ident,
    fields: HashMap<Ident, ()>,
}

impl Parse for Omit {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![,]>()?;

        let fields = {
            let content;
            bracketed!(content in input);

            let mut fields = HashMap::new();
            while !content.is_empty() {
                fields.insert(content.parse()?, ());
                if content.is_empty() {
                    break;
                }
                content.parse::<Token![,]>()?;
            }
            fields
        };

        Ok(Omit { name, fields })
    }
}

#[proc_macro_attribute]
pub fn omit(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as Omit);
    let struct_name = attr.name;

    let item = parse_macro_input!(item as ItemStruct);
    let fields: Vec<_> = item
        .fields
        .into_iter()
        .filter_map(|field| {
            if let Some(ref ident) = field.ident {
                if attr.fields.contains_key(ident) {
                    return None;
                }
            };
            Some(field)
        })
        .collect();

    if fields.is_empty() {
        quote! {
            struct #struct_name;
        }
        .into()
    } else {
        quote! {
            struct #struct_name {
                #(#fields),*
            }
        }
        .into()
    }
}
