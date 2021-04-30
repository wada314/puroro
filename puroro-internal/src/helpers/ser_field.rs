use crate::tags;
use crate::tags::FieldTypeAndLabelTag;
use crate::{ErrorKind, Result};

pub trait SerializableField<T>
where
    T: FieldTypeAndLabelTag,
{
    fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
    where
        S: crate::ser::MessageSerializer;
}

fn enum_to_i32<T>(e: &std::result::Result<T, i32>) -> i32
where
    i32: From<T>,
    T: Clone,
{
    match e {
        Ok(x) => i32::from(x.clone()),
        Err(i) => *i,
    }
}

macro_rules! define_ser_required_variant {
    ($ty:ty, $ttag:ty) => {
        impl SerializableField<($ttag, tags::Required)> for $ty {
            fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
            where
                S: crate::ser::MessageSerializer,
            {
                serializer.serialize_variant::<$ttag>(field_number, *self)?;
                Ok(())
            }
        }
    };
}
define_ser_required_variant!(i32, tags::Int32);
define_ser_required_variant!(i64, tags::Int64);
define_ser_required_variant!(i32, tags::SInt32);
define_ser_required_variant!(i64, tags::SInt64);
define_ser_required_variant!(u32, tags::UInt32);
define_ser_required_variant!(u64, tags::UInt64);
define_ser_required_variant!(bool, tags::Bool);

impl<T> SerializableField<(tags::Enum<T>, tags::Required)> for std::result::Result<T, i32>
where
    i32: From<T>,
    T: Clone,
{
    fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
    where
        S: crate::ser::MessageSerializer,
    {
        serializer.serialize_variant::<tags::Int32>(field_number, enum_to_i32(self))?;
        Ok(())
    }
}

macro_rules! define_ser_required_ld {
    ($ty:ty, $ttag:ty, $func:ident) => {
        impl<'bump> SerializableField<($ttag, tags::Required)> for $ty {
            fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
            where
                S: crate::ser::MessageSerializer,
            {
                serializer
                    .serialize_bytes_twice(field_number, self.$func().map(|b| Ok(b.clone())))?;
                Ok(())
            }
        }
    };
}
define_ser_required_ld!(String, tags::String, bytes);
define_ser_required_ld!(Vec<u8>, tags::Bytes, iter);
#[cfg(feature = "puroro-bumpalo")]
define_ser_required_ld!(::bumpalo::collections::String<'bump>, tags::String, bytes);
#[cfg(feature = "puroro-bumpalo")]
define_ser_required_ld!(::bumpalo::collections::Vec<'bump, u8>, tags::Bytes, iter);

macro_rules! define_ser_optional2_field_using_required {
    ($ty:ty, $ttag:ty) => {
        impl SerializableField<($ttag, tags::Optional2)> for Option<$ty> {
            fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
            where
                S: crate::ser::MessageSerializer,
            {
                if let Some(x) = self {
                    <$ty as SerializableField<($ttag, tags::Required)>>::ser(
                        x,
                        serializer,
                        field_number,
                    )?;
                }
                Ok(())
            }
        }
    };
}
define_ser_optional2_field_using_required!(i32, tags::Int32);

impl<T> SerializableField<(tags::Enum<T>, tags::Optional2)> for Option<std::result::Result<T, i32>>
where
    i32: From<T>,
    T: Clone,
{
    fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
    where
        S: crate::ser::MessageSerializer,
    {
        if let Some(e) = self {
            serializer.serialize_variant::<tags::Int32>(field_number, enum_to_i32(e))?
        }
        Ok(())
    }
}

impl SerializableField<(tags::String, tags::Optional2)> for Option<String> {
    fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
    where
        S: crate::ser::MessageSerializer,
    {
        if let Some(s) = self {
            serializer.serialize_bytes_twice(field_number, s.bytes().map(|b| Ok(b)))?;
        }
        Ok(())
    }
}

impl SerializableField<(tags::Int32, tags::Optional3)> for i32 {
    fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
    where
        S: crate::ser::MessageSerializer,
    {
        if *self != 0 {
            serializer.serialize_variant::<tags::Int32>(field_number, *self)?;
        }
        Ok(())
    }
}

impl<T> SerializableField<(tags::Enum<T>, tags::Optional3)> for std::result::Result<T, i32>
where
    i32: From<T>,
    T: Clone,
{
    fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
    where
        S: crate::ser::MessageSerializer,
    {
        let i = enum_to_i32(self);
        if i != 0 {
            serializer.serialize_variant::<tags::Int32>(field_number, i)?
        }
        Ok(())
    }
}

impl SerializableField<(tags::String, tags::Optional3)> for String {
    fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
    where
        S: crate::ser::MessageSerializer,
    {
        if !self.is_empty() {
            serializer.serialize_bytes_twice(field_number, self.bytes().map(|b| Ok(b)))?;
        }
        Ok(())
    }
}

impl SerializableField<(tags::Int32, tags::Repeated)> for Vec<i32> {
    fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
    where
        S: crate::ser::MessageSerializer,
    {
        serializer
            .serialize_variants_twice::<tags::Int32, _>(field_number, self.iter().map(|x| Ok(*x)))
    }
}

impl<T> SerializableField<(tags::Enum<T>, tags::Repeated)> for Vec<std::result::Result<T, i32>>
where
    i32: From<T>,
    T: Clone,
{
    fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
    where
        S: crate::ser::MessageSerializer,
    {
        serializer.serialize_variants_twice::<tags::Int32, _>(
            field_number,
            self.iter().map(|e| Ok(enum_to_i32(e))),
        )?;
        Ok(())
    }
}

impl SerializableField<(tags::String, tags::Repeated)> for Vec<String> {
    fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
    where
        S: crate::ser::MessageSerializer,
    {
        for s in self {
            serializer.serialize_bytes_twice(field_number, s.bytes().map(|b| Ok(b)))?;
        }
        Ok(())
    }
}
