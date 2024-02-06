
/// A trait for reducing boilerplate code related to common access patterns in
/// new-type-like variant enums. Because of this, for derived usage, all the
/// variants in the enumerator need to be new-types (e.g. Int(i32), Person(Person))
pub trait EnumDeref: std::ops::Deref + std::ops::DerefMut {}

impl<T: std::ops::Deref + std::ops::DerefMut> EnumDeref for T {}
