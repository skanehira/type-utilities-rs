use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token, Field, Ident, ItemStruct, Token,
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
