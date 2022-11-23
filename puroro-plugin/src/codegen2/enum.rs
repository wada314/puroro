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

//! For the enum details, see the official document:
//!  - [proto2 document](https://developers.google.com/protocol-buffers/docs/proto#enum)
//!  - [proto3 document](https://developers.google.com/protocol-buffers/docs/proto3#enum)
//!  - [c++ generated code](https://developers.google.com/protocol-buffers/docs/reference/cpp-generated#enum)

use super::util::{StrExt, WeakExt};
use super::{InputFile, PackageOrMessage, Syntax};
use crate::{ErrorKind, Result};
use ::once_cell::unsync::OnceCell;
use ::proc_macro2::TokenStream;
use ::puroro_protobuf_compiled::google::protobuf::EnumDescriptorProto;
use ::quote::{format_ident, quote};
use ::std::fmt::Debug;
use ::std::rc::{Rc, Weak};

pub(super) trait Enum: Debug {
    fn name(&self) -> &str;
    fn gen_enum(&self) -> Result<TokenStream>;
    fn gen_rust_enum_path(&self) -> Result<Rc<TokenStream>>;
}

#[derive(Debug)]
pub(super) struct EnumImpl {
    name: String,
    input_file: Weak<dyn InputFile>,
    parent: Weak<dyn PackageOrMessage>,
    values: Vec<(String, i32)>,
    rust_enum_path: OnceCell<Rc<TokenStream>>,
}

impl EnumImpl {
    pub(super) fn new(
        proto: &EnumDescriptorProto,
        input_file: Weak<dyn InputFile>,
        parent: Weak<dyn PackageOrMessage>,
    ) -> Rc<Self> {
        let values = proto
            .value()
            .into_iter()
            .map(|v| (v.name().to_string(), v.number()))
            .collect::<Vec<_>>();
        Rc::new(EnumImpl {
            name: proto.name().to_string(),
            input_file,
            parent,
            values,
            rust_enum_path: OnceCell::new(),
        })
    }

    fn parent(&self) -> Result<Rc<dyn PackageOrMessage>> {
        Ok(self.parent.try_upgrade()?)
    }
    fn syntax(&self) -> Result<Syntax> {
        Ok(self.input_file.try_upgrade()?.syntax()?)
    }
}

impl Enum for EnumImpl {
    fn name(&self) -> &str {
        &self.name
    }

    fn gen_enum(&self) -> Result<TokenStream> {
        let ident = format_ident!("{}", self.name().to_camel_case().escape_rust_keywords());
        let value_idents = self
            .values
            .iter()
            .map(|(name, _)| format_ident!("{}", name.to_camel_case().escape_rust_keywords()))
            .collect::<Vec<_>>();
        let syntax = self.syntax()?;
        let maybe_extra_value = match syntax {
            Syntax::Proto2 => quote! {},
            Syntax::Proto3 => quote! { _None(i32), },
        };
        let first_value_ident = value_idents.first().ok_or(ErrorKind::NoEnumValues)?;

        Ok(quote! {
            #[derive(
                ::std::clone::Clone,
                ::std::marker::Copy,
                ::std::cmp::PartialEq,
                ::std::cmp::Eq,
                ::std::cmp::PartialOrd,
                ::std::cmp::Ord,
                ::std::hash::Hash,
            )]
            pub enum #ident {
                #(#value_idents,)*
                #maybe_extra_value
            }

            impl ::std::default::Default for #ident {
                fn default() -> Self {
                    Self::#first_value_ident
                }
            }
        })
    }

    fn gen_rust_enum_path(&self) -> Result<Rc<TokenStream>> {
        self.rust_enum_path
            .get_or_try_init(|| {
                let ident = format_ident!("{}", self.name().to_camel_case().escape_rust_keywords());
                let parent = self.parent()?.gen_rust_module_path()?;
                Ok(Rc::new(quote! { #parent :: #ident }))
            })
            .cloned()
    }
}

#[cfg(test)]
pub struct EnumFake;

#[cfg(test)]
impl EnumFake {
    fn try_new(proto: &EnumDescriptorProto) -> Result<Self> {
        Ok(EnumFake)
    }
}
