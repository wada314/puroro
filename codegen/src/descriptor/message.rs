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
use itertools::Itertools;
use puroro::google::protobuf::DescriptorProto;
use puroro::Result as PResult;
use std::cell::OnceCell;
use std::fmt::Debug;

use super::*;

// region: Descriptor

#[derive(Debug, Clone)]
pub struct DescriptorBase {
    name: String,
    fields: Vec<FieldDescriptorBase>,
    oneof_decls: Vec<OneofDescriptorBase>,
    nested_types: Vec<DescriptorBase>,
    enum_types: Vec<EnumDescriptorBase>,
}

impl TryFrom<&DescriptorProto> for DescriptorBase {
    type Error = ErrorKind;
    fn try_from(proto: &DescriptorProto) -> Result<Self> {
        Ok(Self {
            name: proto.name()?.try_into_string("No Descriptor name")?,
            fields: proto
                .field()
                .into_iter()
                .map_ok(TryInto::try_into)
                .collect::<PResult<Result<Vec<_>>>>()??,
            oneof_decls: proto
                .oneof_decl()
                .into_iter()
                .map_ok(TryInto::try_into)
                .collect::<PResult<Result<Vec<_>>>>()??,
            nested_types: proto
                .nested_type()
                .into_iter()
                .map_ok(TryInto::try_into)
                .collect::<PResult<Result<Vec<_>>>>()??,
            enum_types: proto
                .enum_type()
                .into_iter()
                .map_ok(TryInto::try_into)
                .collect::<PResult<Result<Vec<_>>>>()??,
        })
    }
}

#[cfg(test)]
#[derive(Default)]
pub struct DebugDescriptor<'a> {
    pub name: &'a str,
    pub nested_types: Vec<DebugDescriptor<'a>>,
    pub enum_types: Vec<DebugEnumDescriptor<'a>>,
}

#[cfg(test)]
impl From<DebugDescriptor<'_>> for DescriptorBase {
    fn from(debug: DebugDescriptor) -> Self {
        Self {
            name: debug.name.to_string(),
            fields: vec![],
            oneof_decls: vec![],
            nested_types: debug.nested_types.into_iter().map(Into::into).collect(),
            enum_types: debug.enum_types.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug)]
pub struct Descriptor<'a> {
    file: &'a FileDescriptor<'a>,
    maybe_containing: Option<&'a Descriptor<'a>>,
    base: &'a DescriptorBase,
    cache: DescriptorCache<'a>,
}

#[derive(Default, Debug)]
pub struct DescriptorCache<'a> {
    full_path: OnceCell<ProtoPathBuf>,
    all_fields: OnceCell<Vec<FieldDescriptor<'a>>>,
    all_oneofs: OnceCell<Vec<OneofDescriptor<'a>>>,
    nested_types: OnceCell<Vec<Descriptor<'a>>>,
    enum_types: OnceCell<Vec<EnumDescriptor<'a>>>,
}

impl<'a> Descriptor<'a> {
    pub fn new(
        file: &'a FileDescriptor<'a>,
        maybe_containing: Option<&'a Descriptor<'a>>,
        base: &'a DescriptorBase,
    ) -> Self {
        Self {
            file,
            maybe_containing,
            base,
            cache: Default::default(),
        }
    }
    pub fn file(&'a self) -> &FileDescriptor {
        self.file
    }
    pub fn root(&'a self) -> &RootContext {
        self.file().root()
    }
    pub fn name(&'a self) -> Result<&str> {
        Ok(&self.base.name)
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
                // todo!("absl path check?");
                full_path.push(&self.base.name);
                Ok(full_path)
            })
            .map(|s| s.as_ref())
    }
    pub fn all_fields(&'a self) -> impl Iterator<Item = &FieldDescriptor> {
        self.cache
            .all_fields
            .get_or_init(|| {
                self.base
                    .fields
                    .iter()
                    .map(|f| FieldDescriptor::new(f, self))
                    .collect()
            })
            .iter()
    }
    pub fn all_oneofs(&'a self) -> impl Iterator<Item = &OneofDescriptor> {
        self.cache
            .all_oneofs
            .get_or_init(|| {
                self.base
                    .oneof_decls
                    .iter()
                    .map(|o| OneofDescriptor::new(o, self))
                    .collect()
            })
            .iter()
    }
    pub fn filtered_fields(
        &'a self,
        f: impl 'a + Fn(&FieldDescriptor) -> bool,
    ) -> Result<impl Iterator<Item = &FieldDescriptor>> {
        Ok(self.all_fields().filter(move |field| f(*field)))
    }
    pub fn non_oneof_fields(&'a self) -> Result<impl Iterator<Item = &FieldDescriptor>> {
        Ok(self
            .all_fields()
            .filter(|f| f.oneof_index().is_none() || f.is_proto3_optional()))
    }
    pub fn real_oneof_fields(&'a self) -> Result<impl Iterator<Item = &FieldDescriptor>> {
        Ok(self
            .all_fields()
            .filter(|f| f.oneof_index().is_some() && !f.is_proto3_optional()))
    }
    pub fn real_oneofs(&'a self) -> Result<impl Iterator<Item = &OneofDescriptor>> {
        Ok(self
            .all_oneofs()
            .filter(|o| o.is_synthetic().is_ok_and(|b| !b)))
    }
    pub fn synthetic_oneofs(&'a self) -> Result<impl Iterator<Item = &'a OneofDescriptor>> {
        Ok(self
            .all_oneofs()
            .filter(|o| o.is_synthetic().is_ok_and(|b| b)))
    }
    pub fn nested_types(&'a self) -> impl Iterator<Item = &Descriptor> {
        self.cache
            .nested_types
            .get_or_init(|| {
                self.base
                    .nested_types
                    .iter()
                    .map(|m| Descriptor::new(self.file, Some(self), m))
                    .collect()
            })
            .iter()
    }
    pub fn enum_types(&'a self) -> impl Iterator<Item = &EnumDescriptor> {
        self.cache
            .enum_types
            .get_or_init(|| {
                self.base
                    .enum_types
                    .iter()
                    .map(|e| EnumDescriptor::new(self.file(), Some(self), e))
                    .collect()
            })
            .iter()
    }
    pub fn all_descendant_messages_or_enums(
        &'a self,
    ) -> impl Iterator<Item = MessageOrEnum<&Descriptor, &EnumDescriptor>> {
        self.all_descendant_messages()
            .map(MessageOrEnum::Message)
            .chain(self.all_descendant_enums().map(MessageOrEnum::Enum))
    }
    pub fn all_descendant_messages(&'a self) -> impl Iterator<Item = &Descriptor> {
        let direct_messages = self.nested_types();
        let indirect_messages_vec = self
            .nested_types()
            .map(|child| child.all_descendant_messages())
            .collect::<Vec<_>>();
        let indirect_messages = indirect_messages_vec
            .into_iter()
            .flat_map(|v| v.into_iter());
        Box::new(direct_messages.chain(indirect_messages)) as Box<dyn Iterator<Item = _>>
    }
    pub fn all_descendant_enums(&'a self) -> impl Iterator<Item = &EnumDescriptor> {
        let direct_enums = self.enum_types();
        let indirect_enums_vec = self
            .nested_types()
            .map(|child| child.all_descendant_enums())
            .collect::<Vec<_>>();
        let indirect_enums = indirect_enums_vec.into_iter().flat_map(|v| v.into_iter());
        Box::new(direct_enums.chain(indirect_enums)) as Box<dyn Iterator<Item = _>>
    }
}
