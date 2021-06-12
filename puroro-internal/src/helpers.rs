pub mod field_clone;
pub mod field_merge_from_iter;
pub mod field_merge_from_slice;
pub mod field_new;
pub mod field_ser;
pub mod field_take_or_init;
pub mod field_types;
pub mod msg_field_types;
pub mod repeated_slice_view;
use std::borrow::Borrow;

use std::convert::TryFrom;

pub use field_clone::FieldClone;
pub use field_merge_from_iter::FieldMergeFromIter;
pub use field_merge_from_slice::FieldMergeFromSlice;
pub use field_new::FieldNew;
pub use field_ser::FieldSer;
pub use field_take_or_init::FieldTakeOrInit;
pub use field_types::{BytesType, StringType, VecType, WrappedFieldType};
pub use msg_field_types::{MapEntryWrapper, MapType, RepeatedMessageType, WrappedMessageFieldType};
pub use repeated_slice_view::RepeatedSliceViewField;

use crate::ser::MessageSerializer;
use crate::tags;
use crate::Result;

pub trait DoDefaultCheck {
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
    fn ser_kv<T, Q, R>(key: &Q, value: &R, serializer: &mut T) -> Result<()>
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
pub trait Default {
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

pub trait Bits32TypeTag {
    type NativeType;
    fn from_bytes(bytes: [u8; 4]) -> Self::NativeType;
    fn into_bytes(from: Self::NativeType) -> [u8; 4];
}
pub trait Bits64TypeTag {
    type NativeType;
    fn from_bytes(bytes: [u8; 8]) -> Self::NativeType;
    fn into_bytes(from: Self::NativeType) -> [u8; 8];
}
macro_rules! impl_fixed_bytes_type_tags {
    ($tr:ident, $tag:ty, $native:ty, $bytes:ty) => {
        impl $tr for $tag {
            type NativeType = $native;
            fn from_bytes(bytes: $bytes) -> Self::NativeType {
                <$native>::from_le_bytes(bytes)
            }
            fn into_bytes(from: Self::NativeType) -> $bytes {
                <$native>::to_le_bytes(from)
            }
        }
    };
}
impl_fixed_bytes_type_tags!(Bits32TypeTag, tags::value::Float, f32, [u8; 4]);
impl_fixed_bytes_type_tags!(Bits32TypeTag, tags::value::Fixed32, u32, [u8; 4]);
impl_fixed_bytes_type_tags!(Bits32TypeTag, tags::value::SFixed32, i32, [u8; 4]);
impl_fixed_bytes_type_tags!(Bits64TypeTag, tags::value::Double, f64, [u8; 8]);
impl_fixed_bytes_type_tags!(Bits64TypeTag, tags::value::Fixed64, u64, [u8; 8]);
impl_fixed_bytes_type_tags!(Bits64TypeTag, tags::value::SFixed64, i64, [u8; 8]);
