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

use super::super::util::{AnonymousCache, WeakExt};
use super::super::{FieldRule, FieldType, Message};
use crate::Result;
use ::once_cell::unsync::OnceCell;
use ::puroro_protobuf_compiled::google::protobuf::{field_descriptor_proto, FieldDescriptorProto};
use ::std::fmt::Debug;
use ::std::rc::{Rc, Weak};

pub trait Field: Debug {
    fn cache(&self) -> &AnonymousCache;
    fn name(&self) -> Result<&str>;
    fn message(&self) -> Result<Rc<dyn Message>>;
    fn number(&self) -> i32;
    fn rule(&self) -> Result<FieldRule>;
    fn r#type(&self) -> Result<&FieldType>;
}

#[derive(Debug)]
pub struct FieldImpl {
    cache: AnonymousCache,
    name: String,
    message: Weak<dyn Message>,
    rule: OnceCell<FieldRule>,
    r#type: OnceCell<FieldType>,
    proto3_optional: bool,
    label_opt: Option<field_descriptor_proto::Label>,
    type_opt: Option<field_descriptor_proto::Type>,
    number: i32,
    type_name: String,
}

impl Field for FieldImpl {
    fn cache(&self) -> &AnonymousCache {
        &self.cache
    }
    fn name(&self) -> Result<&str> {
        Ok(&self.name)
    }
    fn message(&self) -> Result<Rc<dyn Message>> {
        Ok(self.message.try_upgrade()?)
    }
    fn number(&self) -> i32 {
        self.number
    }
    fn rule(&self) -> Result<FieldRule> {
        self.rule
            .get_or_try_init(|| {
                let syntax = self.message()?.input_file()?.syntax()?;
                Ok(FieldRule::try_new(
                    self.label_opt.clone(),
                    syntax,
                    self.proto3_optional,
                )?)
            })
            .cloned()
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
}

impl FieldImpl {
    pub fn new(proto: &FieldDescriptorProto, message: Weak<dyn Message>) -> Rc<Self> {
        Rc::new(FieldImpl {
            cache: Default::default(),
            name: proto.name().to_string(),
            message,
            rule: OnceCell::new(),
            r#type: OnceCell::new(),
            proto3_optional: proto.proto3_optional(),
            label_opt: proto.label_opt(),
            type_opt: proto.type_opt(),
            number: proto.number(),
            type_name: proto.type_name().to_string(),
        })
    }
}
