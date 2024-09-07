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
use ::derive_more::{Deref, DerefMut, From, Into};
use ::ref_cast::RefCast;
use ::std::alloc::{Allocator, Global};

#[derive(Deref, DerefMut, From, Into, Default, Debug, RefCast, Clone)]
#[repr(transparent)]
pub struct FileDescriptorSet<A: Allocator = Global>(GenericMessage<A>);
impl<A: Allocator + Clone> FileDescriptorSet<A> {
    pub fn file(&self) -> impl Iterator<Item = &FileDescriptorProto<A>> {
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

#[derive(Deref, DerefMut, From, Into, Default, Debug, RefCast, Clone)]
#[repr(transparent)]
pub struct FileDescriptorProto<A: Allocator = Global>(GenericMessage<A>);
impl<A: Allocator + Clone> FileDescriptorProto<A> {
    pub fn name(&self) -> Option<&str> {
        self.as_scalar_string(1)
    }
    pub fn package(&self) -> Option<&str> {
        self.as_scalar_string(2)
    }
    pub fn dependency(&self) -> impl Iterator<Item = &str> {
        self.as_repeated_string(3)
    }
    pub fn public_dependency(&self) -> impl '_ + Iterator<Item = i32> {
        self.as_repeated_int32(10)
    }
    pub fn weak_dependency(&self) -> impl '_ + Iterator<Item = i32> {
        self.as_repeated_int32(11)
    }
    pub fn message_type(&self) -> impl Iterator<Item = &DescriptorProto<A>> {
        self.as_repeated_message(4)
    }
    pub fn enum_type(&self) -> impl Iterator<Item = &EnumDescriptorProto<A>> {
        self.as_repeated_message(5)
    }
    pub fn syntax(&self) -> Option<&str> {
        self.as_scalar_string(12)
    }
    pub fn edition(&self) -> Option<Edition> {
        self.as_scalar_enum(14)
    }
}

#[derive(Deref, DerefMut, From, Into, Default, Debug, RefCast, Clone)]
#[repr(transparent)]
pub struct DescriptorProto<A: Allocator = Global>(GenericMessage<A>);
impl<A: Allocator + Clone> DescriptorProto<A> {
    pub fn name(&self) -> Option<&str> {
        self.as_scalar_string(1)
    }
    pub fn field(&self) -> impl Iterator<Item = &FieldDescriptorProto<A>> {
        self.as_repeated_message(2)
    }
    pub fn extension(&self) -> impl Iterator<Item = &FieldDescriptorProto<A>> {
        self.as_repeated_message(6)
    }
    pub fn nested_type(&self) -> impl Iterator<Item = &DescriptorProto<A>> {
        self.as_repeated_message(3)
    }
    pub fn enum_type(&self) -> impl Iterator<Item = &EnumDescriptorProto<A>> {
        self.as_repeated_message(4)
    }
    pub fn oneof_decl(&self) -> impl Iterator<Item = &OneofDescriptorProto<A>> {
        self.as_repeated_message(8)
    }
}

#[derive(Deref, DerefMut, From, Into, Default, Debug, RefCast, Clone)]
#[repr(transparent)]
pub struct FieldDescriptorProto<A: Allocator = Global>(GenericMessage<A>);
impl<A: Allocator + Clone> FieldDescriptorProto<A> {
    pub fn name(&self) -> Option<&str> {
        self.as_scalar_string(1)
    }
    pub fn number(&self) -> Option<i32> {
        self.as_scalar_int32(3)
    }
    pub fn label(&self) -> Option<self::field_descriptor_proto::Label> {
        self.as_scalar_enum(4)
    }
    pub fn type_(&self) -> Option<field_descriptor_proto::Type> {
        self.as_scalar_enum(5)
    }
    pub fn type_name(&self) -> Option<&str> {
        self.as_scalar_string(6)
    }
    pub fn extendee(&self) -> Option<&str> {
        self.as_scalar_string(2)
    }
    pub fn default_value(&self) -> Option<&str> {
        self.as_scalar_string(7)
    }
    pub fn oneof_index(&self) -> Option<i32> {
        self.as_scalar_int32(9)
    }
    pub fn json_name(&self) -> Option<&str> {
        self.as_scalar_string(10)
    }
    pub fn proto3_optional(&self) -> Option<bool> {
        self.as_scalar_int32(17).map(|x| x != 0)
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

#[derive(Deref, DerefMut, From, Into, Default, Debug, RefCast, Clone)]
#[repr(transparent)]
pub struct OneofDescriptorProto<A: Allocator = Global>(GenericMessage<A>);
impl<A: Allocator + Clone> OneofDescriptorProto<A> {
    pub fn name(&self) -> Option<&str> {
        self.as_scalar_string(1)
    }
}

#[derive(Deref, DerefMut, From, Into, Default, Debug, RefCast, Clone)]
#[repr(transparent)]
pub struct EnumDescriptorProto<A: Allocator = Global>(GenericMessage<A>);
impl<A: Allocator + Clone> EnumDescriptorProto<A> {
    pub fn name(&self) -> Option<&str> {
        self.as_scalar_string(1)
    }
    pub fn value(&self) -> impl Iterator<Item = &EnumValueDescriptorProto<A>> {
        self.as_repeated_message(2)
    }
}

#[derive(Deref, DerefMut, From, Into, Default, Debug, RefCast, Clone)]
#[repr(transparent)]
pub struct EnumValueDescriptorProto<A: Allocator = Global>(GenericMessage<A>);
impl<A: Allocator + Clone> EnumValueDescriptorProto<A> {
    pub fn name(&self) -> Option<&str> {
        self.as_scalar_string(1)
    }
    pub fn number(&self) -> Option<i32> {
        self.as_scalar_int32(2)
    }
}

impl_message_trait_for_trivial_types! {
    pub trait FileDescriptorSetTrait {
        fn file(&self) -> impl Iterator<Item = impl FileDescriptorProtoTrait>;
    }

    pub trait FileDescriptorProtoTrait {
        fn name(&self) -> Option<&str>;
        fn package(&self) -> Option<&str>;
        fn dependency(&self) -> impl Iterator<Item = &str>;
        fn public_dependency(&self) -> impl Iterator<Item = i32>;
        fn weak_dependency(&self) -> impl Iterator<Item = i32>;
        fn message_type(&self) -> impl Iterator<Item = impl DescriptorProtoTrait>;
        fn enum_type(&self) -> impl Iterator<Item = impl EnumDescriptorProtoTrait>;
        fn syntax(&self) -> Option<&str>;
        fn edition(&self) -> Edition;
    }

    pub trait DescriptorProtoTrait {
        fn name(&self) -> Option<&str>;
        fn field(&self) -> impl Iterator<Item = impl FieldDescriptorProtoTrait>;
        fn extension(&self) -> impl Iterator<Item = impl FieldDescriptorProtoTrait>;
        fn nested_type(&self) -> impl Iterator<Item = impl DescriptorProtoTrait>;
        fn enum_type(&self) -> impl Iterator<Item = impl EnumDescriptorProtoTrait>;
        fn oneof_decl(&self) -> impl Iterator<Item = impl OneofDescriptorProtoTrait>;
    }

    pub trait FieldDescriptorProtoTrait {
        fn name(&self) -> Option<&str>;
        fn number(&self) -> Option<i32>;
        fn label(&self) -> field_descriptor_proto::Label;
        fn r#type(&self) -> field_descriptor_proto::Type;
        fn type_name(&self) -> Option<&str>;
        fn extendee(&self) -> Option<&str>;
        fn default_value(&self) -> Option<&str>;
        fn oneof_index(&self) -> Option<i32>;
        fn json_name(&self) -> Option<&str>;
        fn proto3_optional(&self) -> bool;
    }

    pub trait OneofDescriptorProtoTrait {
        fn name(&self) -> Option<&str>;
    }

    pub trait EnumDescriptorProtoTrait {
        fn name(&self) -> Option<&str>;
        fn value(&self) -> impl Iterator<Item = impl EnumValueDescriptorProtoTrait>;
    }

    pub trait EnumValueDescriptorProtoTrait {
        fn name(&self) -> Option<&str>;
        fn number(&self) -> Option<i32>;
    }
}
