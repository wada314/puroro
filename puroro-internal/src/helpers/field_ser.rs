use puroro::Message;

use super::{BytesType, DoDefaultCheck, StringType, WrappedFieldType, WrappedMessageFieldType};
use crate::ser::{MessageSerializer, SerializableMessage};
use crate::variant;
use crate::Result;
use ::puroro::tags;
use ::puroro::tags::{FieldLabelTag, WireAndValueTypeTag};

pub trait FieldSer<'a, TypeTag, LabelTag>
where
    TypeTag: WireAndValueTypeTag,
    LabelTag: FieldLabelTag,
{
    fn ser<S>(&'a self, serializer: &mut S, field_number: usize) -> Result<()>
    where
        S: crate::ser::MessageSerializer;
}

impl<'a, V, L, T> FieldSer<'a, (tags::wire::Variant, V), L> for T
where
    V: tags::VariantTypeTag + variant::VariantTypeTag,
    L: tags::FieldLabelTag + DoDefaultCheck,
    T: WrappedFieldType<L, Item = <V as variant::VariantTypeTag>::NativeType>,
{
    fn ser<S>(&'a self, serializer: &mut S, field_number: usize) -> Result<()>
    where
        S: MessageSerializer,
    {
        let slice = self.as_slice();
        if slice.len() <= 1 {
            if let Some(item) = slice.first() {
                let variant = V::to_variant(item.clone())?;
                if !L::DO_DEFAULT_CHECK || !variant.is_zero() {
                    serializer.serialize_variant::<V>(field_number, item.clone())?;
                }
            }
        } else {
            serializer.serialize_variants_twice::<V, _>(
                field_number,
                slice.iter().cloned().map(|x| Ok(x)),
            )?;
        }
        Ok(())
    }
}

impl<'a, L, T> FieldSer<'a, tags::String, L> for T
where
    L: tags::FieldLabelTag + DoDefaultCheck,
    T: WrappedFieldType<L>,
    T::Item: StringType,
{
    fn ser<S>(&'a self, serializer: &mut S, field_number: usize) -> Result<()>
    where
        S: MessageSerializer,
    {
        self.try_for_each(|item| -> Result<_> {
            if !L::DO_DEFAULT_CHECK || <T::Item as StringType>::len(item) != 0 {
                serializer.serialize_bytes_twice(
                    field_number,
                    item.as_bytes().iter().cloned().map(|x| Ok(x)),
                )?;
            }
            Ok(())
        })?;
        Ok(())
    }
}

impl<'a, L, T> FieldSer<'a, tags::Bytes, L> for T
where
    L: tags::FieldLabelTag + DoDefaultCheck,
    T: WrappedFieldType<L>,
    T::Item: BytesType,
{
    fn ser<S>(&'a self, serializer: &mut S, field_number: usize) -> Result<()>
    where
        S: MessageSerializer,
    {
        self.try_for_each(|item| -> Result<_> {
            if !L::DO_DEFAULT_CHECK || <T::Item as BytesType>::len(item) != 0 {
                serializer.serialize_bytes_twice(
                    field_number,
                    <T::Item as BytesType>::as_slice(item)
                        .iter()
                        .cloned()
                        .map(|x| Ok(x)),
                )?;
            }
            Ok(())
        })?;
        Ok(())
    }
}

// Note: For map implementation, `M` might not be equal to `T::Item`
impl<'a, L, M, T> FieldSer<'a, tags::Message<M>, L> for T
where
    L: tags::FieldLabelTag + DoDefaultCheck,
    T: WrappedMessageFieldType<'a, M, L>,
    M: Message,
    T::Item: Message + SerializableMessage,
{
    fn ser<S>(&'a self, serializer: &mut S, field_number: usize) -> Result<()>
    where
        S: MessageSerializer,
    {
        self.try_for_each(|item| -> Result<_> {
            serializer.serialize_message_twice(field_number, item)?;
            Ok(())
        })
    }
}

impl<'a, V, L, T> FieldSer<'a, (tags::wire::Bits32, V), L> for T
where
    V: tags::Bits32TypeTag + super::Bits32TypeTag<NativeType = T::Item>,
    L: tags::FieldLabelTag + DoDefaultCheck,
    T: WrappedFieldType<L>,
    T::Item: Clone,
{
    fn ser<S>(&'a self, serializer: &mut S, field_number: usize) -> Result<()>
    where
        S: crate::ser::MessageSerializer,
    {
        self.try_for_each(|item| -> Result<_> {
            let bytes = <V as super::Bits32TypeTag>::into_bytes(item.clone());
            if !L::DO_DEFAULT_CHECK || bytes.iter().any(|x| *x != 0) {
                serializer.serialize_fixed_bits::<4>(field_number, bytes)?;
            }
            Ok(())
        })?;
        Ok(())
    }
}

impl<'a, V, L, T> FieldSer<'a, (tags::wire::Bits64, V), L> for T
where
    V: tags::Bits64TypeTag + super::Bits64TypeTag<NativeType = T::Item>,
    L: tags::FieldLabelTag + DoDefaultCheck,
    T: WrappedFieldType<L>,
    T::Item: Clone,
{
    fn ser<S>(&'a self, serializer: &mut S, field_number: usize) -> Result<()>
    where
        S: crate::ser::MessageSerializer,
    {
        self.try_for_each(|item| -> Result<_> {
            let bytes = <V as super::Bits64TypeTag>::into_bytes(item.clone());
            if !L::DO_DEFAULT_CHECK || bytes.iter().any(|x| *x != 0) {
                serializer.serialize_fixed_bits::<8>(field_number, bytes)?;
            }
            Ok(())
        })?;
        Ok(())
    }
}

///////////////////////////////////////////////////////////////////////////////
// Map field
///////////////////////////////////////////////////////////////////////////////
/*
impl<Entry> FieldSer<'a, tags::Message<Entry>, tags::Repeated>
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
*/
