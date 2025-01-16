use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput};

use crate::utils::errors::non_struct_error;

pub fn into_field_vecs_inner(ast: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let gen = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();

    let fields: Vec<&_> = match &ast.data {
        Data::Struct(s) => s.fields.iter().collect(),
        _ => return Err(non_struct_error()),
    };

    let (types, idents, vecs): (Vec<_>, Vec<_>, Vec<_>) = fields
        .iter()
        .filter(|field| field.ident.is_some())
        .map(|field| {
            let ty = field.ty.to_token_stream();
            let ident = field.ident.as_ref().unwrap();
            let vec = syn::Ident::new(&format!("vec_{ident}"), ident.span());
            (ty, ident, vec)
        })
        .fold(
            Default::default(),
            |(mut vec_types, mut vec_idents, mut vec_vecs), (types, idents, vecs)| {
                vec_types.push(types);
                vec_idents.push(idents);
                vec_vecs.push(vecs);
                (vec_types, vec_idents, vec_vecs)
            },
        );

    let field_arrays_type_name = syn::Ident::new(&format!("{name}FieldVecs"), name.span());

    Ok(quote! {
        pub type #field_arrays_type_name = (
            #( Vec<#types> ),*
        );

        impl #impl_generics ::enutil::IntoFieldVecs for #name #ty_generics #where_clause {
            type Target = #field_arrays_type_name;

            fn into_field_vecs(els: Vec<Self>) -> Self::Target {
                els.into_iter()
                    .map(|el| (
                        #( el.#idents ),*
                    ))
                    .fold((#( Vec::<#types>::new() ),*), |( #( mut #vecs ),* ), ( #( #idents ),* )| {
                        #( #vecs.push(#idents); )*
                        ( #( #vecs ),* )
                    })
            }
        }
    })
}
