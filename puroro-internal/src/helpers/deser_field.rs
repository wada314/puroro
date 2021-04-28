use crate::deser::BytesIter;
use crate::tags::{self, VariantTypeTag};
use crate::tags::{FieldLabelTag, FieldTypeAndLabelTag, FieldTypeTag};
use crate::types::FieldData;
use crate::{variant, ErrorKind, PuroroError, Result};
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

pub trait DeserializableFromIterField<T>
where
    T: FieldTypeAndLabelTag,
{
    type Item;
    fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut BytesIter<'a, I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item;
}

// For Variant types excpet enums: Int32, Int64, UInt32, UInt64, SInt32, SInt64, Bool
impl<T, U> DeserializableFromIterField<(T, tags::Required)> for U
where
    T: FieldTypeTag + variant::VariantType<NativeType = U>,
{
    type Item = U;
    fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut BytesIter<'a, I>>, _f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item,
    {
        match field {
            FieldData::Variant(variant) => {
                *self = variant.to_native::<T>()?;
                Ok(())
            }
            FieldData::LengthDelimited(bytes_iter) => {
                *self = bytes_iter
                    .variants()
                    .last()
                    .transpose()?
                    .ok_or(PuroroError::from(ErrorKind::ZeroLengthPackedField))
                    .and_then(|variant| variant.to_native::<T>())?;
                Ok(())
            }
            _ => Err(ErrorKind::InvalidWireType)?,
        }
    }
}

pub trait EnumNativeType: Sized {
    fn try_from_i32(val: i32) -> Result<Self>;
}
impl<T> EnumNativeType for std::result::Result<T, i32>
where
    T: TryFrom<i32, Error = i32>,
{
    fn try_from_i32(val: i32) -> Result<Self> {
        Ok(T::try_from(val))
    }
}

impl<T> DeserializableFromIterField<(tags::Enum<T>, tags::Required)> for T
where
    T: EnumNativeType,
{
    type Item = Self;
    fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut BytesIter<'a, I>>, _f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item,
    {
        match field {
            FieldData::Variant(variant) => {
                *self = Self::try_from_i32(variant.to_native::<tags::Int32>()?)?;
                Ok(())
            }
            FieldData::LengthDelimited(bytes_iter) => {
                *self = Self::try_from_i32(
                    bytes_iter
                        .variants()
                        .last()
                        .transpose()?
                        .ok_or(PuroroError::from(ErrorKind::ZeroLengthPackedField))?
                        .to_native::<tags::Int32>()?,
                )?;
                Ok(())
            }
            _ => Err(ErrorKind::InvalidWireType)?,
        }
    }
}

macro_rules! define_deser_scalar_ld {
    ($ty:ty, $ttag:ty, $ltag:ty, $method:ident) => {
        impl<'bump> DeserializableFromIterField<($ttag, $ltag)> for $ty {
            type Item = $ty;
            fn deser<'a, I, F>(
                &mut self,
                field: FieldData<&'a mut BytesIter<'a, I>>,
                _f: F,
            ) -> Result<()>
            where
                I: Iterator<Item = std::io::Result<u8>>,
                F: Fn() -> Self::Item,
            {
                if let FieldData::LengthDelimited(bytes_iter) = field {
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
define_deser_scalar_ld!(String, tags::String, tags::Required, chars);
define_deser_scalar_ld!(Vec<u8>, tags::Bytes, tags::Required, bytes);
#[cfg(feature = "puroro-bumpalo")]
define_deser_scalar_ld!(
    ::bumpalo::collections::String<'bump>,
    tags::String,
    tags::Required,
    chars
);
#[cfg(feature = "puroro-bumpalo")]
define_deser_scalar_ld!(
    ::bumpalo::collections::Vec<'bump, u8>,
    tags::Bytes,
    tags::Required,
    bytes
);

// Unlike C++ implementation, the required message field in Rust is not
// wrapped by `Option` (and neither `Box`).
impl<T> DeserializableFromIterField<(tags::Message<T>, tags::Required)> for T
where
    T: crate::deser::DeserializableMessageFromIter,
{
    type Item = T;
    fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut BytesIter<'a, I>>, _f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item,
    {
        if let FieldData::LengthDelimited(bytes_iter) = field {
            bytes_iter.deser_message(self)
        } else {
            Err(ErrorKind::UnexpectedWireType)?
        }
    }
}

impl<KT, VT, KR, VR> DeserializableFromIterField<tags::Map<KT, VT>> for HashMap<KR, VR>
where
    KT: FieldTypeTag,
    VT: FieldTypeTag,
    KR: DeserializableFromIterField<(KT, tags::Required)>,
    VR: DeserializableFromIterField<(VT, tags::Required)>,
{
    type Item = (KR, VR);

    fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut BytesIter<'a, I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item,
    {
        todo!()
    }
}

macro_rules! define_deser_scalar_fixed {
    ($ty:ty, $ttag:ty, $ltag:ty, $bits:ident) => {
        impl DeserializableFromIterField<($ttag, $ltag)> for $ty {
            type Item = $ty;
            fn deser<'a, I, F>(
                &mut self,
                field: FieldData<&'a mut BytesIter<'a, I>>,
                _f: F,
            ) -> Result<()>
            where
                I: Iterator<Item = std::io::Result<u8>>,
                F: Fn() -> Self::Item,
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
define_deser_scalar_fixed!(f32, tags::Float, tags::Required, Bits32);
define_deser_scalar_fixed!(i32, tags::SFixed32, tags::Required, Bits32);
define_deser_scalar_fixed!(u32, tags::Fixed32, tags::Required, Bits32);
define_deser_scalar_fixed!(f64, tags::Double, tags::Required, Bits64);
define_deser_scalar_fixed!(i64, tags::SFixed64, tags::Required, Bits64);
define_deser_scalar_fixed!(u64, tags::Fixed64, tags::Required, Bits64);

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

impl<T, U> DeserializableFromIterField<(T, tags::Optional3)> for U
where
    T: FieldTypeTag + NotOptionaInNoPresenceDiscipline,
    U: DeserializableFromIterField<(T, tags::Required)>,
{
    type Item = <U as DeserializableFromIterField<(T, tags::Required)>>::Item;
    fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut BytesIter<'a, I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item,
    {
        <U as DeserializableFromIterField<(T, tags::Required)>>::deser(self, field, f)
    }
}

impl<T, U> DeserializableFromIterField<(T, tags::Repeated)> for Vec<U>
where
    T: FieldTypeTag + variant::VariantType<NativeType = U>,
    U: DeserializableFromIterField<(T, tags::Required)>,
{
    type Item = U;
    fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut BytesIter<'a, I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item,
    {
        match field {
            FieldData::Variant(variant) => {
                self.push(variant.to_native::<T>()?);
            }
            FieldData::LengthDelimited(bytes_iter) => {
                for rvar in bytes_iter.variants() {
                    self.push(rvar?.to_native::<T>()?);
                }
            }
            FieldData::Bits32(_) | FieldData::Bits64(_) => Err(ErrorKind::UnexpectedWireType)?,
        }
        Ok(())
    }
}

impl DeserializableFromIterField<(tags::Float, tags::Repeated)> for Vec<f32> {
    type Item = f32;

    fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut BytesIter<'a, I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item,
    {
        todo!()
    }
}
