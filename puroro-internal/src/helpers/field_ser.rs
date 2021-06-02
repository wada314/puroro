use std::collections::HashMap;

use num_traits::Zero;

use crate::ser::{MessageSerializer, SerializableMessage};
use crate::tags::{self, FieldLabelTag, WireAndValueTypeTag};
use crate::Result;

use super::MapEntryForNormalImpl;

pub trait FieldSer<TypeTag, LabelTag>
where
    TypeTag: WireAndValueTypeTag,
    LabelTag: FieldLabelTag,
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

///////////////////////////////////////////////////////////////////////////////
// Required fields
///////////////////////////////////////////////////////////////////////////////

macro_rules! define_ser_required_variant {
    ($ty:ty, $ttag:ty) => {
        impl FieldSer<$ttag, tags::Required> for $ty {
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

impl<T> FieldSer<tags::Enum<T>, tags::Required> for std::result::Result<T, i32>
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
        impl<'bump> FieldSer<$ttag, tags::Required> for $ty {
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

impl<T> FieldSer<tags::Message<T>, tags::Required> for T
where
    T: SerializableMessage,
{
    fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
    where
        S: crate::ser::MessageSerializer,
    {
        serializer.serialize_message_twice(field_number, self)?;
        Ok(())
    }
}

macro_rules! define_ser_required_fixed {
    ($ty:ty, $ttag:ty) => {
        impl FieldSer<$ttag, tags::Required> for $ty {
            fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
            where
                S: crate::ser::MessageSerializer,
            {
                serializer.serialize_fixed_bits(field_number, self.to_le_bytes())?;
                Ok(())
            }
        }
    };
}
define_ser_required_fixed!(f32, tags::Float);
define_ser_required_fixed!(f64, tags::Double);
define_ser_required_fixed!(u32, tags::Fixed32);
define_ser_required_fixed!(u64, tags::Fixed64);
define_ser_required_fixed!(i32, tags::SFixed32);
define_ser_required_fixed!(i64, tags::SFixed64);

///////////////////////////////////////////////////////////////////////////////
// Optional2 fields
///////////////////////////////////////////////////////////////////////////////

macro_rules! define_ser_optional2_field_using_required {
    ($ty:ty, $ttag:ty) => {
        impl<'bump> FieldSer<$ttag, tags::Optional2> for Option<$ty> {
            fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
            where
                S: crate::ser::MessageSerializer,
            {
                if let Some(x) = self {
                    <$ty as FieldSer<$ttag, tags::Required>>::ser(x, serializer, field_number)?;
                }
                Ok(())
            }
        }
    };
}
define_ser_optional2_field_using_required!(i32, tags::Int32);
define_ser_optional2_field_using_required!(i64, tags::Int64);
define_ser_optional2_field_using_required!(u32, tags::UInt32);
define_ser_optional2_field_using_required!(u64, tags::UInt64);
define_ser_optional2_field_using_required!(i32, tags::SInt32);
define_ser_optional2_field_using_required!(i64, tags::SInt64);
define_ser_optional2_field_using_required!(bool, tags::Bool);

impl<T> FieldSer<tags::Enum<T>, tags::Optional2> for Option<std::result::Result<T, i32>>
where
    i32: From<T>,
    T: Clone,
{
    fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
    where
        S: crate::ser::MessageSerializer,
    {
        if let Some(e) = self {
            <::std::result::Result<T, i32> as FieldSer<tags::Enum<T>, tags::Required>>::ser(
                e,
                serializer,
                field_number,
            )?;
        }
        Ok(())
    }
}

define_ser_optional2_field_using_required!(String, tags::String);
define_ser_optional2_field_using_required!(Vec<u8>, tags::Bytes);
#[cfg(feature = "puroro-bumpalo")]
define_ser_optional2_field_using_required!(::bumpalo::collections::String<'bump>, tags::String);
#[cfg(feature = "puroro-bumpalo")]
define_ser_optional2_field_using_required!(::bumpalo::collections::Vec<'bump, u8>, tags::Bytes);

macro_rules! define_ser_optional_message {
    ($box:ty, $ltag:ty) => {
        impl<'bump, T> FieldSer<tags::Message<T>, $ltag> for Option<$box>
        where
            T: SerializableMessage,
        {
            fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
            where
                S: crate::ser::MessageSerializer,
            {
                if let Some(bm) = self {
                    <T as FieldSer<tags::Message<T>, tags::Required>>::ser(
                        bm,
                        serializer,
                        field_number,
                    )?;
                }
                Ok(())
            }
        }
    };
}
define_ser_optional_message!(Box<T>, tags::Optional2);
#[cfg(feature = "puroro-bumpalo")]
define_ser_optional_message!(::bumpalo::boxed::Box<'bump, T>, tags::Optional2);

define_ser_optional2_field_using_required!(f32, tags::Float);
define_ser_optional2_field_using_required!(f64, tags::Double);
define_ser_optional2_field_using_required!(u32, tags::Fixed32);
define_ser_optional2_field_using_required!(u64, tags::Fixed64);
define_ser_optional2_field_using_required!(i32, tags::SFixed32);
define_ser_optional2_field_using_required!(i64, tags::SFixed64);

///////////////////////////////////////////////////////////////////////////////
// Optional3 fields
///////////////////////////////////////////////////////////////////////////////

macro_rules! define_ser_optional3_field_using_required {
    ($ty:ty, $ttag:ty, $isdefault_f:expr) => {
        impl<'bump> FieldSer<$ttag, tags::Optional3> for $ty {
            fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
            where
                S: crate::ser::MessageSerializer,
            {
                if !($isdefault_f)(self) {
                    <$ty as FieldSer<$ttag, tags::Required>>::ser(self, serializer, field_number)?;
                }
                Ok(())
            }
        }
    };
}
define_ser_optional3_field_using_required!(i32, tags::Int32, |x: &i32| *x == 0);
define_ser_optional3_field_using_required!(i64, tags::Int64, |x: &i64| *x == 0);
define_ser_optional3_field_using_required!(u32, tags::UInt32, |x: &u32| *x == 0);
define_ser_optional3_field_using_required!(u64, tags::UInt64, |x: &u64| *x == 0);
define_ser_optional3_field_using_required!(i32, tags::SInt32, |x: &i32| *x == 0);
define_ser_optional3_field_using_required!(i64, tags::SInt64, |x: &i64| *x == 0);
define_ser_optional3_field_using_required!(bool, tags::Bool, |x: &bool| !*x);

impl<T> FieldSer<tags::Enum<T>, tags::Optional3> for std::result::Result<T, i32>
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
            <::std::result::Result<T, i32> as FieldSer<tags::Enum<T>, tags::Required>>::ser(
                self,
                serializer,
                field_number,
            )?;
        }
        Ok(())
    }
}

define_ser_optional3_field_using_required!(String, tags::String, |s: &String| s.is_empty());
define_ser_optional3_field_using_required!(Vec<u8>, tags::Bytes, |b: &Vec<u8>| b.is_empty());
#[cfg(feature = "puroro-bumpalo")]
define_ser_optional3_field_using_required!(
    ::bumpalo::collections::String<'bump>,
    tags::String,
    |s: &::bumpalo::collections::String<'bump>| s.is_empty()
);
#[cfg(feature = "puroro-bumpalo")]
define_ser_optional3_field_using_required!(
    ::bumpalo::collections::Vec<'bump, u8>,
    tags::Bytes,
    |b: &::bumpalo::collections::Vec<'bump, u8>| b.is_empty()
);

define_ser_optional_message!(Box<T>, tags::Optional3);
#[cfg(feature = "puroro-bumpalo")]
define_ser_optional_message!(::bumpalo::boxed::Box<'bump, T>, tags::Optional3);

// TODO: Needs confirmation: is f32.is_zero() really suit for our purpose? What about -0.0?
define_ser_optional3_field_using_required!(f32, tags::Float, |x: &f32| x.is_zero());
define_ser_optional3_field_using_required!(f64, tags::Double, |x: &f64| x.is_zero());
define_ser_optional3_field_using_required!(u32, tags::Fixed32, |x: &u32| *x == 0);
define_ser_optional3_field_using_required!(u64, tags::Fixed64, |x: &u64| *x == 0);
define_ser_optional3_field_using_required!(i32, tags::SFixed32, |x: &i32| *x == 0);
define_ser_optional3_field_using_required!(i64, tags::SFixed64, |x: &i64| *x == 0);

///////////////////////////////////////////////////////////////////////////////
// Repeated fields
///////////////////////////////////////////////////////////////////////////////

macro_rules! define_ser_repeated_variant {
    ($ty:ty, $ttag:ty) => {
        define_ser_repeated_variant!($ty, $ttag, Vec<$ty>);
        #[cfg(feature = "puroro-bumpalo")]
        define_ser_repeated_variant!($ty, $ttag, ::bumpalo::collections::Vec<'bump, $ty>);
    };
    ($ty:ty, $ttag:ty, $vec:ty) => {
        impl<'bump> FieldSer<$ttag, tags::Repeated> for $vec {
            fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
            where
                S: crate::ser::MessageSerializer,
            {
                serializer
                    .serialize_variants_twice::<$ttag, _>(field_number, self.iter().map(|x| Ok(*x)))
            }
        }
    };
}
define_ser_repeated_variant!(i32, tags::Int32);
define_ser_repeated_variant!(i64, tags::Int64);
define_ser_repeated_variant!(u32, tags::UInt32);
define_ser_repeated_variant!(u64, tags::UInt64);
define_ser_repeated_variant!(i32, tags::SInt32);
define_ser_repeated_variant!(i64, tags::SInt64);
define_ser_repeated_variant!(bool, tags::Bool);

macro_rules! define_ser_repeated_enum {
    ($vec:ty) => {
        impl<'bump, T> FieldSer<tags::Enum<T>, tags::Repeated> for $vec
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
    };
}
define_ser_repeated_enum!(Vec<std::result::Result<T, i32>>);
#[cfg(feature = "puroro-bumpalo")]
define_ser_repeated_enum!(::bumpalo::collections::Vec<'bump, std::result::Result<T, i32>>);

macro_rules! define_ser_repeated_ld_using_required {
    ($ty:ty, $ttag:ty) => {
        define_ser_repeated_ld_using_required!($ty, $ttag, Vec<$ty>);
        #[cfg(feature = "puroro-bumpalo")]
        define_ser_repeated_ld_using_required!($ty, $ttag, ::bumpalo::collections::Vec<'bump, $ty>);
    };
    ($ty:ty, $ttag:ty, $vec:ty) => {
        impl<'bump> FieldSer<$ttag, tags::Repeated> for $vec {
            fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
            where
                S: crate::ser::MessageSerializer,
            {
                for x in self {
                    <$ty as FieldSer<$ttag, tags::Required>>::ser(x, serializer, field_number)?;
                }
                Ok(())
            }
        }
    };
}
define_ser_repeated_ld_using_required!(String, tags::String);
define_ser_repeated_ld_using_required!(Vec<u8>, tags::Bytes);
#[cfg(feature = "puroro-bumpalo")]
define_ser_repeated_ld_using_required!(::bumpalo::collections::String<'bump>, tags::String);
#[cfg(feature = "puroro-bumpalo")]
define_ser_repeated_ld_using_required!(::bumpalo::collections::Vec<'bump, u8>, tags::Bytes);

macro_rules! define_ser_repeated_message {
    ($vec:ty) => {
        impl<'bump, T> FieldSer<tags::Message<T>, tags::Repeated> for $vec
        where
            T: SerializableMessage,
        {
            fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
            where
                S: crate::ser::MessageSerializer,
            {
                for m in self {
                    serializer.serialize_message_twice(field_number, m)?;
                }
                Ok(())
            }
        }
    };
}
define_ser_repeated_message!(Vec<T>);
#[cfg(feature = "puroro-bumpalo")]
define_ser_repeated_message!(::bumpalo::collections::Vec<'bump, T>);

macro_rules! define_ser_repeated_fixed {
    ($ty:ty, $ttag:ty) => {
        define_ser_repeated_fixed!($ty, $ttag, Vec<$ty>);
        #[cfg(feature = "puroro-bumpalo")]
        define_ser_repeated_fixed!($ty, $ttag, ::bumpalo::collections::Vec<'bump, $ty>);
    };
    ($ty:ty, $ttag:ty, $vec:ty) => {
        impl<'bump> FieldSer<$ttag, tags::Repeated> for $vec {
            fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
            where
                S: crate::ser::MessageSerializer,
            {
                for x in self {
                    <$ty as FieldSer<$ttag, tags::Required>>::ser(x, serializer, field_number)?;
                }
                Ok(())
            }
        }
    };
}
define_ser_repeated_fixed!(f32, tags::Float);
define_ser_repeated_fixed!(f64, tags::Double);
define_ser_repeated_fixed!(u32, tags::Fixed32);
define_ser_repeated_fixed!(u64, tags::Fixed64);
define_ser_repeated_fixed!(i32, tags::SFixed32);
define_ser_repeated_fixed!(i64, tags::SFixed64);

///////////////////////////////////////////////////////////////////////////////
// Map field
///////////////////////////////////////////////////////////////////////////////

impl<Entry> FieldSer<tags::Message<Entry>, tags::Repeated>
    for HashMap<Entry::OwnedKeyType, Entry::OwnedValueType>
where
    Entry: MapEntryForNormalImpl,
{
    fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
    where
        S: crate::ser::MessageSerializer,
    {
        struct SerializableMapEntry<'a, Entry: MapEntryForNormalImpl>(
            &'a Entry::OwnedKeyType,
            &'a Entry::OwnedValueType,
        );
        impl<'a, Entry: MapEntryForNormalImpl> crate::ser::SerializableMessage
            for SerializableMapEntry<'a, Entry>
        {
            fn serialize<T: MessageSerializer>(&self, serializer: &mut T) -> Result<()> {
                Entry::ser_kv(self.0, self.1, serializer)
            }
        }
        // I believe I can ignore the unknown fields of the map entry..
        for (key, value) in self {
            let entry = SerializableMapEntry::<Entry>(key, value);
            serializer.serialize_message_twice(field_number, &entry)?;
        }
        Ok(())
    }
}
