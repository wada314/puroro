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
use crate::generator::message_trait::FieldWrapper;
use crate::proto_path::{ProtoPath, ProtoPathBuf};
use crate::Result;
use ::quote::{format_ident, quote};
use ::syn::{parse2, parse_str, Ident, Item, Path, Type};
use ::syn::{Expr, Lifetime};

pub struct UntypedMessageImpls {
    rust_trait_name: Ident,
    fields: Vec<Field>,
}

impl UntypedMessageImpls {
    pub fn try_new<'a>(desc: &'a DescriptorWithContext<'a>) -> Result<Self> {
        Ok(Self {
            rust_trait_name: MessageTrait::rust_name_from_message_name(desc.name()?)?,
            fields: desc
                .non_oneof_fields()?
                .into_iter()
                .map(Field::try_new)
                .collect::<Result<Vec<_>>>()?,
        })
    }

    pub fn gen_impl_message_trait(&self) -> Result<Item> {
        let trait_name = &self.rust_trait_name;
        let getters = self
            .fields
            .iter()
            .map(Field::gen_getter)
            .collect::<Result<Vec<_>>>()?;
        Ok(parse2(quote! {
            impl self::#trait_name for ::puroro::untyped_message::UntypedMessage<'_> {
                #(#getters)*
            }
        })?)
    }
}

pub struct Field {
    number: i32,
    trait_field: TraitField,
}

impl Field {
    fn try_new<'a>(desc: &'a FieldDescriptorWithContext<'a>) -> Result<Self> {
        Ok(Self {
            number: desc.number(),
            trait_field: TraitField::try_new(desc)?,
        })
    }

    fn gen_getter(&self) -> Result<Item> {
        let signature = self.trait_field.gen_getter_signature()?;
        let number = self.number;
        let body = self.gen_getter_body(&parse_str("f")?)?;
        Ok(parse2(quote! {
            #signature {
                let f = self.field(#number);
                #body
            }
        })?)
    }

    fn gen_getter_body(&self, field_expr: &Expr) -> Result<Expr> {
        let result = match self.trait_field.wrapper() {
            FieldWrapper::Bare => todo!(),
            FieldWrapper::Optional => todo!(),
            FieldWrapper::OptionalBoxed => todo!(),
            FieldWrapper::Vec => todo!(),
        };
        Ok(result)
    }
}
