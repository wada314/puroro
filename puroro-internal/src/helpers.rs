pub mod field_clone;
pub mod field_deser;
pub mod field_new;
pub mod field_ser;
pub mod field_take_or_init;
pub mod repeated_slice_view;
use std::borrow::Borrow;
use std::convert::TryFrom;

pub use field_clone::FieldClone;
pub use field_deser::{FieldDeserFromIter, FieldDeserFromSlice};
pub use field_new::FieldNew;
pub use field_ser::FieldSer;
pub use field_take_or_init::FieldTakeOrInit;
pub use repeated_slice_view::RepeatedSliceViewField;

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

pub trait MapEntryForNormalImpl {
    type OwnedKeyType;
    type OwnedValueType;
    fn into_tuple(self) -> (Self::OwnedKeyType, Self::OwnedValueType);
    fn ser_kv<T: MessageSerializer, Q, R>(key: &Q, value: &R, serializer: &mut T) -> Result<()>
    where
        T: MessageSerializer,
        Self::OwnedKeyType: Borrow<Q>,
        Self::OwnedValueType: Borrow<R>;
}

pub trait MapEntryForSliceViewImpl<'slice> {
    type OwnedKeyType;
    type ValueGetterType;
    fn key_eq<Q>(&self, key: &Q) -> bool
    where
        Self::OwnedKeyType: Borrow<Q>,
        Q: Eq + ?Sized;
    fn value(&self) -> Self::ValueGetterType;
}

/// An alternative for `std::default::Default` just only because of `Result<Enum, i32>`.
trait Default {
    fn default() -> Self;
}
macro_rules! impl_default {
    ($ty:ty) => {
        impl Default for $ty {
            fn default() -> Self {
                ::std::default::Default::default()
            }
        }
    };
}
impl_default!(i32);
impl_default!(i64);
impl_default!(u32);
impl_default!(u64);
impl_default!(f32);
impl_default!(f64);
impl_default!(bool);
impl_default!(String);
impl_default!(Vec<u8>);
impl<T: TryFrom<i32, Error = i32>> Default for ::std::result::Result<T, i32> {
    fn default() -> Self {
        T::try_from(0i32)
    }
}
