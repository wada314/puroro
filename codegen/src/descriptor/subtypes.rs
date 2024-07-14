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

#[derive(Clone, Default)]
pub enum FieldType<M, E> {
    Bool,
    Bytes,
    Double,
    Enum(E),
    Fixed32,
    Fixed64,
    Float,
    Group,
    #[default]
    Int32,
    Int64,
    Message(M),
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
impl<M, E> Debug for FieldType<M, E> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool => write!(f, "Bool"),
            Self::Bytes => write!(f, "Bytes"),
            Self::Double => write!(f, "Double"),
            Self::Enum(_) => write!(f, "Enum"),
            Self::Fixed32 => write!(f, "Fixed32"),
            Self::Fixed64 => write!(f, "Fixed64"),
            Self::Float => write!(f, "Float"),
            Self::Group => write!(f, "Group"),
            Self::Int32 => write!(f, "Int32"),
            Self::Int64 => write!(f, "Int64"),
            Self::Message(_) => write!(f, "Message"),
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
impl<M, E> FieldType<M, E> {
    pub fn map<FM, FE, M2, E2>(self, fm: FM, fe: FE) -> FieldType<M2, E2>
    where
        FM: FnOnce(M) -> M2,
        FE: FnOnce(E) -> E2,
    {
        match self {
            FieldType::Bool => FieldType::Bool,
            FieldType::Bytes => FieldType::Bytes,
            FieldType::Double => FieldType::Double,
            FieldType::Enum(e) => FieldType::Enum(fe(e)),
            FieldType::Fixed32 => FieldType::Fixed32,
            FieldType::Fixed64 => FieldType::Fixed64,
            FieldType::Float => FieldType::Float,
            FieldType::Group => FieldType::Group,
            FieldType::Int32 => FieldType::Int32,
            FieldType::Int64 => FieldType::Int64,
            FieldType::Message(m) => FieldType::Message(fm(m)),
            FieldType::SFixed32 => FieldType::SFixed32,
            FieldType::SFixed64 => FieldType::SFixed64,
            FieldType::SInt32 => FieldType::SInt32,
            FieldType::SInt64 => FieldType::SInt64,
            FieldType::String => FieldType::String,
            FieldType::UInt32 => FieldType::UInt32,
            FieldType::UInt64 => FieldType::UInt64,
        }
    }
    pub fn try_map<FM, FE, M2, E2>(self, fm: FM, fe: FE) -> Result<FieldType<M2, E2>>
    where
        FM: FnOnce(M) -> Result<M2>,
        FE: FnOnce(E) -> Result<E2>,
    {
        Ok(match self {
            FieldType::Bool => FieldType::Bool,
            FieldType::Bytes => FieldType::Bytes,
            FieldType::Double => FieldType::Double,
            FieldType::Enum(e) => FieldType::Enum(fe(e)?),
            FieldType::Fixed32 => FieldType::Fixed32,
            FieldType::Fixed64 => FieldType::Fixed64,
            FieldType::Float => FieldType::Float,
            FieldType::Group => FieldType::Group,
            FieldType::Int32 => FieldType::Int32,
            FieldType::Int64 => FieldType::Int64,
            FieldType::Message(m) => FieldType::Message(fm(m)?),
            FieldType::SFixed32 => FieldType::SFixed32,
            FieldType::SFixed64 => FieldType::SFixed64,
            FieldType::SInt32 => FieldType::SInt32,
            FieldType::SInt64 => FieldType::SInt64,
            FieldType::String => FieldType::String,
            FieldType::UInt32 => FieldType::UInt32,
            FieldType::UInt64 => FieldType::UInt64,
        })
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
    pub fn with_type_ref<'a, F, G, M, E>(
        self,
        type_name: Option<&str>,
        msg_case: F,
        enum_case: G,
    ) -> Result<FieldType<M, E>>
    where
        F: FnOnce(&ProtoPath) -> Result<M>,
        G: FnOnce(&ProtoPath) -> Result<E>,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WireType<V, I32, I64, L> {
    Varint(V),
    I64(I64),
    Len(L),
    StartGroup,
    EndGroup,
    I32(I32),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WireTypeCase {
    Varint,
    I64,
    Len,
    StartGroup,
    EndGroup,
    I32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VariantType<E> {
    Int32,
    Int64,
    UInt32,
    UInt64,
    SInt32,
    SInt64,
    Bool,
    Enum(E),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VariantTypeCase {
    Int32,
    Int64,
    UInt32,
    UInt64,
    SInt32,
    SInt64,
    Bool,
    Enum,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum I32Type {
    Float,
    Fixed32,
    SFixed32,
}

pub type I32TypeCase = I32Type;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum I64Type {
    Double,
    Fixed64,
    SFixed64,
}

pub type I64TypeCase = I64Type;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum LenType<M> {
    Bytes,
    String,
    Message(M),
}

pub enum LenTypeCase {
    Bytes,
    String,
    Message,
}

impl<M> Debug for LenType<M> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            LenType::Bytes => write!(f, "Bytes"),
            LenType::String => write!(f, "String"),
            LenType::Message(_) => write!(f, "Message"),
        }
    }
}

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
