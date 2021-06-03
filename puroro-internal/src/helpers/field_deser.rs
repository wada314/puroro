use itertools::{Either, Itertools};
use puroro::Message;

use crate::deser::{DeserializableMessageFromIter, LdIter, LdSlice};
use crate::types::{FieldData, SliceViewField};
use crate::variant;
use crate::variant::VariantTypeTag;
use crate::{tags, ResultHelper};
use crate::{ErrorKind, Result};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::hash::Hash;
use std::intrinsics::transmute;
use std::io::Read;

use super::{DoDefaultCheck, MapEntryForNormalImpl};

pub trait FieldDeserFromIter<TypeTag, LabelTag>
where
    TypeTag: tags::WireAndValueTypeTag,
    LabelTag: tags::FieldLabelTag,
{
    /// The return type of the default instance generator passed to `deser` method.
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
    fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, f: F) -> Result<()>
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
    fn deser(
        &mut self,
        field: FieldData<LdSlice<'slice>>,
        slice_from_this_field: LdSlice<'slice>,
        enclosing_slice: LdSlice<'slice>,
    ) -> Result<()>;
}

impl<V, L, T> FieldDeserFromIter<(tags::wire::Variant, V), L> for T
where
    V: tags::VariantTypeTag + variant::VariantTypeTag,
    L: tags::FieldLabelTag + DoDefaultCheck,
    T: WrappedFieldType<L, Item = <V as variant::VariantTypeTag>::NativeType>,
{
    type Item = <V as variant::VariantTypeTag>::NativeType;

    fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item,
    {
        let iter = to_variant_value_iter::<V, L, I>(field)?;
        self.merge_items(iter)
    }
}

impl<L, T> FieldDeserFromIter<(tags::wire::LengthDelimited, tags::value::String), L> for T
where
    L: tags::FieldLabelTag + DoDefaultCheck,
    T: WrappedFieldType<L>,
    T::Item: StringType,
{
    type Item = String;
    fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, f: F) -> Result<()>
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

impl<L, T> FieldDeserFromIter<(tags::wire::LengthDelimited, tags::value::Bytes), L> for T
where
    L: tags::FieldLabelTag + DoDefaultCheck,
    T: WrappedFieldType<L>,
    T::Item: VecType<Item = u8>,
{
    type Item = String;
    fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, f: F) -> Result<()>
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

impl<T> FieldDeserFromIter<(tags::wire::LengthDelimited, tags::value::Message<T>), tags::Required>
    for Option<T>
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
            ld_iter.deser_message(self.get_or_insert_with(f))?;
        } else {
            Err(ErrorKind::UnexpectedWireType)?
        }
        Ok(())
    }
}

impl<'bump, T>
    FieldDeserFromIter<(tags::wire::LengthDelimited, tags::value::Message<T>), tags::Optional2>
    for Option<<T as Message<'bump>>::BoxedType>
where
    T: crate::deser::DeserializableMessageFromIter + Message<'bump>,
{
    type Item = T;
    fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item,
    {
        if let FieldData::LengthDelimited(ld_iter) = field {
            ld_iter.deser_message(self.get_or_insert_with(|| (f)().into_boxed()))?;
        } else {
            Err(ErrorKind::UnexpectedWireType)?
        }
        Ok(())
    }
}

impl<'bump, T>
    FieldDeserFromIter<(tags::wire::LengthDelimited, tags::value::Message<T>), tags::Optional3>
    for Option<<T as Message<'bump>>::BoxedType>
where
    T: crate::deser::DeserializableMessageFromIter + Message<'bump>,
{
    type Item = T;
    fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item,
    {
        if let FieldData::LengthDelimited(ld_iter) = field {
            ld_iter.deser_message(self.get_or_insert_with(|| (f)().into_boxed()))?;
        } else {
            Err(ErrorKind::UnexpectedWireType)?
        }
        Ok(())
    }
}

impl<'bump, T, VT>
    FieldDeserFromIter<(tags::wire::LengthDelimited, tags::value::Message<T>), tags::Repeated>
    for VT
where
    T: crate::deser::DeserializableMessageFromIter,
    VT: VecType<Item = T>,
{
    type Item = T;
    fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item,
    {
        if let FieldData::LengthDelimited(ld_iter) = field {
            self.push((f)());
            ld_iter.deser_message(self.last_mut().unwrap())?;
        } else {
            Err(ErrorKind::UnexpectedWireType)?
        }
        Ok(())
    }
}

impl<V, L, T> FieldDeserFromIter<(tags::wire::Bits32, V), L> for T
where
    V: tags::Bits32TypeTag,
    L: tags::FieldLabelTag + DoDefaultCheck,
    T: WrappedFieldType<L>,
    T::Item: FromBits32<Tag = V>,
{
    type Item = T::Item;
    fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, f: F) -> Result<()>
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

impl<V, L, T> FieldDeserFromIter<(tags::wire::Bits64, V), L> for T
where
    V: tags::Bits64TypeTag,
    L: tags::FieldLabelTag + DoDefaultCheck,
    T: WrappedFieldType<L>,
    T::Item: FromBits64<Tag = V>,
{
    type Item = T::Item;
    fn deser<'a, I, F>(&mut self, field: FieldData<&'a mut LdIter<I>>, f: F) -> Result<()>
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

trait WrappedFieldType<LabelTag>
where
    LabelTag: tags::FieldLabelTag,
{
    type Item;
    fn merge_items<I>(&mut self, iter: I) -> Result<()>
    where
        I: Iterator<Item = Result<Self::Item>>;
    fn get_or_insert_with<F>(&mut self, f: F) -> &mut Self::Item
    where
        F: FnOnce() -> Self::Item;
}
impl<T> WrappedFieldType<tags::Required> for T {
    type Item = T;
    fn merge_items<I>(&mut self, iter: I) -> Result<()>
    where
        I: Iterator<Item = Result<Self::Item>>,
    {
        if let Some(item) = iter.last().transpose()? {
            *self = item;
        }
        Ok(())
    }
    fn get_or_insert_with<F>(&mut self, f: F) -> &mut Self::Item
    where
        F: FnOnce() -> Self::Item,
    {
        self
    }
}
impl<T> WrappedFieldType<tags::Optional2> for Option<T> {
    type Item = T;
    fn merge_items<I>(&mut self, iter: I) -> Result<()>
    where
        I: Iterator<Item = Result<Self::Item>>,
    {
        if let Some(item) = iter.last().transpose()? {
            *self = Some(item);
        }
        Ok(())
    }
    fn get_or_insert_with<F>(&mut self, f: F) -> &mut Self::Item
    where
        F: FnOnce() -> Self::Item,
    {
        self.get_or_insert_with(f)
    }
}
impl<T> WrappedFieldType<tags::Optional3> for T {
    type Item = T;
    fn merge_items<I>(&mut self, iter: I) -> Result<()>
    where
        I: Iterator<Item = Result<Self::Item>>,
    {
        if let Some(item) = iter.last().transpose()? {
            *self = item;
        }
        Ok(())
    }
    fn get_or_insert_with<F>(&mut self, f: F) -> &mut Self::Item
    where
        F: FnOnce() -> Self::Item,
    {
        self
    }
}
impl<VT> WrappedFieldType<tags::Repeated> for VT
where
    VT: VecType,
{
    type Item = VT::Item;
    fn merge_items<I>(&mut self, iter: I) -> Result<()>
    where
        I: Iterator<Item = Result<Self::Item>>,
    {
        for ritem in iter {
            self.push(ritem?);
        }
        Ok(())
    }
    fn get_or_insert_with<F>(&mut self, f: F) -> &mut Self::Item
    where
        F: FnOnce() -> Self::Item,
    {
        self.push((f)());
        self.last_mut().unwrap()
    }
}

trait VecType {
    type Item;
    fn push(&mut self, item: Self::Item);
    fn last_mut(&mut self) -> Option<&mut Self::Item>;
}
impl<T> VecType for Vec<T> {
    type Item = T;
    fn push(&mut self, item: Self::Item) {
        <Vec<Self::Item>>::push(self, item)
    }
    fn last_mut(&mut self) -> Option<&mut Self::Item> {
        <Vec<Self::Item>>::last_mut(self)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump, T> VecType for ::bumpalo::collections::Vec<'bump, T> {
    type Item = T;
    fn push(&mut self, item: Self::Item) {
        <::bumpalo::collections::Vec<'bump, Self::Item>>::push(self, item)
    }
    fn last_mut(&mut self) -> Option<&mut Self::Item> {
        <::bumpalo::collections::Vec<'bump, Self::Item>>::last_mut(self)
    }
}

trait StringType {
    fn push(&mut self, c: char);
    fn clear(&mut self);
    fn reserve(&mut self, bytes_len: usize);
}
impl StringType for String {
    fn push(&mut self, c: char) {
        <String>::push(self, c)
    }
    fn clear(&mut self) {
        <String>::clear(self)
    }
    fn reserve(&mut self, bytes_len: usize) {
        <String>::reserve(self, bytes_len)
    }
}
#[cfg(feature = "puroro-bumpalo")]
impl<'bump> StringType for ::bumpalo::collections::String<'bump> {
    fn push(&mut self, c: char) {
        <::bumpalo::collections::String<'bump>>::push(self, c)
    }
    fn clear(&mut self) {
        <::bumpalo::collections::String<'bump>>::clear(self)
    }
    fn reserve(&mut self, bytes_len: usize) {
        <::bumpalo::collections::String<'bump>>::reserve(self, bytes_len)
    }
}

trait FromBits32: Sized {
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

trait FromBits64: Sized {
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

fn to_variant_value_iter<V, L, I>(
    field: FieldData<&mut LdIter<I>>,
) -> Result<impl Iterator<Item = Result<<V as variant::VariantTypeTag>::NativeType>>>
where
    V: tags::VariantTypeTag + variant::VariantTypeTag,
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

///////////////////////////////////////////////////////////////////////////////
// Map field
///////////////////////////////////////////////////////////////////////////////

impl<Entry> FieldDeserFromIter<tags::Message<Entry>, tags::Repeated>
    for HashMap<Entry::OwnedKeyType, Entry::OwnedValueType>
where
    Entry: MapEntryForNormalImpl + DeserializableMessageFromIter,
    Entry::OwnedKeyType: Hash + Eq,
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
