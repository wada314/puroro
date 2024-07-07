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
use crate::generator::avoid_reserved_keywords;
use crate::proto_path::ProtoPath;
use crate::Result;
use ::quote::{format_ident, quote};
use ::syn::{parse2, parse_str, Ident, Item, Path};

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
    pub fn rust_path_from_proto_path(path: &ProtoPath) -> Result<Path> {
        path.to_rust_path_with(|s| Self::rust_name_from_message_name(s))
    }

    pub fn gen_message_trait(&self) -> Result<Item> {
        let trait_name = &self.rust_name;
        let field_items_vv = self
            .fields
            .iter()
            .map(Field::gen_field_items)
            .collect::<Result<Vec<_>>>()?;
        let field_items = field_items_vv.into_iter().flatten().collect::<Vec<_>>();
        Ok(parse2(quote! {
            pub trait #trait_name {
                #(#field_items)*
            }
        })?)
    }
}

pub struct Field {
    original_name: String,
    wrapper: FieldWrapper,
    scalar_type: FieldTypeCase,
}

impl Field {
    fn try_new(desc: &FieldDescriptorWithContext) -> Result<Self> {
        Ok(Self {
            original_name: desc.name()?.to_string(),
            wrapper: FieldWrapper::try_from_field_desc(desc)?,
            scalar_type: desc.type_case(),
        })
    }

    fn gen_field_items(&self) -> Result<Vec<Item>> {
        let getter = self.gen_getter()?;
        Ok(vec![getter])
    }

    fn gen_getter(&self) -> Result<Item> {
        let getter_name: Ident = {
            let lower_cased = convert_into_case(&self.original_name, Case::LowerSnakeCase);
            parse_str(&avoid_reserved_keywords(&lower_cased))?
        };
        Ok(parse2(quote! {
            fn #getter_name(&self) -> ();
        })?)
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
