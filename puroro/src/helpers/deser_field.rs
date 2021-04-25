use std::convert::{TryFrom, TryInto};

use crate::deser::BytesIter;
use crate::tags;
use crate::types::FieldData;
use crate::{PuroroError, Result};

pub trait ScalarField {}
impl ScalarField for i32 {}
impl ScalarField for i64 {}
impl ScalarField for u32 {}
impl ScalarField for u64 {}
impl ScalarField for f32 {}
impl ScalarField for f64 {}
impl ScalarField for bool {}
//impl<T> ScalarField for std::result::Result<T, i32> {}
impl ScalarField for Vec<u8> {}
impl ScalarField for String {}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ScalarField for ::bumpalo::collections::Vec<'bump, u8> {}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> ScalarField for ::bumpalo::collections::String<'bump> {}

pub trait DeserializableFromBytesField<T, L>
where
    T: tags::FieldTypeTag,
    L: tags::FieldLabelTag,
{
    fn deser<'a, I, F>(&mut self, field: FieldData<BytesIter<'a, I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: FnOnce() -> Self;
}
macro_rules! define_deser_variants {
    ($ty:ty, $ttag:ty, $ltag:ty) => {
        impl DeserializableFromBytesField<$ttag, $ltag> for $ty {
            fn deser<'a, I, F>(&mut self, field: FieldData<BytesIter<'a, I>>, _f: F) -> Result<()>
            where
                I: Iterator<Item = std::io::Result<u8>>,
                F: FnOnce() -> Self,
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
                            .ok_or(PuroroError::ZeroLengthPackedField)
                            .and_then(|variant| variant.to_native::<$ttag>())?;
                        Ok(())
                    }
                    _ => Err(PuroroError::InvalidWireType)?,
                }
            }
        }
    };
}
define_deser_variants!(i32, tags::Int32, tags::Required);
define_deser_variants!(i64, tags::Int64, tags::Required);
define_deser_variants!(i32, tags::SInt32, tags::Required);
define_deser_variants!(i64, tags::SInt64, tags::Required);
define_deser_variants!(u32, tags::UInt32, tags::Required);
define_deser_variants!(u64, tags::UInt64, tags::Required);
define_deser_variants!(bool, tags::Bool, tags::Required);

impl<T> DeserializableFromBytesField<tags::Enum<T>, tags::Required> for std::result::Result<T, i32>
where
    T: TryFrom<i32, Error = i32>,
{
    fn deser<'a, I, F>(&mut self, field: FieldData<BytesIter<'a, I>>, _f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: FnOnce() -> Self,
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
                    .ok_or(PuroroError::ZeroLengthPackedField)
                    .and_then(|variant| variant.to_native::<tags::Int32>())?
                    .try_into();
                Ok(())
            }
            _ => Err(PuroroError::InvalidWireType)?,
        }
    }
}

macro_rules! define_deser_lengthdelimited {
    ($ty:ty, $ttag:ty, $ltag:ty, $method:ident) => {
        impl<'bump> DeserializableFromBytesField<$ttag, $ltag> for $ty {
            fn deser<'a, I, F>(&mut self, field: FieldData<BytesIter<'a, I>>, _f: F) -> Result<()>
            where
                I: Iterator<Item = std::io::Result<u8>>,
                F: FnOnce() -> Self,
            {
                if let FieldData::LengthDelimited(mut bytes_iter) = field {
                    self.clear();
                    self.reserve(bytes_iter.len());
                    for rv in bytes_iter.$method() {
                        self.push(rv?);
                    }
                    Ok(())
                } else {
                    Err(PuroroError::UnexpectedWireType)?
                }
            }
        }
    };
}
define_deser_lengthdelimited!(String, tags::String, tags::Required, chars);
define_deser_lengthdelimited!(Vec<u8>, tags::Bytes, tags::Required, bytes);
#[cfg(feature = "puroro-bumpalo")]
define_deser_lengthdelimited!(
    ::bumpalo::collections::String<'bump>,
    tags::String,
    tags::Required,
    chars
);
#[cfg(feature = "puroro-bumpalo")]
define_deser_lengthdelimited!(
    ::bumpalo::collections::Vec<'bump, u8>,
    tags::Bytes,
    tags::Required,
    bytes
);

impl<T> DeserializableFromBytesField<tags::Message<T>, tags::Required> for T
where
    T: crate::deser::DeserializableFromBytes,
{
    fn deser<'a, I, F>(&mut self, field: FieldData<BytesIter<'a, I>>, _f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: FnOnce() -> Self,
    {
        if let FieldData::LengthDelimited(bytes_iter) = field {
            self.deserialize_from_bytes_iter(bytes_iter)
        } else {
            Err(PuroroError::UnexpectedWireType)?
        }
    }
}

macro_rules! define_deser_fixedlengths {
    ($ty:ty, $ttag:ty, $ltag:ty, $bits:ident) => {
        impl DeserializableFromBytesField<$ttag, $ltag> for $ty {
            fn deser<'a, I, F>(&mut self, field: FieldData<BytesIter<'a, I>>, _f: F) -> Result<()>
            where
                I: Iterator<Item = std::io::Result<u8>>,
                F: FnOnce() -> Self,
            {
                if let FieldData::$bits(array) = field {
                    *self = <$ty>::from_le_bytes(array);
                    Ok(())
                } else {
                    Err(PuroroError::UnexpectedWireType)?
                }
            }
        }
    };
}
define_deser_fixedlengths!(f32, tags::Float, tags::Required, Bits32);
define_deser_fixedlengths!(i32, tags::SFixed32, tags::Required, Bits32);
define_deser_fixedlengths!(u32, tags::Fixed32, tags::Required, Bits32);
define_deser_fixedlengths!(f64, tags::Double, tags::Required, Bits64);
define_deser_fixedlengths!(i64, tags::SFixed64, tags::Required, Bits64);
define_deser_fixedlengths!(u64, tags::Fixed64, tags::Required, Bits64);
