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
use super::super::{FieldBase, FieldType, Message, Oneof};
use crate::Result;
use ::once_cell::unsync::OnceCell;
use ::puroro_protobuf_compiled::google::protobuf::{field_descriptor_proto, FieldDescriptorProto};
use ::std::fmt::Debug;
use ::std::rc::{Rc, Weak};

pub trait OneofField: FieldBase + Debug {
    fn cache(&self) -> &AnonymousCache;
    fn oneof(&self) -> Result<Rc<dyn Oneof>>;
}

#[derive(Debug)]
pub struct OneofFieldImpl {
    cache1: AnonymousCache,
    cache2: AnonymousCache,
    oneof: Weak<dyn Oneof>,
    name: String,
    number: i32,
    type_opt: Option<field_descriptor_proto::Type>,
    type_name: String,
    r#type: OnceCell<FieldType>,
    default_value: Option<String>,
}

impl FieldBase for OneofFieldImpl {
    fn cache_base(&self) -> &AnonymousCache {
        &self.cache1
    }
    fn name(&self) -> Result<&str> {
        Ok(&self.name)
    }
    fn message(&self) -> Result<Rc<dyn Message>> {
        Ok(self.oneof()?.message()?)
    }
    fn number(&self) -> Result<i32> {
        Ok(self.number)
    }
    fn r#type(&self) -> Result<&FieldType> {
        self.r#type.get_or_try_init(|| {
            let syntax = self.message()?.input_file()?.syntax()?;
            Ok(FieldType::try_new(
                self.type_opt.clone(),
                &self.type_name,
                syntax,
                self.message()?,
            )?)
        })
    }
    fn default_value(&self) -> Result<Option<&str>> {
        Ok(self.default_value.as_deref())
    }
}

impl OneofField for OneofFieldImpl {
    fn cache(&self) -> &AnonymousCache {
        &self.cache2
    }
    fn oneof(&self) -> Result<Rc<dyn Oneof>> {
        Ok(self.oneof.try_upgrade()?)
    }
}

impl OneofFieldImpl {
    pub fn new(proto: &FieldDescriptorProto, oneof: Weak<dyn Oneof>) -> Rc<Self> {
        Rc::new(Self {
            cache1: Default::default(),
            cache2: Default::default(),
            oneof,
            name: proto.name().to_string(),
            number: proto.number(),
            type_opt: proto.type_opt(),
            type_name: proto.type_name().to_string(),
            r#type: OnceCell::new(),
            default_value: proto.default_value_opt().map(|s| s.to_string()),
        })
    }
}
