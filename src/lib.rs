use proc_macro::TokenStream;
use quote::{quote, ToTokens as _};
use std::collections::HashMap;
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

        let mut fields = HashMap::new();
        if !input.peek(Token![,]) {
            return Ok(Omit { name, fields });
        }

        input.parse::<Token![,]>()?;

        let content;
        bracketed!(content in input);

        while !content.is_empty() {
            fields.insert(content.parse()?, ());
            if content.is_empty() {
                break;
            }
            content.parse::<Token![,]>()?;
        }

        if fields.is_empty() {
            return Err(syn::Error::new(
                content.span(),
                "omit attribute must have at least one field",
            ));
        }

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

    let vis = item.vis.to_token_stream();

    if fields.is_empty() {
        quote! {
            #vis struct #struct_name;
        }
        .into()
    } else {
        quote! {
            #vis struct #struct_name {
                #(#fields),*
            }
        }
        .into()
    }
}
