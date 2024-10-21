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

use ::std::rc::Rc;

use crate::cases::{convert_into_case, Case};
use crate::descriptor::{EnumDescriptorExt, EnumValueDescriptor};
use crate::proto_path::ProtoPath;
use crate::Result;
use ::proc_macro2::TokenStream;
use ::quote::quote;
use ::syn::{parse2, parse_str, Ident, Item, Path, Variant};

use super::CodeGeneratorOptions;

pub struct GenEnumItems {
    name: Ident,
    variants: Vec<EnumVariant>,
    options: Rc<CodeGeneratorOptions>,
}

struct EnumVariant {
    name: Ident,
    number: i32,
    is_first: bool,
}

impl GenEnumItems {
    pub fn try_new<'a>(
        desc: &'a EnumDescriptorExt<'a>,
        options: Rc<CodeGeneratorOptions>,
    ) -> Result<Self> {
        Ok(Self {
            name: Self::rust_name_from_enum_name(desc.name())?,
            variants: desc
                .values()
                .enumerate()
                .map(|(i, e)| EnumVariant::try_new(e, i == 0))
                .collect::<Result<Vec<_>>>()?,
            options,
        })
    }
    pub fn rust_name_from_enum_name(name: &str) -> Result<Ident> {
        Ok(parse_str(&convert_into_case(name, Case::CamelCase))?)
    }
    pub fn rust_path_from_proto_path(&self, path: &ProtoPath) -> Result<Path> {
        path.to_rust_path_with(&self.options, |s| {
            let ident = Self::rust_name_from_enum_name(s)?;
            Ok(parse2(quote! { #ident })?)
        })
    }
    pub fn rust_items(&self) -> Result<Vec<Item>> {
        Ok(vec![
            self.rust_item_enum()?,
            self.rust_item_try_from_i32()?,
            self.rust_item_into_i32()?,
            self.rust_item_is_empty()?,
        ])
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
            #[derive(Clone, PartialEq, Eq, Debug, Default)]
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
            impl ::std::convert::TryFrom::<i32> for self::#name {
                type Error = i32;
                fn try_from(value: i32) -> ::std::result::Result::<Self, Self::Error> {
                    match value {
                        #( #var_numbers => Ok(Self::#var_names), )*
                        _ => Err(value),
                    }
                }
            }
        })?)
    }
    fn rust_item_into_i32(&self) -> Result<Item> {
        let name = &self.name;
        let (var_names, var_numbers) = self
            .variants
            .iter()
            .map(|v| (&v.name, v.number))
            .collect::<(Vec<_>, Vec<_>)>();
        Ok(parse2(quote! {
            impl ::std::convert::From::<self::#name> for i32 {
                fn from(value: self::#name) -> i32 {
                    match value {
                        #( self::#name::#var_names => #var_numbers, )*
                    }
                }
            }
        })?)
    }
    fn rust_item_is_empty(&self) -> Result<Item> {
        let name = &self.name;
        Ok(parse2(quote! {
            impl ::puroro::IsEmpty for self::#name {
                fn is_empty(&self) -> bool {
                    Self::default() == Self::clone(self)
                }
            }
        })?)
    }
}

impl EnumVariant {
    fn try_new<'a>(desc: &'a EnumValueDescriptor<'a>, is_first: bool) -> Result<Self> {
        Ok(Self {
            name: parse_str(&convert_into_case(desc.name(), Case::CamelCase))?,
            number: desc.number(),
            is_first,
        })
    }

    fn rust_enum_variant(&self) -> Result<Variant> {
        let name = &self.name;
        let maybe_default_attr: Option<TokenStream> = if self.is_first {
            Some(quote! { #[default] })
        } else {
            None
        };
        Ok(parse2(quote! {
            #maybe_default_attr
            #name
        })?)
    }
}
