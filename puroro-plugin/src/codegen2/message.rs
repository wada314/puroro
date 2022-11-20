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
    fn input_file(&self) -> Result<Rc<dyn InputFileTrait>>;
    fn bitfield_size(&self) -> Result<usize>;
    fn name(&self) -> &str;

    fn messages(&self) -> Box<dyn '_ + Iterator<Item = Weak<dyn MessageTrait>>>;
    fn enums(&self) -> Box<dyn '_ + Iterator<Item = Weak<dyn EnumTrait>>>;
}

#[derive(Debug)]
pub(super) struct Message {
    name: String,
    fields: Vec<Rc<dyn FieldTrait>>,
    input_file: Weak<dyn InputFileTrait>,

    bitfield_size: OnceCell<usize>,
}

impl MessageTrait for Message {
    fn name(&self) -> &str {
        &self.name
    }

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
        let bitfield_size_in_u32_array = (self.bitfield_size()? + 31) / 32;
        Ok(quote! {
            pub struct #ident {
                #(#fields)*
                _bitfield: self::_puroro::bitvec::BitArray<#bitfield_size_in_u32_array>,
            }
        })
    }
    fn input_file(&self) -> Result<Rc<dyn InputFileTrait>> {
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

    fn messages(&self) -> Box<dyn '_ + Iterator<Item = Weak<dyn MessageTrait>>> {
        Box::new(::std::iter::empty())
    }

    fn enums(&self) -> Box<dyn '_ + Iterator<Item = Weak<dyn EnumTrait>>> {
        Box::new(::std::iter::empty())
    }
}

impl Message {
    pub(super) fn new<F>(proto: &DescriptorProto, input_file: &Weak<F>) -> Rc<Self>
    where
        F: 'static + InputFileTrait,
    {
        Self::new_with(proto, input_file, |fd, weak| Field::new(fd, &weak))
    }

    pub(super) fn new_with<F, FF>(
        proto: &DescriptorProto,
        input_file: &Weak<F>,
        mut ff: FF,
    ) -> Rc<Self>
    where
        F: 'static + InputFileTrait,
        FF: FnMut(&FieldDescriptorProto, Weak<Message>) -> Rc<dyn FieldTrait>,
    {
        let name = proto.name().to_string();
        Rc::new_cyclic(|weak| Message {
            name,
            input_file: Weak::clone(input_file) as Weak<dyn InputFileTrait>,
            fields: proto
                .field()
                .into_iter()
                .filter(|f| !f.has_oneof_index() || f.has_proto3_optional())
                .map(|f| ff(f, Weak::clone(weak)))
                .collect(),
            bitfield_size: OnceCell::new(),
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
