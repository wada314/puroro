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

//! For the enum details, see the official documents:
//!  - [proto2 document](https://developers.google.com/protocol-buffers/docs/proto#enum)
//!  - [proto3 document](https://developers.google.com/protocol-buffers/docs/proto3#enum)
//!  - [c++ generated code](https://developers.google.com/protocol-buffers/docs/reference/cpp-generated#enum)

use super::super::util::*;
use super::super::{InputFile, PackageOrMessage, Syntax};
use crate::Result;
use ::puroro_protobuf_compiled::google::protobuf::EnumDescriptorProto;
use ::std::fmt::Debug;
use ::std::rc::{Rc, Weak};

pub trait Enum: Debug {
    fn cache(&self) -> &AnonymousCache;
    fn name(&self) -> &str;
    fn values(&self) -> Result<Box<dyn '_ + Iterator<Item = (&str, i32)>>>;
    fn parent(&self) -> Result<Rc<dyn PackageOrMessage>>;
    fn syntax(&self) -> Result<Syntax>;
}

#[derive(Debug)]
pub struct EnumImpl {
    cache: AnonymousCache,
    name: String,
    input_file: Weak<dyn InputFile>,
    parent: Weak<dyn PackageOrMessage>,
    values: Vec<(String, i32)>,
}

impl EnumImpl {
    pub fn new(
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
            cache: Default::default(),
            name: proto.name().to_string(),
            input_file,
            parent,
            values,
        })
    }
}

impl Enum for EnumImpl {
    fn cache(&self) -> &AnonymousCache {
        &self.cache
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn values(&self) -> Result<Box<dyn '_ + Iterator<Item = (&str, i32)>>> {
        Ok(Box::new(self.values.iter().map(|(s, n)| (s.as_str(), *n))))
    }
    fn parent(&self) -> Result<Rc<dyn PackageOrMessage>> {
        Ok(self.parent.try_upgrade()?)
    }
    fn syntax(&self) -> Result<Syntax> {
        Ok(self.input_file.try_upgrade()?.syntax()?)
    }
}
