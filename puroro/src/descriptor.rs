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

use crate::descriptor_proto::{
    field_descriptor_proto::Label as FieldLabelProto,
    field_descriptor_proto::Type as FieldTypeProto, DescriptorProto, Edition as EditionProto,
    EnumDescriptorProto, EnumValueDescriptorProto, FieldDescriptorProto, FileDescriptorProto,
    FileDescriptorSet, OneofDescriptorProto,
};
use crate::{ErrorKind, Result};
use ::itertools::Itertools;
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum FieldTypeTemplate<E, M> {
    DOUBLE,
    FLOAT,
    INT64,
    UINT64,
    #[default]
    INT32,
    FIXED64,
    FIXED32,
    BOOL,
    STRING,
    GROUP,
    MESSAGE(M),
    BYTES,
    UINT32,
    ENUM(E),
    SFIXED32,
    SFIXED64,
    SINT32,
    SINT64,
}
pub type FieldType = FieldTypeTemplate<(), ()>;
pub type FieldTypeWithConcreteType<'a> =
    FieldTypeTemplate<&'a EnumDescriptorWithContext<'a>, &'a DescriptorWithContext<'a>>;

impl From<FieldTypeProto> for FieldType {
    fn from(proto: FieldTypeProto) -> Self {
        match proto {
            FieldTypeProto::TypeDouble => FieldType::DOUBLE,
            FieldTypeProto::TypeFloat => FieldType::FLOAT,
            FieldTypeProto::TypeInt64 => FieldType::INT64,
            FieldTypeProto::TypeUInt64 => FieldType::UINT64,
            FieldTypeProto::TypeInt32 => FieldType::INT32,
            FieldTypeProto::TypeFixed64 => FieldType::FIXED64,
            FieldTypeProto::TypeFixed32 => FieldType::FIXED32,
            FieldTypeProto::TypeBool => FieldType::BOOL,
            FieldTypeProto::TypeString => FieldType::STRING,
            FieldTypeProto::TypeGroup => FieldType::GROUP,
            FieldTypeProto::TypeMessage => FieldType::MESSAGE(()),
            FieldTypeProto::TypeBytes => FieldType::BYTES,
            FieldTypeProto::TypeUInt32 => FieldType::UINT32,
            FieldTypeProto::TypeEnum => FieldType::ENUM(()),
            FieldTypeProto::TypeSFixed32 => FieldType::SFIXED32,
            FieldTypeProto::TypeSFixed64 => FieldType::SFIXED64,
            FieldTypeProto::TypeSInt32 => FieldType::SINT32,
            FieldTypeProto::TypeSInt64 => FieldType::SINT64,
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

#[derive(Debug, Clone)]
pub struct FileDescriptor {
    name: String,
    package: String,
    message_types: Vec<Descriptor>,
    enum_types: Vec<EnumDescriptor>,
    syntax: String,
    edition: Edition,
}

impl<'a> TryFrom<FileDescriptorProto<'a>> for FileDescriptor {
    type Error = ErrorKind;
    fn try_from(proto: FileDescriptorProto) -> Result<Self> {
        Ok(Self {
            name: proto.name()?.try_into_string("No FileDescriptor name")?,
            package: proto
                .package()?
                .try_into_string("No FileDescriptor package")?,
            message_types: proto
                .message_type()
                .into_iter()
                .map_ok(Descriptor::try_from)
                .collect::<Result<Result<Vec<_>>>>()??,
            enum_types: proto
                .enum_type()
                .into_iter()
                .map_ok(EnumDescriptor::try_from)
                .collect::<Result<Result<Vec<_>>>>()??,
            syntax: proto
                .syntax()?
                .try_into_string("No FileDescriptor syntax")?,
            edition: match proto.syntax()? {
                "proto2" => Edition::Proto2,
                "proto3" => Edition::Proto3,
                "proto3.2023" => Edition::Edition2023,
                "proto3.2024" => Edition::Edition2024,
                _ => Edition::Unknown,
            },
        })
    }
}

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
                .collect::<Result<Result<Vec<_>>>>()??,
            oneof_decls: proto
                .oneof_decl()
                .into_iter()
                .map_ok(OneofDescriptor::try_from)
                .collect::<Result<Result<Vec<_>>>>()??,
            nested_types: proto
                .nested_type()
                .into_iter()
                .map_ok(Descriptor::try_from)
                .collect::<Result<Result<Vec<_>>>>()??,
            enum_types: proto
                .enum_type()
                .into_iter()
                .map_ok(EnumDescriptor::try_from)
                .collect::<Result<Result<Vec<_>>>>()??,
        })
    }
}

// endregion:

// region: FieldDescriptor

#[derive(Debug, Clone)]
pub struct FieldDescriptor {
    name: String,
    number: i32,
    type_: FieldType,
    type_name: Option<String>,
    label: Option<FieldLabel>,
    oneof_index: Option<i32>,
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
            oneof_index: proto.oneof_index()?,
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
                .collect::<Result<Result<Vec<_>>>>()??,
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

pub struct Context {
    files: Vec<FileDescriptor>,
}
impl From<FileDescriptor> for Context {
    fn from(f: FileDescriptor) -> Self {
        Self { files: vec![f] }
    }
}
impl From<Vec<FileDescriptor>> for Context {
    fn from(files: Vec<FileDescriptor>) -> Self {
        Self { files }
    }
}
impl<const N: usize> From<[FileDescriptor; N]> for Context {
    fn from(files: [FileDescriptor; N]) -> Self {
        Self {
            files: files.into(),
        }
    }
}
impl Context {
    fn files(&self) -> impl IntoIterator<Item = Result<FileDescriptorWithContext>> {
        (0..(self.files.len())).map(move |i| self.file_from_index(i))
    }
    fn file_from_index(&self, index: usize) -> Result<FileDescriptorWithContext> {
        Ok(FileDescriptorWithContext {
            root: self,
            body: &self.files[index],
            cache: Default::default(),
        })
    }
}

pub struct FileDescriptorWithContext<'a> {
    root: &'a Context,
    body: &'a FileDescriptor,
    cache: FileDescriptorCache<'a>,
}

#[derive(Default)]
pub struct FileDescriptorCache<'a> {
    dependencies: OnceCell<Vec<FileDescriptorWithContext<'a>>>,
    messages: OnceCell<Vec<DescriptorWithContext<'a>>>,
    enums: OnceCell<Vec<EnumDescriptorWithContext<'a>>>,
}

impl<'a> FileDescriptorWithContext<'a> {
    fn name(&self) -> Result<&str> {
        Ok(&self.body.name)
    }
    fn package(&self) -> Result<&str> {
        Ok(&self.body.package)
    }
    fn dependencies(&'a self) -> Result<impl 'a + IntoIterator<Item = &FileDescriptorWithContext>> {
        self.cache.dependencies.get_or_try_init(|| {
            self.body
                .dependency_indices
                .iter()
                .map(|&i| self.root.file_from_index(i))
                .collect()
        })
    }
    fn messages(&'a self) -> Result<impl 'a + IntoIterator<Item = &DescriptorWithContext>> {
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
    fn enums(&'a self) -> Result<impl 'a + IntoIterator<Item = &EnumDescriptorWithContext>> {
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

pub struct DescriptorWithContext<'a> {
    file: &'a FileDescriptorWithContext<'a>,
    maybe_containing: Option<&'a DescriptorWithContext<'a>>,
    body: &'a Descriptor,
    cache: DescriptorCache<'a>,
}

#[derive(Default)]
pub struct DescriptorCache<'a> {
    full_name: OnceCell<String>,
    fields: OnceCell<Vec<FieldDescriptorWithContext<'a>>>,
    oneofs: OnceCell<Vec<OneofDescriptorWithContext<'a>>>,
    real_oneofs: OnceCell<Vec<OneofDescriptorWithContext<'a>>>,
    nested_types: OnceCell<Vec<DescriptorWithContext<'a>>>,
    enum_types: OnceCell<Vec<EnumDescriptorWithContext<'a>>>,
}

impl<'a> DescriptorWithContext<'a> {
    fn name(&self) -> Result<&str> {
        Ok(&self.body.name)
    }
    fn full_name(&self) -> Result<&str> {
        self.cache
            .full_name
            .get_or_try_init(|| {
                let mut full_name = if let Some(nested) = self.maybe_containing {
                    nested.full_name()?.to_string()
                } else {
                    self.file.package()?.to_string()
                };
                if !full_name.is_empty() {
                    full_name.push('.');
                }
                full_name.push_str(&self.body.name);
                Ok(full_name)
            })
            .map(|s| s.as_ref())
    }
    fn file(&'a self) -> Result<&FileDescriptorWithContext> {
        Ok(self.file)
    }
    fn fields(&'a self) -> Result<impl 'a + IntoIterator<Item = &FieldDescriptorWithContext>> {
        self.cache.fields.get_or_try_init(|| {
            self.body
                .fields
                .iter()
                .map(|f| {
                    Ok(FieldDescriptorWithContext {
                        message: self,
                        body: f,
                        cache: Default::default(),
                    })
                })
                .collect()
        })
    }
    fn oneofs(&'a self) -> Result<impl 'a + IntoIterator<Item = &OneofDescriptorWithContext>> {
        self.cache.oneofs.get_or_try_init(|| {
            self.body
                .oneof_decls
                .iter()
                .map(|o| {
                    Ok(OneofDescriptorWithContext {
                        message: self,
                        body: o,
                        cache: Default::default(),
                    })
                })
                .collect()
        })
    }
    fn real_oneofs(&'a self) -> Result<impl 'a + IntoIterator<Item = &OneofDescriptorWithContext>> {
        self.cache.real_oneofs.get_or_try_init(|| {
            self.body
                .oneof_decls
                .iter()
                .map(|o| {
                    Ok(OneofDescriptorWithContext {
                        message: self,
                        body: o,
                        cache: Default::default(),
                    })
                })
                .collect()
        })
    }
    fn nested_types(&'a self) -> Result<impl 'a + IntoIterator<Item = &DescriptorWithContext>> {
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
    fn enum_types(&'a self) -> Result<impl 'a + IntoIterator<Item = &EnumDescriptorWithContext>> {
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

pub struct EnumDescriptorWithContext<'a> {
    file: &'a FileDescriptorWithContext<'a>,
    maybe_containing: Option<&'a DescriptorWithContext<'a>>,
    body: &'a EnumDescriptor,
    cache: EnumDescriptorCache<'a>,
}

#[derive(Default)]
pub struct EnumDescriptorCache<'a> {
    full_name: OnceCell<String>,
    values: OnceCell<Vec<EnumValueDescriptorWithContext<'a>>>,
}

impl<'a> EnumDescriptorWithContext<'a> {
    fn name(&self) -> Result<&str> {
        Ok(self.body.name.as_ref())
    }
    fn full_name(&self) -> Result<&str> {
        self.cache
            .full_name
            .get_or_try_init(|| {
                let mut full_name = if let Some(nested) = self.maybe_containing {
                    nested.full_name()?.to_string()
                } else {
                    self.file.package()?.to_string()
                };
                if !full_name.is_empty() {
                    full_name.push('.');
                }
                full_name.push_str(&self.body.name);
                Ok(full_name)
            })
            .map(|s| s.as_ref())
    }
    fn file(&'a self) -> Result<&FileDescriptorWithContext> {
        Ok(self.file)
    }
    fn values(&'a self) -> Result<impl 'a + IntoIterator<Item = &EnumValueDescriptorWithContext>> {
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

pub struct EnumValueDescriptorWithContext<'a> {
    enum_: &'a EnumDescriptorWithContext<'a>,
    body: &'a EnumValueDescriptor,
    cache: EnumValueDescriptorCache,
}
#[derive(Default)]
pub struct EnumValueDescriptorCache {
    full_name: OnceCell<String>,
}
impl<'a> EnumValueDescriptorWithContext<'a> {
    fn name(&self) -> Result<&str> {
        Ok(self.body.name.as_ref())
    }
    fn full_name(&self) -> Result<&str> {
        self.cache
            .full_name
            .get_or_try_init(|| {
                // This full_name is a sibling of EnumDescriptor, not a child.
                let mut full_name = if let Some(m) = self.enum_.maybe_containing {
                    m.full_name()?.to_string()
                } else {
                    self.enum_.file.package()?.to_string()
                };
                if !full_name.is_empty() {
                    full_name.push('.');
                }
                full_name.push_str(&self.body.name);
                Ok(full_name)
            })
            .map(|s| s.as_ref())
    }
}

pub struct FieldDescriptorWithContext<'a> {
    message: &'a DescriptorWithContext<'a>,
    body: &'a FieldDescriptor,
    cache: FieldDescriptorCache,
}

#[derive(Default)]
pub struct FieldDescriptorCache {
    full_name: OnceCell<String>,
    type_: OnceCell<()>,
}

impl<'a> FieldDescriptorWithContext<'a> {
    fn name(&self) -> Result<&str> {
        Ok(&self.body.name)
    }
    fn full_name(&self) -> Result<&str> {
        self.cache
            .full_name
            .get_or_try_init(|| {
                let mut full_name = self.message.full_name()?.to_string();
                full_name.push('.');
                full_name.push_str(&self.body.name);
                Ok(full_name)
            })
            .map(|s| s.as_ref())
    }
}

pub struct OneofDescriptorWithContext<'a> {
    message: &'a DescriptorWithContext<'a>,
    body: &'a OneofDescriptor,
    cache: OneofDescriptorCache,
}

#[derive(Default)]
pub struct OneofDescriptorCache {}

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
