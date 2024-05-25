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

#![allow(unused)]

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

// region: Edition

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Edition {
    #[default]
    Unknown,
    Proto2,
    Proto3,
    Edition2023,
    Edition2024,
}

impl TryFrom<EditionProto> for Edition {
    type Error = ErrorKind;
    fn try_from(proto: EditionProto) -> Result<Self> {
        match proto {
            EditionProto::EditionProto2 => Ok(Edition::Proto2),
            EditionProto::EditionProto3 => Ok(Edition::Proto3),
            EditionProto::Edition2023 => Ok(Edition::Edition2023),
            EditionProto::Edition2024 => Ok(Edition::Edition2024),
            _ => Err(ErrorKind::UnknownEdition),
        }
    }
}

// endregion:

// region: FieldType

#[derive(Debug, Default)]
pub enum FieldType<'a> {
    BOOL,
    BYTES,
    DOUBLE,
    ENUM(&'a EnumDescriptorWithContext<'a>),
    FIXED32,
    FIXED64,
    FLOAT,
    GROUP,
    #[default]
    INT32,
    INT64,
    MESSAGE(&'a DescriptorWithContext<'a>),
    SFIXED32,
    SFIXED64,
    SINT32,
    SINT64,
    STRING,
    UINT32,
    UINT64,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum FieldTypeCase {
    BOOL,
    BYTES,
    DOUBLE,
    ENUM,
    FIXED32,
    FIXED64,
    FLOAT,
    GROUP,
    #[default]
    INT32,
    INT64,
    MESSAGE,
    SFIXED32,
    SFIXED64,
    SINT32,
    SINT64,
    STRING,
    UINT32,
    UINT64,
}

impl From<FieldTypeProto> for FieldTypeCase {
    fn from(proto: FieldTypeProto) -> Self {
        match proto {
            FieldTypeProto::TypeBool => FieldTypeCase::BOOL,
            FieldTypeProto::TypeBytes => FieldTypeCase::BYTES,
            FieldTypeProto::TypeDouble => FieldTypeCase::DOUBLE,
            FieldTypeProto::TypeEnum => FieldTypeCase::ENUM,
            FieldTypeProto::TypeFixed32 => FieldTypeCase::FIXED32,
            FieldTypeProto::TypeFixed64 => FieldTypeCase::FIXED64,
            FieldTypeProto::TypeFloat => FieldTypeCase::FLOAT,
            FieldTypeProto::TypeGroup => FieldTypeCase::GROUP,
            FieldTypeProto::TypeInt32 => FieldTypeCase::INT32,
            FieldTypeProto::TypeInt64 => FieldTypeCase::INT64,
            FieldTypeProto::TypeMessage => FieldTypeCase::MESSAGE,
            FieldTypeProto::TypeSFixed32 => FieldTypeCase::SFIXED32,
            FieldTypeProto::TypeSFixed64 => FieldTypeCase::SFIXED64,
            FieldTypeProto::TypeSInt32 => FieldTypeCase::SINT32,
            FieldTypeProto::TypeSInt64 => FieldTypeCase::SINT64,
            FieldTypeProto::TypeString => FieldTypeCase::STRING,
            FieldTypeProto::TypeUInt32 => FieldTypeCase::UINT32,
            FieldTypeProto::TypeUInt64 => FieldTypeCase::UINT64,
        }
    }
}

// endregion:

// region: FieldLabel

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum FieldLabel {
    #[default]
    OPTIONAL,
    REQUIRED,
    REPEATED,
}

impl From<FieldLabelProto> for FieldLabel {
    fn from(proto: FieldLabelProto) -> Self {
        match proto {
            FieldLabelProto::LabelOptional => FieldLabel::OPTIONAL,
            FieldLabelProto::LabelRequired => FieldLabel::REQUIRED,
            FieldLabelProto::LabelRepeated => FieldLabel::REPEATED,
        }
    }
}

// endregion:

// region: Descriptor raw structs

/// Structs for the each descriptor types.
/// These structs are strictly read-only and only knows about its children, not parent.

// region: FileDescriptor

#[derive(Debug, Clone)]
pub struct FileDescriptor {
    name: String,
    dependencies: Vec<String>,
    package: Option<ProtoPathBuf>,
    message_types: Vec<Descriptor>,
    enum_types: Vec<EnumDescriptor>,
    syntax: Option<String>,
    edition: Option<Edition>,
}

impl<'a> TryFrom<FileDescriptorProto<'a>> for FileDescriptor {
    type Error = ErrorKind;
    fn try_from(proto: FileDescriptorProto) -> Result<Self> {
        Ok(Self {
            name: proto.name()?.try_into_string("No FileDescriptor name")?,
            dependencies: proto
                .dependency()
                .into_iter()
                .map_ok(str::to_string)
                .collect::<PResult<_>>()?,
            package: proto.package()?.map(|s| s.to_string().into()),
            message_types: proto
                .message_type()
                .into_iter()
                .map_ok(Descriptor::try_from)
                .collect::<PResult<Result<Vec<_>>>>()??,
            enum_types: proto
                .enum_type()
                .into_iter()
                .map_ok(EnumDescriptor::try_from)
                .collect::<PResult<Result<Vec<_>>>>()??,
            syntax: proto.syntax()?.map(str::to_string),
            edition: proto.edition()?.map(EditionProto::try_into).transpose()?,
        })
    }
}

// endregion:

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

// region: FieldDescriptor

#[derive(Debug, Clone)]
pub struct FieldDescriptor {
    name: String,
    number: i32,
    type_: FieldTypeCase,
    type_name: Option<String>,
    label: Option<FieldLabel>,
    oneof_index: Option<usize>,
    proto3_optional: bool,
}

impl<'a> TryFrom<FieldDescriptorProto<'a>> for FieldDescriptor {
    type Error = ErrorKind;
    fn try_from(proto: FieldDescriptorProto) -> Result<Self> {
        Ok(Self {
            name: proto.name()?.try_into_string("No FieldDescriptor name")?,
            number: proto
                .number()?
                .try_into_number("No FieldDescriptor number")?,
            type_: proto
                .type_()?
                .ok_or_else(|| {
                    ErrorKind::DescriptorProtoValidationError("No FieldDescriptor type".to_string())
                })?
                .into(),
            type_name: proto.type_name()?.map(str::to_string),
            label: proto.label()?.map(FieldLabelProto::into),
            oneof_index: proto.oneof_index()?.map(|i| i.try_into()).transpose()?,
            proto3_optional: proto.proto3_optional()?.unwrap_or(false),
        })
    }
}

// endregion:

// region: EnumDescriptor

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

// endregion:

// region: EnumValueDescriptor

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

// endregion:

// region: OneofDescriptor

#[derive(Debug, Clone)]
pub struct OneofDescriptor {
    name: String,
}

impl<'a> TryFrom<OneofDescriptorProto<'a>> for OneofDescriptor {
    type Error = ErrorKind;
    fn try_from(proto: OneofDescriptorProto) -> Result<Self> {
        Ok(Self {
            name: proto.name()?.try_into_string("No OneofDescriptor name")?,
        })
    }
}

// endregion:

// endregion:

/// The struct types with the "context".
/// The context here means the path from the root of the descriptor tree to the current node.

// region: RootContext

#[derive(Debug)]
pub struct RootContext<'a> {
    files: Vec<(FileDescriptor, OnceCell<FileDescriptorWithContext<'a>>)>,
}
impl From<FileDescriptor> for RootContext<'_> {
    fn from(f: FileDescriptor) -> Self {
        Self {
            files: vec![(f, OnceCell::default())],
        }
    }
}
impl<T: IntoIterator<Item = FileDescriptor>> From<T> for RootContext<'_> {
    fn from(files: T) -> Self {
        Self {
            files: files
                .into_iter()
                .map(|f| (f, OnceCell::default()))
                .collect(),
        }
    }
}
impl<'a> RootContext<'a> {
    pub fn files(&'a self) -> impl 'a + IntoIterator<Item = &'a FileDescriptorWithContext<'a>> {
        self.files.iter().map(|(f, c)| {
            c.get_or_init(|| FileDescriptorWithContext {
                root: self,
                body: f,
                cache: Default::default(),
            })
        })
    }
    pub fn file_from_name(&'a self, name: &str) -> Result<&'a FileDescriptorWithContext<'a>> {
        self.files()
            .into_iter()
            .find(|f| f.name().is_ok_and(|n| n == name))
            .ok_or_else(|| ErrorKind::DescriptorStructureError("No such file".to_string()))
    }
}

// endregion:

// region: FileDescriptorWithContext

#[derive(Debug)]
pub struct FileDescriptorWithContext<'a> {
    root: &'a RootContext<'a>,
    body: &'a FileDescriptor,
    cache: FileDescriptorCache<'a>,
}

#[derive(Default, Debug)]
pub struct FileDescriptorCache<'a> {
    dependencies: OnceCell<Vec<&'a FileDescriptorWithContext<'a>>>,
    messages: OnceCell<Vec<DescriptorWithContext<'a>>>,
    enums: OnceCell<Vec<EnumDescriptorWithContext<'a>>>,
}

impl<'a> FileDescriptorWithContext<'a> {
    pub fn name(&self) -> Result<&str> {
        Ok(&self.body.name)
    }
    pub fn package(&self) -> Result<Option<&ProtoPath>> {
        Ok(self.body.package.as_deref())
    }
    pub fn dependencies(
        &'a self,
    ) -> Result<impl IntoIterator<Item = &'a FileDescriptorWithContext<'a>>> {
        self.cache
            .dependencies
            .get_or_try_init(|| {
                Ok(self
                    .body
                    .dependencies
                    .iter()
                    .map(|name| self.root.file_from_name(name))
                    .collect::<Result<Vec<_>>>()?)
            })
            .map(|v| v.into_iter().map(|f| *f))
    }
    pub fn messages(&'a self) -> Result<impl 'a + IntoIterator<Item = &DescriptorWithContext>> {
        self.cache.messages.get_or_try_init(|| {
            self.body
                .message_types
                .iter()
                .map(|m| {
                    Ok(DescriptorWithContext {
                        file: self,
                        maybe_containing: None,
                        body: m,
                        cache: Default::default(),
                    })
                })
                .collect()
        })
    }
    pub fn enums(&'a self) -> Result<impl 'a + IntoIterator<Item = &EnumDescriptorWithContext>> {
        self.cache.enums.get_or_try_init(|| {
            self.body
                .enum_types
                .iter()
                .map(|e| {
                    Ok(EnumDescriptorWithContext {
                        file: self,
                        maybe_containing: None,
                        body: e,
                        cache: Default::default(),
                    })
                })
                .collect()
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
    full_name: OnceCell<ProtoPathBuf>,
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
    pub fn name(&self) -> Result<&str> {
        Ok(&self.body.name)
    }
    pub fn full_name(&self) -> Result<&ProtoPath> {
        self.cache
            .full_name
            .get_or_try_init(|| {
                let mut full_name = if let Some(nested) = self.maybe_containing {
                    nested.full_name()?.to_owned()
                } else {
                    self.file
                        .package()?
                        .map_or_else(ProtoPathBuf::new, |p| p.to_owned())
                };
                // todo!("absl path check?");
                full_name.push(ProtoPath::new(&self.body.name));
                Ok(full_name)
            })
            .map(|s| s.as_ref())
    }
    pub fn file(&'a self) -> Result<&FileDescriptorWithContext> {
        Ok(self.file)
    }
    pub fn all_fields(
        &'a self,
    ) -> Result<impl 'a + IntoIterator<Item = &FieldDescriptorWithContext>> {
        Ok(self
            .non_oneof_fields()?
            .into_iter()
            .chain(self.real_oneof_fields()?.into_iter())
            .chain(self.synthetic_oneof_fields()?.into_iter()))
    }
    pub fn filtered_fields(
        &'a self,
        f: impl Fn(&&FieldDescriptor) -> bool,
    ) -> Result<Vec<FieldDescriptorWithContext<'a>>> {
        self.body
            .fields
            .iter()
            .filter(f)
            .map(|f| {
                Ok(FieldDescriptorWithContext {
                    message: self,
                    body: f,
                    cache: Default::default(),
                })
            })
            .collect()
    }
    pub fn non_oneof_fields(
        &'a self,
    ) -> Result<impl 'a + IntoIterator<Item = &FieldDescriptorWithContext>> {
        self.cache
            .non_oneof_fields
            .get_or_try_init(|| self.filtered_fields(|f| f.oneof_index.is_none()))
    }
    pub fn real_oneof_fields(
        &'a self,
    ) -> Result<impl 'a + IntoIterator<Item = &FieldDescriptorWithContext>> {
        self.cache.real_oneof_fields.get_or_try_init(|| {
            self.filtered_fields(|f| f.oneof_index.is_some() && !f.proto3_optional)
        })
    }
    pub fn synthetic_oneof_fields(
        &'a self,
    ) -> Result<impl 'a + IntoIterator<Item = &FieldDescriptorWithContext>> {
        self.cache.synthetic_oneof_fields.get_or_try_init(|| {
            self.filtered_fields(|f| f.oneof_index.is_some() && f.proto3_optional)
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
                if let Some(i) = f.body.oneof_index {
                    is_oneof_synthetic[i] |= f.body.proto3_optional;
                }
            }
            let (real, synthetic): (Vec<_>, Vec<_>) = self
                .body
                .oneof_decls
                .iter()
                .enumerate()
                .partition_map(|(i, o)| {
                    let oneof = Ok(OneofDescriptorWithContext {
                        message: self,
                        body: o,
                        cache: Default::default(),
                    });
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
                .map(|e| {
                    Ok(EnumDescriptorWithContext {
                        file: self.file,
                        maybe_containing: Some(self),
                        body: e,
                        cache: Default::default(),
                    })
                })
                .collect()
        })
    }
}

// endregion:

// region: EnumDescriptorWithContext

#[derive(Debug)]
pub struct EnumDescriptorWithContext<'a> {
    file: &'a FileDescriptorWithContext<'a>,
    maybe_containing: Option<&'a DescriptorWithContext<'a>>,
    body: &'a EnumDescriptor,
    cache: EnumDescriptorCache<'a>,
}

#[derive(Default, Debug)]
pub struct EnumDescriptorCache<'a> {
    full_name: OnceCell<ProtoPathBuf>,
    values: OnceCell<Vec<EnumValueDescriptorWithContext<'a>>>,
}

impl<'a> EnumDescriptorWithContext<'a> {
    pub fn name(&self) -> Result<&str> {
        Ok(self.body.name.as_ref())
    }
    pub fn full_name(&self) -> Result<&str> {
        self.cache
            .full_name
            .get_or_try_init(|| {
                let mut full_name = if let Some(nested) = self.maybe_containing {
                    nested.full_name()?.to_owned()
                } else {
                    self.file
                        .package()?
                        .map_or_else(ProtoPathBuf::new, |p| p.to_owned())
                };
                full_name.push(ProtoPath::new(&self.body.name));
                Ok(full_name)
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

// endregion:

// region: EnumValueDescriptorWithContext

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
                    m.full_name()?.to_owned()
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
}

// endregion:

// region: FieldDescriptorWithContext

#[derive(Debug)]
pub struct FieldDescriptorWithContext<'a> {
    message: &'a DescriptorWithContext<'a>,
    body: &'a FieldDescriptor,
    cache: FieldDescriptorCache,
}

#[derive(Default, Debug)]
pub struct FieldDescriptorCache {
    full_name: OnceCell<ProtoPathBuf>,
    type_: OnceCell<()>,
}

impl<'a> FieldDescriptorWithContext<'a> {
    pub fn name(&self) -> Result<&str> {
        Ok(&self.body.name)
    }
    pub fn full_name(&self) -> Result<&str> {
        self.cache
            .full_name
            .get_or_try_init(|| {
                let mut full_name = self.message.full_name()?.to_owned();
                full_name.push(ProtoPath::new(&format!(".{}", self.body.name)));
                Ok(full_name)
            })
            .map(|s| s.as_ref())
    }
}

// endregion:

// region: OneofDescriptorWithContext

#[derive(Debug)]
pub struct OneofDescriptorWithContext<'a> {
    message: &'a DescriptorWithContext<'a>,
    body: &'a OneofDescriptor,
    cache: OneofDescriptorCache,
}

#[derive(Default, Debug)]
pub struct OneofDescriptorCache {}

// endregion:

// region: utils

trait TryIntoString {
    fn try_into_string(self, error_message: &str) -> Result<String>;
}
impl TryIntoString for Option<&str> {
    fn try_into_string(self, error_message: &str) -> Result<String> {
        self.ok_or_else(|| ErrorKind::DescriptorProtoValidationError(error_message.to_string()))
            .map(str::to_string)
    }
}
trait TryIntoNumber<T> {
    fn try_into_number(self, error_message: &str) -> Result<T>;
}
impl<T> TryIntoNumber<T> for Option<T> {
    fn try_into_number(self, error_message: &str) -> Result<T> {
        self.ok_or_else(|| ErrorKind::DescriptorProtoValidationError(error_message.to_string()))
    }
}

// endregion:
