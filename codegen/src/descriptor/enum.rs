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
use ::itertools::{Either, Itertools};
use ::puroro::google::protobuf::{
    field_descriptor_proto::Label as FieldLabelProto,
    field_descriptor_proto::Type as FieldTypeProto, DescriptorProto, Edition as EditionProto,
    EnumDescriptorProto, EnumValueDescriptorProto, FieldDescriptorProto, FileDescriptorProto,
    FileDescriptorSet, OneofDescriptorProto,
};
use ::puroro::Result as PResult;
use ::std::cell::OnceCell;
use ::std::collections::HashMap;
use ::std::fmt::Debug;
use ::std::ops::Deref;

use super::*;

#[derive(Debug, Clone)]
pub struct EnumDescriptor {
    name: String,
    values: Vec<EnumValueDescriptor>,
}

impl<'a> TryFrom<EnumDescriptorProto<'a>> for EnumDescriptor {
    type Error = ErrorKind;
    fn try_from(proto: EnumDescriptorProto) -> Result<Self> {
        Ok(Self {
            name: proto.name()?.try_into_string("No EnumDescriptor name")?,
            values: proto
                .value()
                .into_iter()
                .map_ok(EnumValueDescriptor::try_from)
                .collect::<PResult<Result<Vec<_>>>>()??,
        })
    }
}

#[derive(Debug)]
pub struct EnumDescriptorWithContext<'a> {
    file: &'a FileDescriptorWithContext<'a>,
    maybe_containing: Option<&'a DescriptorWithContext<'a>>,
    body: &'a EnumDescriptor,
    cache: EnumDescriptorCache<'a>,
}

#[derive(Default, Debug)]
pub struct EnumDescriptorCache<'a> {
    full_path: OnceCell<ProtoPathBuf>,
    values: OnceCell<Vec<EnumValueDescriptorWithContext<'a>>>,
}

impl<'a> EnumDescriptorWithContext<'a> {
    pub fn name(&self) -> Result<&str> {
        Ok(self.body.name.as_ref())
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
                full_path.push(&self.body.name);
                Ok(full_path)
            })
            .map(|s| s.as_ref())
    }
    pub fn file(&'a self) -> Result<&FileDescriptorWithContext> {
        Ok(self.file)
    }
    pub fn values(
        &'a self,
    ) -> Result<impl 'a + IntoIterator<Item = &EnumValueDescriptorWithContext>> {
        self.cache.values.get_or_try_init(|| {
            self.body
                .values
                .iter()
                .map(|v| {
                    Ok(EnumValueDescriptorWithContext {
                        enum_: self,
                        body: v,
                        cache: Default::default(),
                    })
                })
                .collect()
        })
    }
}

#[derive(Debug, Clone)]
pub struct EnumValueDescriptor {
    name: String,
    number: i32,
}

impl<'a> TryFrom<EnumValueDescriptorProto<'a>> for EnumValueDescriptor {
    type Error = ErrorKind;
    fn try_from(proto: EnumValueDescriptorProto) -> Result<Self> {
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

#[derive(Debug)]
pub struct EnumValueDescriptorWithContext<'a> {
    enum_: &'a EnumDescriptorWithContext<'a>,
    body: &'a EnumValueDescriptor,
    cache: EnumValueDescriptorCache,
}
#[derive(Default, Debug)]
pub struct EnumValueDescriptorCache {
    full_name: OnceCell<ProtoPathBuf>,
}
impl<'a> EnumValueDescriptorWithContext<'a> {
    pub fn name(&self) -> Result<&str> {
        Ok(self.body.name.as_ref())
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
                        .package()?
                        .map_or_else(ProtoPathBuf::new, |p| p.to_owned())
                };
                full_name.push(ProtoPath::new(&self.enum_.name()?));
                Ok(full_name)
            })
            .map(|s| s.as_ref())
    }
    pub fn number(&self) -> Result<i32> {
        Ok(self.body.number)
    }
}
