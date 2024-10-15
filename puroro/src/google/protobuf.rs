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

use super::DynamicMessageExt;
use crate::dynamic_message::DynamicMessage;
use crate::internal::impl_message_trait_for_trivial_types;
use ::derive_more::{Deref, DerefMut, From, Into, TryFrom};
use ::ref_cast::RefCast;
use ::std::alloc::{Allocator, Global};

#[derive(Deref, DerefMut, From, Into, Default, Debug, RefCast, Clone)]
#[repr(transparent)]
pub struct FileDescriptorSet<A: Allocator = Global>(DynamicMessage<A>);
impl<A: Allocator> FileDescriptorSet<A> {
    pub const FILE_FIELD_NUMBER: i32 = 1;
}
impl<A: Allocator + Clone> FileDescriptorSet<A> {
    pub fn file(&self) -> impl Iterator<Item = &FileDescriptorProto<A>> {
        self.0.as_repeated_message(Self::FILE_FIELD_NUMBER)
    }
}

#[derive(Default, Debug, Clone, PartialEq, TryFrom)]
#[try_from(repr)]
#[repr(i32)]
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
impl From<Edition> for i32 {
    fn from(value: Edition) -> i32 {
        value as i32
    }
}

#[derive(Deref, DerefMut, From, Into, Default, Debug, RefCast, Clone)]
#[repr(transparent)]
pub struct FileDescriptorProto<A: Allocator = Global>(DynamicMessage<A>);
impl<A: Allocator> FileDescriptorProto<A> {
    pub const NAME_FIELD_NUMBER: i32 = 1;
    pub const PACKAGE_FIELD_NUMBER: i32 = 2;
    pub const DEPENDENCY_FIELD_NUMBER: i32 = 3;
    pub const PUBLIC_DEPENDENCY_FIELD_NUMBER: i32 = 10;
    pub const WEAK_DEPENDENCY_FIELD_NUMBER: i32 = 11;
    pub const MESSAGE_TYPE_FIELD_NUMBER: i32 = 4;
    pub const ENUM_TYPE_FIELD_NUMBER: i32 = 5;
    pub const OPTIONS_FIELD_NUMBER: i32 = 8;
    pub const SYNTAX_FIELD_NUMBER: i32 = 12;
    pub const EDITION_FIELD_NUMBER: i32 = 14;
}
impl<A: Allocator + Clone> FileDescriptorProto<A> {
    pub fn name(&self) -> Option<&str> {
        self.as_scalar_string(Self::NAME_FIELD_NUMBER)
    }
    pub fn package(&self) -> Option<&str> {
        self.as_scalar_string(Self::PACKAGE_FIELD_NUMBER)
    }
    pub fn dependency(&self) -> impl '_ + Iterator<Item = &str> {
        self.as_repeated_string(Self::DEPENDENCY_FIELD_NUMBER)
    }
    pub fn public_dependency(&self) -> impl '_ + Iterator<Item = i32> {
        self.as_repeated_int32(Self::PUBLIC_DEPENDENCY_FIELD_NUMBER)
    }
    pub fn weak_dependency(&self) -> impl '_ + Iterator<Item = i32> {
        self.as_repeated_int32(Self::WEAK_DEPENDENCY_FIELD_NUMBER)
    }
    pub fn message_type(&self) -> impl '_ + Iterator<Item = &DescriptorProto<A>> {
        self.as_repeated_message(Self::MESSAGE_TYPE_FIELD_NUMBER)
    }
    pub fn enum_type(&self) -> impl '_ + Iterator<Item = &EnumDescriptorProto<A>> {
        self.as_repeated_message(Self::ENUM_TYPE_FIELD_NUMBER)
    }
    pub fn options(&self) -> Option<&FileOptions<A>> {
        self.as_scalar_message(Self::OPTIONS_FIELD_NUMBER)
    }
    pub fn syntax(&self) -> Option<&str> {
        self.as_scalar_string(Self::SYNTAX_FIELD_NUMBER)
    }
    pub fn edition(&self) -> Edition {
        self.as_scalar_enum(Self::EDITION_FIELD_NUMBER)
            .unwrap_or_default()
    }
}

#[derive(Deref, DerefMut, From, Into, Default, Debug, RefCast, Clone)]
#[repr(transparent)]
pub struct DescriptorProto<A: Allocator = Global>(DynamicMessage<A>);
impl<A: Allocator> DescriptorProto<A> {
    pub const NAME_FIELD_NUMBER: i32 = 1;
    pub const FIELD_FIELD_NUMBER: i32 = 2;
    pub const NESTED_TYPE_FIELD_NUMBER: i32 = 3;
    pub const ENUM_TYPE_FIELD_NUMBER: i32 = 4;
    pub const EXTENSION_FIELD_NUMBER: i32 = 6;
    pub const ONEOF_DECL_FIELD_NUMBER: i32 = 8;
    pub const OPTIONS_FIELD_NUMBER: i32 = 7;
}
impl<A: Allocator + Clone> DescriptorProto<A> {
    pub fn name(&self) -> Option<&str> {
        self.as_scalar_string(Self::NAME_FIELD_NUMBER)
    }
    pub fn field(&self) -> impl '_ + Iterator<Item = &FieldDescriptorProto<A>> {
        self.as_repeated_message(Self::FIELD_FIELD_NUMBER)
    }
    pub fn nested_type(&self) -> impl '_ + Iterator<Item = &DescriptorProto<A>> {
        self.as_repeated_message(Self::NESTED_TYPE_FIELD_NUMBER)
    }
    pub fn enum_type(&self) -> impl '_ + Iterator<Item = &EnumDescriptorProto<A>> {
        self.as_repeated_message(Self::ENUM_TYPE_FIELD_NUMBER)
    }
    pub fn extension(&self) -> impl '_ + Iterator<Item = &FieldDescriptorProto<A>> {
        self.as_repeated_message(Self::EXTENSION_FIELD_NUMBER)
    }
    pub fn oneof_decl(&self) -> impl '_ + Iterator<Item = &OneofDescriptorProto<A>> {
        self.as_repeated_message(Self::ONEOF_DECL_FIELD_NUMBER)
    }
    pub fn options(&self) -> Option<&MessageOptions<A>> {
        self.as_scalar_message(Self::OPTIONS_FIELD_NUMBER)
    }
}

#[derive(Deref, DerefMut, From, Into, Default, Debug, RefCast, Clone)]
#[repr(transparent)]
pub struct FieldDescriptorProto<A: Allocator = Global>(DynamicMessage<A>);
impl<A: Allocator> FieldDescriptorProto<A> {
    pub const NAME_FIELD_NUMBER: i32 = 1;
    pub const NUMBER_FIELD_NUMBER: i32 = 3;
    pub const LABEL_FIELD_NUMBER: i32 = 4;
    pub const TYPE_FIELD_NUMBER: i32 = 5;
    pub const TYPE_NAME_FIELD_NUMBER: i32 = 6;
    pub const EXTENDEE_FIELD_NUMBER: i32 = 2;
    pub const DEFAULT_VALUE_FIELD_NUMBER: i32 = 7;
    pub const ONEOF_INDEX_FIELD_NUMBER: i32 = 9;
    pub const JSON_NAME_FIELD_NUMBER: i32 = 10;
    pub const OPTIONS_FIELD_NUMBER: i32 = 8;
    pub const PROTO3_OPTIONAL_FIELD_NUMBER: i32 = 17;
}
impl<A: Allocator + Clone> FieldDescriptorProto<A> {
    pub fn name(&self) -> Option<&str> {
        self.as_scalar_string(Self::NAME_FIELD_NUMBER)
    }
    pub fn number(&self) -> Option<i32> {
        self.as_scalar_int32(Self::NUMBER_FIELD_NUMBER)
    }
    pub fn label(&self) -> Option<field_descriptor_proto::Label> {
        self.as_scalar_enum(Self::LABEL_FIELD_NUMBER)
    }
    pub fn r#type(&self) -> Option<field_descriptor_proto::Type> {
        self.as_scalar_enum(Self::TYPE_FIELD_NUMBER)
    }
    pub fn type_name(&self) -> Option<&str> {
        self.as_scalar_string(Self::TYPE_NAME_FIELD_NUMBER)
    }
    pub fn extendee(&self) -> Option<&str> {
        self.as_scalar_string(Self::EXTENDEE_FIELD_NUMBER)
    }
    pub fn default_value(&self) -> Option<&str> {
        self.as_scalar_string(Self::DEFAULT_VALUE_FIELD_NUMBER)
    }
    pub fn oneof_index(&self) -> Option<i32> {
        self.as_scalar_int32(Self::ONEOF_INDEX_FIELD_NUMBER)
    }
    pub fn json_name(&self) -> Option<&str> {
        self.as_scalar_string(Self::JSON_NAME_FIELD_NUMBER)
    }
    pub fn options(&self) -> Option<&FieldOptions<A>> {
        self.as_scalar_message(Self::OPTIONS_FIELD_NUMBER)
    }
    pub fn proto3_optional(&self) -> Option<bool> {
        self.as_scalar_int32(Self::PROTO3_OPTIONAL_FIELD_NUMBER)
            .map(|v| v != 0)
    }
}

pub mod field_descriptor_proto {
    use ::derive_more::TryFrom;

    #[derive(Default, Debug, Clone, PartialEq, TryFrom)]
    #[try_from(repr)]
    #[repr(i32)]
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

    #[derive(Default, Debug, Clone, PartialEq, TryFrom)]
    #[try_from(repr)]
    #[repr(i32)]
    pub enum Label {
        #[default]
        LabelOptional = 1,
        LabelRepeated = 3,
        LabelRequired = 2,
    }

    impl From<Type> for i32 {
        fn from(value: Type) -> i32 {
            value as i32
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
pub struct OneofDescriptorProto<A: Allocator = Global>(DynamicMessage<A>);
impl<A: Allocator> OneofDescriptorProto<A> {
    pub const NAME_FIELD_NUMBER: i32 = 1;
    pub const OPTIONS_FIELD_NUMBER: i32 = 2;
}
impl<A: Allocator + Clone> OneofDescriptorProto<A> {
    pub fn name(&self) -> Option<&str> {
        self.as_scalar_string(Self::NAME_FIELD_NUMBER)
    }
    pub fn options(&self) -> Option<&OneofOptions<A>> {
        self.as_scalar_message(Self::OPTIONS_FIELD_NUMBER)
    }
}

#[derive(Deref, DerefMut, From, Into, Default, Debug, RefCast, Clone)]
#[repr(transparent)]
pub struct EnumDescriptorProto<A: Allocator = Global>(DynamicMessage<A>);
impl<A: Allocator> EnumDescriptorProto<A> {
    pub const NAME_FIELD_NUMBER: i32 = 1;
    pub const VALUE_FIELD_NUMBER: i32 = 2;
    pub const OPTIONS_FIELD_NUMBER: i32 = 3;
}
impl<A: Allocator + Clone> EnumDescriptorProto<A> {
    pub fn name(&self) -> Option<&str> {
        self.as_scalar_string(Self::NAME_FIELD_NUMBER)
    }
    pub fn value(&self) -> impl Iterator<Item = &EnumValueDescriptorProto<A>> {
        self.as_repeated_message(Self::VALUE_FIELD_NUMBER)
    }
    pub fn options(&self) -> Option<&EnumOptions<A>> {
        self.as_scalar_message(Self::OPTIONS_FIELD_NUMBER)
    }
}

#[derive(Deref, DerefMut, From, Into, Default, Debug, RefCast, Clone)]
#[repr(transparent)]
pub struct EnumValueDescriptorProto<A: Allocator = Global>(DynamicMessage<A>);
impl<A: Allocator> EnumValueDescriptorProto<A> {
    pub const NAME_FIELD_NUMBER: i32 = 1;
    pub const NUMBER_FIELD_NUMBER: i32 = 2;
    pub const OPTIONS_FIELD_NUMBER: i32 = 3;
}
impl<A: Allocator + Clone> EnumValueDescriptorProto<A> {
    pub fn name(&self) -> Option<&str> {
        self.as_scalar_string(Self::NAME_FIELD_NUMBER)
    }
    pub fn number(&self) -> Option<i32> {
        self.as_scalar_int32(Self::NUMBER_FIELD_NUMBER)
    }
    pub fn options(&self) -> Option<&EnumValueOptions<A>> {
        self.as_scalar_message(Self::OPTIONS_FIELD_NUMBER)
    }
}

#[derive(Deref, DerefMut, From, Into, Default, Debug, RefCast, Clone)]
#[repr(transparent)]
pub struct FileOptions<A: Allocator = Global>(DynamicMessage<A>);
impl<A: Allocator> FileOptions<A> {
    pub const FEATURES_FIELD_NUMBER: i32 = 50;
}
impl<A: Allocator + Clone> FileOptions<A> {
    pub fn features(&self) -> Option<&FeatureSet<A>> {
        self.as_scalar_message(Self::FEATURES_FIELD_NUMBER)
    }
}

#[derive(Deref, DerefMut, From, Into, Default, Debug, RefCast, Clone)]
#[repr(transparent)]
pub struct MessageOptions<A: Allocator = Global>(DynamicMessage<A>);
impl<A: Allocator> MessageOptions<A> {
    pub const MAP_ENTRY_FIELD_NUMBER: i32 = 7;
    pub const FEATURES_FIELD_NUMBER: i32 = 12;
}
impl<A: Allocator + Clone> MessageOptions<A> {
    pub fn map_entry(&self) -> Option<bool> {
        self.as_scalar_int32(Self::MAP_ENTRY_FIELD_NUMBER)
            .map(|v| v != 0)
    }
    pub fn features(&self) -> Option<&FeatureSet<A>> {
        self.as_scalar_message(Self::FEATURES_FIELD_NUMBER)
    }
}

#[derive(Deref, DerefMut, From, Into, Default, Debug, RefCast, Clone)]
#[repr(transparent)]
pub struct FieldOptions<A: Allocator = Global>(DynamicMessage<A>);
impl<A: Allocator> FieldOptions<A> {
    pub const PACKED_FIELD_NUMBER: i32 = 2;
    pub const LAZY_FIELD_NUMBER: i32 = 5;
    pub const UNVERIFIED_LAZY_FIELD_NUMBER: i32 = 15;
    pub const FEATURES_FIELD_NUMBER: i32 = 21;
}
impl<A: Allocator + Clone> FieldOptions<A> {
    pub fn packed(&self) -> Option<bool> {
        self.as_scalar_int32(Self::PACKED_FIELD_NUMBER)
            .map(|v| v != 0)
    }
    pub fn lazy(&self) -> Option<bool> {
        self.as_scalar_int32(Self::LAZY_FIELD_NUMBER)
            .map(|v| v != 0)
    }
    pub fn unverified_lazy(&self) -> Option<bool> {
        self.as_scalar_int32(Self::UNVERIFIED_LAZY_FIELD_NUMBER)
            .map(|v| v != 0)
    }
    pub fn features(&self) -> Option<&FeatureSet<A>> {
        self.as_scalar_message(Self::FEATURES_FIELD_NUMBER)
    }
}

#[derive(Deref, DerefMut, From, Into, Default, Debug, RefCast, Clone)]
#[repr(transparent)]
pub struct OneofOptions<A: Allocator = Global>(DynamicMessage<A>);
impl<A: Allocator> OneofOptions<A> {
    pub const FEATURES_FIELD_NUMBER: i32 = 1;
}
impl<A: Allocator + Clone> OneofOptions<A> {
    pub fn features(&self) -> Option<&FeatureSet<A>> {
        self.as_scalar_message(Self::FEATURES_FIELD_NUMBER)
    }
}

#[derive(Deref, DerefMut, From, Into, Default, Debug, RefCast, Clone)]
#[repr(transparent)]
pub struct EnumOptions<A: Allocator = Global>(DynamicMessage<A>);
impl<A: Allocator> EnumOptions<A> {
    pub const ALLOW_ALIAS_FIELD_NUMBER: i32 = 2;
    pub const FEATURES_FIELD_NUMBER: i32 = 7;
}
impl<A: Allocator + Clone> EnumOptions<A> {
    pub fn allow_alias(&self) -> Option<bool> {
        self.as_scalar_int32(Self::ALLOW_ALIAS_FIELD_NUMBER)
            .map(|v| v != 0)
    }
    pub fn features(&self) -> Option<&FeatureSet<A>> {
        self.as_scalar_message(Self::FEATURES_FIELD_NUMBER)
    }
}

#[derive(Deref, DerefMut, From, Into, Default, Debug, RefCast, Clone)]
#[repr(transparent)]
pub struct EnumValueOptions<A: Allocator = Global>(DynamicMessage<A>);
impl<A: Allocator> EnumValueOptions<A> {
    pub const FEATURES_FIELD_NUMBER: i32 = 2;
}
impl<A: Allocator + Clone> EnumValueOptions<A> {
    pub fn features(&self) -> Option<&FeatureSet<A>> {
        self.as_scalar_message(Self::FEATURES_FIELD_NUMBER)
    }
}

#[derive(Deref, DerefMut, From, Into, Default, Debug, RefCast, Clone)]
#[repr(transparent)]
pub struct FeatureSet<A: Allocator = Global>(DynamicMessage<A>);
impl<A: Allocator> FeatureSet<A> {
    pub const FIELD_PRESENCE_FIELD_NUMBER: i32 = 1;
    pub const ENUM_TYPE_FIELD_NUMBER: i32 = 2;
    pub const REPEATED_FIELD_PRESENCE_FIELD_NUMBER: i32 = 3;
    pub const UTF8_VALIDATION_FIELD_NUMBER: i32 = 4;
    pub const MESSAGE_ENCODING_FIELD_NUMBER: i32 = 5;
    pub const JSON_FORMAT_FIELD_NUMBER: i32 = 6;
}
impl<A: Allocator + Clone> FeatureSet<A> {
    pub fn field_presence(&self) -> Option<feature_set::FieldPresence> {
        self.as_scalar_enum(Self::FIELD_PRESENCE_FIELD_NUMBER)
    }
    pub fn enum_type(&self) -> Option<feature_set::EnumType> {
        self.as_scalar_enum(Self::ENUM_TYPE_FIELD_NUMBER)
    }
    pub fn repeated_field_presence(&self) -> Option<feature_set::RepeatedFieldPresence> {
        self.as_scalar_enum(Self::REPEATED_FIELD_PRESENCE_FIELD_NUMBER)
    }
    pub fn utf8_validation(&self) -> Option<feature_set::Utf8Validation> {
        self.as_scalar_enum(Self::UTF8_VALIDATION_FIELD_NUMBER)
    }
    pub fn message_encoding(&self) -> Option<feature_set::MessageEncoding> {
        self.as_scalar_enum(Self::MESSAGE_ENCODING_FIELD_NUMBER)
    }
    pub fn json_format(&self) -> Option<feature_set::JsonFormat> {
        self.as_scalar_enum(Self::JSON_FORMAT_FIELD_NUMBER)
    }
}

pub mod feature_set {
    use ::derive_more::{Debug, TryFrom};

    #[derive(Default, Debug, Clone, PartialEq, TryFrom)]
    #[try_from(repr)]
    #[repr(i32)]
    pub enum FieldPresence {
        #[default]
        FieldPresenceUnknown = 0,
        Explicit = 1,
        Implicit = 2,
        LegacyRequired = 3,
    }
    impl From<FieldPresence> for i32 {
        fn from(value: FieldPresence) -> i32 {
            value as i32
        }
    }

    #[derive(Default, Debug, Clone, PartialEq, TryFrom)]
    #[try_from(repr)]
    #[repr(i32)]
    pub enum EnumType {
        #[default]
        EnumTypeUnknown = 0,
        Open = 1,
        Closed = 2,
    }
    impl From<EnumType> for i32 {
        fn from(value: EnumType) -> i32 {
            value as i32
        }
    }

    #[derive(Default, Debug, Clone, PartialEq, TryFrom)]
    #[try_from(repr)]
    #[repr(i32)]
    pub enum RepeatedFieldPresence {
        #[default]
        RepeatedFieldPresenceUnknown = 0,
        Packed = 1,
        Extended = 2,
    }
    impl From<RepeatedFieldPresence> for i32 {
        fn from(value: RepeatedFieldPresence) -> i32 {
            value as i32
        }
    }

    #[derive(Default, Debug, Clone, PartialEq, TryFrom)]
    #[try_from(repr)]
    #[repr(i32)]
    pub enum Utf8Validation {
        #[default]
        Utf8ValidationUnknown = 0,
        Verify = 2,
        None = 3,
    }
    impl From<Utf8Validation> for i32 {
        fn from(value: Utf8Validation) -> i32 {
            value as i32
        }
    }

    #[derive(Default, Debug, Clone, PartialEq, TryFrom)]
    #[try_from(repr)]
    #[repr(i32)]
    pub enum MessageEncoding {
        #[default]
        MessageEncodingUnknown = 0,
        LengthPrefixed = 1,
        Delimited = 2,
    }
    impl From<MessageEncoding> for i32 {
        fn from(value: MessageEncoding) -> i32 {
            value as i32
        }
    }

    #[derive(Default, Debug, Clone, PartialEq, TryFrom)]
    #[try_from(repr)]
    #[repr(i32)]
    pub enum JsonFormat {
        #[default]
        JsonFormatUnknown = 0,
        Allow = 1,
        LegacyBestEffort = 2,
    }
    impl From<JsonFormat> for i32 {
        fn from(value: JsonFormat) -> i32 {
            value as i32
        }
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
