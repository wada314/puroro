use itertools::{Either, Itertools};
use puroro::Message;

use crate::deser::{DeserializableMessageFromIter, LdIter, LdSlice};
use crate::types::FieldData;
use crate::variant;
use crate::{tags, ResultHelper};
use crate::{ErrorKind, Result};
use std::collections::HashMap;
use std::hash::Hash;

use super::{
    DoDefaultCheck, MapEntryForNormalImpl, StringType, VecType, WrappedFieldType,
    WrappedMessageFieldType,
};

pub trait FieldMergeFromIter<TypeTag, LabelTag>
where
    TypeTag: tags::WireAndValueTypeTag,
    LabelTag: tags::FieldLabelTag,
{
    /// The return type of the default instance generator passed to `merge` method.
    type Item;
    /// Deserialize binary data into this field.
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
    fn merge<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item;
}
pub trait FieldDeserFromSlice<'slice, TypeTag, LabelTag>
where
    TypeTag: tags::WireAndValueTypeTag,
    LabelTag: tags::FieldLabelTag,
{
    /// Deserialize binary data into this field.
    /// * `field` - A data of the field, where the wire type and (for length delimited wire
    /// type) the field length are already load. For variants and fixed bytes fields,
    /// the content data is also already load.
    /// * `slice_from_this_field` - a subslice of `enclosing_slice` starting from the field's
    /// first byte (including the bytes for wire_type, field_number and field_length).
    /// * `enclosing_slice` - Slice for this field's owner's fields. If the owner message is
    /// split into multiple instances in the input slice, then the instance of the one that
    /// this field is included.
    fn merge(
        &mut self,
        field: FieldData<LdSlice<'slice>>,
        slice_from_this_field: LdSlice<'slice>,
        enclosing_slice: LdSlice<'slice>,
    ) -> Result<()>;
}

impl<V, L, T> FieldMergeFromIter<(tags::wire::Variant, V), L> for T
where
    V: tags::VariantTypeTag + variant::VariantTypeTag,
    L: tags::FieldLabelTag + DoDefaultCheck,
    T: WrappedFieldType<L, Item = <V as variant::VariantTypeTag>::NativeType>,
{
    type Item = <V as variant::VariantTypeTag>::NativeType;

    fn merge<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item,
    {
        let iter = to_variant_value_iter::<V, L, I>(field)?;
        self.merge_items(iter)
    }
}

impl<L, T> FieldMergeFromIter<tags::String, L> for T
where
    L: tags::FieldLabelTag + DoDefaultCheck,
    T: WrappedFieldType<L>,
    T::Item: StringType,
{
    type Item = T::Item;
    fn merge<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item,
    {
        if let FieldData::LengthDelimited(ld_iter) = field {
            let expected_len = ld_iter.len();
            let mut iter = ld_iter.chars().peekable();
            if !L::DO_DEFAULT_CHECK || matches!(iter.peek(), Some(_)) {
                // Do not invoke get_or_insert_with until we make sure
                // that the input value is not empty
                let item = self.get_or_insert_with(f);
                item.clear();
                item.reserve(expected_len);
                for rv in iter {
                    item.push(rv?);
                }
            }
            Ok(())
        } else {
            Err(ErrorKind::UnexpectedWireType)?
        }
    }
}

impl<L, T> FieldMergeFromIter<tags::Bytes, L> for T
where
    L: tags::FieldLabelTag + DoDefaultCheck,
    T: WrappedFieldType<L>,
    T::Item: VecType<Item = u8>,
{
    type Item = T::Item;
    fn merge<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item,
    {
        if let FieldData::LengthDelimited(ld_iter) = field {
            let expected_len = ld_iter.len();
            let mut iter = ld_iter.bytes().peekable();
            if !L::DO_DEFAULT_CHECK || matches!(iter.peek(), Some(_)) {
                // Do not invoke get_or_insert_with until we make sure
                // that the input value is not empty
                let item = self.get_or_insert_with(f);
                item.clear();
                item.reserve(expected_len);
                for rv in iter {
                    item.push(rv?);
                }
            }
            Ok(())
        } else {
            Err(ErrorKind::UnexpectedWireType)?
        }
    }
}

impl<M, L, T> FieldMergeFromIter<tags::Message<M>, L> for T
where
    M: Message + crate::deser::DeserializableMessageFromIter,
    L: tags::FieldLabelTag,
    T: WrappedMessageFieldType<M, L, Item = M>,
{
    type Item = M;
    fn merge<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item,
    {
        if let FieldData::LengthDelimited(ld_iter) = field {
            ld_iter.deser_message(self.get_or_insert_with(f))?;
        } else {
            Err(ErrorKind::UnexpectedWireType)?
        }
        Ok(())
    }
}

impl<V, L, T> FieldMergeFromIter<(tags::wire::Bits32, V), L> for T
where
    V: tags::Bits32TypeTag,
    L: tags::FieldLabelTag + DoDefaultCheck,
    T: WrappedFieldType<L>,
    T::Item: FromBits32<Tag = V>,
{
    type Item = T::Item;
    fn merge<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item,
    {
        if let FieldData::Bits32(array) = field {
            if !L::DO_DEFAULT_CHECK || array.iter().any(|b| *b != 0) {
                *self.get_or_insert_with(f) = <T::Item as FromBits32>::from(array);
            }
            Ok(())
        } else {
            Err(ErrorKind::UnexpectedWireType)?
        }
    }
}

impl<V, L, T> FieldMergeFromIter<(tags::wire::Bits64, V), L> for T
where
    V: tags::Bits64TypeTag,
    L: tags::FieldLabelTag + DoDefaultCheck,
    T: WrappedFieldType<L>,
    T::Item: FromBits64<Tag = V>,
{
    type Item = T::Item;
    fn merge<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item,
    {
        if let FieldData::Bits64(array) = field {
            if !L::DO_DEFAULT_CHECK || array.iter().any(|b| *b != 0) {
                *self.get_or_insert_with(f) = <T::Item as FromBits64>::from(array);
            }
            Ok(())
        } else {
            Err(ErrorKind::UnexpectedWireType)?
        }
    }
}

pub trait FromBits32: Sized {
    type Tag: tags::Bits32TypeTag;
    fn from(array: [u8; 4]) -> Self;
}
impl FromBits32 for f32 {
    type Tag = tags::value::Float;
    fn from(array: [u8; 4]) -> Self {
        f32::from_le_bytes(array)
    }
}
impl FromBits32 for u32 {
    type Tag = tags::value::Fixed32;
    fn from(array: [u8; 4]) -> Self {
        u32::from_le_bytes(array)
    }
}
impl FromBits32 for i32 {
    type Tag = tags::value::SFixed32;
    fn from(array: [u8; 4]) -> Self {
        i32::from_le_bytes(array)
    }
}

pub trait FromBits64: Sized {
    type Tag: tags::Bits64TypeTag;
    fn from(array: [u8; 8]) -> Self;
}
impl FromBits64 for f64 {
    type Tag = tags::value::Double;
    fn from(array: [u8; 8]) -> Self {
        f64::from_le_bytes(array)
    }
}
impl FromBits64 for u64 {
    type Tag = tags::value::Fixed64;
    fn from(array: [u8; 8]) -> Self {
        u64::from_le_bytes(array)
    }
}
impl FromBits64 for i64 {
    type Tag = tags::value::SFixed64;
    fn from(array: [u8; 8]) -> Self {
        i64::from_le_bytes(array)
    }
}

fn to_variant_value_iter<'a, V, L, I>(
    field: FieldData<&'a mut LdIter<I>>,
) -> Result<impl 'a + Iterator<Item = Result<<V as variant::VariantTypeTag>::NativeType>>>
where
    V: tags::VariantTypeTag + variant::VariantTypeTag,
    <V as variant::VariantTypeTag>::NativeType: 'a,
    L: tags::FieldLabelTag + DoDefaultCheck,
    I: Iterator<Item = std::io::Result<u8>>,
{
    Ok(match field {
        FieldData::Variant(variant) => {
            let iter = if !L::DO_DEFAULT_CHECK || !variant.is_zero() {
                Some(variant.to_native::<V>())
            } else {
                None
            }
            .into_iter();
            Either::Left(iter)
        }
        FieldData::LengthDelimited(ld_iter) => {
            let mut variants = ld_iter.variants().peekable();
            if let None = variants.peek() {
                Err(ErrorKind::ZeroLengthPackedField)?
            }
            let iter = variants
                .filter_map_ok(|v| {
                    if !L::DO_DEFAULT_CHECK || !v.is_zero() {
                        Some(v.to_native::<V>())
                    } else {
                        None
                    }
                })
                .map(|rrvalue| rrvalue.flatten());
            Either::Right(iter)
        }
        _ => Err(ErrorKind::InvalidWireType)?,
    })
}
