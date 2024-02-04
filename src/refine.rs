use crate::attribute::{Attribute, AttributeType};
use quote::quote;
use syn::{Field, ItemStruct, Type};

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

pub(super) fn into_optional(mut field: Field) -> Field {
    let ty = match field.ty {
        Type::Path(ref type_path) => {
            // when type is Option<T>, do nothing
            if let Some(seg) = type_path.path.segments.first() {
                if seg.ident == "Option" {
                    return field;
                }
            }
            syn::parse2(quote::quote! { Option<#type_path> })
                .expect("Faield to wrap Option around type")
        }
        Type::Array(ty) => syn::parse2(quote::quote! { Option<#ty> })
            .expect("Faield to wrap Option around type array"),
        Type::Tuple(ty) => syn::parse2(quote::quote! { Option<#ty> })
            .expect("Faield to wrap Option around type tuple"),
        Type::Ptr(ty) => {
            syn::parse2(quote! { Option<#ty> }).expect("Faield to wrap Option around type ptr")
        }
        Type::Reference(ty) => syn::parse2(quote::quote! { Option<#ty> })
            .expect("Faield to wrap Option around type reference"),
        // trait object and slice will cannot be know at compilation time when wrap with Option
        // we wrap with option because we should notify error message
        Type::TraitObject(ty) => syn::parse2(quote::quote! { Option<#ty> })
            .expect("Faield to wrap Option around type trait object"),
        Type::Slice(ty) => syn::parse2(quote::quote! { Option<#ty> })
            .expect("Faield to wrap Option around type slice"),
        _ => unreachable!("Unsupported type"),
    };

    field.ty = ty;
    field
}
