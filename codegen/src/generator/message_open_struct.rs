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
use ::proc_macro2::TokenStream;
use ::quote::{quote, ToTokens, TokenStreamExt};
use ::syn::{parse2, parse_str, Ident, Type};

pub struct MessageOpenStruct {
    name: Ident,
    fields: Vec<Field>,
}

struct Field {
    name: Ident,
    r#type: Type,
}

impl MessageOpenStruct {
    pub fn try_new<'a>(desc: &'a DescriptorWithContext<'a>) -> Result<Self> {
        Ok(Self {
            name: parse_str(&convert_into_case(desc.name()?, Case::CamelCase))?,
            fields: desc
                .non_oneof_fields()?
                .into_iter()
                .map(Field::try_new)
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

impl Field {
    pub fn try_new<'a>(desc: &'a FieldDescriptorWithContext<'a>) -> Result<Self> {
        Ok(Self {
            name: parse_str(&convert_into_case(desc.name()?, Case::LowerSnakeCase))?,
            r#type: Self::gen_type(desc.r#type()?, desc.label()?, desc.is_proto3_optional()?)?,
        })
    }

    fn gen_type(
        ty: FieldType,
        label: Option<FieldLabel>,
        is_proto3_optional: bool,
    ) -> Result<Type> {
        match (label, is_proto3_optional) {
            (Some(FieldLabel::Repeated), _) => Self::gen_repeated_type(ty),
            (None, false) => Self::gen_scalar_type(ty),
            _ => Self::gen_optional_type(ty),
        }
    }

    fn gen_scalar_type(ty: FieldType) -> Result<Type> {
        let tmp;
        Ok(parse_str(match ty {
            FieldType::Bool => "bool",
            FieldType::Bytes => "::std::vec::Vec<u8, A>",
            FieldType::Double => "f64",
            FieldType::Enum(e) => {
                tmp = e.full_path()?.to_rust_path()?;
                &tmp
            }
            FieldType::Fixed32 => "u32",
            FieldType::Fixed64 => "u64",
            FieldType::Float => "f32",
            FieldType::Group => todo!(),
            FieldType::Int32 => "i32",
            FieldType::Int64 => "i64",
            FieldType::Message(m) => {
                tmp = format!("::std::boxed::Box::<{}, A>", m.full_path()?.to_rust_path()?);
                &tmp
            }
            FieldType::SFixed32 => "i32",
            FieldType::SFixed64 => "i64",
            FieldType::SInt32 => "i32",
            FieldType::SInt64 => "i64",
            FieldType::String => "::puroro::string::String<A>",
            FieldType::UInt32 => "u32",
            FieldType::UInt64 => "u64",
        })?)
    }

    fn gen_optional_type(ty: FieldType) -> Result<Type> {
        let scalar = Self::gen_scalar_type(ty)?;
        Ok(parse2(quote! {
            ::std::option::Option::<#scalar>
        })?)
    }

    fn gen_repeated_type(ty: FieldType) -> Result<Type> {
        let scalar_type = match ty {
            FieldType::Message(m) => parse_str(&m.full_path()?.to_rust_path()?)?,
            _ => Self::gen_scalar_type(ty)?,
        };
        Ok(parse2(quote! {quote! {::std::vec::Vec::<#scalar_type>}})?)
    }
}

impl ToTokens for MessageOpenStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let fields = &self.fields;
        tokens.append_all(quote! {
            pub struct #name<#[cfg(allocator)]A: ::std::alloc::Allocator = ::std::alloc::Global> {
                #(#fields)*
            }
        })
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let ty = &self.r#type;
        tokens.append_all(quote! {
            pub #name: #ty,
        })
    }
}
