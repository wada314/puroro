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

use super::GenericMessageExt;
use crate::generic_message::GenericMessage;
use crate::internal::impl_message_trait_for_trivial_types;
use crate::variant::variant_types::{Bool, Enum, Int32, Int64, UInt64};
use crate::Result;
use ::derive_more::{Deref as DDeref, DerefMut as DDerefMut, From as DFrom, Into as DInto};
use ::ref_cast::RefCast;
use ::std::alloc::Allocator;

#[derive(DDeref, DDerefMut, DFrom, DInto, Default, Debug, RefCast)]
#[repr(transparent)]
pub struct FileDescriptorSet(GenericMessage);
impl FileDescriptorSet {
    pub fn file(&self) -> impl Iterator<Item = &FileDescriptorProto> {
        self.0.as_repeated_message(1)
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

#[derive(DDeref, DDerefMut, DFrom, DInto, Default, Debug, RefCast)]
#[repr(transparent)]
pub struct FileDescriptorProto(GenericMessage);
impl FileDescriptorProto {
    pub fn name(&self) -> &str {
        self.as_scalar_string(1)
    }
    pub fn package(&self) -> &str {
        self.as_scalar_string(2)
    }
    pub fn dependency(&self) -> impl Iterator<Item = &str> {
        self.as_repeated_string(3)
    }
    pub fn public_dependency(&self) -> impl '_ + Iterator<Item = i32> {
        self.as_repeated_i32(10)
    }
    pub fn weak_dependency(&self) -> impl '_ + Iterator<Item = i32> {
        self.as_repeated_i32(11)
    }
    pub fn message_type(&self) -> impl Iterator<Item = &DescriptorProto> {
        self.as_repeated_message(4)
    }
    pub fn enum_type(&self) -> impl Iterator<Item = &EnumDescriptorProto> {
        self.as_repeated_message(5)
    }
    // pub fn service(&self) -> impl Iterator<Item = &ServiceDescriptorProto>
    // pub fn extension(&self) -> impl Iterator<Item = &FieldDescriptorProto>
    pub fn options(&self) -> Option<&FileOptionsProto> {
        self.as_scalar_message(8)
    }
    // pub fn source_code_info(&self) -> Option<&SourceCodeInfoProto>
    pub fn syntax(&self) -> &str {
        self.as_scalar_string(12)
    }
    pub fn edition(&self) -> Edition {
        self.as_scalar_enum(14)
    }
}

#[derive(DDeref, DDerefMut, DFrom, DInto, Default, Debug, RefCast)]
#[repr(transparent)]
pub struct DescriptorProto(GenericMessage);
impl DescriptorProto {
    pub fn name(&self) -> Result<Option<&str>> {
        self.0.field(1).try_as_scalar_string_opt()
    }
    pub fn field(&self) -> impl Iterator<Item = Result<&FieldDescriptorProto>> {
        self.try_as_repeated_message(2)
    }
    pub fn extension(&self) -> impl Iterator<Item = Result<&FieldDescriptorProto>> {
        self.try_as_repeated_message(6)
    }
    pub fn nested_type(&self) -> impl Iterator<Item = Result<&DescriptorProto>> {
        self.try_as_repeated_message(3)
    }
    // pub fn extension_range(&self) -> impl Iterator<Item = Result<descriptor_proto::ExtensionRangeProto>>
    pub fn enum_type(&self) -> impl Iterator<Item = Result<&EnumDescriptorProto>> {
        self.try_as_repeated_message(4)
    }
    pub fn oneof_decl(&self) -> impl Iterator<Item = Result<&OneofDescriptorProto>> {
        self.try_as_repeated_message(8)
    }
    // pub fn options(&self) -> Result<Option<MessageOptions>>
    // pub fn reserved_range(&self) -> impl Iterator<Item = Result<descriptor_proto::ReservedRangeProto>>
    // pub fn reserved_name(&self) -> impl Iterator<Item = Result<&'a str>>
}

#[derive(DDeref, DDerefMut, DFrom, DInto, Default, Debug, RefCast)]
#[repr(transparent)]
pub struct FieldDescriptorProto(GenericMessage);
impl FieldDescriptorProto {
    pub fn name(&self) -> Result<Option<&str>> {
        self.0.field(1).try_as_scalar_string_opt()
    }
    pub fn number(&self) -> Result<Option<i32>> {
        self.0.field(3).try_as_scalar_variant_opt::<Int32>(false)
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
        self.0.field(9).try_as_scalar_variant_opt::<Int32>(false)
    }
    pub fn json_name(&self) -> Result<Option<&str>> {
        self.0.field(10).try_as_scalar_string_opt()
    }
    // pub fn options(&self) -> Result<Option<FieldOptions>>
    pub fn proto3_optional(&self) -> Result<Option<bool>> {
        self.0.field(17).try_as_scalar_variant_opt::<Bool>(false)
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

#[derive(DDeref, DDerefMut, DFrom, DInto, Default, Debug, RefCast)]
#[repr(transparent)]
pub struct OneofDescriptorProto(GenericMessage);
impl OneofDescriptorProto {
    pub fn name(&self) -> Result<Option<&str>> {
        self.0.field(1).try_as_scalar_string_opt()
    }
    // pub fn options(&self) -> Result<Option<OneofOptions>>
}

#[derive(DDeref, DDerefMut, DFrom, DInto, Default, Debug, RefCast)]
#[repr(transparent)]
pub struct EnumDescriptorProto(GenericMessage);
impl EnumDescriptorProto {
    pub fn name(&self) -> Result<Option<&str>> {
        self.0.field(1).try_as_scalar_string_opt()
    }
    pub fn value(&self) -> impl Iterator<Item = Result<&EnumValueDescriptorProto>> {
        self.try_as_repeated_message(2)
    }
    // pub fn options(&self) -> Result<Option<EnumOptions>>
    // pub fn reserved_range(&self) -> impl Iterator<Item = Result<ReservedRange>>
    // pub fn reserved_name(&self) -> impl Iterator<Item = Result<&'a str>>
}

#[derive(DDeref, DDerefMut, DFrom, DInto, Default, Debug, RefCast)]
#[repr(transparent)]
pub struct EnumValueDescriptorProto(GenericMessage);
impl EnumValueDescriptorProto {
    pub fn name(&self) -> Result<Option<&str>> {
        self.0.field(1).try_as_scalar_string_opt()
    }
    pub fn number(&self) -> Result<Option<i32>> {
        self.0.field(2).try_as_scalar_variant_opt::<Int32>(false)
    }
}

#[derive(DDeref, DDerefMut, DFrom, DInto, Default, Debug, RefCast)]
#[repr(transparent)]
pub struct FileOptionsProto(GenericMessage);
impl FileOptionsProto {
    pub fn uninterpreted_option(&self) -> impl Iterator<Item = Result<&UninterpretedOptionProto>> {
        self.try_as_repeated_message(9)
    }
}

#[derive(DDeref, DDerefMut, DFrom, DInto, Default, Debug, RefCast)]
#[repr(transparent)]
pub struct UninterpretedOptionProto(GenericMessage);
impl UninterpretedOptionProto {
    pub fn name(&self) -> impl Iterator<Item = Result<&uninterpreted_option::NamePart>> {
        self.0.try_as_repeated_message(2)
    }
    pub fn identifier_value(&self) -> Result<Option<&str>> {
        self.0.field(3).try_as_scalar_string_opt()
    }
    pub fn positive_int_value(&self) -> Result<Option<u64>> {
        self.0.field(4).try_as_scalar_variant_opt::<UInt64>(false)
    }
    pub fn negative_int_value(&self) -> Result<Option<i64>> {
        self.0.field(5).try_as_scalar_variant_opt::<Int64>(false)
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
    #[derive(DDeref, DDerefMut, DFrom, DInto, Default, Debug, RefCast)]
    #[repr(transparent)]
    pub struct NamePart(pub(crate) GenericMessage);
    impl NamePart {
        pub fn name_part(&self) -> Result<Option<&str>> {
            self.0.field(1).try_as_scalar_string_opt()
        }
        pub fn is_extension(&self) -> Result<Option<bool>> {
            self.0.field(2).try_as_scalar_variant_opt::<Bool>(false)
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

impl<A: Allocator + Clone> FileDescriptorSetTrait for GenericMessage<A> {
    fn file(&self) -> impl Iterator<Item = impl FileDescriptorTrait> {
        self.field(1).as_repeated_message()
    }
}

impl<A: Allocator + Clone> FileDescriptorTrait for GenericMessage<A> {
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

impl<A: Allocator + Clone> DescriptorTrait for GenericMessage<A> {
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

impl<A: Allocator + Clone> FieldDescriptorTrait for GenericMessage<A> {
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

impl<A: Allocator + Clone> OneofDescriptorTrait for GenericMessage<A> {
    fn name(&self) -> &str {
        self.field(1).as_scalar_string()
    }
}

impl<A: Allocator + Clone> EnumDescriptorTrait for GenericMessage<A> {
    fn name(&self) -> &str {
        self.field(1).as_scalar_string()
    }
    fn value(&self) -> impl Iterator<Item = impl EnumValueDescriptorTrait> {
        self.field(2).as_repeated_message()
    }
}

impl<A: Allocator + Clone> EnumValueDescriptorTrait for GenericMessage<A> {
    fn name(&self) -> &str {
        self.field(1).as_scalar_string()
    }
    fn number(&self) -> i32 {
        self.field(2).as_scalar_variant::<Int32>(false)
    }
}
