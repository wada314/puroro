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

use crate::internal::deser::record::{Payload, Record};
use crate::internal::deser::{deser_from_bufread, DeserMessageHandler};
use crate::internal::WireType;
use crate::message::MessageLite;
use crate::variant::{ReadExtVariant, UInt32, Variant, VariantIntegerType, WriteExtVariant};
use crate::{ErrorKind, Result};
use ::itertools::Either;
use ::std::borrow::Cow;
use ::std::collections::HashMap;
use ::std::io::{BufReader, Read, Write};

/// Assuming proto2 syntax.
#[derive(Clone, Debug, Default)]
pub struct UntypedMessage<'a> {
    fields: HashMap<i32, Vec<WireTypeAndPayload<'a>>>,
}

impl MessageLite for UntypedMessage<'_> {
    fn merge_from_read<R: Read>(&mut self, read: R) -> Result<()> {
        deser_from_bufread(BufReader::new(read), self)
    }
    fn write<W: Write>(&self, mut write: W) -> Result<usize> {
        let mut total_bytes = 0;
        for (number, wire_and_payloads) in &self.fields {
            for wire_and_payload in wire_and_payloads {
                let tag = (TryInto::<u32>::try_into(*number)? << 3)
                    | Into::<u32>::into(wire_and_payload.wire_type());
                total_bytes += write.write_variant(UInt32::try_into_variant(tag)?)?;
                total_bytes += match wire_and_payload {
                    WireTypeAndPayload::Variant(variant) => write.write_variant(variant.clone())?,
                    WireTypeAndPayload::Fixed64(buf) => {
                        write.write_all(buf)?;
                        4usize
                    }
                    WireTypeAndPayload::Fixed32(buf) => {
                        write.write_all(buf)?;
                        8usize
                    }
                    WireTypeAndPayload::Len(ld) => {
                        write.write_all(ld)?;
                        ld.len()
                    }
                };
            }
        }
        Ok(total_bytes)
    }
}

#[derive(Debug, Clone)]
pub enum WireTypeAndPayload<'a> {
    Variant(Variant),
    Fixed64([u8; 8]),
    Fixed32([u8; 4]),
    Len(Cow<'a, [u8]>),
    // StartGroup,
    // EndGroup,
}
impl WireTypeAndPayload<'_> {
    pub(crate) fn wire_type(&self) -> WireType {
        match self {
            WireTypeAndPayload::Variant(_) => WireType::Variant,
            WireTypeAndPayload::Fixed64(_) => WireType::I64,
            WireTypeAndPayload::Fixed32(_) => WireType::I32,
            WireTypeAndPayload::Len(_) => WireType::Len,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Field<'a> {
    number: i32,
    wire_and_payloads: &'a [WireTypeAndPayload<'a>],
}

impl<'a> UntypedMessage<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_buffer(buf: &'a [u8]) -> Result<Self> {
        let mut message = UntypedMessage::default();
        message.merge_from_buffer(buf)?;
        Ok(message)
    }

    pub fn merge_from_buffer(&mut self, buf: &'a [u8]) -> Result<()> {
        deser_from_bufread(buf, self)?;
        Ok(())
    }

    pub fn field(&self, number: i32) -> Field {
        self.fields.get(&number).map_or_else(
            || Field {
                number,
                wire_and_payloads: &[],
            },
            |wire_and_payloads| Field {
                number,
                wire_and_payloads,
            },
        )
    }

    pub fn fields(&self) -> impl '_ + Iterator<Item = Field> {
        self.fields.iter().map(|(number, wire_and_payloads)| Field {
            number: *number,
            wire_and_payloads,
        })
    }

    pub fn payloads_for_field(&self, number: i32) -> Option<&[WireTypeAndPayload]> {
        self.fields.get(&number).map(|v| v.as_slice())
    }
    pub fn payloads_for_field_mut<'this>(
        &'this mut self,
        number: i32,
    ) -> &'this mut Vec<WireTypeAndPayload<'a>> {
        self.fields.entry(number).or_default()
    }
}

impl<'a> Field<'a> {
    pub fn number(&self) -> i32 {
        self.number
    }

    pub fn as_scalar_variant(&self, allow_packed: bool) -> Result<Option<Variant>> {
        self.as_repeated_variant(allow_packed)
            .into_iter()
            .last()
            .transpose()
    }

    pub fn as_repeated_variant(
        &self,
        allow_packed: bool,
    ) -> impl 'a + IntoIterator<Item = Result<Variant>> {
        self.wire_and_payloads
            .iter()
            .flat_map(move |record| match (allow_packed, record) {
                (_, WireTypeAndPayload::Variant(variant)) => {
                    Either::Left(Some(Ok(variant.clone())).into_iter())
                }
                (true, WireTypeAndPayload::Len(ld)) => Either::Right(ld.into_variant_iter()),
                _ => Either::Left(Some(Err(ErrorKind::GenericMessageFieldTypeError)).into_iter()),
            })
    }

    pub fn as_scalar_string(&self) -> Result<Option<&'a str>> {
        self.as_repeated_string().into_iter().last().transpose()
    }

    pub fn as_repeated_string(&self) -> impl 'a + IntoIterator<Item = Result<&'a str>> {
        self.wire_and_payloads.iter().map(|record| match record {
            WireTypeAndPayload::Len(ld) => {
                ::std::str::from_utf8(&ld).map_err(|_| ErrorKind::GenericMessageFieldTypeError)
            }
            _ => Err(ErrorKind::GenericMessageFieldTypeError),
        })
    }

    pub fn as_scalar_message(&self) -> Result<Option<UntypedMessage<'a>>> {
        let mut message_opt: Option<UntypedMessage> = None;
        for wire_and_payload in self.wire_and_payloads {
            let WireTypeAndPayload::Len(buf) = wire_and_payload else {
                Err(ErrorKind::GenericMessageFieldTypeError)?
            };
            message_opt
                .get_or_insert_with(UntypedMessage::default)
                .merge_from_buffer(buf)?;
        }
        Ok(message_opt)
    }

    pub fn as_repeated_message(&self) -> impl 'a + IntoIterator<Item = Result<UntypedMessage<'a>>> {
        self.wire_and_payloads.iter().map(|wire_and_payload| {
            let WireTypeAndPayload::Len(buf) = wire_and_payload else {
                Err(ErrorKind::GenericMessageFieldTypeError)?
            };
            UntypedMessage::from_buffer(buf)
        })
    }
}

impl<'a, T> From<Record<T>> for WireTypeAndPayload<'a>
where
    T: 'a,
    Cow<'a, [u8]>: From<T>,
{
    fn from(record: Record<T>) -> Self {
        match record.payload {
            Payload::Variant(variant) => WireTypeAndPayload::Variant(variant),
            Payload::I64(buf) => WireTypeAndPayload::Fixed64(buf),
            Payload::I32(buf) => WireTypeAndPayload::Fixed32(buf),
            Payload::Len(buf) => WireTypeAndPayload::Len(buf.into()),
        }
    }
}

impl<'a, R: Read> DeserMessageHandler<R> for UntypedMessage<'a> {
    fn start_message(&mut self, #[allow(unused)] num: i32) -> Result<()> {
        unimplemented!()
    }
    fn end_message(&mut self) -> Result<()> {
        unimplemented!()
    }
    fn parse_variant(&mut self, num: i32, var: Variant) -> Result<()> {
        self.payloads_for_field_mut(num)
            .push(WireTypeAndPayload::Variant(var));
        Ok(())
    }
    fn parse_i32(&mut self, num: i32, val: [u8; 4]) -> Result<()> {
        self.payloads_for_field_mut(num)
            .push(WireTypeAndPayload::Fixed32(val));
        Ok(())
    }
    fn parse_i64(&mut self, num: i32, val: [u8; 8]) -> Result<()> {
        self.payloads_for_field_mut(num)
            .push(WireTypeAndPayload::Fixed64(val));
        Ok(())
    }
    fn parse_len(&mut self, num: i32, val: &mut R) -> Result<()> {
        let mut buf = Vec::new();
        val.read_to_end(&mut buf)?;
        self.payloads_for_field_mut(num)
            .push(WireTypeAndPayload::Len(buf.into()));
        Ok(())
    }
    fn is_message_field(&self, #[allow(unused)] num: i32) -> bool {
        false
    }
}
