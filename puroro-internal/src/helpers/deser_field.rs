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

macro_rules! define_deser_scalar_variants {
    ($ty:ty, $ttag:ty, $ltag:ty) => {
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
                match field {
                    FieldData::Variant(variant) => {
                        *self = variant.to_native::<$ttag>()?;
                        Ok(())
                    }
                    FieldData::LengthDelimited(bytes_iter) => {
                        *self = bytes_iter
                            .variants()
                            .last()
                            .transpose()?
                            .ok_or(PuroroError::from(ErrorKind::ZeroLengthPackedField))?
                            .to_native::<$ttag>()?;
                        Ok(())
                    }
                    _ => Err(ErrorKind::InvalidWireType)?,
                }
            }
        }
    };
}
define_deser_scalar_variants!(i32, tags::Int32, tags::Required);
define_deser_scalar_variants!(i64, tags::Int64, tags::Required);
define_deser_scalar_variants!(i32, tags::SInt32, tags::Required);
define_deser_scalar_variants!(i64, tags::SInt64, tags::Required);
define_deser_scalar_variants!(u32, tags::UInt32, tags::Required);
define_deser_scalar_variants!(u64, tags::UInt64, tags::Required);
define_deser_scalar_variants!(bool, tags::Bool, tags::Required);
define_deser_scalar_variants!(i32, tags::Int32, tags::Optional3);
define_deser_scalar_variants!(i64, tags::Int64, tags::Optional3);
define_deser_scalar_variants!(i32, tags::SInt32, tags::Optional3);
define_deser_scalar_variants!(i64, tags::SInt64, tags::Optional3);
define_deser_scalar_variants!(u32, tags::UInt32, tags::Optional3);
define_deser_scalar_variants!(u64, tags::UInt64, tags::Optional3);
define_deser_scalar_variants!(bool, tags::Bool, tags::Optional3);

macro_rules! define_deser_scalar_enum {
    ($ty:ty, $ttag:ty, $ltag:ty) => {
        impl<T> DeserializableFromIterField<($ttag, $ltag)> for $ty
        where
            T: TryFrom<i32, Error = i32>,
        {
            type Item = Self;
            fn deser<'a, I, F>(
                &mut self,
                field: FieldData<&'a mut BytesIter<'a, I>>,
                _: F,
            ) -> Result<()>
            where
                I: Iterator<Item = std::io::Result<u8>>,
                F: Fn() -> Self::Item,
            {
                let mut ival = 0i32;
                <i32 as DeserializableFromIterField<(tags::Int32, tags::Required)>>::deser(
                    &mut ival,
                    field,
                    Default::default,
                )?;
                *self = T::try_from(ival);
                Ok(())
            }
        }
    };
}
define_deser_scalar_enum!(std::result::Result<T, i32>, tags::Enum<T>, tags::Required);
define_deser_scalar_enum!(std::result::Result<T, i32>, tags::Enum<T>, tags::Optional3);

macro_rules! define_deser_scalar_ld {
    ($ty:ty, $ttag:ty, $ltag:ty, $method:ident) => {
        impl<'bump> DeserializableFromIterField<($ttag, $ltag)> for $ty {
            type Item = $ty;
            fn deser<'a, I, F>(
                &mut self,
                field: FieldData<&'a mut BytesIter<'a, I>>,
                _: F,
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
define_deser_scalar_ld!(String, tags::String, tags::Optional3, chars);
define_deser_scalar_ld!(Vec<u8>, tags::Bytes, tags::Optional3, bytes);
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
#[cfg(feature = "puroro-bumpalo")]
define_deser_scalar_ld!(
    ::bumpalo::collections::String<'bump>,
    tags::String,
    tags::Optional3,
    chars
);
#[cfg(feature = "puroro-bumpalo")]
define_deser_scalar_ld!(
    ::bumpalo::collections::Vec<'bump, u8>,
    tags::Bytes,
    tags::Optional3,
    bytes
);

// Unlike C++ implementation, the required message field in Rust is not
// wrapped by `Option` (and neither `Box`).
// We don't need to worry about the recursive struct when the field is required.
impl<T> DeserializableFromIterField<(tags::Message<T>, tags::Required)> for T
where
    T: crate::deser::DeserializableMessageFromIter,
{
    type Item = T;
    fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut BytesIter<'a, I>>, _: F) -> Result<()>
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
                _: F,
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
define_deser_scalar_fixed!(f32, tags::Float, tags::Optional3, Bits32);
define_deser_scalar_fixed!(i32, tags::SFixed32, tags::Optional3, Bits32);
define_deser_scalar_fixed!(u32, tags::Fixed32, tags::Optional3, Bits32);
define_deser_scalar_fixed!(f64, tags::Double, tags::Optional3, Bits64);
define_deser_scalar_fixed!(i64, tags::SFixed64, tags::Optional3, Bits64);
define_deser_scalar_fixed!(u64, tags::Fixed64, tags::Optional3, Bits64);

macro_rules! define_deser_optional_fields_from_scalar {
    ($ty:ty, $ttag:ty, $ltag:ty) => {
        impl<'bump> DeserializableFromIterField<($ttag, $ltag)> for Option<$ty> {
            type Item = $ty;
            fn deser<'a, I, F>(
                &mut self,
                field: FieldData<&'a mut BytesIter<'a, I>>,
                f: F,
            ) -> Result<()>
            where
                I: Iterator<Item = std::io::Result<u8>>,
                F: Fn() -> Self::Item,
            {
                <Self::Item as DeserializableFromIterField<($ttag, tags::Required)>>::deser(
                    self.get_or_insert_with(f),
                    field,
                    || unreachable!(),
                )
            }
        }
    };
}
define_deser_optional_fields_from_scalar!(i32, tags::Int32, tags::Optional2);
define_deser_optional_fields_from_scalar!(i64, tags::Int64, tags::Optional2);
define_deser_optional_fields_from_scalar!(i32, tags::SInt32, tags::Optional2);
define_deser_optional_fields_from_scalar!(i64, tags::SInt64, tags::Optional2);
define_deser_optional_fields_from_scalar!(u32, tags::UInt32, tags::Optional2);
define_deser_optional_fields_from_scalar!(u64, tags::UInt64, tags::Optional2);
define_deser_optional_fields_from_scalar!(bool, tags::Bool, tags::Optional2);
define_deser_optional_fields_from_scalar!(String, tags::String, tags::Optional2);
define_deser_optional_fields_from_scalar!(Vec<u8>, tags::Bytes, tags::Optional2);
#[cfg(feature = "puroro-bumpalo")]
define_deser_optional_fields_from_scalar!(
    ::bumpalo::collections::String<'bump>,
    tags::String,
    tags::Optional2
);
#[cfg(feature = "puroro-bumpalo")]
define_deser_optional_fields_from_scalar!(
    ::bumpalo::collections::Vec<'bump, u8>,
    tags::Bytes,
    tags::Optional2
);
define_deser_optional_fields_from_scalar!(f32, tags::Float, tags::Optional2);
define_deser_optional_fields_from_scalar!(i32, tags::SFixed32, tags::Optional2);
define_deser_optional_fields_from_scalar!(u32, tags::Fixed32, tags::Optional2);
define_deser_optional_fields_from_scalar!(f64, tags::Double, tags::Optional2);
define_deser_optional_fields_from_scalar!(i64, tags::SFixed64, tags::Optional2);
define_deser_optional_fields_from_scalar!(u64, tags::Fixed64, tags::Optional2);
// Enum, essentially same with the macro above but needs a generic type parameter.
impl<T> DeserializableFromIterField<(tags::Enum<T>, tags::Optional2)>
    for Option<std::result::Result<T, i32>>
where
    T: TryFrom<i32, Error = i32>,
{
    type Item = std::result::Result<T, i32>;
    fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut BytesIter<'a, I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item,
    {
        <Self::Item as DeserializableFromIterField<(tags::Enum<T>, tags::Required)>>::deser(
            self.get_or_insert_with(f),
            field,
            || unreachable!(),
        )
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
