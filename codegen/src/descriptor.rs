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
use ::std::collections::HashMap;
use ::std::fmt::Debug;

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
    fn try_from(edition: EditionProto) -> Result<Self> {
        match edition {
            EditionProto::EditionProto2 => Ok(Edition::Proto2),
            EditionProto::EditionProto3 => Ok(Edition::Proto3),
            EditionProto::Edition2023 => Ok(Edition::Edition2023),
            EditionProto::Edition2024 => Ok(Edition::Edition2024),
            _ => Err(format!("Unknown edition: {:?}", edition).into()),
        }
    }
}

// endregion:

// region: FieldType

#[derive(Clone, Default)]
pub enum FieldType<'a> {
    Bool,
    Bytes,
    Double,
    Enum(&'a EnumDescriptorWithContext<'a>),
    Fixed32,
    Fixed64,
    Float,
    Group,
    #[default]
    Int32,
    Int64,
    Message(&'a DescriptorWithContext<'a>),
    SFixed32,
    SFixed64,
    SInt32,
    SInt64,
    String,
    UInt32,
    UInt64,
}
// We need a special implementation for Debug to avoid infinite recursion like:
// field -> message -> field -> message -> ...
impl Debug for FieldType<'_> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool => write!(f, "Bool"),
            Self::Bytes => write!(f, "Bytes"),
            Self::Double => write!(f, "Double"),
            Self::Enum(arg0) => {
                let full_path = &arg0
                    .full_path()
                    .unwrap_or_else(|_| ProtoPath::new("<error>"));
                f.debug_tuple("Enum").field(full_path).finish()
            }
            Self::Fixed32 => write!(f, "Fixed32"),
            Self::Fixed64 => write!(f, "Fixed64"),
            Self::Float => write!(f, "Float"),
            Self::Group => write!(f, "Group"),
            Self::Int32 => write!(f, "Int32"),
            Self::Int64 => write!(f, "Int64"),
            Self::Message(arg0) => {
                let full_path = &arg0
                    .full_path()
                    .unwrap_or_else(|_| ProtoPath::new("<error>"));
                f.debug_tuple("Message").field(full_path).finish()
            }
            Self::SFixed32 => write!(f, "SFixed32"),
            Self::SFixed64 => write!(f, "SFixed64"),
            Self::SInt32 => write!(f, "SInt32"),
            Self::SInt64 => write!(f, "SInt64"),
            Self::String => write!(f, "String"),
            Self::UInt32 => write!(f, "UInt32"),
            Self::UInt64 => write!(f, "UInt64"),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum FieldTypeCase {
    Bool,
    Bytes,
    Double,
    Enum,
    Fixed32,
    Fixed64,
    Float,
    Group,
    #[default]
    Int32,
    Int64,
    Message,
    SFixed32,
    SFixed64,
    SInt32,
    SInt64,
    String,
    UInt32,
    UInt64,
}

impl From<FieldTypeProto> for FieldTypeCase {
    fn from(proto: FieldTypeProto) -> Self {
        match proto {
            FieldTypeProto::TypeBool => FieldTypeCase::Bool,
            FieldTypeProto::TypeBytes => FieldTypeCase::Bytes,
            FieldTypeProto::TypeDouble => FieldTypeCase::Double,
            FieldTypeProto::TypeEnum => FieldTypeCase::Enum,
            FieldTypeProto::TypeFixed32 => FieldTypeCase::Fixed32,
            FieldTypeProto::TypeFixed64 => FieldTypeCase::Fixed64,
            FieldTypeProto::TypeFloat => FieldTypeCase::Float,
            FieldTypeProto::TypeGroup => FieldTypeCase::Group,
            FieldTypeProto::TypeInt32 => FieldTypeCase::Int32,
            FieldTypeProto::TypeInt64 => FieldTypeCase::Int64,
            FieldTypeProto::TypeMessage => FieldTypeCase::Message,
            FieldTypeProto::TypeSFixed32 => FieldTypeCase::SFixed32,
            FieldTypeProto::TypeSFixed64 => FieldTypeCase::SFixed64,
            FieldTypeProto::TypeSInt32 => FieldTypeCase::SInt32,
            FieldTypeProto::TypeSInt64 => FieldTypeCase::SInt64,
            FieldTypeProto::TypeString => FieldTypeCase::String,
            FieldTypeProto::TypeUInt32 => FieldTypeCase::UInt32,
            FieldTypeProto::TypeUInt64 => FieldTypeCase::UInt64,
        }
    }
}

impl FieldTypeCase {
    pub fn with_type_ref<'a, F, G>(
        self,
        type_name: Option<&str>,
        msg_case: F,
        enum_case: G,
    ) -> Result<FieldType<'a>>
    where
        F: FnOnce(&ProtoPath) -> Result<&'a DescriptorWithContext<'a>>,
        G: FnOnce(&ProtoPath) -> Result<&'a EnumDescriptorWithContext<'a>>,
    {
        match self {
            FieldTypeCase::Bool => Ok(FieldType::Bool),
            FieldTypeCase::Bytes => Ok(FieldType::Bytes),
            FieldTypeCase::Double => Ok(FieldType::Double),
            FieldTypeCase::Enum => {
                let type_name: ProtoPathBuf = type_name
                    .ok_or_else(|| {
                        format!("No enum type name \"{}\"", type_name.unwrap_or_default())
                    })?
                    .into();
                Ok(FieldType::Enum(enum_case(&type_name)?))
            }
            FieldTypeCase::Fixed32 => Ok(FieldType::Fixed32),
            FieldTypeCase::Fixed64 => Ok(FieldType::Fixed64),
            FieldTypeCase::Float => Ok(FieldType::Float),
            FieldTypeCase::Group => Ok(FieldType::Group),
            FieldTypeCase::Int32 => Ok(FieldType::Int32),
            FieldTypeCase::Int64 => Ok(FieldType::Int64),
            FieldTypeCase::Message => {
                let type_name: ProtoPathBuf = type_name
                    .ok_or_else(|| {
                        format!("No message type name \"{}\"", type_name.unwrap_or_default())
                    })?
                    .into();
                Ok(FieldType::Message(msg_case(&type_name)?))
            }
            FieldTypeCase::SFixed32 => Ok(FieldType::SFixed32),
            FieldTypeCase::SFixed64 => Ok(FieldType::SFixed64),
            FieldTypeCase::SInt32 => Ok(FieldType::SInt32),
            FieldTypeCase::SInt64 => Ok(FieldType::SInt64),
            FieldTypeCase::String => Ok(FieldType::String),
            FieldTypeCase::UInt32 => Ok(FieldType::UInt32),
            FieldTypeCase::UInt64 => Ok(FieldType::UInt64),
        }
    }
}

// endregion:

// region: FieldLabel

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum FieldLabel {
    #[default]
    Optional,
    Required,
    Repeated,
}

impl From<FieldLabelProto> for FieldLabel {
    fn from(proto: FieldLabelProto) -> Self {
        match proto {
            FieldLabelProto::LabelOptional => FieldLabel::Optional,
            FieldLabelProto::LabelRequired => FieldLabel::Required,
            FieldLabelProto::LabelRepeated => FieldLabel::Repeated,
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
    r#type: FieldTypeCase,
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
            r#type: proto
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
    package_to_files: OnceCell<HashMap<ProtoPathBuf, Vec<&'a FileDescriptorWithContext<'a>>>>,
}
impl From<FileDescriptor> for RootContext<'_> {
    fn from(f: FileDescriptor) -> Self {
        Self {
            files: vec![(f, OnceCell::default())],
            package_to_files: Default::default(),
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
            package_to_files: Default::default(),
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
        Ok(self
            .files()
            .into_iter()
            .find(|f| f.name().is_ok_and(|n| n == name))
            .ok_or_else(|| format!("No such file: {}", name))?)
    }
    pub fn package_to_files(
        &'a self,
        package: impl AsRef<ProtoPath>,
    ) -> Result<impl 'a + IntoIterator<Item = &'a FileDescriptorWithContext<'a>>> {
        let package = if package.as_ref().is_relative() {
            // This method is a root method, so the relative path should be converted
            // to the absolute path by just adding '.' at the beginning.
            format!(".{}", package.as_ref()).into()
        } else {
            package.as_ref().to_owned()
        };
        debug_assert!(package.is_absolute());
        let map = self.package_to_files.get_or_try_init(|| -> Result<_> {
            let mut map = HashMap::new();
            for fd in self.files() {
                let package = fd.absolute_package()?.to_owned();
                map.entry(package.clone()).or_insert_with(Vec::new).push(fd);
            }
            Ok(map)
        })?;
        Ok(map
            .get(&package)
            .map_or(Default::default(), Vec::as_slice)
            .into_iter()
            .map(|f| *f))
    }
    pub fn resolve_path(
        &'a self,
        path: impl AsRef<ProtoPath>,
    ) -> Result<MessageOrEnum<&DescriptorWithContext, &EnumDescriptorWithContext>> {
        let path = path.as_ref();
        let path = if path.is_relative() {
            ProtoPathBuf::from(format!(".{}", path))
        } else {
            path.to_owned()
        };
        return Ok(self
            .resolve_absolute_path(&path)?
            .ok_or_else(|| format!("Path not found: {}", path))?);
        Err(format!("Path not found: {}", path))?
    }
    pub fn resolve_relative_path(
        &'a self,
        path: impl AsRef<ProtoPath>,
        cur: impl AsRef<ProtoPath>,
    ) -> Result<MessageOrEnum<&DescriptorWithContext, &EnumDescriptorWithContext>> {
        let path = path.as_ref();
        let cur = cur.as_ref();
        if path.is_absolute() {
            return self.resolve_path(path);
        }
        if !cur.is_absolute() {
            Err(format!("Path not found: \"{cur}\" / \"{path}\""))?;
        }
        let cur = cur.to_owned();
        for cur2 in cur.ancestors() {
            let full_path = {
                let mut full_path = cur2.to_owned();
                full_path.push(path);
                full_path
            };
            if let Some(result) = self.resolve_absolute_path(&full_path)? {
                return Ok(result);
            }
        }
        Err(format!("Path not found: \"{cur}\" / \"{path}\""))?
    }

    fn resolve_absolute_path(
        &'a self,
        path: &ProtoPath,
    ) -> Result<Option<MessageOrEnum<&DescriptorWithContext, &EnumDescriptorWithContext>>> {
        debug_assert!(path.is_absolute());
        // Can improve the complexity here. Maybe later.
        for package_path in path.ancestors() {
            for file in self.package_to_files(package_path)? {
                for message_or_enum in file.all_messages_or_enums()? {
                    if message_or_enum.full_path()? == path {
                        return Ok(Some(message_or_enum));
                    }
                }
            }
        }
        Ok(None)
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
    pub fn absolute_package(&self) -> Result<ProtoPathBuf> {
        let mut package = self
            .package()?
            .map_or_else(ProtoPathBuf::new, |p| p.to_owned());
        if package.is_relative() {
            let mut new_package = Into::<ProtoPathBuf>::into(".");
            new_package.push(&package);
            package = new_package;
        }
        Ok(package)
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
        let direct_messages = self.messages()?.into_iter();
        let indirect_messages_vec = self
            .messages()?
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
        let direct_enums = self.enums()?.into_iter();
        let indirect_enums_vec = self
            .messages()?
            .into_iter()
            .map(|child| child.all_enums())
            .collect::<Result<Vec<_>>>()?;
        let indirect_enums = indirect_enums_vec.into_iter().flat_map(|v| v.into_iter());
        let boxed: Box<dyn Iterator<Item = _>> = Box::new(direct_enums.chain(indirect_enums));
        Ok(boxed)
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

// endregion:

// region: FieldDescriptorWithContext

#[derive(Debug)]
pub struct FieldDescriptorWithContext<'a> {
    message: &'a DescriptorWithContext<'a>,
    body: &'a FieldDescriptor,
    cache: FieldDescriptorCache<'a>,
}

#[derive(Default, Debug)]
pub struct FieldDescriptorCache<'a> {
    full_name: OnceCell<ProtoPathBuf>,
    r#type: OnceCell<FieldType<'a>>,
}

impl<'a> FieldDescriptorWithContext<'a> {
    pub fn name(&self) -> Result<&str> {
        Ok(&self.body.name)
    }
    pub fn full_name(&self) -> Result<&str> {
        self.cache
            .full_name
            .get_or_try_init(|| {
                let mut full_name = self.message.full_path()?.to_owned();
                full_name.push(ProtoPath::new(&format!(".{}", self.body.name)));
                Ok(full_name)
            })
            .map(|s| s.as_ref())
    }
    pub fn r#type(&self) -> Result<FieldType<'a>> {
        self.cache
            .r#type
            .get_or_try_init(|| {
                self.body.r#type.with_type_ref(
                    self.body.type_name.as_deref(),
                    |name| {
                        Ok(self
                            .message
                            .file
                            .root
                            .resolve_relative_path(&name, self.message.full_path()?)?
                            .maybe_message()
                            .ok_or_else(|| format!("Not a message: {}", name))?)
                    },
                    |name| {
                        Ok(self
                            .message
                            .file
                            .root
                            .resolve_relative_path(&name, self.message.full_path()?)?
                            .maybe_enum()
                            .ok_or_else(|| format!("Not an enum: {}", name))?)
                    },
                )
            })
            .cloned()
    }
    pub fn label(&self) -> Result<Option<FieldLabel>> {
        Ok(self.body.label)
    }
    pub fn is_proto3_optional(&self) -> Result<bool> {
        Ok(self.body.proto3_optional)
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

#[derive(Debug, Clone)]
pub enum MessageOrEnum<M, E> {
    Message(M),
    Enum(E),
}
impl<'a> MessageOrEnum<&'a DescriptorWithContext<'a>, &'a EnumDescriptorWithContext<'a>> {
    pub fn full_path(&self) -> Result<&ProtoPath> {
        match self {
            MessageOrEnum::Message(m) => m.full_path(),
            MessageOrEnum::Enum(e) => e.full_path(),
        }
    }
}
impl<M, E> MessageOrEnum<M, E> {
    pub fn maybe_message(self) -> Option<M> {
        match self {
            MessageOrEnum::Message(m) => Some(m),
            MessageOrEnum::Enum(_) => None,
        }
    }
    pub fn maybe_enum(self) -> Option<E> {
        match self {
            MessageOrEnum::Message(_) => None,
            MessageOrEnum::Enum(e) => Some(e),
        }
    }
}

trait TryIntoString {
    fn try_into_string(self, error_message: &str) -> Result<String>;
}
impl TryIntoString for Option<&str> {
    fn try_into_string(self, error_message: &str) -> Result<String> {
        Ok(self
            .ok_or_else(|| error_message.to_string())
            .map(str::to_string)?)
    }
}
trait TryIntoNumber<T> {
    fn try_into_number(self, error_message: &str) -> Result<T>;
}
impl<T> TryIntoNumber<T> for Option<T> {
    fn try_into_number(self, error_message: &str) -> Result<T> {
        Ok(self.ok_or_else(|| error_message.to_string())?)
    }
}

// endregion:

#[cfg(test)]
mod tests {
    use super::*;
    use ::std::assert_matches::assert_matches;

    const FD_DEFAULT: FileDescriptor = FileDescriptor {
        name: String::new(),
        dependencies: vec![],
        package: None,
        message_types: vec![],
        enum_types: vec![],
        syntax: None,
        edition: None,
    };
    const MD_DEFAULT: Descriptor = Descriptor {
        name: String::new(),
        fields: vec![],
        oneof_decls: vec![],
        nested_types: vec![],
        enum_types: vec![],
    };
    const ED_DEFAULT: EnumDescriptor = EnumDescriptor {
        name: String::new(),
        values: vec![],
    };

    #[test]
    fn test_package_to_files() {
        fn make_fd(name: &str, package: &str) -> FileDescriptor {
            FileDescriptor {
                name: name.to_string(),
                package: Some(package.into()),
                ..FD_DEFAULT
            }
        }
        let fd0 = make_fd("fd0.proto", "");
        let fd1 = make_fd("fd1.proto", "a");
        let fd2 = make_fd("fd2.proto", "a.b");
        let fd3 = make_fd("fd3.proto", "a.b");
        let fd4 = make_fd("fd4.proto", "a.b.c");
        let root = RootContext::from(vec![fd0, fd1, fd2, fd3, fd4]);

        let root_package_files = root.package_to_files("").unwrap().into_iter().collect_vec();
        let package_a_files = root
            .package_to_files("a")
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>();
        let package_a_b_files = root
            .package_to_files("a.b")
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>();
        let package_a_b_c_files = root
            .package_to_files("a.b.c")
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>();

        assert_eq!(1, root_package_files.len());
        assert_eq!(1, package_a_files.len());
        assert_eq!(2, package_a_b_files.len());
        assert_eq!(1, package_a_b_c_files.len());
        assert!(root_package_files
            .iter()
            .any(|f| f.name().unwrap() == "fd0.proto"));
        assert!(package_a_files
            .iter()
            .any(|f| f.name().unwrap() == "fd1.proto"));
        assert!(package_a_b_files
            .iter()
            .any(|f| f.name().unwrap() == "fd2.proto"));
        assert!(package_a_b_files
            .iter()
            .any(|f| f.name().unwrap() == "fd3.proto"));
        assert!(package_a_b_c_files
            .iter()
            .any(|f| f.name().unwrap() == "fd4.proto"));
    }

    #[test]
    fn test_resolve_path() {
        fn make_fd(name: &str, package: &str) -> FileDescriptor {
            FileDescriptor {
                name: name.to_string(),
                package: Some(package.into()),
                message_types: vec![
                    Descriptor {
                        name: "A".to_string(),
                        nested_types: vec![
                            Descriptor {
                                name: "B".to_string(),
                                nested_types: vec![Descriptor {
                                    name: "C".to_string(),
                                    ..MD_DEFAULT
                                }],
                                ..MD_DEFAULT
                            },
                            Descriptor {
                                name: "B2".to_string(),
                                ..MD_DEFAULT
                            },
                        ],
                        enum_types: vec![EnumDescriptor {
                            name: "F".to_string(),
                            ..ED_DEFAULT
                        }],
                        ..MD_DEFAULT
                    },
                    Descriptor {
                        name: "A2".to_string(),
                        ..MD_DEFAULT
                    },
                ],
                enum_types: vec![EnumDescriptor {
                    name: "E".to_string(),
                    ..ED_DEFAULT
                }],
                ..FD_DEFAULT
            }
        }
        let fd0 = make_fd("fd0.proto", "");
        let fd1 = make_fd("fd1.proto", "a");
        let fd2 = make_fd("fd2.proto", "a.b");
        let fd3 = make_fd("fd3.proto", "a.b2");
        let fd4 = make_fd("fd4.proto", "a.b.c");

        let root = RootContext::from(vec![fd0, fd1, fd2, fd3, fd4]);

        let assert_is_message = |path: &str, name: &str| {
            let result = root.resolve_path(path).unwrap();
            let MessageOrEnum::Message(m) = result else {
                panic!("Expected a message: {}", path);
            };
            assert_eq!(m.name().unwrap(), name);
        };
        let assert_is_enum = |path: &str, name: &str| {
            let result = root.resolve_path(path).unwrap();
            let MessageOrEnum::Enum(e) = result else {
                panic!("Expected an enum: {}", path);
            };
            assert_eq!(e.name().unwrap(), name);
        };
        assert_is_message("A", "A");
        assert_is_message("A.B", "B");
        assert_is_message("A.B2", "B2");
        assert_is_message("A.B.C", "C");
        assert_is_message("A2", "A2");
        assert_is_message(".A", "A");
        assert_is_message(".A.B", "B");
        assert_is_message(".A.B2", "B2");
        assert_is_message(".A.B.C", "C");
        assert_is_message(".A2", "A2");
        assert_is_message("a.A", "A");
        assert_is_message("a.A.B", "B");
        assert_is_message("a.A.B2", "B2");
        assert_is_message("a.A.B.C", "C");
        assert_is_message("a.A2", "A2");
        assert_is_message(".a.A", "A");
        assert_is_message(".a.A.B", "B");
        assert_is_message(".a.A.B2", "B2");
        assert_is_message(".a.A.B.C", "C");
        assert_is_message(".a.A2", "A2");
        assert_is_message("a.b.A", "A");
        assert_is_message("a.b.A.B", "B");
        assert_is_message("a.b.A.B2", "B2");
        assert_is_message("a.b.A.B.C", "C");
        assert_is_message("a.b.A2", "A2");
        assert_is_message(".a.b.A", "A");
        assert_is_message(".a.b.A.B", "B");
        assert_is_message(".a.b.A.B2", "B2");
        assert_is_message(".a.b.A.B.C", "C");
        assert_is_message(".a.b.A2", "A2");
        assert_is_message("a.b.c.A", "A");
        assert_is_message("a.b.c.A.B", "B");
        assert_is_message("a.b.c.A.B2", "B2");
        assert_is_message("a.b.c.A.B.C", "C");
        assert_is_message("a.b.c.A2", "A2");
        assert_is_message(".a.b.c.A", "A");
        assert_is_message(".a.b.c.A.B", "B");
        assert_is_message(".a.b.c.A.B2", "B2");
        assert_is_message(".a.b.c.A.B.C", "C");
        assert_is_message(".a.b.c.A2", "A2");
        assert_is_message("a.b2.A", "A");
        assert_is_message("a.b2.A.B", "B");
        assert_is_message("a.b2.A.B2", "B2");
        assert_is_message("a.b2.A.B.C", "C");
        assert_is_message("a.b2.A2", "A2");
        assert_is_message(".a.b2.A", "A");
        assert_is_message(".a.b2.A.B", "B");
        assert_is_message(".a.b2.A.B2", "B2");
        assert_is_message(".a.b2.A.B.C", "C");
        assert_is_message(".a.b2.A2", "A2");
        assert_is_enum("E", "E");
        assert_is_enum("A.F", "F");
        assert_is_enum(".E", "E");
        assert_is_enum(".A.F", "F");
        assert_is_enum("a.E", "E");
        assert_is_enum("a.A.F", "F");
        assert_is_enum(".a.E", "E");
        assert_is_enum(".a.A.F", "F");
        assert_is_enum("a.b.E", "E");
        assert_is_enum("a.b.A.F", "F");
        assert_is_enum(".a.b.E", "E");
        assert_is_enum(".a.b.A.F", "F");
        assert_is_enum("a.b.c.E", "E");
        assert_is_enum("a.b.c.A.F", "F");
        assert_is_enum(".a.b.c.E", "E");
        assert_is_enum(".a.b.c.A.F", "F");
        assert_is_enum("a.b2.E", "E");
        assert_is_enum("a.b2.A.F", "F");
        assert_is_enum(".a.b2.E", "E");
        assert_is_enum(".a.b2.A.F", "F");
    }
}
