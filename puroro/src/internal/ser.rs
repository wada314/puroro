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

use crate::internal::tags;
use crate::internal::variant::Variant;
use crate::{PuroroError, Result};
use ::std::convert::TryFrom;
use ::std::io::{Result as IoResult, Write};

#[derive(Debug, Clone, PartialEq)]
pub enum FieldData<T> {
    Variant(Variant),
    LengthDelimited(T),
    Bits32([u8; 4]),
    Bits64([u8; 8]),
}

#[derive(Debug, Clone, PartialEq)]
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

impl<T, E> FieldData<::std::result::Result<T, E>> {
    pub fn transpose(self) -> ::std::result::Result<FieldData<T>, E> {
        Ok(match self {
            FieldData::Variant(x) => FieldData::Variant(x),
            FieldData::LengthDelimited(x) => FieldData::LengthDelimited(x?),
            FieldData::Bits32(x) => FieldData::Bits32(x),
            FieldData::Bits64(x) => FieldData::Bits64(x),
        })
    }
}

impl<'a, I: Iterator<Item = IoResult<u8>>> FieldData<ScopedIter<'a, I>> {
    pub fn from_bytes_scoped_iter<'b: 'a>(
        bytes: &'a mut ScopedIter<'b, I>,
    ) -> Result<Option<(i32, Self)>> {
        if let Some(var) = Variant::decode_bytes(bytes.by_ref())? {
            let var_u32 = var.get_u32()?;
            let wire_type: WireType = (var_u32 & 0x7).try_into()?;
            let number = (var_u32 >> 3) as i32;

            let field_data = match wire_type {
                WireType::Variant => FieldData::Variant(
                    Variant::decode_bytes(bytes)?.ok_or(PuroroError::UnexpectedInputTermination)?,
                ),
                WireType::LengthDelimited => {
                    let length: usize = Variant::decode_bytes(bytes.by_ref())?
                        .ok_or(PuroroError::UnexpectedInputTermination)?
                        .get_i32()?
                        .try_into()?;
                    FieldData::LengthDelimited(bytes.scope(length))
                }
                WireType::StartGroup => todo!(),
                WireType::EndGroup => todo!(),
                WireType::Bits32 => {
                    let bits32 = [
                        read_byte(bytes.by_ref())?,
                        read_byte(bytes.by_ref())?,
                        read_byte(bytes.by_ref())?,
                        read_byte(bytes.by_ref())?,
                    ];
                    FieldData::Bits32(bits32)
                }
                WireType::Bits64 => {
                    let bits64 = [
                        read_byte(bytes.by_ref())?,
                        read_byte(bytes.by_ref())?,
                        read_byte(bytes.by_ref())?,
                        read_byte(bytes.by_ref())?,
                        read_byte(bytes.by_ref())?,
                        read_byte(bytes.by_ref())?,
                        read_byte(bytes.by_ref())?,
                        read_byte(bytes.by_ref())?,
                    ];
                    FieldData::Bits64(bits64)
                }
            };

            Ok(Some((number, field_data)))
        } else {
            Ok(None)
        }
    }
}

impl TryFrom<u32> for WireType {
    type Error = PuroroError;

    fn try_from(value: u32) -> ::std::result::Result<Self, Self::Error> {
        Ok(match value {
            0 => WireType::Variant,
            1 => WireType::Bits64,
            2 => WireType::LengthDelimited,
            3 => WireType::StartGroup,
            4 => WireType::EndGroup,
            5 => WireType::Bits32,
            _ => Err(PuroroError::InvalidWireType(value))?,
        })
    }
}

pub(crate) fn ser_wire_and_number<W: Write>(
    wire: WireType,
    number: i32,
    out: &mut W,
) -> Result<()> {
    let var = Variant::from_u32(((number << 3) | (wire as i32)) as u32);
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

fn read_byte<I: Iterator<Item = IoResult<u8>>>(bytes: &mut I) -> Result<u8> {
    Ok(bytes
        .next()
        .ok_or(PuroroError::UnexpectedInputTermination)??)
}

pub struct PosIter<I> {
    iter: I,
    pos: usize,
}
impl<I> PosIter<I> {
    pub fn new(iter: I) -> Self {
        Self { iter, pos: 0 }
    }
    pub fn pos(&self) -> usize {
        self.pos
    }
}
impl<I: Iterator> Iterator for PosIter<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let next_val = self.iter.next();
        if next_val.is_some() {
            self.pos += 1;
        }
        next_val
    }
}

pub struct ScopedIter<'a, I> {
    iter: &'a mut PosIter<I>,
    end: Option<usize>,
}
impl<'a, I> ScopedIter<'a, I> {
    pub fn from_mut_pos_iter(iter: &'a mut PosIter<I>) -> Self {
        Self { iter, end: None }
    }
    pub fn drop_and_check_scope_completed(self) -> Result<()> {
        if let Some(end) = self.end {
            if end != self.iter.pos() {
                Err(PuroroError::UnexpectedInputTermination)?;
            }
        }
        ::std::mem::forget(self);
        Ok(())
    }
}
impl<'a, I: Iterator> ScopedIter<'a, I> {
    pub fn scope<'b>(&'b mut self, len: usize) -> ScopedIter<'b, I>
    where
        'a: 'b,
    {
        let end = Some(self.iter.pos() + len);
        ScopedIter {
            iter: self.iter,
            end,
        }
    }
}
impl<'a, I: Iterator> Iterator for ScopedIter<'a, I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(end) = self.end {
            if self.iter.pos() < end {
                self.iter.next()
            } else {
                None
            }
        } else {
            self.iter.next()
        }
    }
}
