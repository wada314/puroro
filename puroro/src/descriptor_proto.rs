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

use itertools::Itertools;

use crate::untyped_message::UntypedMessage;
use crate::{ErrorKind, Result};

pub enum Edition {
    EditionUnknown = 0,
    EditionProto2 = 998,
    EditionProto3 = 999,
    Edition2023 = 1000,
    Edition2024 = 1001,
    Edition1TestOnly = 1,
    Edition2TestOnly = 2,
    Edition99997TestOnly = 99997,
    Edition99998TestOnly = 99998,
    Edition99999TestOnly = 99999,
}

pub struct FileDescriptorProto<'a>(UntypedMessage<'a>);
impl<'a> FileDescriptorProto<'a> {
    pub fn name(&self) -> Result<Option<&str>> {
        self.0.field(1).as_scalar_string()
    }
    pub fn package(&self) -> Result<Option<&str>> {
        self.0.field(2).as_scalar_string()
    }
    pub fn dependency(&self) -> impl IntoIterator<Item = Result<&str>> {
        self.0.field(3).as_repeated_string()
    }
    pub fn public_dependency(&self) -> impl '_ + IntoIterator<Item = Result<i32>> {
        self.0
            .field(10)
            .as_repeated_variant(false)
            .into_iter()
            .map_ok(|v| v.try_as_int32())
            .map(Result::flatten)
    }
    pub fn weak_dependency(&self) -> impl '_ + IntoIterator<Item = Result<i32>> {
        self.0
            .field(11)
            .as_repeated_variant(false)
            .into_iter()
            .map_ok(|v| v.try_as_int32())
            .map(Result::flatten)
    }
    pub fn message_type(&self) -> impl IntoIterator<Item = Result<DescriptorProto>> {
        self.0
            .field(4)
            .as_repeated_message()
            .into_iter()
            .map_ok(DescriptorProto)
    }
    pub fn enum_type(&self) -> impl IntoIterator<Item = Result<EnumDescriptorProto>> {
        self.0
            .field(5)
            .as_repeated_message()
            .into_iter()
            .map_ok(EnumDescriptorProto)
    }
    // pub fn service(&self) -> impl IntoIterator<Item = Result<ServiceDescriptorProto>>
    // pub fn extension(&self) -> impl IntoIterator<Item = Result<FieldDescriptorProto>>
    // pub fn options(&self) -> Result<Option<FileOptions>>
    // pub fn source_code_info(&self) -> Result<Option<SourceCodeInfo>>
    // pub fn syntax(&self) -> Result<Option<&'a str>>
}

pub struct DescriptorProto<'a>(UntypedMessage<'a>);
impl<'a> DescriptorProto<'a> {
    pub fn name(&self) -> Result<Option<&str>> {
        self.0.field(1).as_scalar_string()
    }
    pub fn field(&self) -> impl IntoIterator<Item = Result<FieldDescriptorProto>> {
        self.0
            .field(2)
            .as_repeated_message()
            .into_iter()
            .map_ok(FieldDescriptorProto)
    }
    pub fn extension(&self) -> impl IntoIterator<Item = Result<FieldDescriptorProto>> {
        self.0
            .field(6)
            .as_repeated_message()
            .into_iter()
            .map_ok(FieldDescriptorProto)
    }
    pub fn nested_type(&self) -> impl IntoIterator<Item = Result<DescriptorProto>> {
        self.0
            .field(3)
            .as_repeated_message()
            .into_iter()
            .map_ok(DescriptorProto)
    }
    // pub fn extension_range(&self) -> impl IntoIterator<Item = Result<descriptor_proto::ExtensionRangeProto>>
    pub fn enum_type(&self) -> impl IntoIterator<Item = Result<EnumDescriptorProto>> {
        self.0
            .field(4)
            .as_repeated_message()
            .into_iter()
            .map_ok(EnumDescriptorProto)
    }
    pub fn oneof_decl(&self) -> impl IntoIterator<Item = Result<OneofDescriptorProto>> {
        self.0
            .field(8)
            .as_repeated_message()
            .into_iter()
            .map_ok(OneofDescriptorProto)
    }
    // pub fn options(&self) -> Result<Option<MessageOptions>>
    // pub fn reserved_range(&self) -> impl IntoIterator<Item = Result<descriptor_proto::ReservedRangeProto>>
    // pub fn reserved_name(&self) -> impl IntoIterator<Item = Result<&'a str>>
}

pub struct FieldDescriptorProto<'a>(UntypedMessage<'a>);
impl<'a> FieldDescriptorProto<'a> {
    pub fn name(&self) -> Result<Option<&str>> {
        self.0.field(1).as_scalar_string()
    }
    pub fn number(&self) -> Result<Option<i32>> {
        self.0
            .field(3)
            .as_scalar_variant(true)?
            .map(|v| v.try_as_int32())
            .transpose()
    }
    pub fn label(&self) -> Result<Option<self::field_descriptor_proto::Label>> {
        self.0
            .field(4)
            .as_scalar_variant(true)?
            .map(|v| v.try_as_int32()?.try_into())
            .transpose()
    }
    pub fn type_(&self) -> Result<Option<field_descriptor_proto::Type>> {
        self.0
            .field(5)
            .as_scalar_variant(true)?
            .map(|v| v.try_as_int32()?.try_into())
            .transpose()
    }
    pub fn type_name(&self) -> Result<Option<&str>> {
        self.0.field(6).as_scalar_string()
    }
    pub fn extendee(&self) -> Result<Option<&str>> {
        self.0.field(2).as_scalar_string()
    }
    pub fn default_value(&self) -> Result<Option<&str>> {
        self.0.field(7).as_scalar_string()
    }
    pub fn oneof_index(&self) -> Result<Option<i32>> {
        self.0
            .field(9)
            .as_scalar_variant(true)?
            .map(|v| v.try_as_int32())
            .transpose()
    }
    pub fn json_name(&self) -> Result<Option<&str>> {
        self.0.field(10).as_scalar_string()
    }
    // pub fn options(&self) -> Result<Option<FieldOptions>>
    pub fn proto3_optional(&self) -> Result<Option<bool>> {
        self.0
            .field(17)
            .as_scalar_variant(true)?
            .map(|v| v.try_as_bool())
            .transpose()
    }
}

pub mod field_descriptor_proto {
    use super::*;
    pub enum Type {
        TypeDouble = 1,
        TypeFloat = 2,
        TypeInt64 = 3,
        TypeUInt64 = 4,
        TypeInt32 = 5,
        TypeFixed64 = 6,
        TypeFixed32 = 7,
        TypeBool = 8,
        TypeString = 9,
        TypeGroup = 10,
        TypeMessage = 11,
        TypeBytes = 12,
        TypeUInt32 = 13,
        TypeEnum = 14,
        TypeSFixed32 = 15,
        TypeSFixed64 = 16,
        TypeSInt32 = 17,
        TypeSInt64 = 18,
    }

    pub enum Label {
        LabelOptional = 1,
        LabelRepeated = 2,
        LabelRequired = 3,
    }

    impl TryFrom<i32> for Type {
        type Error = ErrorKind;
        fn try_from(value: i32) -> Result<Self> {
            match value {
                1 => Ok(Self::TypeDouble),
                2 => Ok(Self::TypeFloat),
                3 => Ok(Self::TypeInt64),
                4 => Ok(Self::TypeUInt64),
                5 => Ok(Self::TypeInt32),
                6 => Ok(Self::TypeFixed64),
                7 => Ok(Self::TypeFixed32),
                8 => Ok(Self::TypeBool),
                9 => Ok(Self::TypeString),
                10 => Ok(Self::TypeGroup),
                11 => Ok(Self::TypeMessage),
                12 => Ok(Self::TypeBytes),
                13 => Ok(Self::TypeUInt32),
                14 => Ok(Self::TypeEnum),
                15 => Ok(Self::TypeSFixed32),
                16 => Ok(Self::TypeSFixed64),
                17 => Ok(Self::TypeSInt32),
                18 => Ok(Self::TypeSInt64),
                _ => Err(ErrorKind::TryFromIntIntoEnumError(value)),
            }
        }
    }

    impl TryFrom<i32> for Label {
        type Error = ErrorKind;
        fn try_from(value: i32) -> Result<Self> {
            match value {
                1 => Ok(Self::LabelOptional),
                2 => Ok(Self::LabelRepeated),
                3 => Ok(Self::LabelRequired),
                _ => Err(ErrorKind::TryFromIntIntoEnumError(value)),
            }
        }
    }
}

pub struct OneofDescriptorProto<'a>(UntypedMessage<'a>);
impl<'a> OneofDescriptorProto<'a> {
    pub fn name(&self) -> Result<Option<&str>> {
        self.0.field(1).as_scalar_string()
    }
    // pub fn options(&self) -> Result<Option<OneofOptions>>
}

pub struct EnumDescriptorProto<'a>(UntypedMessage<'a>);
impl<'a> EnumDescriptorProto<'a> {
    pub fn name(&self) -> Result<Option<&str>> {
        self.0.field(1).as_scalar_string()
    }
    pub fn value(&self) -> impl IntoIterator<Item = Result<EnumValueDescriptorProto>> {
        self.0
            .field(2)
            .as_repeated_message()
            .into_iter()
            .map_ok(EnumValueDescriptorProto)
    }
    // pub fn options(&self) -> Result<Option<EnumOptions>>
    // pub fn reserved_range(&self) -> impl IntoIterator<Item = Result<ReservedRange>>
    // pub fn reserved_name(&self) -> impl IntoIterator<Item = Result<&'a str>>
}

pub struct EnumValueDescriptorProto<'a>(UntypedMessage<'a>);
impl<'a> EnumValueDescriptorProto<'a> {
    pub fn name(&self) -> Result<Option<&str>> {
        self.0.field(1).as_scalar_string()
    }
    pub fn number(&self) -> Result<Option<i32>> {
        self.0
            .field(2)
            .as_scalar_variant(true)?
            .map(|v| v.try_as_int32())
            .transpose()
    }
}
