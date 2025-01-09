#[cfg(feature = "derive")]
pub use enutil_macros::*;

/// A trait for reducing boilerplate code related to common access patterns in
/// new-type-like variant enums. Because of this, for derived usage, all the
/// variants in the enumerator need to be new-types (e.g. Int(i32), Person(Person))
pub trait EnumDeref: std::ops::Deref + std::ops::DerefMut {}

impl<T: std::ops::Deref + std::ops::DerefMut> EnumDeref for T {}

/// A trait for converting a Vec<T> into multiple Vec's for each field of T
/// (Vec<T.a>, Vec<T.b>, ...).
///
/// Its intended use case is to allow for inserting multiple rows to the
/// database when using [SQLx]'s macros. [Read more]
///
/// [SQLx]: https://docs.rs/sqlx
/// [Read more]: https://github.com/launchbadge/sqlx/blob/main/FAQ.md#how-can-i-bind-an-array-to-a-values-clause-how-can-i-do-bulk-inserts
pub trait IntoFieldVecs: Sized {
    type Target;

    fn into_field_vecs(rows: Vec<Self>) -> Self::Target;
}
