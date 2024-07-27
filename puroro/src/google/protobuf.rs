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

pub mod compiler;

use crate::generic_message::GenericMessage;
use crate::internal::impl_message_trait_for_trivial_types;
use crate::variant::variant_types::{Bool, Enum, Int32, Int64, UInt64};
use crate::Result;
use ::derive_more::{Deref as DDeref, DerefMut as DDerefMut, From as DFrom, Into as DInto};

#[derive(DDeref, DDerefMut, DFrom, DInto, Default, Debug)]
pub struct FileDescriptorSet<'a>(GenericMessage<'a>);
impl<'a> FileDescriptorSet<'a> {
    pub fn file(&self) -> impl Iterator<Item = Result<FileDescriptorProto>> {
        self.0.repeated_message_field(1, FileDescriptorProto)
    }
}

#[derive(Default, Debug)]
pub enum Edition {
    #[default]
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
impl TryFrom<i32> for Edition {
    type Error = i32;
    fn try_from(value: i32) -> ::std::result::Result<Self, i32> {
        match value {
            0 => Ok(Self::EditionUnknown),
            998 => Ok(Self::EditionProto2),
            999 => Ok(Self::EditionProto3),
            1000 => Ok(Self::Edition2023),
            1001 => Ok(Self::Edition2024),
            1 => Ok(Self::Edition1TestOnly),
            2 => Ok(Self::Edition2TestOnly),
            99997 => Ok(Self::Edition99997TestOnly),
            99998 => Ok(Self::Edition99998TestOnly),
            99999 => Ok(Self::Edition99999TestOnly),
            _ => Err(value),
        }
    }
}
impl From<Edition> for i32 {
    fn from(value: Edition) -> i32 {
        value as i32
    }
}

#[derive(DDeref, DDerefMut, DFrom, DInto, Default, Debug)]
pub struct FileDescriptorProto<'a>(GenericMessage<'a>);
impl<'a> FileDescriptorProto<'a> {
    pub fn name(&self) -> Result<Option<&str>> {
        self.0.field(1).try_as_scalar_string_opt()
    }
    pub fn package(&self) -> Result<Option<&str>> {
        self.0.field(2).try_as_scalar_string_opt()
    }
    pub fn dependency(&self) -> impl Iterator<Item = Result<&str>> {
        self.0.field(3).try_as_repeated_string()
    }
    pub fn public_dependency(&self) -> impl '_ + Iterator<Item = Result<i32>> {
        self.0.repeated_variant_field::<Int32>(10)
    }
    pub fn weak_dependency(&self) -> impl '_ + Iterator<Item = Result<i32>> {
        self.0.repeated_variant_field::<Int32>(11)
    }
    pub fn message_type(&self) -> impl Iterator<Item = Result<DescriptorProto>> {
        self.0.repeated_message_field(4, DescriptorProto)
    }
    pub fn enum_type(&self) -> impl Iterator<Item = Result<EnumDescriptorProto>> {
        self.0.repeated_message_field(5, EnumDescriptorProto)
    }
    // pub fn service(&self) -> impl Iterator<Item = Result<ServiceDescriptorProto>>
    // pub fn extension(&self) -> impl Iterator<Item = Result<FieldDescriptorProto>>
    pub fn options(&self) -> Result<Option<FileOptionsProto>> {
        self.0.scalar_message_field(8, FileOptionsProto)
    }
    // pub fn source_code_info(&self) -> Result<Option<SourceCodeInfoProto>>
    pub fn syntax(&self) -> Result<Option<&str>> {
        self.0.field(12).try_as_scalar_string_opt()
    }
    pub fn edition(&self) -> Result<Option<Edition>> {
        self.0
            .field(14)
            .try_as_scalar_variant_opt::<Enum<Edition>>(false)
    }
}

#[derive(DDeref, DDerefMut, DFrom, DInto, Default, Debug)]
pub struct DescriptorProto<'a>(GenericMessage<'a>);
impl<'a> DescriptorProto<'a> {
    pub fn name(&self) -> Result<Option<&str>> {
        self.0.field(1).try_as_scalar_string_opt()
    }
    pub fn field(&self) -> impl Iterator<Item = Result<FieldDescriptorProto>> {
        self.0.repeated_message_field(2, FieldDescriptorProto)
    }
    pub fn extension(&self) -> impl Iterator<Item = Result<FieldDescriptorProto>> {
        self.0.repeated_message_field(6, FieldDescriptorProto)
    }
    pub fn nested_type(&self) -> impl Iterator<Item = Result<DescriptorProto>> {
        self.0.repeated_message_field(3, DescriptorProto)
    }
    // pub fn extension_range(&self) -> impl Iterator<Item = Result<descriptor_proto::ExtensionRangeProto>>
    pub fn enum_type(&self) -> impl Iterator<Item = Result<EnumDescriptorProto>> {
        self.0.repeated_message_field(4, EnumDescriptorProto)
    }
    pub fn oneof_decl(&self) -> impl Iterator<Item = Result<OneofDescriptorProto>> {
        self.0.repeated_message_field(8, OneofDescriptorProto)
    }
    // pub fn options(&self) -> Result<Option<MessageOptions>>
    // pub fn reserved_range(&self) -> impl Iterator<Item = Result<descriptor_proto::ReservedRangeProto>>
    // pub fn reserved_name(&self) -> impl Iterator<Item = Result<&'a str>>
}

#[derive(DDeref, DDerefMut, DFrom, DInto, Default, Debug)]
pub struct FieldDescriptorProto<'a>(GenericMessage<'a>);
impl<'a> FieldDescriptorProto<'a> {
    pub fn name(&self) -> Result<Option<&str>> {
        self.0.field(1).try_as_scalar_string_opt()
    }
    pub fn number(&self) -> Result<Option<i32>> {
        self.0.scalar_variant_field::<Int32>(3)
    }
    pub fn label(&self) -> Result<Option<self::field_descriptor_proto::Label>> {
        self.0
            .field(4)
            .try_as_scalar_variant_opt::<Enum<self::field_descriptor_proto::Label>>(false)
    }
    pub fn type_(&self) -> Result<Option<field_descriptor_proto::Type>> {
        self.0
            .field(5)
            .try_as_scalar_variant_opt::<Enum<field_descriptor_proto::Type>>(false)
    }
    pub fn type_name(&self) -> Result<Option<&str>> {
        self.0.field(6).try_as_scalar_string_opt()
    }
    pub fn extendee(&self) -> Result<Option<&str>> {
        self.0.field(2).try_as_scalar_string_opt()
    }
    pub fn default_value(&self) -> Result<Option<&str>> {
        self.0.field(7).try_as_scalar_string_opt()
    }
    pub fn oneof_index(&self) -> Result<Option<i32>> {
        self.0.scalar_variant_field::<Int32>(9)
    }
    pub fn json_name(&self) -> Result<Option<&str>> {
        self.0.field(10).try_as_scalar_string_opt()
    }
    // pub fn options(&self) -> Result<Option<FieldOptions>>
    pub fn proto3_optional(&self) -> Result<Option<bool>> {
        self.0.scalar_variant_field::<Bool>(17)
    }
}

pub mod field_descriptor_proto {
    #[derive(Default, Debug)]
    pub enum Type {
        TypeDouble = 1,
        TypeFloat = 2,
        TypeInt64 = 3,
        TypeUInt64 = 4,
        #[default]
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

    #[derive(Default, Debug)]
    pub enum Label {
        #[default]
        LabelOptional = 1,
        LabelRepeated = 3,
        LabelRequired = 2,
    }

    impl TryFrom<i32> for Type {
        type Error = i32;
        fn try_from(value: i32) -> ::std::result::Result<Self, i32> {
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
                _ => Err(value),
            }
        }
    }
    impl From<Type> for i32 {
        fn from(value: Type) -> i32 {
            value as i32
        }
    }

    impl TryFrom<i32> for Label {
        type Error = i32;
        fn try_from(value: i32) -> ::std::result::Result<Self, i32> {
            match value {
                1 => Ok(Self::LabelOptional),
                3 => Ok(Self::LabelRepeated),
                2 => Ok(Self::LabelRequired),
                _ => Err(value),
            }
        }
    }
    impl From<Label> for i32 {
        fn from(value: Label) -> i32 {
            value as i32
        }
    }
}

#[derive(DDeref, DDerefMut, DFrom, DInto, Default, Debug)]
pub struct OneofDescriptorProto<'a>(GenericMessage<'a>);
impl<'a> OneofDescriptorProto<'a> {
    pub fn name(&self) -> Result<Option<&str>> {
        self.0.field(1).try_as_scalar_string_opt()
    }
    // pub fn options(&self) -> Result<Option<OneofOptions>>
}

#[derive(DDeref, DDerefMut, DFrom, DInto, Default, Debug)]
pub struct EnumDescriptorProto<'a>(GenericMessage<'a>);
impl<'a> EnumDescriptorProto<'a> {
    pub fn name(&self) -> Result<Option<&str>> {
        self.0.field(1).try_as_scalar_string_opt()
    }
    pub fn value(&self) -> impl Iterator<Item = Result<EnumValueDescriptorProto>> {
        self.0.repeated_message_field(2, EnumValueDescriptorProto)
    }
    // pub fn options(&self) -> Result<Option<EnumOptions>>
    // pub fn reserved_range(&self) -> impl Iterator<Item = Result<ReservedRange>>
    // pub fn reserved_name(&self) -> impl Iterator<Item = Result<&'a str>>
}

#[derive(DDeref, DDerefMut, DFrom, DInto, Default, Debug)]
pub struct EnumValueDescriptorProto<'a>(GenericMessage<'a>);
impl<'a> EnumValueDescriptorProto<'a> {
    pub fn name(&self) -> Result<Option<&str>> {
        self.0.field(1).try_as_scalar_string_opt()
    }
    pub fn number(&self) -> Result<Option<i32>> {
        self.0.scalar_variant_field::<Int32>(2)
    }
}

#[derive(DDeref, DDerefMut, DFrom, DInto, Default, Debug)]
pub struct FileOptionsProto<'a>(GenericMessage<'a>);
impl<'a> FileOptionsProto<'a> {
    pub fn uninterpreted_option(&self) -> impl Iterator<Item = Result<UninterpretedOptionProto>> {
        self.0.repeated_message_field(999, UninterpretedOptionProto)
    }
}

#[derive(DDeref, DDerefMut, DFrom, DInto, Default, Debug)]
pub struct UninterpretedOptionProto<'a>(GenericMessage<'a>);
impl<'a> UninterpretedOptionProto<'a> {
    pub fn name(&self) -> impl Iterator<Item = Result<uninterpreted_option::NamePart>> {
        self.0
            .repeated_message_field(2, uninterpreted_option::NamePart)
    }
    pub fn identifier_value(&self) -> Result<Option<&str>> {
        self.0.field(3).try_as_scalar_string_opt()
    }
    pub fn positive_int_value(&self) -> Result<Option<u64>> {
        self.0.scalar_variant_field::<UInt64>(4)
    }
    pub fn negative_int_value(&self) -> Result<Option<i64>> {
        self.0.scalar_variant_field::<Int64>(5)
    }
    pub fn double_value(&self) -> Result<Option<f64>> {
        Ok(self
            .0
            .field(6)
            .try_as_scalar_i64_opt()?
            .map(f64::from_le_bytes))
    }
    pub fn string_value(&self) -> Result<Option<&[u8]>> {
        self.0.field(7).try_as_scalar_bytes_opt()
    }
    pub fn aggregate_value(&self) -> Result<Option<&str>> {
        self.0.field(8).try_as_scalar_string_opt()
    }
}

pub mod uninterpreted_option {
    use super::*;
    #[derive(DDeref, DDerefMut, DFrom, DInto, Default, Debug)]
    pub struct NamePart<'a>(pub(crate) GenericMessage<'a>);
    impl<'a> NamePart<'a> {
        pub fn name_part(&self) -> Result<Option<&str>> {
            self.0.field(1).try_as_scalar_string_opt()
        }
        pub fn is_extension(&self) -> Result<Option<bool>> {
            self.0.scalar_variant_field::<Bool>(2)
        }
    }
}

impl_message_trait_for_trivial_types! {
    pub trait FileDescriptorSetTrait {
        fn file(&self) -> impl Iterator<Item = impl FileDescriptorTrait>;
    }

    pub trait FileDescriptorTrait {
        fn name(&self) -> &str;
        fn package(&self) -> &str;
        fn dependency(&self) -> impl Iterator<Item = &str>;
        fn public_dependency(&self) -> impl Iterator<Item = i32>;
        fn weak_dependency(&self) -> impl Iterator<Item = i32>;
        fn message_type(&self) -> impl Iterator<Item = impl DescriptorTrait>;
        fn enum_type(&self) -> impl Iterator<Item = impl EnumDescriptorTrait>;
        fn syntax(&self) -> &str;
        fn edition(&self) -> Edition;
    }

    pub trait DescriptorTrait {
        fn name(&self) -> &str;
        fn field(&self) -> impl Iterator<Item = impl FieldDescriptorTrait>;
        fn extension(&self) -> impl Iterator<Item = impl FieldDescriptorTrait>;
        fn nested_type(&self) -> impl Iterator<Item = impl DescriptorTrait>;
        fn enum_type(&self) -> impl Iterator<Item = impl EnumDescriptorTrait>;
        fn oneof_decl(&self) -> impl Iterator<Item = impl OneofDescriptorTrait>;
    }

    pub trait FieldDescriptorTrait {
        fn name(&self) -> &str;
        fn number(&self) -> i32;
        fn label(&self) -> field_descriptor_proto::Label;
        fn r#type(&self) -> field_descriptor_proto::Type;
        fn type_name(&self) -> &str;
        fn extendee(&self) -> &str;
        fn default_value(&self) -> &str;
        fn oneof_index(&self) -> i32;
        fn json_name(&self) -> &str;
        fn proto3_optional(&self) -> bool;
    }

    pub trait OneofDescriptorTrait {
        fn name(&self) -> &str;
    }

    pub trait EnumDescriptorTrait {
        fn name(&self) -> &str;
        fn value(&self) -> impl Iterator<Item = impl EnumValueDescriptorTrait>;
    }

    pub trait EnumValueDescriptorTrait {
        fn name(&self) -> &str;
        fn number(&self) -> i32;
    }
}

impl FileDescriptorSetTrait for GenericMessage<'_> {
    fn file(&self) -> impl Iterator<Item = impl FileDescriptorTrait> {
        self.field(1).as_repeated_message()
    }
}

impl FileDescriptorTrait for GenericMessage<'_> {
    fn name(&self) -> &str {
        self.field(1).as_scalar_string()
    }
    fn package(&self) -> &str {
        self.field(2).as_scalar_string()
    }
    fn dependency(&self) -> impl Iterator<Item = &str> {
        self.field(3).as_repeated_string()
    }
    fn public_dependency(&self) -> impl Iterator<Item = i32> {
        self.field(10).as_repeated_variant::<Int32>(false)
    }
    fn weak_dependency(&self) -> impl Iterator<Item = i32> {
        self.field(11).as_repeated_variant::<Int32>(false)
    }
    fn message_type(&self) -> impl Iterator<Item = impl DescriptorTrait> {
        self.field(4).as_repeated_message()
    }
    fn enum_type(&self) -> impl Iterator<Item = impl EnumDescriptorTrait> {
        self.field(5).as_repeated_message()
    }
    fn syntax(&self) -> &str {
        self.field(12).as_scalar_string()
    }
    fn edition(&self) -> Edition {
        self.field(14).as_scalar_variant::<Enum<Edition>>(false)
    }
}

impl DescriptorTrait for GenericMessage<'_> {
    fn name(&self) -> &str {
        self.field(1).as_scalar_string()
    }
    fn field(&self) -> impl Iterator<Item = impl FieldDescriptorTrait> {
        self.field(2).as_repeated_message()
    }
    fn extension(&self) -> impl Iterator<Item = impl FieldDescriptorTrait> {
        self.field(6).as_repeated_message()
    }
    fn nested_type(&self) -> impl Iterator<Item = impl DescriptorTrait> {
        self.field(3).as_repeated_message()
    }
    fn enum_type(&self) -> impl Iterator<Item = impl EnumDescriptorTrait> {
        self.field(4).as_repeated_message()
    }
    fn oneof_decl(&self) -> impl Iterator<Item = impl OneofDescriptorTrait> {
        self.field(8).as_repeated_message()
    }
}

impl FieldDescriptorTrait for GenericMessage<'_> {
    fn name(&self) -> &str {
        self.field(1).as_scalar_string()
    }
    fn number(&self) -> i32 {
        self.field(3).as_scalar_variant::<Int32>(false)
    }
    fn label(&self) -> field_descriptor_proto::Label {
        self.field(4)
            .as_scalar_variant::<Enum<field_descriptor_proto::Label>>(false)
    }
    fn r#type(&self) -> field_descriptor_proto::Type {
        self.field(5)
            .as_scalar_variant::<Enum<field_descriptor_proto::Type>>(false)
    }
    fn type_name(&self) -> &str {
        self.field(6).as_scalar_string()
    }
    fn extendee(&self) -> &str {
        self.field(2).as_scalar_string()
    }
    fn default_value(&self) -> &str {
        self.field(7).as_scalar_string()
    }
    fn oneof_index(&self) -> i32 {
        self.field(9).as_scalar_variant::<Int32>(false)
    }
    fn json_name(&self) -> &str {
        self.field(10).as_scalar_string()
    }
    fn proto3_optional(&self) -> bool {
        self.field(17).as_scalar_variant::<Bool>(false)
    }
}

impl OneofDescriptorTrait for GenericMessage<'_> {
    fn name(&self) -> &str {
        self.field(1).as_scalar_string()
    }
}

impl EnumDescriptorTrait for GenericMessage<'_> {
    fn name(&self) -> &str {
        self.field(1).as_scalar_string()
    }
    fn value(&self) -> impl Iterator<Item = impl EnumValueDescriptorTrait> {
        self.field(2).as_repeated_message()
    }
}

impl EnumValueDescriptorTrait for GenericMessage<'_> {
    fn name(&self) -> &str {
        self.field(1).as_scalar_string()
    }
    fn number(&self) -> i32 {
        self.field(2).as_scalar_variant::<Int32>(false)
    }
}
