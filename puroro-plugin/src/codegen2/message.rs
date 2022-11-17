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
use ::once_cell::unsync::OnceCell;
use ::proc_macro2::TokenStream;
use ::puroro_protobuf_compiled::google::protobuf::{
    DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, OneofDescriptorProto,
};
use ::quote::{format_ident, quote};
use ::std::fmt::Debug;
use ::std::ops::Deref;
use ::std::rc::{Rc, Weak};

pub(super) trait MessageTrait: Debug {
    fn gen_struct(&self) -> Result<TokenStream>;
    fn input_file(&self) -> Result<Rc<Box<dyn InputFileTrait>>>;
    fn bitfield_size(&self) -> Result<usize>;
}

#[derive(Debug)]
pub(super) struct Message {
    name: String,
    fields: Vec<Rc<Box<dyn FieldTrait>>>,
    input_file: Weak<Box<dyn InputFileTrait>>,

    bitfield_size: OnceCell<usize>,
}

impl MessageTrait for Message {
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
    fn input_file(&self) -> Result<Rc<Box<dyn InputFileTrait>>> {
        Ok(self.input_file.try_upgrade()?)
    }

    fn bitfield_size(&self) -> Result<usize> {
        self.bitfield_size
            .get_or_try_init(|| {
                let mut tail = 0;
                for field in &self.fields {
                    if let Some(next_tail) = field.maybe_allocated_bitfield_tail()? {
                        tail = next_tail;
                    } else {
                        tail = field.assign_and_get_bitfield_tail(tail)?;
                    }
                }
                Ok(tail)
            })
            .cloned()
    }
}

impl Message {
    pub(super) fn try_new(
        proto: &DescriptorProto,
        input_file: &Weak<Box<dyn InputFileTrait>>,
    ) -> Result<Rc<Box<dyn MessageTrait>>> {
        Self::try_new_with(proto, input_file, |fd, weak| Field::try_new(fd, &weak))
    }

    pub(super) fn try_new_with<FF>(
        proto: &DescriptorProto,
        input_file: &Weak<Box<dyn InputFileTrait>>,
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
            Ok(Box::new(Message {
                name,
                input_file: Weak::clone(input_file),
                fields: proto
                    .field()
                    .into_iter()
                    .filter(|f| !f.has_oneof_index() || f.has_proto3_optional())
                    .map(|f| ff(f, Weak::clone(weak)))
                    .collect::<Result<Vec<_>>>()?,
                bitfield_size: OnceCell::new(),
            }))
        })
    }
}

#[cfg(test)]
#[derive(Debug)]
pub struct MessageFake;

#[cfg(test)]
impl MessageFake {
    pub fn try_new(proto: &DescriptorProto) -> Result<Rc<Box<Self>>> {
        Ok(Rc::new(Box::new(MessageFake)))
    }
}
