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
use puroro::google::protobuf::{
    field_descriptor_proto::Label as FieldLabelProto, FieldDescriptorProto, OneofDescriptorProto,
};
use std::cell::OnceCell;
use std::fmt::Debug;
use std::ops::Deref;

use super::*;

#[derive(Debug, Clone)]
pub struct FieldDescriptorBase {
    name: String,
    number: i32,
    type_case: FieldTypeCase,
    type_name: Option<String>,
    label: Option<FieldLabel>,
    oneof_index: Option<usize>,
    proto3_optional: bool,
}

impl FieldDescriptorBase {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn number(&self) -> i32 {
        self.number
    }
    pub fn type_case(&self) -> FieldTypeCase {
        self.type_case
    }
    pub fn type_name(&self) -> Option<&str> {
        self.type_name.as_deref()
    }
    pub fn label(&self) -> Option<FieldLabel> {
        self.label
    }
    pub fn oneof_index(&self) -> Option<usize> {
        self.oneof_index
    }
    pub fn is_proto3_optional(&self) -> bool {
        self.proto3_optional
    }
}

impl<'a> TryFrom<FieldDescriptorProto<'a>> for FieldDescriptorBase {
    type Error = ErrorKind;
    fn try_from(proto: FieldDescriptorProto) -> Result<Self> {
        Ok(Self {
            name: proto.name()?.try_into_string("No FieldDescriptor name")?,
            number: proto
                .number()?
                .try_into_number("No FieldDescriptor number")?,
            type_case: proto
                .type_()?
                .ok_or_else(|| format!("No FieldDescriptor type"))?
                .into(),
            type_name: proto.type_name()?.map(str::to_string),
            label: proto.label()?.map(FieldLabelProto::into),
            oneof_index: proto.oneof_index()?.map(|i| i.try_into()).transpose()?,
            proto3_optional: proto.proto3_optional()?.unwrap_or(false),
        })
    }
}

#[derive(Debug)]
pub struct FieldDescriptor<'a> {
    message: &'a Descriptor<'a>,
    base: &'a FieldDescriptorBase,
    cache: FieldDescriptorCache<'a>,
}

#[derive(Default, Debug)]
pub struct FieldDescriptorCache<'a> {
    full_name: OnceCell<ProtoPathBuf>,
    r#type: OnceCell<FieldType<&'a Descriptor<'a>, &'a EnumDescriptor<'a>>>,
}

impl<'a> FieldDescriptor<'a> {
    pub fn new(base: &'a FieldDescriptorBase, message: &'a Descriptor<'a>) -> Self {
        Self {
            message,
            base,
            cache: Default::default(),
        }
    }
    pub fn name(&self) -> Result<&str> {
        Ok(&self.base.name)
    }
    pub fn full_name(&self) -> Result<&str> {
        self.cache
            .full_name
            .get_or_try_init(|| {
                let mut full_name = self.message.full_path()?.to_owned();
                full_name.push(ProtoPath::new(&format!(".{}", self.base.name)));
                Ok(full_name)
            })
            .map(|s| s.as_ref())
    }
    pub fn r#type(
        &self,
    ) -> Result<FieldType<&'a Descriptor<'a>, &'a EnumDescriptor<'a>>> {
        let init = || {
            self.base.type_case.with_type_ref(
                self.base.type_name.as_deref(),
                |name| {
                    Ok(self
                        .message
                        .root()
                        .resolve_relative_path(&name, self.message.full_path()?)?
                        .maybe_message()
                        .ok_or_else(|| format!("Not a message: {}", name))?)
                },
                |name| {
                    Ok(self
                        .message
                        .root()
                        .resolve_relative_path(&name, self.message.full_path()?)?
                        .maybe_enum()
                        .ok_or_else(|| format!("Not an enum: {}", name))?)
                },
            )
        };
        self.cache.r#type.get_or_try_init(init).cloned()
    }
    pub fn type_with_full_path(&self) -> Result<FieldType<ProtoPathBuf, ProtoPathBuf>> {
        Ok(self.r#type()?.try_map(
            |m| Ok(m.full_path()?.to_owned()),
            |e| Ok(e.full_path()?.to_owned()),
        )?)
    }
}

impl Deref for FieldDescriptor<'_> {
    type Target = FieldDescriptorBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

#[derive(Debug, Clone)]
pub struct OneofDescriptorBase {
    #[allow(unused)]
    name: String,
}

impl<'a> TryFrom<OneofDescriptorProto<'a>> for OneofDescriptorBase {
    type Error = ErrorKind;
    fn try_from(proto: OneofDescriptorProto) -> Result<Self> {
        Ok(Self {
            name: proto.name()?.try_into_string("No OneofDescriptor name")?,
        })
    }
}

#[derive(Debug)]
pub struct OneofDescriptor<'a> {
    #[allow(unused)]
    message: &'a Descriptor<'a>,
    #[allow(unused)]
    body: &'a OneofDescriptorBase,
    #[allow(unused)]
    cache: OneofDescriptorCache,
}

#[derive(Default, Debug)]
pub struct OneofDescriptorCache {}

impl<'a> OneofDescriptor<'a> {
    pub fn new(body: &'a OneofDescriptorBase, message: &'a Descriptor<'a>) -> Self {
        Self {
            message,
            body,
            cache: Default::default(),
        }
    }
}
