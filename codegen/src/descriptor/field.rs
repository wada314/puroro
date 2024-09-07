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
use puroro::google::protobuf;
use std::cell::OnceCell;
use std::fmt::Debug;

use super::*;

#[derive(Debug)]
pub struct FieldDescriptorExt<'a> {
    message: &'a DescriptorExt<'a>,
    base: &'a protobuf::FieldDescriptorProto,
    cache: FieldDescriptorCache<'a>,
}

#[derive(Default, Debug)]
pub struct FieldDescriptorCache<'a> {
    full_name: OnceCell<ProtoPathBuf>,
    r#type: OnceCell<FieldType<&'a DescriptorExt<'a>, &'a EnumDescriptorExt<'a>>>,
}

impl<'a> FieldDescriptorExt<'a> {
    pub fn new(base: &'a protobuf::FieldDescriptorProto, message: &'a DescriptorExt<'a>) -> Self {
        Self {
            message,
            base,
            cache: Default::default(),
        }
    }
    pub fn name(&self) -> &str {
        debug_assert!(self.base.name().is_some() && !self.base.name().unwrap().is_empty());
        self.base.name().unwrap_or_default()
    }
    pub fn number(&self) -> i32 {
        debug_assert!(self.base.number().is_some() && self.base.number().unwrap() > 0);
        self.base.number().unwrap_or_default()
    }
    pub fn type_case(&self) -> Option<FieldTypeCase> {
        self.base.type_().map(Into::into)
    }
    pub fn type_name(&self) -> Option<&str> {
        self.base.type_name()
    }
    pub fn label(&self) -> Option<FieldLabel> {
        self.base.label().map(Into::into)
    }
    pub fn oneof_index(&self) -> Option<i32> {
        self.base.oneof_index()
    }
    pub fn is_proto3_optional(&self) -> bool {
        self.base.proto3_optional().unwrap_or_default()
    }
    pub fn full_name(&self) -> &str {
        self.cache
            .full_name
            .get_or_init(|| {
                let mut full_name = self.message.full_path().to_owned();
                full_name.push(ProtoPath::new(&format!(".{}", self.name())));
                full_name
            })
            .as_str()
    }
    pub fn r#type(&self) -> Result<FieldType<&'a DescriptorExt<'a>, &'a EnumDescriptorExt<'a>>> {
        self.cache
            .r#type
            .get_or_try_init(|| {
                self.type_case().unwrap_or_default().with_type_ref(
                    self.type_name().unwrap_or_default(),
                    |name| {
                        Ok(self
                            .message
                            .root()
                            .resolve_relative_path(&name, self.message.full_path())?
                            .maybe_message()
                            .ok_or_else(|| format!("Not a message: {}", name))?)
                    },
                    |name| {
                        Ok(self
                            .message
                            .root()
                            .resolve_relative_path(&name, self.message.full_path())?
                            .maybe_enum()
                            .ok_or_else(|| format!("Not an enum: {}", name))?)
                    },
                )
            })
            .cloned()
    }
    pub fn type_with_full_path(&self) -> Result<FieldType<ProtoPathBuf, ProtoPathBuf>> {
        Ok(self.r#type()?.try_map(
            |m| Ok(m.full_path().to_owned()),
            |e| Ok(e.full_path().to_owned()),
        )?)
    }
}

#[derive(Debug)]
pub struct OneofDescriptorExt<'a> {
    message: &'a DescriptorExt<'a>,
    base: &'a protobuf::OneofDescriptorProto,
    #[allow(unused)]
    cache: OneofDescriptorCache,
}

#[derive(Default, Debug)]
pub struct OneofDescriptorCache {
    index_in_oneofs: OnceCell<i32>,
    is_synthetic: OnceCell<bool>,
}

impl<'a> OneofDescriptorExt<'a> {
    pub fn new(base: &'a protobuf::OneofDescriptorProto, message: &'a DescriptorExt<'a>) -> Self {
        Self {
            message,
            base,
            cache: Default::default(),
        }
    }
    pub fn name(&self) -> &str {
        debug_assert!(self.base.name().is_some() && !self.base.name().unwrap().is_empty());
        self.base.name().unwrap_or_default()
    }

    pub fn is_synthetic(&'a self) -> Result<bool> {
        self.cache
            .is_synthetic
            .get_or_try_init(|| {
                let index = self.index_in_oneofs()?;
                let fields = self
                    .message
                    .all_fields()
                    .filter(|f| f.oneof_index() == Some(index))
                    .collect::<Vec<_>>();
                if let Some(first) = fields.first() {
                    if fields.len() == 1 && first.is_proto3_optional() {
                        return Ok(true);
                    }
                }
                Ok(false)
            })
            .copied()
    }

    pub fn index_in_oneofs(&'a self) -> Result<i32> {
        self.cache
            .index_in_oneofs
            .get_or_try_init(|| {
                let index = self
                    .message
                    .all_oneofs()
                    .position(|o| o.name() == self.name())
                    .ok_or_else(|| format!("Oneof not found: {}", self.name()))?;
                Ok(index as i32)
            })
            .copied()
    }
}
