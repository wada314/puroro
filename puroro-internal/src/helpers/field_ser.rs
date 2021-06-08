use puroro::Message;

use super::{BytesType, DoDefaultCheck, StringType, WrappedFieldType, WrappedMessageFieldType};
use crate::ser::{MessageSerializer, SerializableMessage};
use crate::tags;
use crate::tags::{FieldLabelTag, WireAndValueTypeTag};
use crate::variant;
use crate::Result;

pub trait FieldSer<TypeTag, LabelTag>
where
    TypeTag: WireAndValueTypeTag,
    LabelTag: FieldLabelTag,
{
    fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
    where
        S: crate::ser::MessageSerializer;
}

impl<V, L, T> FieldSer<(tags::wire::Variant, V), L> for T
where
    V: tags::VariantTypeTag + variant::VariantTypeTag,
    L: tags::FieldLabelTag + DoDefaultCheck,
    T: WrappedFieldType<L, Item = <V as variant::VariantTypeTag>::NativeType>,
{
    fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
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

impl<L, T> FieldSer<tags::String, L> for T
where
    L: tags::FieldLabelTag + DoDefaultCheck,
    T: WrappedFieldType<L>,
    T::Item: StringType,
{
    fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
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

impl<L, T> FieldSer<tags::Bytes, L> for T
where
    L: tags::FieldLabelTag + DoDefaultCheck,
    T: WrappedFieldType<L>,
    T::Item: BytesType,
{
    fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
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
impl<L, M, T> FieldSer<tags::Message<M>, L> for T
where
    L: tags::FieldLabelTag + DoDefaultCheck,
    T: WrappedMessageFieldType<M, L>,
    M: Message,
    T::Item: Message + SerializableMessage,
{
    fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
    where
        S: MessageSerializer,
    {
        self.try_for_each(|item| -> Result<_> {
            serializer.serialize_message_twice(field_number, item)?;
            Ok(())
        })
    }
}

impl<V, L, T> FieldSer<(tags::wire::Bits32, V), L> for T
where
    V: tags::Bits32TypeTag,
    L: tags::FieldLabelTag + DoDefaultCheck,
    T: WrappedFieldType<L>,
    T::Item: IntoBits32<Tag = V> + Clone,
{
    fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
    where
        S: crate::ser::MessageSerializer,
    {
        self.try_for_each(|item| -> Result<_> {
            let bytes = <T::Item as IntoBits32>::into(item.clone());
            if !L::DO_DEFAULT_CHECK || bytes.iter().any(|x| *x != 0) {
                serializer.serialize_fixed_bits::<4>(field_number, bytes)?;
            }
            Ok(())
        })?;
        Ok(())
    }
}

impl<V, L, T> FieldSer<(tags::wire::Bits64, V), L> for T
where
    V: tags::Bits64TypeTag,
    L: tags::FieldLabelTag + DoDefaultCheck,
    T: WrappedFieldType<L>,
    T::Item: IntoBits64<Tag = V> + Clone,
{
    fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
    where
        S: crate::ser::MessageSerializer,
    {
        self.try_for_each(|item| -> Result<_> {
            let bytes = <T::Item as IntoBits64>::into(item.clone());
            if !L::DO_DEFAULT_CHECK || bytes.iter().any(|x| *x != 0) {
                serializer.serialize_fixed_bits::<8>(field_number, bytes)?;
            }
            Ok(())
        })?;
        Ok(())
    }
}

pub trait IntoBits32 {
    type Tag: tags::Bits32TypeTag;
    fn into(self) -> [u8; 4];
}
impl IntoBits32 for f32 {
    type Tag = tags::value::Float;
    fn into(self) -> [u8; 4] {
        self.to_le_bytes()
    }
}
impl IntoBits32 for u32 {
    type Tag = tags::value::Fixed32;
    fn into(self) -> [u8; 4] {
        self.to_le_bytes()
    }
}
impl IntoBits32 for i32 {
    type Tag = tags::value::SFixed32;
    fn into(self) -> [u8; 4] {
        self.to_le_bytes()
    }
}

pub trait IntoBits64 {
    type Tag: tags::Bits64TypeTag;
    fn into(self) -> [u8; 8];
}
impl IntoBits64 for f64 {
    type Tag = tags::value::Double;
    fn into(self) -> [u8; 8] {
        self.to_le_bytes()
    }
}
impl IntoBits64 for u64 {
    type Tag = tags::value::Fixed64;
    fn into(self) -> [u8; 8] {
        self.to_le_bytes()
    }
}
impl IntoBits64 for i64 {
    type Tag = tags::value::SFixed64;
    fn into(self) -> [u8; 8] {
        self.to_le_bytes()
    }
}

///////////////////////////////////////////////////////////////////////////////
// Map field
///////////////////////////////////////////////////////////////////////////////
/*
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
*/
