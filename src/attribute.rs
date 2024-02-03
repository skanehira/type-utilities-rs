use std::collections::HashMap;
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    Ident, Token,
};

#[derive(PartialEq, Eq, Clone)]
pub(crate) enum AttributeType {
    Omit,
    Pick,
}

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct Attribute {
    /// New struct name
    pub(crate) name: Ident,
    /// Specified fields in the attribute
    pub(crate) fields: HashMap<Ident, ()>,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;

        let mut fields = HashMap::new();
        if !input.peek(Token![,]) {
            return Ok(Attribute { name, fields });
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
                "Attribute must have at least one field",
            ));
        }

        Ok(Attribute { name, fields })
    }
}
