use crate::deser::{DeserializableMessageFromIter, LdIter};
use crate::tags;
use crate::tags::{FieldLabelTag, FieldTypeTag};
use crate::types::FieldData;
use crate::{ErrorKind, Result};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::hash::Hash;
use std::io::Read;
use std::marker::PhantomData;

use super::{DoDefaultCheck, MapEntry};

pub trait FieldDeserFromIter<TypeTag, LabelTag>
where
    TypeTag: FieldTypeTag,
    LabelTag: FieldLabelTag,
{
    /// The return type of the default instance generator passed to `deser` method.
    type Item;
    /// Deserialize bynary data into this field.
    /// * `field` - A data of the field, where the wire type and (for length delimited wire
    /// type) the field length are already load. For variants and fixed bytes fields,
    /// the content data is also already load.
    /// * `f` - A function object that generates default instance of the field "item".
    /// It depends on the field type what type the field item is.
    /// ** numeric types (except enum) - The corresponding rust's numeric types.
    /// Typically it's just a `Default::default`.
    /// ** Enum types - Because our rust's corresponding type `Result<T, i32>` does not
    /// implement `Default`, we need our own initializing function object.
    /// ** string, bytes types - The corresponding rust's types (e.g. `String`, `Vec<u8>`).
    /// It seems to be trivial and we can just use `Default::default`, but if we use a
    /// custom allocator then we need an allocator instance value to instanciate the default
    /// value, which `Default::default` cannot support.
    /// ** Message types - `Option<Box<T>>` for the both proto2 and proto3's optional types,
    /// otherwise just a raw message type. This is because of an implementation details...
    fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item;
}
pub trait FieldDeserFromSlice<TypeTag, LabelTag>
where
    TypeTag: FieldTypeTag,
    LabelTag: FieldLabelTag,
{
    /// Deserialize bynary data into this field.
    /// * `field` - A data of the field, where the wire type and (for length delimited wire
    /// type) the field length are already load. For variants and fixed bytes fields,
    /// the content data is also already load.
    fn deser<'slice>(&mut self, field: FieldData<&'slice [u8]>) -> Result<()> {
        let iter_field = match field {
            FieldData::LengthDelimited(slice) => {
                todo!();
                let mut temp_iter = std::io::stdin().bytes();
                let mut temp_ld_iter = LdIter::new(&mut temp_iter);
                FieldData::LengthDelimited(&mut temp_ld_iter)
            }
            FieldData::Variant(v) => FieldData::Variant(v),
            FieldData::Bits32(x) => FieldData::Bits32(x),
            FieldData::Bits64(x) => FieldData::Bits64(x),
        };
        Ok(())
    }
}

///////////////////////////////////////////////////////////////////////////////
// Baretype fields
///////////////////////////////////////////////////////////////////////////////

macro_rules! define_deser_scalar_variants {
    ($ty:ty, $ttag:ty, $ltag:ty) => {
        impl FieldDeserFromIter<$ttag, $ltag> for $ty {
            type Item = $ty;
            fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, _f: F) -> Result<()>
            where
                I: Iterator<Item = std::io::Result<u8>>,
                F: Fn() -> Self::Item,
            {
                match field {
                    FieldData::Variant(variant) => {
                        if !<$ltag>::DO_DEFAULT_CHECK || !variant.is_zero() {
                            *self = variant.to_native::<$ttag>()?;
                        }
                        Ok(())
                    }
                    FieldData::LengthDelimited(ld_iter) => {
                        let mut variants = ld_iter.variants().peekable();
                        if let None = variants.peek() {
                            Err(ErrorKind::ZeroLengthPackedField)?
                        }
                        let mut last_filtered_variant = None;
                        for rv in variants {
                            let v = rv?;
                            if !<$ltag>::DO_DEFAULT_CHECK || !v.is_zero() {
                                last_filtered_variant = Some(v)
                            }
                        }
                        if let Some(v) = last_filtered_variant {
                            *self = v.to_native::<$ttag>()?;
                        }
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

impl FieldDeserFromSlice<tags::Int32, tags::Required> for i32 {
    fn deser<'slice>(&mut self, field: FieldData<&'slice [u8]>) -> Result<()> {
        todo!()
    }
}

macro_rules! define_deser_scalar_enum {
    ($ty:ty, $ttag:ty, $ltag:ty) => {
        impl<T> FieldDeserFromIter<$ttag, $ltag> for $ty
        where
            T: TryFrom<i32, Error = i32>,
        {
            type Item = Self;
            fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, _: F) -> Result<()>
            where
                I: Iterator<Item = std::io::Result<u8>>,
                F: Fn() -> Self::Item,
            {
                let mut ival = 0i32;
                <i32 as FieldDeserFromIter<tags::Int32, tags::Required>>::deser(
                    &mut ival,
                    field,
                    Default::default,
                )?;
                if !<$ltag>::DO_DEFAULT_CHECK || ival != 0 {
                    *self = T::try_from(ival);
                }
                Ok(())
            }
        }
    };
}
define_deser_scalar_enum!(std::result::Result<T, i32>, tags::Enum<T>, tags::Required);
define_deser_scalar_enum!(std::result::Result<T, i32>, tags::Enum<T>, tags::Optional3);

macro_rules! define_deser_scalar_ld {
    ($ty:ty, $ttag:ty, $ltag:ty, $method:ident) => {
        impl<'bump> FieldDeserFromIter<$ttag, $ltag> for $ty {
            type Item = $ty;
            fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, _: F) -> Result<()>
            where
                I: Iterator<Item = std::io::Result<u8>>,
                F: Fn() -> Self::Item,
            {
                if let FieldData::LengthDelimited(ld_iter) = field {
                    let expected_len = ld_iter.len();
                    if <$ltag>::DO_DEFAULT_CHECK {
                        let mut iter = ld_iter.$method().peekable();
                        if let Some(_) = iter.peek() {
                            self.clear();
                            self.reserve(expected_len);
                            for rv in iter {
                                self.push(rv?);
                            }
                        }
                    } else {
                        self.clear();
                        self.reserve(expected_len);
                        for rv in ld_iter.$method() {
                            self.push(rv?);
                        }
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
impl<T> FieldDeserFromIter<tags::Message<T>, tags::Required> for T
where
    T: crate::deser::DeserializableMessageFromIter,
{
    type Item = T;
    fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, _: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item,
    {
        if let FieldData::LengthDelimited(ld_iter) = field {
            ld_iter.deser_message(self)
        } else {
            Err(ErrorKind::UnexpectedWireType)?
        }
    }
}

macro_rules! define_deser_scalar_fixed {
    ($ty:ty, $ttag:ty, $ltag:ty, $bits:ident) => {
        impl FieldDeserFromIter<$ttag, $ltag> for $ty {
            type Item = $ty;
            fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, _: F) -> Result<()>
            where
                I: Iterator<Item = std::io::Result<u8>>,
                F: Fn() -> Self::Item,
            {
                if let FieldData::$bits(array) = field {
                    if !<$ltag>::DO_DEFAULT_CHECK || !array.iter().all(|x| *x == 0) {
                        *self = <$ty>::from_le_bytes(array);
                    }
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

///////////////////////////////////////////////////////////////////////////////
// Option<> fields
///////////////////////////////////////////////////////////////////////////////

macro_rules! define_deser_optional_fields_from_scalar {
    ($ty:ty, $ttag:ty, $ltag:ty) => {
        impl<'bump> FieldDeserFromIter<$ttag, $ltag> for Option<$ty> {
            type Item = $ty;
            fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, f: F) -> Result<()>
            where
                I: Iterator<Item = std::io::Result<u8>>,
                F: Fn() -> Self::Item,
            {
                <Self::Item as FieldDeserFromIter<$ttag, tags::Required>>::deser(
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
impl<T> FieldDeserFromIter<tags::Enum<T>, tags::Optional2> for Option<std::result::Result<T, i32>>
where
    T: TryFrom<i32, Error = i32>,
{
    type Item = std::result::Result<T, i32>;
    fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item,
    {
        <Self::Item as FieldDeserFromIter<tags::Enum<T>, tags::Required>>::deser(
            self.get_or_insert_with(f),
            field,
            || unreachable!(),
        )
    }
}

// Message. Different from the other types, it is wrapped by Option<Box<_>> for
// the both Optional2 and Optional3.
macro_rules! define_deser_optional_message_field {
    ($ltag:ty) => {
        define_deser_optional_message_field!($ltag, Box<T>);
        #[cfg(feature = "puroro-bumpalo")]
        define_deser_optional_message_field!($ltag, ::bumpalo::boxed::Box<'bump, T>);
    };
    ($ltag:ty, $box:ty) => {
        impl<'bump, T> FieldDeserFromIter<tags::Message<T>, $ltag> for Option<$box>
        where
            T: crate::deser::DeserializableMessageFromIter,
        {
            type Item = $box;
            fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, f: F) -> Result<()>
            where
                I: Iterator<Item = std::io::Result<u8>>,
                F: Fn() -> Self::Item,
            {
                <T as FieldDeserFromIter<tags::Message<T>, tags::Required>>::deser(
                    self.get_or_insert_with(f), // <- Auto deref works at here!
                    field,
                    || unreachable!(),
                )
            }
        }
    };
}
define_deser_optional_message_field!(tags::Optional2);
define_deser_optional_message_field!(tags::Optional3);

///////////////////////////////////////////////////////////////////////////////
// Repeated fields
///////////////////////////////////////////////////////////////////////////////

macro_rules! define_deser_repeated_variants {
    ($scalar:ty, $ttag:ty) => {
        define_deser_repeated_variants!($scalar, $ttag, Vec<$scalar>);
        #[cfg(feature = "puroro-bumpalo")]
        define_deser_repeated_variants!(
            $scalar,
            $ttag,
            ::bumpalo::collections::Vec<'bump, $scalar>
        );
    };
    ($scalar:ty, $ttag:ty, $vec:ty) => {
        impl<'bump> FieldDeserFromIter<$ttag, tags::Repeated> for $vec {
            type Item = $scalar;
            fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, _: F) -> Result<()>
            where
                I: Iterator<Item = std::io::Result<u8>>,
                F: Fn() -> Self::Item,
            {
                match field {
                    FieldData::Variant(variant) => {
                        self.push(variant.to_native::<$ttag>()?);
                    }
                    FieldData::LengthDelimited(ld_iter) => {
                        let mut var_iter = ld_iter.variants();
                        // The spec demands at least one item for a packed repeated field.
                        self.push(
                            var_iter
                                .next()
                                .ok_or(ErrorKind::ZeroLengthPackedField)??
                                .to_native::<$ttag>()?,
                        );
                        for rvariant in var_iter {
                            self.push(rvariant?.to_native::<$ttag>()?);
                        }
                    }
                    FieldData::Bits32(_) | FieldData::Bits64(_) => {
                        Err(ErrorKind::UnexpectedWireType)?
                    }
                }
                Ok(())
            }
        }
    };
}
define_deser_repeated_variants!(i32, tags::Int32);
define_deser_repeated_variants!(i64, tags::Int64);
define_deser_repeated_variants!(i32, tags::SInt32);
define_deser_repeated_variants!(i64, tags::SInt64);
define_deser_repeated_variants!(u32, tags::UInt32);
define_deser_repeated_variants!(u64, tags::UInt64);
define_deser_repeated_variants!(bool, tags::Bool);

macro_rules! define_deser_repeated_enum {
    () => {
        define_deser_repeated_enum!(Vec<std::result::Result<T, i32>>);
        #[cfg(feature = "puroro-bumpalo")]
        define_deser_repeated_enum!(
            ::bumpalo::collections::Vec<'bump, std::result::Result<T, i32>>
        );
    };
    ($vec:ty) => {
        impl<'bump, T> FieldDeserFromIter<tags::Enum<T>, tags::Repeated> for $vec
        where
            T: TryFrom<i32, Error = i32>,
        {
            type Item = std::result::Result<T, i32>;
            fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, _: F) -> Result<()>
            where
                I: Iterator<Item = std::io::Result<u8>>,
                F: Fn() -> Self::Item,
            {
                match field {
                    FieldData::Variant(variant) => {
                        self.push(T::try_from(variant.to_native::<tags::Int32>()?))
                    }
                    FieldData::LengthDelimited(ld_iter) => {
                        for rvariant in ld_iter.variants() {
                            self.push(T::try_from(rvariant?.to_native::<tags::Int32>()?))
                        }
                    }
                    FieldData::Bits32(_) | FieldData::Bits64(_) => {
                        Err(ErrorKind::UnexpectedWireType)?
                    }
                }
                Ok(())
            }
        }
    };
}
define_deser_repeated_enum!();

macro_rules! define_deser_repeated_ld {
    ($scalar:ty, $ttag:ty, $method:ident) => {
        define_deser_repeated_ld!($scalar, $ttag, $method, Vec<$scalar>);
        #[cfg(feature = "puroro-bumpalo")]
        define_deser_repeated_ld!(
            $scalar,
            $ttag,
            $method,
            ::bumpalo::collections::Vec<'bump, $scalar>
        );
    };
    ($scalar:ty, $ttag:ty, $method:ident, $vec:ty) => {
        impl<'bump> FieldDeserFromIter<$ttag, tags::Repeated> for $vec {
            type Item = $scalar;
            fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, f: F) -> Result<()>
            where
                I: Iterator<Item = std::io::Result<u8>>,
                F: Fn() -> Self::Item,
            {
                if let FieldData::LengthDelimited(ld_iter) = field {
                    let mut new_item = (f)();
                    new_item.reserve(ld_iter.len());
                    for rv in ld_iter.$method() {
                        new_item.push(rv?);
                    }
                    self.push(new_item);
                    Ok(())
                } else {
                    Err(ErrorKind::UnexpectedWireType)?
                }
            }
        }
    };
}
define_deser_repeated_ld!(String, tags::String, chars);
define_deser_repeated_ld!(Vec<u8>, tags::Bytes, bytes);
#[cfg(feature = "puroro-bumpalo")]
define_deser_repeated_ld!(::bumpalo::collections::String<'bump>, tags::String, chars);
#[cfg(feature = "puroro-bumpalo")]
define_deser_repeated_ld!(::bumpalo::collections::Vec<'bump, u8>, tags::Bytes, bytes);

macro_rules! define_deser_repeated_message {
    () => {
        define_deser_repeated_message!(Vec<T>);
        #[cfg(feature = "puroro-bumpalo")]
        define_deser_repeated_message!(::bumpalo::collections::Vec<'bump, T>);
    };
    ($vec:ty) => {
        impl<'bump, T> FieldDeserFromIter<tags::Message<T>, tags::Repeated> for $vec
        where
            T: crate::deser::DeserializableMessageFromIter,
        {
            type Item = T;
            fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, f: F) -> Result<()>
            where
                I: Iterator<Item = std::io::Result<u8>>,
                F: Fn() -> Self::Item,
            {
                if let FieldData::LengthDelimited(ld_iter) = field {
                    let mut new_message = (f)();
                    ld_iter.deser_message(&mut new_message)?;
                    self.push(new_message);
                    Ok(())
                } else {
                    Err(ErrorKind::UnexpectedWireType)?
                }
            }
        }
    };
}
define_deser_repeated_message!();

///////////////////////////////////////////////////////////////////////////////
// Map field
///////////////////////////////////////////////////////////////////////////////

impl<Entry> FieldDeserFromIter<tags::Message<Entry>, tags::Repeated>
    for HashMap<Entry::KeyType, Entry::ValueType>
where
    Entry: MapEntry + DeserializableMessageFromIter,
    Entry::KeyType: Hash + Eq,
{
    type Item = Entry;

    fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item,
    {
        if let FieldData::LengthDelimited(ld_iter) = field {
            let mut entry = (f)();
            ld_iter.deser_message(&mut entry)?;
            let kv = entry.into_tuple();
            self.insert(kv.0, kv.1);
            Ok(())
        } else {
            Err(ErrorKind::UnexpectedWireType)?
        }
    }
}
