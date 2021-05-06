pub mod field_clone;
pub mod field_deser;
pub mod field_new;
pub mod field_ser;
pub use field_clone::FieldClone;
pub use field_deser::FieldDeserFromIter;
pub use field_new::FieldNew;
pub use field_ser::FieldSer;

use crate::ser::MessageSerializer;
use crate::tags;
use crate::types::FieldData;
use crate::Result;
use ::puroro::InternalData;
use std::collections::HashMap;

pub trait MapEntry {
    // Note: `KeyType` (or `ValueType`) != the message's `key` (or `value`) field type.
    // The latter may be wrapped by `Option` or `Box<>` depend on the label or the type,
    // but the `KeyType` must be a bare type which will be used for the `HashMap`'s
    // generic param.
    type KeyType;
    type ValueType;
    fn into_tuple(self) -> (Self::KeyType, Self::ValueType);
    fn ser_kv<T: MessageSerializer>(
        key: &Self::KeyType,
        value: &Self::ValueType,
        serializer: &mut T,
    ) -> Result<()>;
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
impl<'bump> InternalData<'bump> for InternalDataForNormalStruct {
    #[cfg(feature = "puroro-bumpalo")]
    fn bumpalo(&self) -> &'bump bumpalo::Bump {
        panic!("The Bumpalo data field is only available for a Bumpalo struct!")
    }
}

#[cfg(feature = "puroro-bumpalo")]
#[derive(Debug, Clone)]
pub struct InternalDataForBumpaloStruct<'bump> {
    // No hashmap implementation in bumpalo...
    unknown_fields: Option<
        ::bumpalo::collections::Vec<
            'bump,
            (usize, FieldData<::bumpalo::collections::Vec<'bump, u8>>),
        >,
    >,
    bump: &'bump ::bumpalo::Bump,
}

#[cfg(feature = "puroro-bumpalo")]
impl<'bump> InternalDataForBumpaloStruct<'bump> {
    pub fn new(bump: &'bump ::bumpalo::Bump) -> Self {
        Self {
            unknown_fields: None,
            bump,
        }
    }
}
impl<'bump> InternalData<'bump> for InternalDataForBumpaloStruct<'bump> {
    /// Note that the returned bumpalo lifetime is not `'bump' but `'_`.
    /// This is because I don't want to introduce the lifetime parameter
    /// `'b` into the trait's definition. The lifetime `'_` might be shorter
    /// than `'b`, but I believe it's not a problem.
    fn bumpalo(&self) -> &'bump bumpalo::Bump {
        self.bump
    }
}
