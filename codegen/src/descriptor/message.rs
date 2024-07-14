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
use itertools::{Either, Itertools};
use puroro::google::protobuf::{
    field_descriptor_proto::Label as FieldLabelProto,
    field_descriptor_proto::Type as FieldTypeProto, DescriptorProto, Edition as EditionProto,
    EnumDescriptorProto, EnumValueDescriptorProto, FieldDescriptorProto, FileDescriptorProto,
    FileDescriptorSet, OneofDescriptorProto,
};
use puroro::Result as PResult;
use std::cell::OnceCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Deref;

use super::*;

// region: Descriptor

#[derive(Debug, Clone)]
pub struct Descriptor {
    name: String,
    fields: Vec<FieldDescriptor>,
    oneof_decls: Vec<OneofDescriptor>,
    nested_types: Vec<Descriptor>,
    enum_types: Vec<EnumDescriptor>,
}

impl<'a> TryFrom<DescriptorProto<'a>> for Descriptor {
    type Error = ErrorKind;
    fn try_from(proto: DescriptorProto) -> Result<Self> {
        Ok(Self {
            name: proto.name()?.try_into_string("No Descriptor name")?,
            fields: proto
                .field()
                .into_iter()
                .map_ok(FieldDescriptor::try_from)
                .collect::<PResult<Result<Vec<_>>>>()??,
            oneof_decls: proto
                .oneof_decl()
                .into_iter()
                .map_ok(OneofDescriptor::try_from)
                .collect::<PResult<Result<Vec<_>>>>()??,
            nested_types: proto
                .nested_type()
                .into_iter()
                .map_ok(Descriptor::try_from)
                .collect::<PResult<Result<Vec<_>>>>()??,
            enum_types: proto
                .enum_type()
                .into_iter()
                .map_ok(EnumDescriptor::try_from)
                .collect::<PResult<Result<Vec<_>>>>()??,
        })
    }
}

// endregion:
// region: DescriptorWithContext

#[derive(Debug)]
pub struct DescriptorWithContext<'a> {
    file: &'a FileDescriptorWithContext<'a>,
    maybe_containing: Option<&'a DescriptorWithContext<'a>>,
    body: &'a Descriptor,
    cache: DescriptorCache<'a>,
}

#[derive(Default, Debug)]
pub struct DescriptorCache<'a> {
    full_path: OnceCell<ProtoPathBuf>,
    all_fields: OnceCell<Vec<FieldDescriptorWithContext<'a>>>,
    non_oneof_fields: OnceCell<Vec<FieldDescriptorWithContext<'a>>>,
    real_oneof_fields: OnceCell<Vec<FieldDescriptorWithContext<'a>>>,
    synthetic_oneof_fields: OnceCell<Vec<FieldDescriptorWithContext<'a>>>,
    real_and_synthetic_oneofs: OnceCell<(
        Vec<OneofDescriptorWithContext<'a>>,
        Vec<OneofDescriptorWithContext<'a>>,
    )>,
    real_oneofs: OnceCell<Vec<OneofDescriptorWithContext<'a>>>,
    synthetic_oneofs: OnceCell<Vec<OneofDescriptorWithContext<'a>>>,
    nested_types: OnceCell<Vec<DescriptorWithContext<'a>>>,
    enum_types: OnceCell<Vec<EnumDescriptorWithContext<'a>>>,
}

impl<'a> DescriptorWithContext<'a> {
    pub fn new(
        file: &'a FileDescriptorWithContext<'a>,
        maybe_containing: Option<&'a DescriptorWithContext<'a>>,
        body: &'a Descriptor,
    ) -> Self {
        Self {
            file,
            maybe_containing,
            body,
            cache: Default::default(),
        }
    }
    pub fn file(&self) -> &'a FileDescriptorWithContext<'a> {
        self.file
    }
    pub fn root(&self) -> &'a RootContext<'a> {
        self.file().root()
    }
    pub fn name(&self) -> Result<&str> {
        Ok(&self.body.name)
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
                // todo!("absl path check?");
                full_path.push(&self.body.name);
                Ok(full_path)
            })
            .map(|s| s.as_ref())
    }
    pub fn all_fields(
        &'a self,
    ) -> Result<impl 'a + IntoIterator<Item = &FieldDescriptorWithContext>> {
        self.cache.all_fields.get_or_try_init(|| {
            self.body
                .fields
                .iter()
                .map(|f| Ok(FieldDescriptorWithContext::new(f, self)))
                .collect()
        })
    }
    pub fn filtered_fields(
        &'a self,
        f: impl Fn(&FieldDescriptor) -> bool,
    ) -> Result<Vec<FieldDescriptorWithContext<'a>>> {
        self.body
            .fields
            .iter()
            .filter(|field| f(field))
            .map(|f| Ok(FieldDescriptorWithContext::new(f, self)))
            .collect()
    }
    pub fn non_oneof_fields(
        &'a self,
    ) -> Result<impl 'a + IntoIterator<Item = &FieldDescriptorWithContext>> {
        self.cache
            .non_oneof_fields
            .get_or_try_init(|| self.filtered_fields(|f| f.oneof_index().is_none()))
    }
    pub fn real_oneof_fields(
        &'a self,
    ) -> Result<impl 'a + IntoIterator<Item = &FieldDescriptorWithContext>> {
        self.cache.real_oneof_fields.get_or_try_init(|| {
            self.filtered_fields(|f| f.oneof_index().is_some() && !f.is_proto3_optional())
        })
    }
    pub fn synthetic_oneof_fields(
        &'a self,
    ) -> Result<impl 'a + IntoIterator<Item = &FieldDescriptorWithContext>> {
        self.cache.synthetic_oneof_fields.get_or_try_init(|| {
            self.filtered_fields(|f| f.oneof_index().is_some() && f.is_proto3_optional())
        })
    }
    pub fn all_oneofs(
        &'a self,
    ) -> Result<impl 'a + IntoIterator<Item = &OneofDescriptorWithContext>> {
        Ok(self
            .real_oneofs()?
            .into_iter()
            .chain(self.synthetic_oneofs()?.into_iter()))
    }
    pub fn real_and_synthetic_oneofs(
        &'a self,
    ) -> Result<(
        impl IntoIterator<Item = &'a OneofDescriptorWithContext<'a>>,
        impl IntoIterator<Item = &'a OneofDescriptorWithContext<'a>>,
    )> {
        let (real, synthetic) = self.cache.real_and_synthetic_oneofs.get_or_try_init(|| {
            let mut is_oneof_synthetic = vec![false; self.body.oneof_decls.len()];
            for f in self.all_fields()? {
                if let Some(i) = f.oneof_index() {
                    is_oneof_synthetic[i] |= f.is_proto3_optional();
                }
            }
            let (real, synthetic): (Vec<_>, Vec<_>) = self
                .body
                .oneof_decls
                .iter()
                .enumerate()
                .partition_map(|(i, o)| {
                    let oneof = Ok(OneofDescriptorWithContext::new(o, self));
                    if is_oneof_synthetic[i] {
                        Either::Right(oneof)
                    } else {
                        Either::Left(oneof)
                    }
                });
            Result::Ok((
                real.into_iter().collect::<Result<_>>()?,
                synthetic.into_iter().collect::<Result<_>>()?,
            ))
        })?;
        Ok((real, synthetic))
    }
    pub fn real_oneofs(
        &'a self,
    ) -> Result<impl IntoIterator<Item = &'a OneofDescriptorWithContext>> {
        Ok(self.real_and_synthetic_oneofs()?.0)
    }
    pub fn synthetic_oneofs(
        &'a self,
    ) -> Result<impl IntoIterator<Item = &'a OneofDescriptorWithContext>> {
        Ok(self.real_and_synthetic_oneofs()?.1)
    }
    pub fn nested_types(&'a self) -> Result<impl 'a + IntoIterator<Item = &DescriptorWithContext>> {
        self.cache.nested_types.get_or_try_init(|| {
            self.body
                .nested_types
                .iter()
                .map(|m| {
                    Ok(DescriptorWithContext {
                        file: self.file,
                        maybe_containing: Some(self),
                        body: m,
                        cache: Default::default(),
                    })
                })
                .collect()
        })
    }
    pub fn enum_types(
        &'a self,
    ) -> Result<impl 'a + IntoIterator<Item = &EnumDescriptorWithContext>> {
        self.cache.enum_types.get_or_try_init(|| {
            self.body
                .enum_types
                .iter()
                .map(|e| Ok(EnumDescriptorWithContext::new(self.file(), Some(self), e)))
                .collect()
        })
    }
    pub fn all_messages_or_enums(
        &'a self,
    ) -> Result<
        impl 'a + IntoIterator<Item = MessageOrEnum<&DescriptorWithContext, &EnumDescriptorWithContext>>,
    > {
        Ok(self
            .all_messages()?
            .into_iter()
            .map(MessageOrEnum::Message)
            .chain(self.all_enums()?.into_iter().map(MessageOrEnum::Enum)))
    }
    pub fn all_messages(&'a self) -> Result<impl 'a + IntoIterator<Item = &DescriptorWithContext>> {
        let direct_messages = self.nested_types()?.into_iter();
        let indirect_messages_vec = self
            .nested_types()?
            .into_iter()
            .map(|child| child.all_messages())
            .collect::<Result<Vec<_>>>()?;
        let indirect_messages = indirect_messages_vec
            .into_iter()
            .flat_map(|v| v.into_iter());
        let boxed: Box<dyn Iterator<Item = _>> = Box::new(direct_messages.chain(indirect_messages));
        Ok(boxed)
    }
    pub fn all_enums(
        &'a self,
    ) -> Result<impl 'a + IntoIterator<Item = &EnumDescriptorWithContext>> {
        let direct_enums = self.enum_types()?.into_iter();
        let indirect_enums_vec = self
            .nested_types()?
            .into_iter()
            .map(|child| child.all_enums())
            .collect::<Result<Vec<_>>>()?;
        let indirect_enums = indirect_enums_vec.into_iter().flat_map(|v| v.into_iter());
        let boxed: Box<dyn Iterator<Item = _>> = Box::new(direct_enums.chain(indirect_enums));
        Ok(boxed)
    }
}

// endregion:
