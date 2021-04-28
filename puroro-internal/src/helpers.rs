mod deser_field;
mod map_entry;
mod maybe_repeated_field;
mod maybe_repeated_variant_field;
pub use maybe_repeated_field::MaybeRepeatedField;
pub use maybe_repeated_variant_field::MaybeRepeatedVariantField;

use crate::types::FieldData;
use std::collections::HashMap;
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

pub trait InternalData {
    #[cfg(feature = "puroro-bumpalo")]
    fn bumpalo(&self) -> &bumpalo::Bump {
        panic!("The Bumpalo data field is only available for a Bumpalo struct!")
    }
}

#[derive(Debug, Clone)]
pub struct InternalDataForNormalStruct {
    unknown_fields: Option<HashMap<usize, FieldData<Vec<u8>>>>,
}
impl InternalDataForNormalStruct {
    pub fn new() -> Self {
        Self {
            unknown_fields: None,
        }
    }
}
impl InternalData for InternalDataForNormalStruct {}

#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct InternalDataForBumpaloStruct<'b> {
    // No hashmap implementation in bumpalo...
    unknown_fields: Option<
        ::bumpalo::collections::Vec<'b, (usize, FieldData<::bumpalo::collections::Vec<'b, u8>>)>,
    >,
    bump: &'b ::bumpalo::Bump,
}

#[cfg(feature = "puroro-bumpalo")]
impl<'b> InternalDataForBumpaloStruct<'b> {
    pub fn new(bump: &'b ::bumpalo::Bump) -> Self {
        Self {
            unknown_fields: None,
            bump,
        }
    }
}
impl<'b> InternalData for InternalDataForBumpaloStruct<'b> {
    /// Note that the returned bumpalo lifetime is not `'b' but `'_`.
    /// This is because I don't want to introduce the lifetime parameter
    /// `'b` into the trait's definition. The lifetime `'_` might be shorter
    /// than `'b`, but I believe it's not a problem.
    fn bumpalo(&self) -> &bumpalo::Bump {
        self.bump
    }
}
