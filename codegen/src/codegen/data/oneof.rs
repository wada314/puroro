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

use super::super::util::*;
use super::{Message, OneofField, OneofFieldImpl};
use crate::Result;
use ::puroro_protobuf_compiled::google::protobuf::{DescriptorProto, OneofDescriptorProto};
use ::std::fmt::Debug;
use ::std::rc::{Rc, Weak};

pub trait Oneof: Debug {
    fn cache(&self) -> &AnonymousCache;
    fn message(&self) -> Result<Rc<dyn Message>>;
    fn name(&self) -> Result<&str>;
    fn fields(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn OneofField>>>>;
}

#[derive(Debug)]
pub struct OneofImpl {
    cache: AnonymousCache,
    message: Weak<dyn Message>,
    name: String,
    fields: Vec<Rc<dyn OneofField>>,
}

impl OneofImpl {
    pub fn new(
        message_proto: &DescriptorProto,
        oneof_proto: &OneofDescriptorProto,
        oneof_index: usize,
        message: Weak<dyn Message>,
    ) -> Rc<OneofImpl> {
        Rc::new_cyclic(|weak| {
            let fields = message_proto
                .field()
                .iter()
                .filter(|f| f.oneof_index() as usize == oneof_index)
                .map(|f| {
                    OneofFieldImpl::new(f, Weak::clone(weak) as Weak<dyn Oneof>)
                        as Rc<dyn OneofField>
                })
                .collect::<Vec<_>>();
            OneofImpl {
                cache: Default::default(),
                message,
                name: oneof_proto.name().to_string(),
                fields,
            }
        })
    }
}

impl Oneof for OneofImpl {
    fn cache(&self) -> &AnonymousCache {
        &self.cache
    }
    fn message(&self) -> Result<Rc<dyn Message>> {
        Ok(self.message.try_upgrade()?)
    }
    fn name(&self) -> Result<&str> {
        Ok(&self.name)
    }
    fn fields(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn OneofField>>>> {
        Ok(Box::new(self.fields.iter().cloned()))
    }
}
