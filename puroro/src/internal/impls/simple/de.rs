// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::internal::de::from_iter::{deser_from_scoped_iter, ScopedIter, Variants};
use crate::internal::de::DeserMessageFromBytesIter;
use crate::internal::fixed_bits::{Bits32TypeTag, Bits64TypeTag};
use crate::internal::types::FieldData;
use crate::internal::utils::VecOrOptionOrBare;
use crate::internal::variant::VariantTypeTag;
use crate::ErrorKind;
use crate::{tags, Result};
use ::std::marker::PhantomData;

// deser from iter methods

pub struct DeserFieldFromBytesIter<L, V>(PhantomData<(L, V)>);

impl<L, V> DeserFieldFromBytesIter<L, tags::Variant<V>>
where
    L: tags::FieldLabelTag,
    tags::Variant<V>: VariantTypeTag,
{
    pub fn deser_field<FieldType, I>(
        field: &mut FieldType,
        input: FieldData<&mut ScopedIter<I>>,
    ) -> Result<()>
    where
        FieldType: VecOrOptionOrBare<<tags::Variant<V> as tags::NumericalTypeTag>::NativeType>,
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        match input {
            FieldData::Variant(v) => {
                let native_value = v.to_native::<tags::Variant<V>>()?;
                if !L::DO_DEFAULT_CHECK || native_value != Default::default() {
                    field.push(native_value);
                }
            }
            FieldData::LengthDelimited(iter) => {
                let variants = Variants::new(iter);
                let values = variants.map(|rv| rv.and_then(|v| v.to_native::<tags::Variant<V>>()));
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

impl<L, V> DeserFieldFromBytesIter<L, tags::Bits32<V>>
where
    L: tags::FieldLabelTag,
    tags::Bits32<V>: Bits32TypeTag,
{
    pub fn deser_field<FieldType, I>(
        field: &mut FieldType,
        input: FieldData<&mut ScopedIter<I>>,
    ) -> Result<()>
    where
        FieldType: VecOrOptionOrBare<<tags::Bits32<V> as tags::NumericalTypeTag>::NativeType>,
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        if let FieldData::Bits32(bytes) = input {
            let native_value = tags::Bits32::<V>::from_array(bytes);
            if !L::DO_DEFAULT_CHECK || native_value != Default::default() {
                field.push(native_value);
            }
        } else {
            Err(ErrorKind::UnexpectedWireType)?;
        }
        Ok(())
    }
}

impl<L, V> DeserFieldFromBytesIter<L, tags::Bits64<V>>
where
    L: tags::FieldLabelTag,
    tags::Bits64<V>: Bits64TypeTag,
{
    pub fn deser_field<FieldType, I>(
        field: &mut FieldType,
        input: FieldData<&mut ScopedIter<I>>,
    ) -> Result<()>
    where
        FieldType: VecOrOptionOrBare<<tags::Bits64<V> as tags::NumericalTypeTag>::NativeType>,
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        if let FieldData::Bits64(bytes) = input {
            let native_value = tags::Bits64::<V>::from_array(bytes);
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

impl<L, M> DeserFieldFromBytesIter<L, tags::Message<M>>
where
    L: tags::FieldLabelTag,
    M: DeserMessageFromBytesIter + Default,
{
    pub fn deser_field<FieldType, I>(
        field: &mut FieldType,
        input: FieldData<&mut ScopedIter<I>>,
    ) -> Result<()>
    where
        FieldType: VecOrOptionOrBare<M>,
        I: Iterator<Item = ::std::io::Result<u8>>,
    {
        if let FieldData::LengthDelimited(mut iter) = input {
            let msg = field.get_or_insert_with(Default::default);
            deser_from_scoped_iter(msg, &mut iter)?;
        } else {
            Err(ErrorKind::UnexpectedWireType)?;
        }
        Ok(())
    }
}
