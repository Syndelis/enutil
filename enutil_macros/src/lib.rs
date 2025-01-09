use syn::DeriveInput;

mod macros;
mod utils;

/// A trait for grouping together different new-type variants that dereference to
/// the same type, reducing boilerplate code.
///
/// Implements [`std::ops::Deref`] and [`std::ops::DerefMut`] for enums composed
/// of  solely of new-type-style variants.
///
/// The variants must hold "compatible" types, which means they must be deref-able
/// to the same type (e.g. String and &str, Box<T> and T).
///
/// The macro accompanies the macro-attribute `enum_deref_target` to define what
/// type the enum should deref to.
///
/// ```
/// use enutil::EnumDeref;
/// use enutil_macros::EnumDeref;
///
/// #[derive(EnumDeref)]
/// #[enum_deref_target([u32])]
/// enum Collection {
///     Fixed([u32; 2]),
///     Dynamic(Vec<u32>),
/// }
///
/// let mut f = Collection::Fixed([5, 6]);
/// assert_eq!(&*f, &[5, 6]);
///
/// f[0] = 10;
/// assert_eq!(&*f, &[10, 6]);
///
/// let mut d = Collection::Dynamic(Vec::from([9]));
/// assert_eq!(&*d, &[9]);
///
/// d[0] = 1;
/// assert_eq!(&*d, &[1]);
/// ```
///
/// You may also deref trait object references using `dyn Trait` for something
/// resembling a `Box<T>`, but with all the benefits of having an enum, such as
/// compile-time known variants.
///
/// ```
/// use enutil::EnumDeref;
/// use enutil_macros::EnumDeref;
///
/// #[derive(EnumDeref)]
/// #[enum_deref_target(dyn core::fmt::Display)]
/// enum Column {
///     Text(String),
///     Number(i32),
///     Float(f32),
/// }
///
/// let t = Column::Text("Hi!".to_string());
/// assert_eq!((*t).to_string(), "Hi!");
///
/// let n = Column::Number(4);
/// assert_eq!((*n).to_string(), "4");
///
/// let f = Column::Float(5.125);
/// assert_eq!((*f).to_string(), "5.125");
///
/// fn prints_something(smth: &dyn core::fmt::Display) {
///     println!("{smth}");
/// }
///
/// prints_something(&*t);
/// prints_something(&*n);
/// prints_something(&*f);
/// ```
#[proc_macro_derive(EnumDeref, attributes(enum_deref_target))]
pub fn enum_deref(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let toks =
        macros::enum_deref::enum_deref_inner(&ast).unwrap_or_else(|err| err.to_compile_error());

    toks.into()
}

#[proc_macro_derive(IntoFieldVecs)]
pub fn into_field_vecs(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    let toks = macros::into_field_vecs::into_field_vecs_inner(&ast)
        .unwrap_or_else(|err| err.to_compile_error());

    toks.into()
}
