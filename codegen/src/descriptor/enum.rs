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

use crate::proto_path::{ProtoPath, ProtoPathBuf};
use crate::Result;
use ::puroro::google::protobuf;
use ::std::cell::OnceCell;
use ::std::fmt::Debug;

use super::*;

#[derive(Debug)]
pub struct EnumDescriptor<'a> {
    file: &'a FileDescriptor<'a>,
    maybe_containing: Option<&'a Descriptor<'a>>,
    base: &'a protobuf::EnumDescriptorProto,
    cache: EnumDescriptorCache<'a>,
}

#[derive(Default, Debug)]
pub struct EnumDescriptorCache<'a> {
    full_path: OnceCell<ProtoPathBuf>,
    values: OnceCell<Vec<EnumValueDescriptor<'a>>>,
}

impl<'a> EnumDescriptor<'a> {
    pub fn new(
        file: &'a FileDescriptor<'a>,
        maybe_containing: Option<&'a Descriptor<'a>>,
        base: &'a protobuf::EnumDescriptorProto,
    ) -> Self {
        Self {
            file,
            maybe_containing,
            base,
            cache: Default::default(),
        }
    }
    pub fn name(&self) -> Option<&str> {
        self.base.name()
    }
    pub fn full_path(&self) -> Result<&ProtoPath> {
        self.cache
            .full_path
            .get_or_try_init(|| {
                let mut full_path = if let Some(nested) = self.maybe_containing {
                    nested.full_path()?.to_owned()
                } else {
                    self.file.absolute_package()?.to_owned()
                };
                full_path.push(&self.name().unwrap_or_default());
                Ok(full_path)
            })
            .map(|s| s.as_ref())
    }
    pub fn file(&self) -> Result<&FileDescriptor<'a>> {
        Ok(self.file)
    }
    pub fn values(&'a self) -> Result<impl Iterator<Item = &'a EnumValueDescriptor<'a>>> {
        Ok(self
            .cache
            .values
            .get_or_init(|| {
                self.base
                    .value()
                    .map(|v| EnumValueDescriptor::new(self, v))
                    .collect()
            })
            .iter())
    }
}

#[derive(Debug)]
pub struct EnumValueDescriptor<'a> {
    enum_: &'a EnumDescriptor<'a>,
    base: &'a protobuf::EnumValueDescriptorProto,
    cache: EnumValueDescriptorCache,
}
#[derive(Default, Debug)]
pub struct EnumValueDescriptorCache {
    full_name: OnceCell<ProtoPathBuf>,
}
impl<'a> EnumValueDescriptor<'a> {
    fn new(enum_: &'a EnumDescriptor<'a>, base: &'a protobuf::EnumValueDescriptorProto) -> Self {
        Self {
            enum_,
            base,
            cache: Default::default(),
        }
    }
    pub fn name(&self) -> Option<&str> {
        self.base.name()
    }
    pub fn full_name(&self) -> Result<&ProtoPath> {
        self.cache
            .full_name
            .get_or_try_init(|| {
                // This full_name is a sibling of EnumDescriptor, not a child.
                let mut full_name = if let Some(m) = self.enum_.maybe_containing {
                    m.full_path()?.to_owned()
                } else {
                    self.enum_
                        .file
                        .package()
                        .map_or_else(ProtoPathBuf::new, |p| p.to_owned())
                };
                full_name.push(ProtoPath::new(&self.enum_.name().unwrap_or_default()));
                Ok(full_name)
            })
            .map(|s| s.as_ref())
    }
    pub fn number(&self) -> Option<i32> {
        self.base.number()
    }
}
