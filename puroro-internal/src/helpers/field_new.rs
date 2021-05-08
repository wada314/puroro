use std::collections::HashMap;
use std::convert::TryFrom;

use crate::types::{SliceViewRepeatedField, SliceViewScalarField};

pub trait FieldNew<'bump>: Sized {
    fn new() -> Self;
    #[cfg(feature = "puroro-bumpalo")]
    fn new_in_bumpalo(_bump: &'bump ::bumpalo::Bump) -> Self {
        Self::new()
    }
}
macro_rules! impl_field_new {
    ($type:ty) => {
        impl<'bump> FieldNew<'bump> for $type {
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
impl<'bump, T> FieldNew<'bump> for Vec<T> {
    fn new() -> Self {
        Default::default()
    }
}
impl<'bump, T> FieldNew<'bump> for Option<T> {
    fn new() -> Self {
        Default::default()
    }
}
impl<'bump, K, V> FieldNew<'bump> for HashMap<K, V> {
    fn new() -> Self {
        Default::default()
    }
}
impl<'bump, T> FieldNew<'bump> for ::std::result::Result<T, i32>
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

impl<'bump, 'slice, T: Clone> FieldNew<'bump> for SliceViewScalarField<'slice, T> {
    fn new() -> Self {
        SliceViewScalarField::NotFound
    }
}
impl<'bump, 'slice, T: Clone> FieldNew<'bump> for SliceViewRepeatedField<'slice, T> {
    fn new() -> Self {
        SliceViewRepeatedField::NotFound
    }
}
