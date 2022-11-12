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

use super::*;
use crate::codegen::utils::StrExt;
use crate::Result;
use ::proc_macro2::TokenStream;
use ::puroro_protobuf_compiled::google::protobuf::{
    DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, OneofDescriptorProto,
};
use ::quote::{format_ident, quote};
use ::std::fmt::Debug;
use ::std::ops::Deref;
use ::std::rc::{Rc, Weak};

pub trait MessageTrait: Debug {
    fn gen_struct(&self) -> Result<TokenStream>;
}

#[derive(Debug)]
pub struct MessageImpl {
    name: String,
    fields: Vec<Rc<Box<dyn FieldTrait>>>,
}

impl MessageTrait for MessageImpl {
    fn gen_struct(&self) -> Result<TokenStream> {
        let ident = format_ident!(
            "{}",
            self.name.to_camel_case().escape_rust_keywords().to_string()
        );
        let fields = self
            .fields
            .iter()
            .map(|f| f.gen_struct_field_decl())
            .collect::<Result<Vec<_>>>()?;
        Ok(quote! {
            pub struct #ident {
                #(#fields)*
            }
        })
    }
}

impl MessageImpl {
    pub fn try_new(proto: &DescriptorProto) -> Result<Rc<Box<dyn MessageTrait>>> {
        Self::try_new_with(proto, |fd, _weak| FieldImpl::try_new(fd))
    }

    pub fn try_new_with<FF>(
        proto: &DescriptorProto,
        mut ff: FF,
    ) -> Result<Rc<Box<dyn MessageTrait>>>
    where
        FF: FnMut(
            &FieldDescriptorProto,
            Weak<Box<dyn MessageTrait>>,
        ) -> Result<Rc<Box<dyn FieldTrait>>>,
    {
        let name = proto.name().to_string();
        Rc::try_new_boxed_cyclic(|weak| -> Result<Box<dyn MessageTrait>> {
            Ok(Box::new(MessageImpl {
                name,
                fields: proto
                    .field()
                    .into_iter()
                    .filter(|f| !f.has_oneof_index() || f.has_proto3_optional())
                    .map(|f| ff(f, Weak::clone(weak)))
                    .collect::<Result<Vec<_>>>()?,
            }))
        })
    }
}

#[cfg(test)]
pub struct MessageFake;

#[cfg(test)]
impl MessageTrait for MessageFake {
    fn try_new(proto: &DescriptorProto, _context: &ContextForMessage) -> Result<Self> {
        Ok(MessageFake)
    }
}
