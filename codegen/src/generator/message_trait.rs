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
use crate::descriptor::{
    DescriptorWithContext, FieldDescriptorWithContext, FieldLabel, FieldTypeCase,
};
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

impl FieldWrapper {
    fn try_from_field_desc(desc: &FieldDescriptorWithContext) -> Result<Self> {
        Ok(match desc.label()? {
            Some(FieldLabel::Repeated) => FieldWrapper::Vec,
            Some(FieldLabel::Optional) => {
                if desc.type_case() == FieldTypeCase::Message {
                    FieldWrapper::OptionalBoxed
                } else {
                    FieldWrapper::Optional
                }
            }
            Some(FieldLabel::Required) => FieldWrapper::Bare,
            None => {
                if desc.type_case() == FieldTypeCase::Message {
                    FieldWrapper::OptionalBoxed
                } else if desc.is_proto3_optional()? {
                    FieldWrapper::Optional
                } else {
                    FieldWrapper::Bare
                }
            }
        })
    }
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

impl FieldScalarType {
    fn try_from_field_desc(desc: &FieldDescriptorWithContext) -> Result<Self> {
        Ok(match desc.type_case()? {
            FieldTypeCase::Int32 => FieldScalarType::Int32,
            FieldTypeCase::SInt32 => FieldScalarType::SInt32,
            FieldTypeCase::UInt32 => FieldScalarType::UInt32,
            FieldTypeCase::Int64 => FieldScalarType::Int64,
            FieldTypeCase::SInt64 => FieldScalarType::SInt64,
            FieldTypeCase::UInt64 => FieldScalarType::UInt64,
            FieldTypeCase::Bool => FieldScalarType::Bool,
            FieldTypeCase::Enum => FieldScalarType::Enum,
            FieldTypeCase::Float => FieldScalarType::Float,
            FieldTypeCase::Fixed32 => FieldScalarType::Fixed32,
            FieldTypeCase::SFixed32 => FieldScalarType::SFixed32,
            FieldTypeCase::Double => FieldScalarType::Double,
            FieldTypeCase::Fixed64 => FieldScalarType::Fixed64,
            FieldTypeCase::SFixed64 => FieldScalarType::SFixed64,
            FieldTypeCase::String => FieldScalarType::String,
            FieldTypeCase::Bytes => FieldScalarType::Bytes,
            FieldTypeCase::Message => FieldScalarType::Message,
        })
    }
}
