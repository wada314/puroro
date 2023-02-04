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
use super::{
    DataTypeBase, InputFile, PackageOrMessage, PackageOrMessageCase, Syntax,
    ENUM_FIELD_NUMBER_IN_FILE_DESCRIPTOR, ENUM_FIELD_NUMBER_IN_MESSAGE_DESCRIPTOR,
    VALUE_FIELD_NUMBER_IN_ENUM_DESCRIPTOR,
};
use crate::Result;
use ::puroro::protobuf::google::protobuf::EnumDescriptorProto;
use ::std::fmt::Debug;
use ::std::rc::{Rc, Weak};

#[derive(Debug)]
pub(crate) struct Enum {
    cache: AnonymousCache,
    name: String,
    input_file: Weak<InputFile>,
    parent: Weak<dyn PackageOrMessage>,
    index_in_parent: usize,
    values: Vec<(String, i32)>,
}

impl Enum {
    pub(crate) fn new(
        proto: &EnumDescriptorProto,
        input_file: Weak<InputFile>,
        parent: Weak<dyn PackageOrMessage>,
        index_in_parent: usize,
    ) -> Rc<Self> {
        let values = proto
            .value()
            .into_iter()
            .map(|v| (v.name().to_string(), v.number()))
            .collect::<Vec<_>>();
        Rc::new(Enum {
            cache: Default::default(),
            name: proto.name().to_string(),
            input_file,
            parent,
            index_in_parent,
            values,
        })
    }

    pub(crate) fn values(&self) -> Result<Box<dyn '_ + Iterator<Item = (&str, i32)>>> {
        Ok(Box::new(self.values.iter().map(|(s, n)| (s.as_str(), *n))))
    }
    pub(crate) fn input_file(&self) -> Result<Rc<InputFile>> {
        Ok(self.input_file.try_upgrade()?)
    }
    pub(crate) fn parent(&self) -> Result<Rc<dyn PackageOrMessage>> {
        Ok(self.parent.try_upgrade()?)
    }
    pub(crate) fn syntax(&self) -> Result<Syntax> {
        Ok(self.input_file.try_upgrade()?.syntax()?)
    }
    pub(crate) fn location_path(&self) -> Result<Box<dyn Iterator<Item = i32>>> {
        Ok(match self.parent()?.either() {
            PackageOrMessageCase::Package(_) => {
                // This message is a direct item under the input file.
                let this_path = [
                    ENUM_FIELD_NUMBER_IN_FILE_DESCRIPTOR,
                    self.index_in_parent.try_into()?,
                ];
                Box::new(this_path.into_iter())
            }
            PackageOrMessageCase::Message(m) => {
                let parent_path = m.location_path()?;
                let this_path = [
                    ENUM_FIELD_NUMBER_IN_MESSAGE_DESCRIPTOR,
                    self.index_in_parent.try_into()?,
                ];
                Box::new(parent_path.chain(this_path.into_iter()))
            }
        })
    }
    pub(crate) fn value_location_path(&self, index: usize) -> Result<impl Iterator<Item = i32>> {
        let this_path = [VALUE_FIELD_NUMBER_IN_ENUM_DESCRIPTOR, index.try_into()?];
        Ok(self.location_path()?.chain(this_path.into_iter()))
    }
}

impl DataTypeBase for Enum {
    fn cache(&self) -> &AnonymousCache {
        &self.cache
    }
    fn name(&self) -> Result<&str> {
        Ok(&self.name)
    }
}
