use std::collections::HashMap;
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    Ident, Token,
};

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct Struct {
    pub(crate) name: Ident,
    pub(crate) fields: HashMap<Ident, ()>,
}

impl Parse for Struct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;

        let mut fields = HashMap::new();
        if !input.peek(Token![,]) {
            return Ok(Struct { name, fields });
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

        Ok(Struct { name, fields })
    }
}
