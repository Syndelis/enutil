use syn::{Fields, FieldsUnnamed, Ident, Type, Variant};

use self::errors::non_new_type_variant_error;

pub mod errors;

pub fn get_new_type_variant(enum_variant: &Variant) -> syn::Result<(Ident, Type)> {
    match &enum_variant.fields {
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            if unnamed.len() != 1 {
                Err(non_new_type_variant_error(
                    "the list of type parameters is different from 1",
                ))
            } else if let Some(new_type) = unnamed.first() {
                Ok((enum_variant.ident.clone(), new_type.ty.clone()))
            } else {
                unreachable!("`unnamed.len()` must be 1 in the previous branch, so `.first()` should not return `None`");
            }
        }
        _ => Err(non_new_type_variant_error(&format!(
            "the variant {} is not a tuple-struct",
            enum_variant.ident
        ))),
    }
}
