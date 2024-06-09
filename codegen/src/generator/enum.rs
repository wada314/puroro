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
use crate::descriptor::{EnumDescriptorWithContext, EnumValueDescriptorWithContext};
use crate::proto_path::ProtoPath;
use crate::Result;
use ::quote::{format_ident, quote};
use ::syn::{parse2, parse_str, Ident, Item, Type, Variant};

pub struct Enum {
    name: Ident,
    variants: Vec<EnumVariant>,
}

struct EnumVariant {
    name: Ident,
    number: i32,
}

impl Enum {
    pub fn try_new<'a>(desc: &'a EnumDescriptorWithContext<'a>) -> Result<Self> {
        Ok(Self {
            name: Self::rust_name_from_enum_name(desc.name()?)?,
            variants: desc
                .values()?
                .into_iter()
                .map(EnumVariant::try_new)
                .collect::<Result<Vec<_>>>()?,
        })
    }
    pub fn rust_name_from_enum_name(name: &str) -> Result<Ident> {
        Ok(format_ident!(
            "{}",
            convert_into_case(name, Case::CamelCase)
        ))
    }
    pub fn rust_path_from_enum_path(path: impl AsRef<ProtoPath>) -> Result<Type> {
        let modules = path
            .as_ref()
            .parent()
            .into_iter()
            .flat_map(|p| p.components())
            .map(|c| format_ident!("{}", c))
            .collect::<Vec<_>>();
        let name = Self::rust_name_from_enum_name(
            path.as_ref()
                .last_component()
                .ok_or_else(|| format!("Enum path is empty: {:?}", path.as_ref()))?,
        )?;
        Ok(parse2(quote! {
            crate #(:: #modules)* :: #name
        })?)
    }
    pub fn rust_items(&self) -> Result<Vec<Item>> {
        Ok(vec![self.rust_item_enum()?, self.rust_item_try_from_i32()?])
    }
    fn rust_item_enum(&self) -> Result<Item> {
        let name = &self.name;
        let variants = self
            .variants
            .iter()
            .map(|v| v.rust_enum_variant())
            .collect::<Result<Vec<_>>>()
            .unwrap();
        Ok(parse2(quote! {
            pub enum #name {
                #(#variants ,)*
            }
        })?)
    }
    fn rust_item_try_from_i32(&self) -> Result<Item> {
        let name = &self.name;
        let (var_names, var_numbers) = self
            .variants
            .iter()
            .map(|v| (&v.name, v.number))
            .collect::<(Vec<_>, Vec<_>)>();
        Ok(parse2(quote! {
            impl ::std::convert::TryFrom<i32> for self::#name {
                type Error = i32;
                fn try_from(value: i32) -> ::std::result::Result<Self, Self::Error> {
                    match value {
                        #( #var_numbers => Ok(Self::#var_names), )*
                        _ => Err(value),
                    }
                }
            }
        })?)
    }
}

impl EnumVariant {
    fn try_new(desc: &EnumValueDescriptorWithContext) -> Result<Self> {
        Ok(Self {
            name: parse_str(&convert_into_case(desc.name()?, Case::CamelCase))?,
            number: desc.number()?,
        })
    }

    fn rust_enum_variant(&self) -> Result<Variant> {
        let name = &self.name;
        let number = self.number;
        Ok(parse2(quote! {
            #name = #number
        })?)
    }
}
