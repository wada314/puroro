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
use crate::{ErrorKind, Result};
use ::itertools::Itertools;
use ::puroro::google::protobuf::{EnumDescriptorProto, EnumValueDescriptorProto};
use ::puroro::Result as PResult;
use ::std::cell::OnceCell;
use ::std::fmt::Debug;

use super::*;

#[derive(Debug, Clone)]
pub struct EnumDescriptorBase {
    name: String,
    values: Vec<EnumValueDescriptorBase>,
}

impl TryFrom<&EnumDescriptorProto> for EnumDescriptorBase {
    type Error = ErrorKind;
    fn try_from(proto: &EnumDescriptorProto) -> Result<Self> {
        Ok(Self {
            name: proto.name()?.try_into_string("No EnumDescriptor name")?,
            values: proto
                .value()
                .into_iter()
                .map_ok(TryInto::try_into)
                .collect::<PResult<Result<Vec<_>>>>()??,
        })
    }
}

#[cfg(test)]
#[derive(Default)]
pub struct DebugEnumDescriptor<'a> {
    pub name: &'a str,
    pub values: Vec<DebugEnumValueDescriptor<'a>>,
}
#[cfg(test)]
impl From<DebugEnumDescriptor<'_>> for EnumDescriptorBase {
    fn from(debug: DebugEnumDescriptor) -> Self {
        Self {
            name: debug.name.to_string(),
            values: debug.values.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug)]
pub struct EnumDescriptor<'a> {
    file: &'a FileDescriptor<'a>,
    maybe_containing: Option<&'a Descriptor<'a>>,
    base: &'a EnumDescriptorBase,
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
        base: &'a EnumDescriptorBase,
    ) -> Self {
        Self {
            file,
            maybe_containing,
            base,
            cache: Default::default(),
        }
    }
    pub fn name(&'a self) -> &str {
        self.base.name.as_ref()
    }
    pub fn full_path(&'a self) -> Result<&ProtoPath> {
        self.cache
            .full_path
            .get_or_try_init(|| {
                let mut full_path = if let Some(nested) = self.maybe_containing {
                    nested.full_path()?.to_owned()
                } else {
                    self.file.absolute_package()?.to_owned()
                };
                full_path.push(&self.base.name);
                Ok(full_path)
            })
            .map(|s| s.as_ref())
    }
    pub fn file(&'a self) -> Result<&FileDescriptor> {
        Ok(self.file)
    }
    pub fn values(&'a self) -> Result<impl Iterator<Item = &EnumValueDescriptor>> {
        Ok(self
            .cache
            .values
            .get_or_init(|| {
                self.base
                    .values
                    .iter()
                    .map(|v| EnumValueDescriptor::new(self, v))
                    .collect()
            })
            .iter())
    }
}

#[derive(Debug, Clone)]
pub struct EnumValueDescriptorBase {
    name: String,
    number: i32,
}

impl TryFrom<&EnumValueDescriptorProto> for EnumValueDescriptorBase {
    type Error = ErrorKind;
    fn try_from(proto: &EnumValueDescriptorProto) -> Result<Self> {
        Ok(Self {
            name: proto
                .name()?
                .try_into_string("No EnumValueDescriptor name")?,
            number: proto
                .number()?
                .try_into_number("No EnumValueDescriptor number")?,
        })
    }
}

#[cfg(test)]
#[derive(Default)]
pub struct DebugEnumValueDescriptor<'a> {
    pub name: &'a str,
    pub number: i32,
}
#[cfg(test)]
impl From<DebugEnumValueDescriptor<'_>> for EnumValueDescriptorBase {
    fn from(debug: DebugEnumValueDescriptor) -> Self {
        Self {
            name: debug.name.to_string(),
            number: debug.number,
        }
    }
}

#[derive(Debug)]
pub struct EnumValueDescriptor<'a> {
    enum_: &'a EnumDescriptor<'a>,
    base: &'a EnumValueDescriptorBase,
    cache: EnumValueDescriptorCache,
}
#[derive(Default, Debug)]
pub struct EnumValueDescriptorCache {
    full_name: OnceCell<ProtoPathBuf>,
}
impl<'a> EnumValueDescriptor<'a> {
    fn new(enum_: &'a EnumDescriptor<'a>, base: &'a EnumValueDescriptorBase) -> Self {
        Self {
            enum_,
            base,
            cache: Default::default(),
        }
    }
    pub fn name(&'a self) -> &str {
        &self.base.name
    }
    pub fn full_name(&'a self) -> Result<&ProtoPath> {
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
                full_name.push(ProtoPath::new(&self.enum_.name()));
                Ok(full_name)
            })
            .map(|s| s.as_ref())
    }
    pub fn number(&self) -> i32 {
        self.base.number
    }
}
