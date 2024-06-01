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
use crate::descriptor::{DescriptorWithContext, FieldDescriptorWithContext, FieldType};
use crate::Result;
use ::proc_macro2::TokenStream;
use ::quote::{quote, ToTokens, TokenStreamExt};
use ::syn::{parse_str, Ident, Type};

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
            r#type: Self::gen_type(desc.r#type()?)?,
        })
    }

    fn gen_type(ty: FieldType) -> Result<Type> {
        // let tmp;
        Ok(parse_str(match ty {
            FieldType::BOOL => "bool",
            FieldType::BYTES => "::std::vec::Vec<u8>",
            FieldType::DOUBLE => "f64",
            FieldType::ENUM(e) => {
                // tmp = e.full_name()?.to_rust_path()?;
                // &tmp
                "i32"
            }
            FieldType::FIXED32 => "u32",
            FieldType::FIXED64 => "u64",
            FieldType::FLOAT => "f32",
            FieldType::GROUP => todo!(),
            FieldType::INT32 => "i32",
            FieldType::INT64 => "i64",
            FieldType::MESSAGE(m) => {
                // tmp = m.full_name()?.to_rust_path()?;
                // tmp = m.full_name()?.to_string();
                // &tmp
                "i32"
            }
            FieldType::SFIXED32 => "i32",
            FieldType::SFIXED64 => "i64",
            FieldType::SINT32 => "i32",
            FieldType::SINT64 => "i64",
            FieldType::STRING => "::std::string::String",
            FieldType::UINT32 => "u32",
            FieldType::UINT64 => "u64",
        })?)
    }
}

impl ToTokens for MessageOpenStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let fields = &self.fields;
        tokens.append_all(quote! {
            pub struct #name {
                #(#fields)*
            }
        })
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let r#type = &self.r#type;
        tokens.append_all(quote! {
            #name: #r#type,
        })
    }
}
