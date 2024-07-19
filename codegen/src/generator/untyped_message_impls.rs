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
use crate::descriptor::{
    Descriptor, FieldDescriptor, I32Type, I64Type, LenType, VariantType, WireType,
};
use crate::generator::message_trait::FieldWrapper;
use crate::proto_path::ProtoPath;
use crate::Result;
use ::quote::quote;
use ::syn::Expr;
use ::syn::{parse2, parse_str, Ident, Item};

pub struct UntypedMessageImpls {
    rust_trait_name: Ident,
    fields: Vec<Field>,
}

impl UntypedMessageImpls {
    pub fn try_new<'a>(desc: &'a Descriptor<'a>) -> Result<Self> {
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
    fn try_new<'a>(desc: &'a FieldDescriptor<'a>) -> Result<Self> {
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
        #[allow(unused)]
        let result = match self.trait_field.wrapper() {
            FieldWrapper::Vec => todo!(),
            _ => self.gen_non_repeated_getter_body(field_expr)?,
        };
        #[allow(unused)]
        Ok(result)
    }

    fn gen_non_repeated_getter_body(&self, field_expr: &Expr) -> Result<Expr> {
        let wire_type: WireType<_, _, _, _> = self.trait_field.scalar_type().into();
        Ok(match wire_type {
            WireType::Variant(t) => self.gen_non_repeated_varint_getter_body(field_expr, t)?,
            WireType::I32(t) => self.gen_non_repeated_i32_getter_body(field_expr, t)?,
            WireType::I64(t) => self.gen_non_repeated_i64_getter_body(field_expr, t)?,
            WireType::Len(t) => self.gen_non_repeated_len_getter_body(field_expr, t)?,
            WireType::StartGroup => todo!(),
            WireType::EndGroup => todo!(),
        })
    }

    fn gen_non_repeated_varint_getter_body(
        &self,
        field_expr: &Expr,
        t: VariantType<&ProtoPath>,
    ) -> Result<Expr> {
        todo!()
    }
    fn gen_non_repeated_i32_getter_body(&self, field_expr: &Expr, t: I32Type) -> Result<Expr> {
        Ok(parse2(match t {
            I32Type::Float => {
                quote! { ::std::f64::from_le_bytes(#field_expr.as_scalar_i32().unwrap()) }
            }
            I32Type::Fixed32 => {
                quote! { ::std::u32::from_le_bytes(#field_expr.as_scalar_i32().unwrap()) }
            }
            I32Type::SFixed32 => {
                quote! { ::std::i32::from_le_bytes(#field_expr.as_scalar_i32().unwrap()) }
            }
        })?)
    }
    fn gen_non_repeated_i64_getter_body(&self, field_expr: &Expr, t: I64Type) -> Result<Expr> {
        todo!()
    }
    fn gen_non_repeated_len_getter_body(
        &self,
        field_expr: &Expr,
        t: LenType<&ProtoPath>,
    ) -> Result<Expr> {
        todo!()
    }
}
