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

use crate::internal::variant::{ReadExtVariant, Variant};
use crate::{ErrorKind, Result};
use ::itertools::Either;

pub struct UntypedMessage {
    fields: Vec<Field>,
}

impl UntypedMessage {
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

pub struct Field {
    number: i32,
    name: String,
    records: Vec<WireTypeAndPayload>,
}

impl Field {
    pub fn number(&self) -> i32 {
        self.number
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn as_scalar_variant(&self, allow_packed: bool) -> Result<Option<Variant>> {
        Ok(self
            .records
            .iter()
            .try_fold(None, |last_var_opt, record| match record {
                WireTypeAndPayload::Variant(variant) => Ok(Some(variant)),
                WireTypeAndPayload::LengthDelimited(ld) => {
                    if allow_packed {
                        let last_value_opt = ld
                            .into_variant_iter()
                            .try_fold(last_var_opt, |_, variant| Result::Ok(Some(variant?)))?;
                        Ok(last_value_opt)
                    } else {
                        Err(ErrorKind::GenericMessageFieldTypeError)
                    }
                }
                _ => Err(ErrorKind::GenericMessageFieldTypeError),
            })?)
    }
}

enum WireTypeAndPayload {
    Variant(Variant),
    Fixed64([u8; 8]),
    Fixed32([u8; 4]),
    LengthDelimited(Vec<u8>),
    // StartGroup,
    // EndGroup,
}
