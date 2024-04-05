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

use crate::internal::deser::record::{Payload, Record, SliceExtReadRecord};
use crate::variant::{ReadExtVariant, Variant};
use crate::{ErrorKind, Result};
use ::itertools::Either;
use ::std::borrow::Cow;
use ::std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct UntypedMessage<'a> {
    fields: HashMap<i32, Vec<WireTypeAndPayload<'a>>>,
}

impl<'a> UntypedMessage<'a> {
    pub fn fields(&self) -> impl Iterator<Item = Field> {
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

#[derive(Debug, Clone)]
pub struct Field<'a> {
    number: i32,
    wire_and_payloads: &'a [WireTypeAndPayload<'a>],
}

impl Field<'_> {
    pub fn number(&self) -> i32 {
        self.number
    }

    pub fn as_scalar_variant(&self, allow_packed: bool) -> Result<Option<Variant>> {
        self.as_repeated_variant(allow_packed)
            .into_iter()
            .try_last()
    }

    pub fn as_repeated_variant(
        &self,
        allow_packed: bool,
    ) -> impl '_ + IntoIterator<Item = Result<Variant>> {
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

    pub fn as_scalar_string(&self) -> Result<Option<&str>> {
        self.as_repeated_string().into_iter().try_last()
    }

    pub fn as_repeated_string(&self) -> impl '_ + IntoIterator<Item = Result<&str>> {
        self.wire_and_payloads.iter().map(|record| match record {
            WireTypeAndPayload::Len(ld) => {
                ::std::str::from_utf8(&ld).map_err(|_| ErrorKind::GenericMessageFieldTypeError)
            }
            _ => Err(ErrorKind::GenericMessageFieldTypeError),
        })
    }

    pub fn as_scalar_message(&self) -> Result<Option<UntypedMessage<'_>>> {
        let mut message_opt: Option<UntypedMessage> = None;
        for wire_and_payload in self.wire_and_payloads {
            let WireTypeAndPayload::Len(buf) = wire_and_payload else {
                Err(ErrorKind::GenericMessageFieldTypeError)?
            };
            for try_record in buf.as_ref().into_records() {
                let record = try_record?;
                message_opt
                    .get_or_insert_with(UntypedMessage::default)
                    .payloads_for_field_mut(record.number.clone())
                    .push(record.into());
            }
        }
        Ok(message_opt)
    }

    pub fn as_repeated_message(&self) -> impl '_ + IntoIterator<Item = Result<UntypedMessage<'_>>> {
        self.wire_and_payloads.iter().map(|wire_and_payload| {
            let WireTypeAndPayload::Len(buf) = wire_and_payload else {
                Err(ErrorKind::GenericMessageFieldTypeError)?
            };
            let mut message = UntypedMessage::default();
            for try_record in buf.as_ref().into_records() {
                let record = try_record?;
                message
                    .payloads_for_field_mut(record.number.clone())
                    .push(record.into());
            }
            Ok(message)
        })
    }
}

trait IteratorExt: Iterator {
    /// Returns the last element if the all elements are Ok.
    /// If any of the elements are Err, returns the first Err.
    /// If no element is found, returns None.
    fn try_last<T, E>(mut self) -> ::std::result::Result<Option<T>, E>
    where
        Self: Sized + Iterator<Item = ::std::result::Result<T, E>>,
    {
        self.try_fold(None, |_, x| x.map(Some))
    }
}
impl<T> IteratorExt for T where T: Iterator {}

#[derive(Debug, Clone)]
pub enum WireTypeAndPayload<'a> {
    Variant(Variant),
    Fixed64([u8; 8]),
    Fixed32([u8; 4]),
    Len(Cow<'a, [u8]>),
    // StartGroup,
    // EndGroup,
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
