pub mod field_clone;
pub mod field_deser;
pub mod field_new;
pub mod field_ser;
pub mod field_take_or_init;
pub use field_clone::FieldClone;
pub use field_deser::{FieldDeserFromIter, FieldDeserFromSlice};
pub use field_new::FieldNew;
pub use field_ser::FieldSer;
pub use field_take_or_init::FieldTakeOrInit;

use crate::ser::MessageSerializer;
use crate::tags;
use crate::Result;

trait DoDefaultCheck {
    const DO_DEFAULT_CHECK: bool = false;
}
impl DoDefaultCheck for tags::Optional3 {
    const DO_DEFAULT_CHECK: bool = true;
}
impl DoDefaultCheck for tags::Required {}
impl DoDefaultCheck for tags::Optional2 {}
impl DoDefaultCheck for tags::Repeated {}

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
