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
use crate::internal::types::WireType;
use crate::internal::variant::{Variant, VariantTypeTag};
use crate::internal::SerializableMessageToIoWrite;
use crate::ErrorKind;
use crate::{tags, Result};
use ::std::convert::TryInto;
use ::std::io::Write;
use ::std::marker::PhantomData;

pub fn write_field_number_and_wire_type<W: Write>(
    out: &mut W,
    field_number: i32,
    wire_type: WireType,
) -> Result<()> {
    let encoding_val = <i32 as TryInto<u32>>::try_into(field_number)
        .map_err(|_| ErrorKind::InvalidFieldNumber)?
        .checked_shl(3)
        .ok_or(ErrorKind::InvalidFieldNumber)?
        | (wire_type as u32);
    let variant = Variant::from_u32(encoding_val)?;
    variant.encode_bytes(out)?;
    Ok(())
}

// ser to Write methods

pub struct SerFieldToIoWrite<L, V>(PhantomData<(L, V)>);

impl<V, _1, _2, _3> SerFieldToIoWrite<tags::LabelNonRepeated<_1, _2, _3>, tags::Variant<V>>
where
    tags::LabelNonRepeated<_1, _2, _3>: tags::FieldLabelTag,
    tags::Variant<V>: VariantTypeTag,
{
    pub fn ser_field<W>(
        field: Option<<tags::Variant<V> as tags::NumericalTypeTag>::NativeType>,
        number: i32,
        out: &mut W,
    ) -> Result<()>
    where
        W: Write,
    {
        if let Some(item) = field {
            write_field_number_and_wire_type(out, number, WireType::Variant)?;
            let variant = Variant::from_native::<tags::Variant<V>>(item.clone())?;
            variant.encode_bytes(out)?;
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
        FieldType: IntoIterator<Item = <tags::Variant<V> as tags::NumericalTypeTag>::NativeType>,
        W: Write,
    {
        let mut encoded = Vec::new();
        for item in field.into_iter() {
            Variant::from_native::<tags::Variant<V>>(item.clone())?.encode_bytes(&mut encoded)?;
        }
        let len = encoded.len();
        if len == 0 {
            return Ok(());
        }
        let len_i32 = len
            .try_into()
            .map_err(|_| crate::ErrorKind::TooLongToSerialize)?;
        write_field_number_and_wire_type(out, number, WireType::LengthDelimited)?;
        Variant::from_i32(len_i32)?.encode_bytes(out)?;
        out.write_all(&encoded)?;
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
            write_field_number_and_wire_type(out, number, WireType::Bits32)?;
            let bytes = <tags::Bits32<V> as Bits32TypeTag>::into_array(item.clone());
            out.write(&bytes)?;
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
            write_field_number_and_wire_type(out, number, WireType::Bits64)?;
            let bytes = <tags::Bits64<V> as Bits64TypeTag>::into_array(item.clone());
            out.write(&bytes)?;
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
            write_field_number_and_wire_type(out, number, WireType::LengthDelimited)?;
            let len_i32: i32 = item
                .len()
                .try_into()
                .map_err(|_| crate::ErrorKind::TooLongToSerialize)?;
            Variant::from_i32(len_i32)?.encode_bytes(out)?;
            out.write(item)?;
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
            write_field_number_and_wire_type(out, number, WireType::LengthDelimited)?;
            let len_i32: i32 = item
                .len()
                .try_into()
                .map_err(|_| crate::ErrorKind::TooLongToSerialize)?;
            Variant::from_i32(len_i32)?.encode_bytes(out)?;
            out.write(item.as_bytes())?;
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
        FieldType: IntoIterator<Item = M>,
        W: Write,
    {
        for item in field.into_iter() {
            let mut encoded = Vec::new();
            <M as SerializableMessageToIoWrite>::ser(&item, &mut encoded)?;
            let len = encoded.len();
            let len_i32: i32 = len
                .try_into()
                .map_err(|_| crate::ErrorKind::TooLongToSerialize)?;
            write_field_number_and_wire_type(out, number, WireType::LengthDelimited)?;
            Variant::from_i32(len_i32)?.encode_bytes(out)?;
            out.write_all(&encoded)?;
        }
        Ok(())
    }
}
