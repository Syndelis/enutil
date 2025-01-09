use proc_macro2::Span;

pub fn non_enum_error() -> syn::Error {
    syn::Error::new(Span::call_site(), "This macro only supports enums.")
}

pub fn non_struct_error() -> syn::Error {
    syn::Error::new(Span::call_site(), "This macro only supports structs.")
}

pub fn no_associated_deref_type_specified() -> syn::Error {
    syn::Error::new(
        Span::call_site(),
        "expected a deref target specified via attribute, e.g. #[enum_deref_target(T)]",
    )
}

pub fn non_new_type_variant_error(additional_info: &str) -> syn::Error {
    syn::Error::new(
        Span::call_site(),
        format!(
            "This macro only supports enums of strictly new type variants, but {additional_info}"
        ),
    )
}
