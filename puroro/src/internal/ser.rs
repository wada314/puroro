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

use crate::internal::variant::Variant;
use crate::tags;
use crate::{ErrorKind, PuroroError, Result};
use ::std::convert::TryFrom;
use ::std::io::{Result as IoResult, Write};
use ::std::iter;

#[derive(Debug, Clone)]
pub enum FieldData<T> {
    Variant(Variant),
    LengthDelimited(T),
    Bits32([u8; 4]),
    Bits64([u8; 8]),
}

#[derive(Debug, Clone)]
pub enum WireType {
    Variant = 0,
    Bits64 = 1,
    LengthDelimited = 2,
    StartGroup = 3,
    EndGroup = 4,
    Bits32 = 5,
}

impl<T> FieldData<T> {
    pub fn as_mut(&mut self) -> FieldData<&mut T> {
        match self {
            FieldData::Variant(x) => FieldData::Variant(x.clone()),
            FieldData::LengthDelimited(x) => FieldData::LengthDelimited(x),
            FieldData::Bits32(x) => FieldData::Bits32(x.clone()),
            FieldData::Bits64(x) => FieldData::Bits64(x.clone()),
        }
    }

    pub fn map<U, F>(self, f: F) -> FieldData<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            FieldData::Variant(x) => FieldData::Variant(x),
            FieldData::LengthDelimited(x) => FieldData::LengthDelimited((f)(x)),
            FieldData::Bits32(x) => FieldData::Bits32(x),
            FieldData::Bits64(x) => FieldData::Bits64(x),
        }
    }

    pub fn wire_type(&self) -> WireType {
        match self {
            FieldData::Variant(_) => WireType::Variant,
            FieldData::LengthDelimited(_) => WireType::LengthDelimited,
            FieldData::Bits32(_) => WireType::Bits32,
            FieldData::Bits64(_) => WireType::Bits64,
        }
    }
}

impl<'a, I: Iterator<Item = IoResult<u8>>> FieldData<iter::Take<&'a mut I>> {
    pub fn from_bytes_iter<'b: 'a>(bytes: &'b mut I) -> Result<Option<(i32, Self)>> {
        if let Some(var) = Variant::decode_bytes(bytes.by_ref())? {
            let var_i32 = var.get_i32()?;
            let wire_type: WireType = (var_i32 & 0x7).try_into()?;
            let number = var_i32 >> 3;

            let field_data = match wire_type {
                WireType::Variant => FieldData::Variant(
                    Variant::decode_bytes(bytes)?.ok_or(ErrorKind::UnexpectedInputTermination)?,
                ),
                WireType::LengthDelimited => {
                    let length: usize = Variant::decode_bytes(bytes.by_ref())?
                        .ok_or(ErrorKind::UnexpectedInputTermination)?
                        .get_i32()?
                        .try_into()?;
                    FieldData::LengthDelimited(bytes.take(length))
                }
                WireType::StartGroup => todo!(),
                WireType::EndGroup => todo!(),
                WireType::Bits32 => todo!(),
                WireType::Bits64 => todo!(),
            };

            Ok(Some((number, field_data)))
        } else {
            Ok(None)
        }
    }
}

impl TryFrom<i32> for WireType {
    type Error = PuroroError;

    fn try_from(value: i32) -> ::std::result::Result<Self, Self::Error> {
        Ok(match value {
            0 => WireType::Variant,
            1 => WireType::Bits64,
            2 => WireType::LengthDelimited,
            3 => WireType::StartGroup,
            4 => WireType::EndGroup,
            5 => WireType::Bits32,
            _ => Err(ErrorKind::InvalidWireType(value))?,
        })
    }
}

pub(crate) fn ser_wire_and_number<W: Write>(
    wire: WireType,
    number: i32,
    out: &mut W,
) -> Result<()> {
    let var = Variant::from_i32((number << 3) | (wire as i32));
    var.encode_bytes(out)?;
    Ok(())
}

pub(crate) fn ser_numerical_shared<RustType, ProtoType, W>(
    val: RustType,
    number: i32,
    out: &mut W,
) -> Result<()>
where
    ProtoType: tags::NumericalType<RustType = RustType>,
    W: Write,
{
    let wire = <ProtoType as tags::NumericalType>::to_wire_type(val)?;
    ser_wire_and_number(
        match wire {
            tags::NumericalWireType::Variant(_) => WireType::Variant,
            tags::NumericalWireType::Bits32(_) => WireType::Bits32,
            tags::NumericalWireType::Bits64(_) => WireType::Bits64,
        },
        number,
        out,
    )?;

    match wire {
        tags::NumericalWireType::Variant(bits) => {
            let var = Variant::new(bits);
            var.encode_bytes(out)?;
        }
        tags::NumericalWireType::Bits32(bits) => {
            out.write_all(&bits)?;
        }
        tags::NumericalWireType::Bits64(bits) => {
            out.write_all(&bits)?;
        }
    }
    Ok(())
}

pub(crate) fn ser_bytes_shared<W: Write>(bytes: &[u8], number: i32, out: &mut W) -> Result<()> {
    ser_wire_and_number(WireType::LengthDelimited, number, out)?;
    Variant::from_i32(bytes.len().try_into()?).encode_bytes(out)?;
    out.write_all(bytes)?;
    Ok(())
}
