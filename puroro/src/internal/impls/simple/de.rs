use super::VecOrOptionOrBare;
use crate::fixed_bits::{Bits32TypeTag, Bits64TypeTag};
use crate::internal::de::from_iter::{deser_from_scoped_iter, ScopedIter, Variants};
use crate::internal::de::DeserFieldsFromBytesIter;
use crate::types::FieldData;
use crate::variant::VariantTypeTag;
use crate::ErrorKind;
use crate::{tags, Result};
use ::std::marker::PhantomData;
use ::std::ops::DerefMut;

// deser from iter methods

pub struct DeserFieldFromBytesIter<L, V>(PhantomData<(L, V)>);

impl<L, V> DeserFieldFromBytesIter<L, tags::wire::Variant<V>>
where
    L: tags::FieldLabelTag,
    tags::wire::Variant<V>: VariantTypeTag,
{
    pub fn deser_field<FieldType, I>(
        field: &mut FieldType,
        input: FieldData<&mut ScopedIter<I>>,
    ) -> Result<()>
    where
        FieldType:
            VecOrOptionOrBare<<tags::wire::Variant<V> as tags::NumericalTypeTag>::NativeType>,
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        match input {
            FieldData::Variant(v) => {
                let native_value = v.to_native::<tags::wire::Variant<V>>()?;
                if !L::DO_DEFAULT_CHECK || native_value != Default::default() {
                    field.push(native_value);
                }
            }
            FieldData::LengthDelimited(iter) => {
                let variants = Variants::new(iter);
                let values =
                    variants.map(|rv| rv.and_then(|v| v.to_native::<tags::wire::Variant<V>>()));
                for rvalue in values {
                    let native_value = rvalue?;
                    if !L::DO_DEFAULT_CHECK || native_value != Default::default() {
                        field.push(native_value);
                    }
                }
            }
            _ => Err(ErrorKind::UnexpectedWireType)?,
        }
        Ok(())
    }
}

impl<L, V> DeserFieldFromBytesIter<L, tags::wire::Bits32<V>>
where
    L: tags::FieldLabelTag,
    tags::wire::Bits32<V>: Bits32TypeTag,
{
    pub fn deser_field<FieldType, I>(
        field: &mut FieldType,
        input: FieldData<&mut ScopedIter<I>>,
    ) -> Result<()>
    where
        FieldType: VecOrOptionOrBare<<tags::wire::Bits32<V> as tags::NumericalTypeTag>::NativeType>,
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        if let FieldData::Bits32(bytes) = input {
            let native_value = tags::wire::Bits32::<V>::from_array(bytes);
            if !L::DO_DEFAULT_CHECK || native_value != Default::default() {
                field.push(native_value);
            }
        } else {
            Err(ErrorKind::UnexpectedWireType)?;
        }
        Ok(())
    }
}

impl<L, V> DeserFieldFromBytesIter<L, tags::wire::Bits64<V>>
where
    L: tags::FieldLabelTag,
    tags::wire::Bits64<V>: Bits64TypeTag,
{
    pub fn deser_field<FieldType, I>(
        field: &mut FieldType,
        input: FieldData<&mut ScopedIter<I>>,
    ) -> Result<()>
    where
        FieldType: VecOrOptionOrBare<<tags::wire::Bits64<V> as tags::NumericalTypeTag>::NativeType>,
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        if let FieldData::Bits64(bytes) = input {
            let native_value = tags::wire::Bits64::<V>::from_array(bytes);
            if !L::DO_DEFAULT_CHECK || native_value != Default::default() {
                field.push(native_value);
            }
        } else {
            Err(ErrorKind::UnexpectedWireType)?;
        }
        Ok(())
    }
}

impl<L> DeserFieldFromBytesIter<L, tags::String>
where
    L: tags::FieldLabelTag,
{
    pub fn deser_field<FieldType, I>(
        field: &mut FieldType,
        input: FieldData<&mut ScopedIter<I>>,
    ) -> Result<()>
    where
        FieldType: VecOrOptionOrBare<String>,
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        if let FieldData::LengthDelimited(iter) = input {
            let string = String::from_utf8(iter.collect::<::std::io::Result<Vec<_>>>()?)
                .map_err(|e| ErrorKind::InvalidUtf8(e))?;
            if !L::DO_DEFAULT_CHECK || !string.is_empty() {
                field.push(string);
            }
        } else {
            Err(ErrorKind::UnexpectedWireType)?;
        }
        Ok(())
    }
}

impl<L> DeserFieldFromBytesIter<L, tags::Bytes>
where
    L: tags::FieldLabelTag,
{
    pub fn deser_field<FieldType, I>(
        field: &mut FieldType,
        input: FieldData<&mut ScopedIter<I>>,
    ) -> Result<()>
    where
        FieldType: VecOrOptionOrBare<Vec<u8>>,
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        if let FieldData::LengthDelimited(iter) = input {
            let bytes = iter.collect::<::std::io::Result<Vec<_>>>()?;
            if !L::DO_DEFAULT_CHECK || !bytes.is_empty() {
                field.push(bytes);
            }
        } else {
            Err(ErrorKind::UnexpectedWireType)?;
        }
        Ok(())
    }
}

impl<M, _1, _2> DeserFieldFromBytesIter<tags::LabelOptReqUnl<_1, _2>, tags::Message<M>>
where
    M: DeserFieldsFromBytesIter + Default,
{
    pub fn deser_field<I>(
        field: &mut Option<Box<M>>,
        input: FieldData<&mut ScopedIter<I>>,
    ) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        if let FieldData::LengthDelimited(mut iter) = input {
            let msg = field.get_or_insert_with(Default::default);
            deser_from_scoped_iter(msg.deref_mut(), &mut iter)?;
        } else {
            Err(ErrorKind::UnexpectedWireType)?;
        }
        Ok(())
    }
}

impl<M> DeserFieldFromBytesIter<tags::Repeated, tags::Message<M>>
where
    M: DeserFieldsFromBytesIter + Default,
{
    pub fn deser_field<I>(field: &mut Vec<M>, input: FieldData<&mut ScopedIter<I>>) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        if let FieldData::LengthDelimited(mut iter) = input {
            field.push(Default::default());
            let msg = field.last_mut().unwrap();
            deser_from_scoped_iter(msg, &mut iter)?;
        } else {
            Err(ErrorKind::UnexpectedWireType)?;
        }
        Ok(())
    }
}

impl<M> DeserFieldFromBytesIter<tags::OneofItem, tags::Message<M>>
where
    M: DeserFieldsFromBytesIter + Default,
{
    pub fn deser_field<I>(field: &mut Box<M>, input: FieldData<&mut ScopedIter<I>>) -> Result<()>
    where
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        if let FieldData::LengthDelimited(mut iter) = input {
            deser_from_scoped_iter(field.deref_mut(), &mut iter)?;
        } else {
            Err(ErrorKind::UnexpectedWireType)?;
        }
        Ok(())
    }
}
