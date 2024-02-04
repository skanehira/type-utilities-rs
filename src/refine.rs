use crate::attribute::{Attribute, AttributeType};
use syn::{punctuated::Punctuated, token, Field, ItemStruct};

pub(super) fn omit_or_pick(
    attr: Attribute,
    mut item: ItemStruct,
    attr_type: AttributeType,
) -> ItemStruct {
    item.ident = syn::Ident::new(&attr.name.to_string(), item.ident.span());

    let is_tuple_or_unit = matches!(item.fields, syn::Fields::Unnamed(_) | syn::Fields::Unit);

    // If the attribute is Omit or Pick, and the struct is not a tuple or unit, then we filter the fields
    if (matches!(attr_type, AttributeType::Omit | AttributeType::Pick)) && !is_tuple_or_unit {
        let fields = item
            .fields
            .clone()
            .into_iter()
            .filter(|field| {
                let should_pick = matches!(attr_type, AttributeType::Pick);
                if let Some(ref ident) = field.ident {
                    if attr.fields.contains_key(ident) {
                        return should_pick;
                    }
                };
                !should_pick
            })
            .collect();

        item.fields = syn::Fields::Named(syn::FieldsNamed {
            brace_token: syn::token::Brace::default(),
            named: fields,
        });
    }

    item
}
