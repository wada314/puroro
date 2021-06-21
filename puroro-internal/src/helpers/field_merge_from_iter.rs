use std::ops::Deref;
use std::ops::DerefMut;

use ::puroro::tags;
use itertools::{Either, Itertools};
use puroro::Message;

use crate::deser::LdIter;
use crate::deser::MergeableMessageFromIter;
use crate::types::FieldData;
use crate::variant;
use crate::ResultHelper;
use crate::{ErrorKind, Result};

use super::{BytesType, DoDefaultCheck, StringType, WrappedFieldType, WrappedMessageFieldType};

pub trait FieldMergeFromIter<'a, TypeTag, LabelTag>
where
    TypeTag: tags::WireAndValueTypeTag,
    LabelTag: tags::FieldLabelTag,
{
    /// The return type of the default instance generator passed to `merge` method.
    type Item;
    /// Deserialize & merge binary data into this field.
    /// * `field` - A data of the field, where the wire type and (for length delimited wire
    /// type) the field length are already load. For variants and fixed bytes fields,
    /// the content data is also already load.
    /// * `f` - A function object that generates default instance of the field `Item`.
    /// It depends on the field type what type the field item is.
    /// ** numeric types (except enum) - The corresponding rust's numeric types.
    /// Typically it's just a `Default::default`.
    /// ** Enum types - Because our rust's corresponding type `Result<T, i32>` does not
    /// implement `Default`, we need our own initializing function object.
    /// ** string, bytes types - The corresponding rust's types (e.g. `String`, `Vec<u8>`).
    /// It seems to be trivial and we can just use `Default::default`, but if we use a
    /// custom allocator then we need an allocator instance value to instanciate the default
    /// value, which `Default::default` cannot support.
    /// ** Message types - A raw message type. Same as above, we cannot use `Default::default`
    /// for custom allocator type.
    fn merge<I, F>(&'a mut self, field: FieldData<&mut LdIter<I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item;
}

impl<'a, V, L, T> FieldMergeFromIter<'a, (tags::wire::Variant, V), L> for T
where
    V: tags::VariantTypeTag + variant::VariantTypeTag,
    L: tags::FieldLabelTag + DoDefaultCheck,
    T: WrappedFieldType<L, Item = <V as variant::VariantTypeTag>::NativeType>,
{
    type Item = <V as variant::VariantTypeTag>::NativeType;

    fn merge<I, F>(&'a mut self, field: FieldData<&mut LdIter<I>>, _: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item,
    {
        let iter = to_variant_value_iter::<V, L, I>(field)?;
        self.merge_items(iter)
    }
}

impl<'a, L, T> FieldMergeFromIter<'a, tags::String, L> for T
where
    L: tags::FieldLabelTag + DoDefaultCheck,
    T: WrappedFieldType<L>,
    T::Item: StringType,
{
    type Item = T::Item;
    fn merge<I, F>(&'a mut self, field: FieldData<&mut LdIter<I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item,
    {
        if let FieldData::LengthDelimited(ld_iter) = field {
            let maybe_expected_len = <LdIter<I>>::len(ld_iter);
            let mut iter = ld_iter.chars().peekable();
            if !L::DO_DEFAULT_CHECK || matches!(iter.peek(), Some(_)) {
                // Do not invoke get_or_insert_with until we make sure
                // that the input value is not empty
                let item = self.get_or_insert_with(f);
                item.clear();
                if let Some(expected_len) = maybe_expected_len {
                    item.reserve(expected_len);
                }
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

impl<'a, L, T> FieldMergeFromIter<'a, tags::Bytes, L> for T
where
    L: tags::FieldLabelTag + DoDefaultCheck,
    T: WrappedFieldType<L>,
    T::Item: BytesType,
{
    type Item = T::Item;
    fn merge<I, F>(&'a mut self, field: FieldData<&mut LdIter<I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item,
    {
        if let FieldData::LengthDelimited(ld_iter) = field {
            let maybe_expected_len = <LdIter<I>>::len(ld_iter);
            let mut iter = ld_iter.bytes().peekable();
            if !L::DO_DEFAULT_CHECK || matches!(iter.peek(), Some(_)) {
                // Do not invoke get_or_insert_with until we make sure
                // that the input value is not empty
                let item = self.get_or_insert_with(f);
                item.clear();
                if let Some(expected_len) = maybe_expected_len {
                    item.reserve(expected_len);
                }
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

impl<'a, MessageTag, L, T> FieldMergeFromIter<'a, tags::Message<MessageTag>, L> for T
where
    L: tags::FieldLabelTag,
    T: WrappedMessageFieldType<'a, MessageTag, L>,
    <T as WrappedMessageFieldType<'a, MessageTag, L>>::Item: Message + MergeableMessageFromIter,
    <<T as WrappedMessageFieldType<'a, MessageTag, L>>::MergeableMut as Deref>::Target:
        MergeableMessageFromIter,
{
    type Item = <T as WrappedMessageFieldType<'a, MessageTag, L>>::Item;
    fn merge<I, F>(&'a mut self, field: FieldData<&mut LdIter<I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item,
    {
        if let FieldData::LengthDelimited(ld_iter) = field {
            ld_iter.merge_message(self.get_or_insert_with(f).deref_mut())?;
        } else {
            Err(ErrorKind::UnexpectedWireType)?
        }
        Ok(())
    }
}

impl<'a, V, L, T> FieldMergeFromIter<'a, (tags::wire::Bits32, V), L> for T
where
    V: tags::Bits32TypeTag + super::Bits32TypeTag<NativeType = T::Item>,
    L: tags::FieldLabelTag + DoDefaultCheck,
    T: WrappedFieldType<L>,
{
    type Item = T::Item;
    fn merge<I, F>(&mut self, field: FieldData<&mut LdIter<I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item,
    {
        if let FieldData::Bits32(array) = field {
            if !L::DO_DEFAULT_CHECK || array.iter().any(|b| *b != 0) {
                *self.get_or_insert_with(f) = <V as super::Bits32TypeTag>::from_bytes(array);
            }
            Ok(())
        } else {
            Err(ErrorKind::UnexpectedWireType)?
        }
    }
}

impl<'a, V, L, T> FieldMergeFromIter<'a, (tags::wire::Bits64, V), L> for T
where
    V: tags::Bits64TypeTag + super::Bits64TypeTag<NativeType = T::Item>,
    L: tags::FieldLabelTag + DoDefaultCheck,
    T: WrappedFieldType<L>,
{
    type Item = T::Item;
    fn merge<I, F>(&mut self, field: FieldData<&mut LdIter<I>>, f: F) -> Result<()>
    where
        I: Iterator<Item = std::io::Result<u8>>,
        F: Fn() -> Self::Item,
    {
        if let FieldData::Bits64(array) = field {
            if !L::DO_DEFAULT_CHECK || array.iter().any(|b| *b != 0) {
                *self.get_or_insert_with(f) = <V as super::Bits64TypeTag>::from_bytes(array);
            }
            Ok(())
        } else {
            Err(ErrorKind::UnexpectedWireType)?
        }
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
