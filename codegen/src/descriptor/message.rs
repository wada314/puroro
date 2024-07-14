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

impl<'a> TryFrom<DescriptorProto<'a>> for DescriptorBase {
    type Error = ErrorKind;
    fn try_from(proto: DescriptorProto) -> Result<Self> {
        Ok(Self {
            name: proto.name()?.try_into_string("No Descriptor name")?,
            fields: proto
                .field()
                .into_iter()
                .map_ok(FieldDescriptorBase::try_from)
                .collect::<PResult<Result<Vec<_>>>>()??,
            oneof_decls: proto
                .oneof_decl()
                .into_iter()
                .map_ok(OneofDescriptorBase::try_from)
                .collect::<PResult<Result<Vec<_>>>>()??,
            nested_types: proto
                .nested_type()
                .into_iter()
                .map_ok(DescriptorBase::try_from)
                .collect::<PResult<Result<Vec<_>>>>()??,
            enum_types: proto
                .enum_type()
                .into_iter()
                .map_ok(EnumDescriptorBase::try_from)
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
    non_oneof_fields: OnceCell<Vec<FieldDescriptor<'a>>>,
    real_oneof_fields: OnceCell<Vec<FieldDescriptor<'a>>>,
    synthetic_oneof_fields: OnceCell<Vec<FieldDescriptor<'a>>>,
    real_and_synthetic_oneofs: OnceCell<(Vec<OneofDescriptor<'a>>, Vec<OneofDescriptor<'a>>)>,
    #[allow(unused)]
    real_oneofs: OnceCell<Vec<OneofDescriptor<'a>>>,
    #[allow(unused)]
    synthetic_oneofs: OnceCell<Vec<OneofDescriptor<'a>>>,
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
    pub fn file(&self) -> &'a FileDescriptor<'a> {
        self.file
    }
    pub fn root(&self) -> &'a RootContext<'a> {
        self.file().root()
    }
    pub fn name(&self) -> Result<&str> {
        Ok(&self.base.name)
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
                full_path.push(&self.base.name);
                Ok(full_path)
            })
            .map(|s| s.as_ref())
    }
    pub fn all_fields(&'a self) -> Result<impl 'a + IntoIterator<Item = &FieldDescriptor>> {
        self.cache.all_fields.get_or_try_init(|| {
            self.base
                .fields
                .iter()
                .map(|f| Ok(FieldDescriptor::new(f, self)))
                .collect()
        })
    }
    pub fn filtered_fields(
        &'a self,
        f: impl Fn(&FieldDescriptorBase) -> bool,
    ) -> Result<Vec<FieldDescriptor<'a>>> {
        self.base
            .fields
            .iter()
            .filter(|field| f(field))
            .map(|f| Ok(FieldDescriptor::new(f, self)))
            .collect()
    }
    pub fn non_oneof_fields(&'a self) -> Result<impl 'a + IntoIterator<Item = &FieldDescriptor>> {
        self.cache
            .non_oneof_fields
            .get_or_try_init(|| self.filtered_fields(|f| f.oneof_index().is_none()))
    }
    pub fn real_oneof_fields(&'a self) -> Result<impl 'a + IntoIterator<Item = &FieldDescriptor>> {
        self.cache.real_oneof_fields.get_or_try_init(|| {
            self.filtered_fields(|f| f.oneof_index().is_some() && !f.is_proto3_optional())
        })
    }
    pub fn synthetic_oneof_fields(
        &'a self,
    ) -> Result<impl 'a + IntoIterator<Item = &FieldDescriptor>> {
        self.cache.synthetic_oneof_fields.get_or_try_init(|| {
            self.filtered_fields(|f| f.oneof_index().is_some() && f.is_proto3_optional())
        })
    }
    pub fn all_oneofs(&'a self) -> Result<impl 'a + IntoIterator<Item = &OneofDescriptor>> {
        Ok(self
            .real_oneofs()?
            .into_iter()
            .chain(self.synthetic_oneofs()?.into_iter()))
    }
    pub fn real_and_synthetic_oneofs(
        &'a self,
    ) -> Result<(
        impl IntoIterator<Item = &'a OneofDescriptor<'a>>,
        impl IntoIterator<Item = &'a OneofDescriptor<'a>>,
    )> {
        let (real, synthetic) = self.cache.real_and_synthetic_oneofs.get_or_try_init(|| {
            let mut is_oneof_synthetic = vec![false; self.base.oneof_decls.len()];
            for f in self.all_fields()? {
                if let Some(i) = f.oneof_index() {
                    is_oneof_synthetic[i] |= f.is_proto3_optional();
                }
            }
            let (real, synthetic): (Vec<_>, Vec<_>) = self
                .base
                .oneof_decls
                .iter()
                .enumerate()
                .partition_map(|(i, o)| {
                    let oneof = Ok(OneofDescriptor::new(o, self));
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
    pub fn real_oneofs(&'a self) -> Result<impl IntoIterator<Item = &'a OneofDescriptor>> {
        Ok(self.real_and_synthetic_oneofs()?.0)
    }
    pub fn synthetic_oneofs(&'a self) -> Result<impl IntoIterator<Item = &'a OneofDescriptor>> {
        Ok(self.real_and_synthetic_oneofs()?.1)
    }
    pub fn nested_types(&'a self) -> Result<impl 'a + IntoIterator<Item = &Descriptor>> {
        self.cache.nested_types.get_or_try_init(|| {
            self.base
                .nested_types
                .iter()
                .map(|m| {
                    Ok(Descriptor {
                        file: self.file,
                        maybe_containing: Some(self),
                        base: m,
                        cache: Default::default(),
                    })
                })
                .collect()
        })
    }
    pub fn enum_types(&'a self) -> Result<impl 'a + IntoIterator<Item = &EnumDescriptor>> {
        self.cache.enum_types.get_or_try_init(|| {
            self.base
                .enum_types
                .iter()
                .map(|e| Ok(EnumDescriptor::new(self.file(), Some(self), e)))
                .collect()
        })
    }
    pub fn all_messages_or_enums(
        &'a self,
    ) -> Result<impl 'a + IntoIterator<Item = MessageOrEnum<&Descriptor, &EnumDescriptor>>> {
        Ok(self
            .all_messages()?
            .into_iter()
            .map(MessageOrEnum::Message)
            .chain(self.all_enums()?.into_iter().map(MessageOrEnum::Enum)))
    }
    pub fn all_messages(&'a self) -> Result<impl 'a + IntoIterator<Item = &Descriptor>> {
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
    pub fn all_enums(&'a self) -> Result<impl 'a + IntoIterator<Item = &EnumDescriptor>> {
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
