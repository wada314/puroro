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

pub trait DeserializableFromBytesField<T>
where
    T: tags::FieldTypeTag,
{
    fn deser<'a, I, F>(&mut self, field: FieldData<BytesIter<'a, I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: FnOnce() -> Self;
}
macro_rules! define_deser_variants {
    ($ty:ty, $tag:ty) => {
        impl DeserializableFromBytesField<$tag> for $ty {
            fn deser<'a, I, F>(&mut self, field: FieldData<BytesIter<'a, I>>, _f: F) -> Result<()>
            where
                I: Iterator<Item = std::io::Result<u8>>,
                F: FnOnce() -> Self,
            {
                match field {
                    FieldData::Variant(variant) => {
                        *self = variant.to_native::<$tag>()?;
                        Ok(())
                    }
                    FieldData::LengthDelimited(mut bytes_iter) => {
                        *self = bytes_iter
                            .variants()
                            .last()
                            .transpose()?
                            .ok_or(PuroroError::ZeroLengthPackedField)
                            .and_then(|variant| variant.to_native::<$tag>())?;
                        Ok(())
                    }
                    _ => Err(PuroroError::InvalidWireType)?,
                }
            }
        }
    };
}
define_deser_variants!(i32, tags::Int32);
define_deser_variants!(i64, tags::Int64);
define_deser_variants!(i32, tags::SInt32);
define_deser_variants!(i64, tags::SInt64);
define_deser_variants!(u32, tags::UInt32);
define_deser_variants!(u64, tags::UInt64);
define_deser_variants!(bool, tags::Bool);

impl<T> DeserializableFromBytesField<tags::Enum<T>> for std::result::Result<T, i32>
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
    ($ty:ty, $tag:ty, $method:ident) => {
        impl<'bump> DeserializableFromBytesField<$tag> for $ty {
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
define_deser_lengthdelimited!(String, tags::String, chars);
define_deser_lengthdelimited!(Vec<u8>, tags::Bytes, bytes);
#[cfg(feature = "puroro-bumpalo")]
define_deser_lengthdelimited!(::bumpalo::collections::String<'bump>, tags::String, chars);
#[cfg(feature = "puroro-bumpalo")]
define_deser_lengthdelimited!(::bumpalo::collections::Vec<'bump, u8>, tags::Bytes, bytes);
