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
use super::super::{Message, Oneof};
use crate::Result;
use ::puroro_protobuf_compiled::google::protobuf::FieldDescriptorProto;
use ::std::fmt::Debug;
use ::std::rc::{Rc, Weak};

pub trait OneofField: Debug {
    fn cache(&self) -> &AnonymousCache;
    fn name(&self) -> Result<&str>;
    fn oneof(&self) -> Result<Rc<dyn Oneof>>;
    fn message(&self) -> Result<Rc<dyn Message>>;
}

#[derive(Debug)]
pub struct OneofFieldImpl {
    cache: AnonymousCache,
    oneof: Weak<dyn Oneof>,
    name: String,
}

impl OneofField for OneofFieldImpl {
    fn cache(&self) -> &AnonymousCache {
        &self.cache
    }
    fn name(&self) -> Result<&str> {
        Ok(&self.name)
    }
    fn oneof(&self) -> Result<Rc<dyn Oneof>> {
        Ok(self.oneof.try_upgrade()?)
    }
    fn message(&self) -> Result<Rc<dyn Message>> {
        Ok(self.oneof()?.message()?)
    }
}

impl OneofFieldImpl {
    pub fn new(proto: &FieldDescriptorProto, oneof: Weak<dyn Oneof>) -> Rc<Self> {
        Rc::new(Self {
            cache: Default::default(),
            oneof,
            name: proto.name().to_string(),
        })
    }
}
