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

use super::message_trait::{Field as TraitField, MessageTrait};
use crate::cases::{convert_into_case, Case};
use crate::descriptor::{
    DescriptorWithContext, FieldDescriptorWithContext, FieldLabel, FieldType, FieldTypeCase,
};
use crate::generator::avoid_reserved_keywords;
use crate::proto_path::{ProtoPath, ProtoPathBuf};
use crate::Result;
use ::quote::{format_ident, quote};
use ::syn::Lifetime;
use ::syn::{parse2, parse_str, Ident, Item, Path, Type};

pub struct UntypedMessageImpls {
    rust_trait_name: Ident,
    fields: Vec<Field>,
}

impl UntypedMessageImpls {
    fn try_new<'a>(desc: &'a DescriptorWithContext<'a>) -> Result<Self> {
        Ok(Self {
            rust_trait_name: MessageTrait::rust_name_from_message_name(desc.name()?)?,
            fields: desc
                .non_oneof_fields()?
                .into_iter()
                .map(Field::try_new)
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

pub struct Field {}

impl Field {
    fn try_new(desc: &FieldDescriptorWithContext) -> Result<Self> {
        Ok(Self {})
    }
}
