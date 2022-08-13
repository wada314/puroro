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
use crate::{ErrorKind, PuroroError, Result};
use ::std::convert::TryFrom;
use ::std::io::Result as IoResult;
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
}

impl<'a, I: Iterator<Item = IoResult<u8>>> FieldData<iter::Take<&'a mut I>> {
    pub fn from_bytes_iter<'b: 'a>(bytes: &'b mut I) -> Result<(i32, Self)> {
        let var_i32 = Variant::decode_bytes(bytes.by_ref())?.get_i32()?;
        let wire_type: WireType = (var_i32 & 0x7).try_into()?;
        let number = var_i32 >> 3;

        let field_data = match wire_type {
            WireType::Variant => FieldData::Variant(Variant::decode_bytes(bytes)?),
            WireType::LengthDelimited => {
                let length: usize = Variant::decode_bytes(bytes.by_ref())?
                    .get_i32()?
                    .try_into()?;
                FieldData::LengthDelimited(bytes.take(length))
            }
            WireType::StartGroup => todo!(),
            WireType::EndGroup => todo!(),
            WireType::Bits32 => todo!(),
            WireType::Bits64 => todo!(),
        };

        Ok((number, field_data))
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
