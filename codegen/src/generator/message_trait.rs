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

use crate::cases::{convert_into_case, Case};
use crate::descriptor::{DescriptorWithContext, FieldDescriptorWithContext, FieldLabel, FieldType};
use crate::Result;
use ::quote::{format_ident, quote};
use ::syn::{parse2, parse_str, Ident, Item, Type};

pub struct MessageTrait {
    rust_name: Ident,
    fields: Vec<Field>,
}

impl MessageTrait {
    pub fn try_new<'a>(desc: &'a DescriptorWithContext<'a>) -> Result<Self> {
        Ok(Self {
            rust_name: Self::rust_name_from_message_name(desc.name()?)?,
            fields: desc
                .non_oneof_fields()?
                .into_iter()
                .map(Field::try_new)
                .collect::<Result<Vec<_>>>()?,
        })
    }

    fn rust_name_from_message_name(name: &str) -> Result<Ident> {
        Ok(format_ident!(
            "{}Trait",
            convert_into_case(name, Case::CamelCase)
        ))
    }
}

pub struct Field {
    original_name: String,
    wrapper: FieldWrapper,
    scalar_type: FieldScalarType,
}

impl Field {
    fn try_new(desc: &FieldDescriptorWithContext) -> Result<Self> {
        Ok(Self {
            original_name: desc.name()?.to_string(),
            wrapper: todo!(),
            scalar_type: todo!(),
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FieldWrapper {
    Bare,
    Optional,
    OptionalBoxed,
    Vec,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FieldScalarType {
    Int32,
    SInt32,
    UInt32,
    Int64,
    SInt64,
    UInt64,
    Bool,
    Enum,
    Float,
    Fixed32,
    SFixed32,
    Double,
    Fixed64,
    SFixed64,
    String,
    Bytes,
    Message,
}
