use std::collections::HashMap;

use num_traits::Zero;

use super::{DoDefaultCheck, MapEntryForNormalImpl, StringType, VecType, WrappedFieldType};
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
                let variant = V::to_variant(*item)?;
                if !L::DO_DEFAULT_CHECK || !variant.is_zero() {
                    serializer.serialize_variant::<V>(field_number, *item)?;
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
        let slice = self.as_slice();
        for item in slice {
            if !L::DO_DEFAULT_CHECK || item.len() != 0 {
                serializer.serialize_bytes_twice(
                    field_number,
                    item.as_bytes().iter().cloned().map(|x| Ok(x)),
                )?
            }
        }
        Ok(())
    }
}

impl<L, T> FieldSer<tags::Bytes, L> for T
where
    L: tags::FieldLabelTag + DoDefaultCheck,
    T: WrappedFieldType<L>,
    T::Item: VecType<Item = u8>,
{
    fn ser<S>(&self, serializer: &mut S, field_number: usize) -> Result<()>
    where
        S: MessageSerializer,
    {
        let slice = self.as_slice();
        for item in slice {
            if !L::DO_DEFAULT_CHECK || item.len() != 0 {
                serializer.serialize_bytes_twice(
                    field_number,
                    <T::Item as VecType>::as_slice(item)
                        .iter()
                        .cloned()
                        .map(|x| Ok(x)),
                )?
            }
        }
        Ok(())
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
        for item in self.as_slice() {
            let bytes = <T::Item as IntoBits32>::into(item.clone());
            if !L::DO_DEFAULT_CHECK || bytes.iter().any(|x| *x != 0) {
                serializer.serialize_fixed_bits::<4>(field_number, bytes)?;
            }
        }
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
        for item in self.as_slice() {
            let bytes = <T::Item as IntoBits64>::into(item.clone());
            if !L::DO_DEFAULT_CHECK || bytes.iter().any(|x| *x != 0) {
                serializer.serialize_fixed_bits::<8>(field_number, bytes)?;
            }
        }
        Ok(())
    }
}

trait IntoBits32 {
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

trait IntoBits64 {
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
