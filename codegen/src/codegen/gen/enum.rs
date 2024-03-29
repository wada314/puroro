// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use self file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! For the enum details, see the official documents:
//!  - [proto2 document](https://developers.google.com/protocol-buffers/docs/proto#enum)
//!  - [proto3 document](https://developers.google.com/protocol-buffers/docs/proto3#enum)
//!  - [c++ generated code](https://developers.google.com/protocol-buffers/docs/reference/cpp-generated#enum)

use super::super::util::*;
use super::{DataTypeBase, Enum, PackageOrMessageExt, Syntax, PURORO_LIB};
use crate::syn;
use crate::syn::{parse2, Attribute, Item, ItemEnum, Path, Type};
use crate::{FatalErrorKind, Result};
use ::once_cell::unsync::OnceCell;
use ::quote::{format_ident, quote};
use ::std::fmt::Debug;
use ::std::rc::Rc;
use ::syn::ItemImpl;

#[derive(Debug, Default)]
struct Cache {
    enum_path: OnceCell<Rc<Path>>,
    enum_type: OnceCell<Rc<Type>>,
}

impl Enum {
    pub(crate) fn gen_enum_items(&self) -> Result<Vec<Item>> {
        let ident = format_ident!("{}", self.name()?.to_camel_case().escape_rust_keywords());
        let value_idents = self
            .values()?
            .map(|(name, _)| format_ident!("{}", name.to_camel_case().escape_rust_keywords()))
            .collect::<Vec<_>>();
        let syntax = self.syntax()?;
        let maybe_extra_value = match syntax {
            Syntax::Proto2 => quote! {},
            Syntax::Proto3 => quote! { _None(i32), },
        };
        let first_value_ident = value_idents.first().ok_or(FatalErrorKind::NoEnumValues)?;
        let item_into_i32 = self.gen_enum_into_i32()?;
        let item_from_i32 = match syntax {
            Syntax::Proto2 => self.gen_enum_try_from_i32()?,
            Syntax::Proto3 => self.gen_enum_from_i32()?,
        };
        let docs = self.gen_enum_doc_attrs()?;
        let value_docs = (0..self.values()?.count())
            .map(|i| self.gen_enum_value_doc_attrs(i))
            .collect::<Result<Vec<_>>>()?;

        let item_enum: ItemEnum = parse2(quote! {
            #[derive(
                ::std::clone::Clone,
                ::std::marker::Copy,
                ::std::cmp::PartialEq,
                ::std::cmp::Eq,
                ::std::cmp::PartialOrd,
                ::std::cmp::Ord,
                ::std::hash::Hash,
                ::std::fmt::Debug,
            )]
            #(#docs)*
            pub enum #ident {
                #(
                    #(#value_docs)*
                    #value_idents,
                )*
                #maybe_extra_value
            }
        })?;
        let item_impl_default: ItemImpl = parse2(quote! {
            impl ::std::default::Default for #ident {
                fn default() -> Self {
                    Self::#first_value_ident
                }
            }
        })?;
        Ok(vec![
            item_enum.into(),
            item_impl_default.into(),
            item_into_i32.into(),
            item_from_i32.into(),
        ])
    }

    pub(crate) fn gen_enum_path(&self) -> Result<Rc<syn::Path>> {
        self.cache()
            .get::<Cache>()?
            .enum_path
            .get_or_try_init(|| {
                let ident =
                    format_ident!("{}", self.name()?.to_camel_case().escape_rust_keywords());
                let parent = self.parent()?.gen_rust_module_path()?;
                Ok(Rc::new(syn::parse2(quote! { #parent :: #ident })?))
            })
            .cloned()
    }

    pub(crate) fn gen_enum_type(&self) -> Result<Rc<syn::Type>> {
        self.cache()
            .get::<Cache>()?
            .enum_type
            .get_or_try_init(|| {
                let path = self.gen_enum_path()?;
                Ok(Rc::new(syn::parse2(quote! { #path })?))
            })
            .cloned()
    }

    fn gen_enum_into_i32(&self) -> Result<ItemImpl> {
        let ident = format_ident!("{}", self.name()?.to_camel_case().escape_rust_keywords());
        let syntax = self.syntax()?;
        let value_idents = self
            .values()?
            .map(|(name, _)| format_ident!("{}", name.to_camel_case().escape_rust_keywords()))
            .collect::<Vec<_>>();
        let value_numbers = self.values()?.map(|(_, number)| number).collect::<Vec<_>>();
        let maybe_none_arm = match syntax {
            Syntax::Proto2 => quote! {},
            Syntax::Proto3 => quote! {
                self::#ident::_None(i) => i,
            },
        };

        Ok(parse2(quote! {
            impl ::std::convert::From::<#ident> for i32 {
                fn from(val: #ident) -> i32 {
                    match val {
                        #(#ident::#value_idents => #value_numbers,)*
                        #maybe_none_arm
                    }
                }
            }
        })?)
    }

    fn gen_enum_from_i32(&self) -> Result<ItemImpl> {
        debug_assert_eq!(self.syntax()?, Syntax::Proto3);
        let ident = format_ident!("{}", self.name()?.to_camel_case().escape_rust_keywords());
        let value_idents = self
            .values()?
            .map(|(name, _)| format_ident!("{}", name.to_camel_case().escape_rust_keywords()))
            .collect::<Vec<_>>();
        let value_numbers = self.values()?.map(|(_, number)| number).collect::<Vec<_>>();
        Ok(parse2(quote! {
            impl ::std::convert::From::<i32> for #ident {
                fn from(val: i32) -> Self {
                    match val {
                        #(#value_numbers => self::#ident::#value_idents,)*
                        _ => #ident::_None(val),
                    }
                }
            }
        })?)
    }

    fn gen_enum_try_from_i32(&self) -> Result<ItemImpl> {
        debug_assert_eq!(self.syntax()?, Syntax::Proto2);
        let ident = format_ident!("{}", self.name()?.to_camel_case().escape_rust_keywords());
        let value_idents = self
            .values()?
            .map(|(name, _)| format_ident!("{}", name.to_camel_case().escape_rust_keywords()))
            .collect::<Vec<_>>();
        let value_numbers = self.values()?.map(|(_, number)| number).collect::<Vec<_>>();
        Ok(parse2(quote! {
            impl ::std::convert::TryFrom::<i32> for #ident {
                type Error = #PURORO_LIB::PuroroError;
                fn try_from(val: i32) -> ::std::result::Result<Self, Self::Error> {
                    match val {
                        #(#value_numbers => ::std::result::Result::Ok(self::#ident::#value_idents),)*
                        _ => ::std::result::Result::Err(#PURORO_LIB::PuroroError::UnknownEnumVariant(val))?,
                    }
                }
            }
        })?)
    }

    fn gen_enum_doc_attrs(&self) -> Result<Vec<Attribute>> {
        let input_file = self.input_file()?;
        let Some(sci) = input_file.source_code_info(self.location_path()?)? else {
            return Ok(Vec::new());
        };
        Ok(sci.gen_doc_attributes()?)
    }

    fn gen_enum_value_doc_attrs(&self, index: usize) -> Result<Vec<Attribute>> {
        let input_file = self.input_file()?;
        let Some(sci) = input_file.source_code_info(self.value_location_path(index)?)? else {
            return Ok(Vec::new());
        };
        Ok(sci.gen_doc_attributes()?)
    }
}
