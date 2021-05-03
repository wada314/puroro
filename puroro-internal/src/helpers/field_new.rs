use std::convert::TryFrom;

pub trait FieldNew<'a>: Sized {
    fn new() -> Self;
    #[cfg(feature = "puroro-bumpalo")]
    fn new_in_bumpalo(_bump: &'a ::bumpalo::Bump) -> Self {
        Self::new()
    }
}
macro_rules! impl_field_new {
    ($type:ty) => {
        impl<'a> FieldNew<'a> for $type {
            fn new() -> Self {
                Default::default()
            }
        }
    };
}
impl_field_new!(i32);
impl_field_new!(i64);
impl_field_new!(u32);
impl_field_new!(u64);
impl_field_new!(f32);
impl_field_new!(f64);
impl_field_new!(bool);
impl_field_new!(::std::string::String);
impl<'a, T> FieldNew<'a> for ::std::vec::Vec<T> {
    fn new() -> Self {
        ::std::vec::Vec::new()
    }
}
impl<'a, T> FieldNew<'a> for ::std::option::Option<T> {
    fn new() -> Self {
        None
    }
}
impl<'a, T> FieldNew<'a> for ::std::result::Result<T, i32>
where
    T: TryFrom<i32, Error = i32>,
{
    fn new() -> Self {
        T::try_from(0i32)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump, T> FieldNew<'bump> for ::bumpalo::collections::Vec<'bump, T> {
    fn new() -> Self {
        unimplemented!("this field must be initialized from new_in_bumpalo!")
    }
    fn new_in_bumpalo(bump: &'bump bumpalo::Bump) -> Self {
        ::bumpalo::collections::Vec::new_in(bump)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> FieldNew<'bump> for ::bumpalo::collections::String<'bump> {
    fn new() -> Self {
        unimplemented!("this field must be initialized from new_in_bumpalo!")
    }
    fn new_in_bumpalo(bump: &'bump bumpalo::Bump) -> Self {
        ::bumpalo::collections::String::new_in(bump)
    }
}
