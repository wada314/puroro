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
    DescriptorWithContext, FieldDescriptorWithContext, FieldLabel, FieldType, FieldTypeCase,
};
use crate::generator::avoid_reserved_keywords;
use crate::proto_path::{ProtoPath, ProtoPathBuf};
use crate::Result;
use ::quote::{format_ident, quote};
use ::syn::Lifetime;
use ::syn::{parse2, parse_str, Ident, Item, Path, Type};

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
        path.to_rust_path_with(|s| {
            let ident = Self::rust_name_from_message_name(s)?;
            Ok(parse2(quote! { #ident })?)
        })
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
    scalar_type: FieldType<ProtoPathBuf, ProtoPathBuf>,
}

impl Field {
    fn try_new(desc: &FieldDescriptorWithContext) -> Result<Self> {
        Ok(Self {
            original_name: desc.name()?.to_string(),
            wrapper: FieldWrapper::try_from_field_desc(desc)?,
            scalar_type: desc.type_with_full_path()?,
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
        let getter_type = match self.wrapper {
            FieldWrapper::Vec => self.scalar_type.gen_repeated_getter_type(None)?,
            _ => self.scalar_type.gen_scalar_getter_type(None)?,
        };
        Ok(parse2(quote! {
            fn #getter_name(&self) -> #getter_type;
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
            Some(FieldLabel::Optional | FieldLabel::Required) => {
                if desc.type_case() == FieldTypeCase::Message {
                    FieldWrapper::OptionalBoxed
                } else {
                    FieldWrapper::Optional
                }
            }
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

impl FieldType<ProtoPathBuf, ProtoPathBuf> {
    fn gen_bare_getter_type(&self, lifetime: Option<&Lifetime>) -> Result<Type> {
        Ok(match self {
            FieldType::Message(path) => {
                let path = path.to_rust_path_with(|name| {
                    let ident = MessageTrait::rust_name_from_message_name(name)?;
                    Ok(parse2(quote! { #ident })?)
                })?;
                parse2(quote! { & #lifetime impl #path })?
            }
            FieldType::Enum(path) => {
                let path = path.to_rust_path()?;
                parse2(quote! { #path })?
            }
            FieldType::Int32 => parse_str("i32")?,
            FieldType::Int64 => parse_str("i64")?,
            FieldType::UInt32 => parse_str("u32")?,
            FieldType::UInt64 => parse_str("u64")?,
            FieldType::SInt32 => parse_str("i32")?,
            FieldType::SInt64 => parse_str("i64")?,
            FieldType::Fixed32 => parse_str("u32")?,
            FieldType::Fixed64 => parse_str("u64")?,
            FieldType::SFixed32 => parse_str("i32")?,
            FieldType::SFixed64 => parse_str("i64")?,
            FieldType::Float => parse_str("f32")?,
            FieldType::Double => parse_str("f64")?,
            FieldType::Bool => parse_str("bool")?,
            FieldType::String => parse2(quote! { & #lifetime str })?,
            FieldType::Bytes => parse2(quote! { & #lifetime [u8] })?,
            FieldType::Group => Err(format!("Group field is not supported"))?,
        })
    }
    fn gen_scalar_getter_type(&self, lifetime: Option<&Lifetime>) -> Result<Type> {
        let bare_type = self.gen_bare_getter_type(lifetime)?;
        Ok(match self {
            FieldType::Message(_) => parse2(quote! {
                ::std::option::Option::< #bare_type >
            })?,
            _ => bare_type,
        })
    }
    fn gen_repeated_getter_type(&self, lifetime: Option<&Lifetime>) -> Result<Type> {
        let bare_type = self.gen_bare_getter_type(lifetime)?;
        Ok(parse2(quote! {
            impl ::std::iter::Iterator::<Item = #bare_type >
        })?)
    }
}
