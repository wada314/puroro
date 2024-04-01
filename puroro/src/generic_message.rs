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

use crate::variant::{ReadExtVariant, Variant};
use crate::{ErrorKind, Result};
use ::itertools::Either;
use ::std::borrow::Cow;

#[derive(Clone, Debug, Default)]
pub struct UntypedMessage<'a> {
    fields: Vec<Field<'a>>,
}

impl UntypedMessage<'_> {
    pub fn fields(&self) -> impl Iterator<Item = &Field> {
        self.fields.iter()
    }
    pub fn field_with_name(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|f| f.name == name)
    }
    pub fn field_with_number(&self, number: i32) -> Option<&Field> {
        self.fields.iter().find(|f| f.number == number)
    }
}

#[derive(Debug, Clone)]
pub struct Field<'a> {
    number: i32,
    name: String,
    records: Vec<WireTypeAndPayload<'a>>,
}

impl Field<'_> {
    pub fn number(&self) -> i32 {
        self.number
    }
    pub fn name(&self) -> &str {
        &self.name
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
        self.records
            .iter()
            .flat_map(move |record| match (allow_packed, record) {
                (_, WireTypeAndPayload::Variant(variant)) => {
                    Either::Left(Some(Ok(variant.clone())).into_iter())
                }
                (true, WireTypeAndPayload::LengthDelimited(ld)) => {
                    Either::Right(ld.into_variant_iter())
                }
                _ => Either::Left(Some(Err(ErrorKind::GenericMessageFieldTypeError)).into_iter()),
            })
    }

    pub fn as_scalar_string(&self) -> Result<Option<&str>> {
        self.as_repeated_string().into_iter().try_last()
    }

    pub fn as_repeated_string(&self) -> impl '_ + IntoIterator<Item = Result<&str>> {
        self.records.iter().map(|record| match record {
            WireTypeAndPayload::LengthDelimited(ld) => {
                ::std::str::from_utf8(&ld).map_err(|_| ErrorKind::GenericMessageFieldTypeError)
            }
            _ => Err(ErrorKind::GenericMessageFieldTypeError),
        })
    }

    pub fn as_scalar_message(&self) -> Result<Option<UntypedMessage<'_>>> {
        todo!()
    }
}

trait IteratorExt: Iterator {
    fn try_last<T, E>(mut self) -> ::std::result::Result<Option<T>, E>
    where
        Self: Sized + Iterator<Item = ::std::result::Result<T, E>>,
    {
        self.try_fold(None, |_, x| x.map(Some))
    }
}
impl<T> IteratorExt for T where T: Iterator {}

#[derive(Debug, Clone)]
enum WireTypeAndPayload<'a> {
    Variant(Variant),
    Fixed64([u8; 8]),
    Fixed32([u8; 4]),
    LengthDelimited(Cow<'a, [u8]>),
    // StartGroup,
    // EndGroup,
}
