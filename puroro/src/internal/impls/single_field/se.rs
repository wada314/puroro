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

use crate::internal::fixed_bits::{Bits32TypeTag, Bits64TypeTag};
use crate::internal::se::to_io_write::write_field_number_and_wire_type;
use crate::internal::types::WireType;
use crate::internal::variant::{Variant, VariantTypeTag};
use crate::internal::SerializableMessageToIoWrite;
use crate::{tags, Result};
use ::std::convert::TryInto;
use ::std::io::Write;
use ::std::marker::PhantomData;

struct NullWrite(usize);
impl Write for NullWrite {
    fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize> {
        if let Some(new_size) = usize::checked_add(self.0, buf.len()) {
            self.0 = new_size;
            Ok(buf.len())
        } else {
            Err(::std::io::Error::new(
                ::std::io::ErrorKind::Unsupported,
                "Too long to serialize",
            ))
        }
    }
    fn flush(&mut self) -> ::std::io::Result<()> {
        Ok(())
    }
}
impl NullWrite {
    fn new() -> Self {
        Self(0usize)
    }
    fn len(&self) -> usize {
        self.0
    }
}

// ser to Write methods

pub struct SerFieldToIoWrite<L, V>(PhantomData<(L, V)>);

impl<V, _1, _2, _3> SerFieldToIoWrite<tags::LabelNonRepeated<_1, _2, _3>, tags::Variant<V>>
where
    tags::LabelNonRepeated<_1, _2, _3>: tags::FieldLabelTag,
    tags::Variant<V>: VariantTypeTag,
{
    pub fn ser_field<FieldType, W>(field: FieldType, number: i32, out: &mut W) -> Result<()>
    where
        FieldType: IntoIterator<Item = <tags::Variant<V> as tags::NumericalTypeTag>::NativeType>,
        W: Write,
    {
        if let Some(item) = field.into_iter().next() {
            if !<tags::LabelNonRepeated<_1, _2, _3> as tags::FieldLabelTag>::DO_DEFAULT_CHECK
                || item.clone() != Default::default()
            {
                write_field_number_and_wire_type(out, number, WireType::Variant)?;
                let variant = Variant::from_native::<tags::Variant<V>>(item.clone())?;
                variant.encode_bytes(out)?;
            }
        }
        Ok(())
    }
}

impl<V> SerFieldToIoWrite<tags::Repeated, tags::Variant<V>>
where
    tags::Variant<V>: VariantTypeTag,
{
    pub fn ser_field<FieldType, W>(field: FieldType, number: i32, out: &mut W) -> Result<()>
    where
        FieldType:
            IntoIterator<Item = <tags::Variant<V> as tags::NumericalTypeTag>::NativeType> + Clone,
        W: Write,
    {
        let len = {
            let mut null_out = NullWrite::new();
            for item in field.clone().into_iter() {
                Variant::from_native::<tags::Variant<V>>(item.clone())?
                    .encode_bytes(&mut null_out)?;
            }
            null_out.len()
        };
        if len == 0 {
            return Ok(());
        }
        let len_i32 = len
            .try_into()
            .map_err(|_| crate::ErrorKind::TooLongToSerialize)?;
        write_field_number_and_wire_type(out, number, WireType::LengthDelimited)?;
        Variant::from_i32(len_i32)?.encode_bytes(out)?;
        for item in field.clone().into_iter() {
            Variant::from_native::<tags::Variant<V>>(item.clone())?.encode_bytes(out)?;
        }
        Ok(())
    }
}

impl<L, V> SerFieldToIoWrite<L, tags::Bits32<V>>
where
    L: tags::FieldLabelTag,
    tags::Bits32<V>: Bits32TypeTag,
{
    pub fn ser_field<FieldType, W>(field: FieldType, number: i32, out: &mut W) -> Result<()>
    where
        FieldType: IntoIterator<Item = <tags::Bits32<V> as tags::NumericalTypeTag>::NativeType>,
        W: Write,
    {
        for item in field.into_iter() {
            if !L::DO_DEFAULT_CHECK || item.clone() != Default::default() {
                write_field_number_and_wire_type(out, number, WireType::Bits32)?;
                let bytes = <tags::Bits32<V> as Bits32TypeTag>::into_array(item.clone());
                out.write(&bytes)?;
            }
        }
        Ok(())
    }
}

impl<L, V> SerFieldToIoWrite<L, tags::Bits64<V>>
where
    L: tags::FieldLabelTag,
    tags::Bits64<V>: Bits64TypeTag,
{
    pub fn ser_field<FieldType, W>(field: FieldType, number: i32, out: &mut W) -> Result<()>
    where
        FieldType: IntoIterator<Item = <tags::Bits64<V> as tags::NumericalTypeTag>::NativeType>,
        W: Write,
    {
        for item in field.into_iter() {
            if !L::DO_DEFAULT_CHECK || item.clone() != Default::default() {
                write_field_number_and_wire_type(out, number, WireType::Bits64)?;
                let bytes = <tags::Bits64<V> as Bits64TypeTag>::into_array(item.clone());
                out.write(&bytes)?;
            }
        }
        Ok(())
    }
}

impl<L> SerFieldToIoWrite<L, tags::Bytes>
where
    L: tags::FieldLabelTag,
{
    pub fn ser_field<'a, FieldType, W>(field: FieldType, number: i32, out: &mut W) -> Result<()>
    where
        FieldType: 'a + IntoIterator<Item = &'a [u8]>,
        W: Write,
    {
        for item in field.into_iter() {
            if !L::DO_DEFAULT_CHECK || !item.is_empty() {
                write_field_number_and_wire_type(out, number, WireType::LengthDelimited)?;
                let len_i32: i32 = item
                    .len()
                    .try_into()
                    .map_err(|_| crate::ErrorKind::TooLongToSerialize)?;
                Variant::from_i32(len_i32)?.encode_bytes(out)?;
                out.write(item)?;
            }
        }
        Ok(())
    }
}

impl<L> SerFieldToIoWrite<L, tags::String>
where
    L: tags::FieldLabelTag,
{
    pub fn ser_field<'a, FieldType, W>(field: FieldType, number: i32, out: &mut W) -> Result<()>
    where
        FieldType: 'a + IntoIterator<Item = &'a str>,
        W: Write,
    {
        for item in field.into_iter() {
            if !L::DO_DEFAULT_CHECK || !item.is_empty() {
                write_field_number_and_wire_type(out, number, WireType::LengthDelimited)?;
                let len_i32: i32 = item
                    .len()
                    .try_into()
                    .map_err(|_| crate::ErrorKind::TooLongToSerialize)?;
                Variant::from_i32(len_i32)?.encode_bytes(out)?;
                out.write(item.as_bytes())?;
            }
        }
        Ok(())
    }
}

impl<L, M> SerFieldToIoWrite<L, tags::Message<M>>
where
    L: tags::FieldLabelTag,
    M: SerializableMessageToIoWrite,
{
    pub fn ser_field<'a, FieldType, W>(field: FieldType, number: i32, out: &mut W) -> Result<()>
    where
        M: 'a,
        FieldType: 'a + IntoIterator<Item = &'a M>,
        W: Write,
    {
        for item in field.into_iter() {
            let len = {
                let mut null_out = NullWrite::new();
                <M as SerializableMessageToIoWrite>::ser(item, &mut null_out)?;
                null_out.len()
            };
            let len_i32: i32 = len
                .try_into()
                .map_err(|_| crate::ErrorKind::TooLongToSerialize)?;
            write_field_number_and_wire_type(out, number, WireType::LengthDelimited)?;
            Variant::from_i32(len_i32)?.encode_bytes(out)?;
            <M as SerializableMessageToIoWrite>::ser(item, out)?;
        }
        Ok(())
    }
}
