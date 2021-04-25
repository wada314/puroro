use crate::deser::BytesIter;
use crate::tags;
use crate::tags::{FieldLabelTag, FieldTypeAndLabelTag, FieldTypeTag};
use crate::types::FieldData;
use crate::{ErrorKind, PuroroError, Result};
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

pub trait NotOptionaInNoPresenceDiscipline {}
impl NotOptionaInNoPresenceDiscipline for tags::Int32 {}
impl NotOptionaInNoPresenceDiscipline for tags::Int64 {}
impl NotOptionaInNoPresenceDiscipline for tags::UInt32 {}
impl NotOptionaInNoPresenceDiscipline for tags::UInt64 {}
impl NotOptionaInNoPresenceDiscipline for tags::SInt32 {}
impl NotOptionaInNoPresenceDiscipline for tags::SInt64 {}
impl NotOptionaInNoPresenceDiscipline for tags::Bool {}
impl NotOptionaInNoPresenceDiscipline for tags::Bytes {}
impl NotOptionaInNoPresenceDiscipline for tags::String {}
impl NotOptionaInNoPresenceDiscipline for tags::Float {}
impl NotOptionaInNoPresenceDiscipline for tags::Double {}
impl NotOptionaInNoPresenceDiscipline for tags::Fixed32 {}
impl NotOptionaInNoPresenceDiscipline for tags::Fixed64 {}
impl NotOptionaInNoPresenceDiscipline for tags::SFixed32 {}
impl NotOptionaInNoPresenceDiscipline for tags::SFixed64 {}
impl<T> NotOptionaInNoPresenceDiscipline for tags::Enum<T> {}

pub trait DeserializableFromBytesField<T>
where
    T: FieldTypeAndLabelTag,
{
    type Item;
    fn deser<'a, I, F>(&mut self, field: FieldData<BytesIter<'a, I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: FnOnce() -> Self::Item;
}

macro_rules! define_deser_req_variants {
    ($ty:ty, $ttag:ty, $ltag:ty) => {
        impl DeserializableFromBytesField<($ttag, $ltag)> for $ty {
            type Item = $ty;
            fn deser<'a, I, F>(&mut self, field: FieldData<BytesIter<'a, I>>, _f: F) -> Result<()>
            where
                I: Iterator<Item = std::io::Result<u8>>,
                F: FnOnce() -> Self::Item,
            {
                match field {
                    FieldData::Variant(variant) => {
                        *self = variant.to_native::<$ttag>()?;
                        Ok(())
                    }
                    FieldData::LengthDelimited(mut bytes_iter) => {
                        *self = bytes_iter
                            .variants()
                            .last()
                            .transpose()?
                            .ok_or(PuroroError::from(ErrorKind::ZeroLengthPackedField))
                            .and_then(|variant| variant.to_native::<$ttag>())?;
                        Ok(())
                    }
                    _ => Err(ErrorKind::InvalidWireType)?,
                }
            }
        }
    };
}
define_deser_req_variants!(i32, tags::Int32, tags::Required);
define_deser_req_variants!(i64, tags::Int64, tags::Required);
define_deser_req_variants!(i32, tags::SInt32, tags::Required);
define_deser_req_variants!(i64, tags::SInt64, tags::Required);
define_deser_req_variants!(u32, tags::UInt32, tags::Required);
define_deser_req_variants!(u64, tags::UInt64, tags::Required);
define_deser_req_variants!(bool, tags::Bool, tags::Required);

impl<T> DeserializableFromBytesField<(tags::Enum<T>, tags::Required)>
    for std::result::Result<T, i32>
where
    T: TryFrom<i32, Error = i32>,
{
    type Item = Self;
    fn deser<'a, I, F>(&mut self, field: FieldData<BytesIter<'a, I>>, _f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: FnOnce() -> Self::Item,
    {
        match field {
            FieldData::Variant(variant) => {
                *self = variant.to_native::<tags::Int32>()?.try_into();
                Ok(())
            }
            FieldData::LengthDelimited(mut bytes_iter) => {
                *self = bytes_iter
                    .variants()
                    .last()
                    .transpose()?
                    .ok_or(PuroroError::from(ErrorKind::ZeroLengthPackedField))
                    .and_then(|variant| variant.to_native::<tags::Int32>())?
                    .try_into();
                Ok(())
            }
            _ => Err(ErrorKind::InvalidWireType)?,
        }
    }
}

macro_rules! define_deser_req_ld {
    ($ty:ty, $ttag:ty, $ltag:ty, $method:ident) => {
        impl<'bump> DeserializableFromBytesField<($ttag, $ltag)> for $ty {
            type Item = $ty;
            fn deser<'a, I, F>(&mut self, field: FieldData<BytesIter<'a, I>>, _f: F) -> Result<()>
            where
                I: Iterator<Item = std::io::Result<u8>>,
                F: FnOnce() -> Self::Item,
            {
                if let FieldData::LengthDelimited(mut bytes_iter) = field {
                    self.clear();
                    self.reserve(bytes_iter.len());
                    for rv in bytes_iter.$method() {
                        self.push(rv?);
                    }
                    Ok(())
                } else {
                    Err(ErrorKind::UnexpectedWireType)?
                }
            }
        }
    };
}
define_deser_req_ld!(String, tags::String, tags::Required, chars);
define_deser_req_ld!(Vec<u8>, tags::Bytes, tags::Required, bytes);
#[cfg(feature = "puroro-bumpalo")]
define_deser_req_ld!(
    ::bumpalo::collections::String<'bump>,
    tags::String,
    tags::Required,
    chars
);
#[cfg(feature = "puroro-bumpalo")]
define_deser_req_ld!(
    ::bumpalo::collections::Vec<'bump, u8>,
    tags::Bytes,
    tags::Required,
    bytes
);

// Unlike C++ implementation, the required message field in Rust is not
// wrapped by `Option` (and neither `Box`).
impl<T> DeserializableFromBytesField<(tags::Message<T>, tags::Required)> for T
where
    T: crate::deser::DeserializableFromIter,
{
    type Item = T;
    fn deser<'a, I, F>(&mut self, field: FieldData<BytesIter<'a, I>>, _f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: FnOnce() -> Self::Item,
    {
        if let FieldData::LengthDelimited(bytes_iter) = field {
            self.deserialize_from_bytes_iter(bytes_iter)
        } else {
            Err(ErrorKind::UnexpectedWireType)?
        }
    }
}

impl<K, V, K2, V2> DeserializableFromBytesField<tags::Map<K, V>> for HashMap<K2, V2>
where
    K: FieldTypeTag,
    V: FieldTypeTag,
    K2: DeserializableFromBytesField<(K, tags::Required)>,
    V2: DeserializableFromBytesField<(V, tags::Required)>,
{
    type Item = (K2, V2);

    fn deser<'a, I, F>(&mut self, field: FieldData<BytesIter<'a, I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: FnOnce() -> Self::Item,
    {
        todo!()
    }
}

macro_rules! define_deser_req_fixed {
    ($ty:ty, $ttag:ty, $ltag:ty, $bits:ident) => {
        impl DeserializableFromBytesField<($ttag, $ltag)> for $ty {
            type Item = $ty;
            fn deser<'a, I, F>(&mut self, field: FieldData<BytesIter<'a, I>>, _f: F) -> Result<()>
            where
                I: Iterator<Item = std::io::Result<u8>>,
                F: FnOnce() -> Self::Item,
            {
                if let FieldData::$bits(array) = field {
                    *self = <$ty>::from_le_bytes(array);
                    Ok(())
                } else {
                    Err(ErrorKind::UnexpectedWireType)?
                }
            }
        }
    };
}
define_deser_req_fixed!(f32, tags::Float, tags::Required, Bits32);
define_deser_req_fixed!(i32, tags::SFixed32, tags::Required, Bits32);
define_deser_req_fixed!(u32, tags::Fixed32, tags::Required, Bits32);
define_deser_req_fixed!(f64, tags::Double, tags::Required, Bits64);
define_deser_req_fixed!(i64, tags::SFixed64, tags::Required, Bits64);
define_deser_req_fixed!(u64, tags::Fixed64, tags::Required, Bits64);

impl<T, U> DeserializableFromBytesField<(T, tags::Optional3)> for Option<U>
where
    T: FieldTypeTag + NotOptionaInNoPresenceDiscipline,
    U: DeserializableFromBytesField<(T, tags::Required)>,
{
    type Item = U;
    fn deser<'a, I, F>(&mut self, field: FieldData<BytesIter<'a, I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: FnOnce() -> Self::Item,
    {
        todo!()
    }
}

impl<T> DeserializableFromBytesField<(tags::Message<T>, tags::Optional3)> for Option<T>
where
    T: crate::deser::DeserializableFromIter
        + DeserializableFromBytesField<(tags::Message<T>, tags::Required)>,
{
    type Item = T;
    fn deser<'a, I, F>(&mut self, field: FieldData<BytesIter<'a, I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: FnOnce() -> Self::Item,
    {
        todo!()
    }
}
