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

use ::std::alloc::{Allocator, Global};

use super::{FileDescriptorProtoTrait, GenericMessageExt};
use crate::generic_message::GenericMessage;
use crate::google::protobuf::FileDescriptorProto;
use crate::internal::{
    impl_message_mut_trait_for_trivial_types, impl_message_trait_for_trivial_types,
};
use crate::variant::variant_types::{Int32, UInt64};
use crate::Result;
use ::derive_more::{Deref, DerefMut, From, Into};
use ::ref_cast::RefCast;

#[derive(Deref, DerefMut, From, Into, Default, Debug, RefCast, Clone)]
#[repr(transparent)]
pub struct Version<A: Allocator = Global>(GenericMessage<A>);
impl<A: Allocator> Version<A> {
    pub const MAJOR_FIELD_NUMBER: i32 = 1;
    pub const MINOR_FIELD_NUMBER: i32 = 2;
    pub const PATCH_FIELD_NUMBER: i32 = 3;
    pub const SUFFIX_FIELD_NUMBER: i32 = 4;
}
impl<A: Allocator + Clone> Version<A> {
    pub fn major(&self) -> Option<i32> {
        self.as_scalar_int32(Self::MAJOR_FIELD_NUMBER)
    }
    pub fn minor(&self) -> Option<i32> {
        self.as_scalar_int32(Self::MINOR_FIELD_NUMBER)
    }
    pub fn patch(&self) -> Option<i32> {
        self.as_scalar_int32(Self::PATCH_FIELD_NUMBER)
    }
    pub fn suffix(&self) -> Option<&str> {
        self.as_scalar_string(Self::SUFFIX_FIELD_NUMBER)
    }
}

#[derive(Deref, DerefMut, From, Into, Default, Debug, RefCast, Clone)]
#[repr(transparent)]
pub struct CodeGeneratorRequest<A: Allocator = Global>(GenericMessage<A>);
impl<A: Allocator> CodeGeneratorRequest<A> {
    pub const FILE_TO_GENERATE_FIELD_NUMBER: i32 = 1;
    pub const PARAMETER_FIELD_NUMBER: i32 = 2;
    pub const PROTO_FILE_FIELD_NUMBER: i32 = 15;
    pub const SOURCE_FILE_DESCRIPTORS_FIELD_NUMBER: i32 = 17;
    pub const COMPILER_VERSION_FIELD_NUMBER: i32 = 3;
}
impl<A: Allocator + Clone> CodeGeneratorRequest<A> {
    pub fn file_to_generate(&self) -> impl '_ + Iterator<Item = &str> {
        self.as_repeated_string(Self::FILE_TO_GENERATE_FIELD_NUMBER)
    }
    pub fn parameter(&self) -> Option<&str> {
        self.as_scalar_string(Self::PARAMETER_FIELD_NUMBER)
    }
    pub fn proto_file(&self) -> impl '_ + Iterator<Item = &FileDescriptorProto<A>> {
        self.as_repeated_message(Self::PROTO_FILE_FIELD_NUMBER)
    }
    pub fn source_file_descriptors(&self) -> impl '_ + Iterator<Item = &FileDescriptorProto<A>> {
        self.as_repeated_message(Self::SOURCE_FILE_DESCRIPTORS_FIELD_NUMBER)
    }
    pub fn compiler_version(&self) -> Option<&Version<A>> {
        self.as_scalar_message(Self::COMPILER_VERSION_FIELD_NUMBER)
    }
}

pub mod code_generator_response {
    use super::*;

    #[derive(Default, Debug)]
    pub enum Feature {
        #[default]
        FeatureNone = 0,
        FeatureProto3Optional = 1,
        FeatureSupportsEditions = 2,
    }
    impl TryFrom<i32> for Feature {
        type Error = i32;
        fn try_from(value: i32) -> ::std::result::Result<Self, i32> {
            match value {
                0 => Ok(Self::FeatureNone),
                1 => Ok(Self::FeatureProto3Optional),
                2 => Ok(Self::FeatureSupportsEditions),
                _ => Err(value),
            }
        }
    }
    impl From<Feature> for i32 {
        fn from(value: Feature) -> i32 {
            match value {
                Feature::FeatureNone => 0,
                Feature::FeatureProto3Optional => 1,
                Feature::FeatureSupportsEditions => 2,
            }
        }
    }

    #[derive(Deref, DerefMut, From, Into, Default, Debug, RefCast, Clone)]
    #[repr(transparent)]
    pub struct File<A: Allocator = Global>(GenericMessage<A>);
    impl<A: Allocator> File<A> {
        pub const NAME_FIELD_NUMBER: i32 = 1;
        pub const INSERTION_POINT_FIELD_NUMBER: i32 = 2;
        pub const CONTENT_FIELD_NUMBER: i32 = 15;
    }
    impl<A: Allocator + Clone> File<A> {
        pub fn name(&self) -> Option<&str> {
            self.as_scalar_string(Self::NAME_FIELD_NUMBER)
        }
        pub fn insertion_point(&self) -> Option<&str> {
            self.as_scalar_string(Self::INSERTION_POINT_FIELD_NUMBER)
        }
        pub fn content(&self) -> Option<&str> {
            self.as_scalar_string(Self::CONTENT_FIELD_NUMBER)
        }

        pub fn set_name(&mut self, name: &str) -> Result<()> {
            self.0.field_mut(Self::NAME_FIELD_NUMBER).push_string(name);
            Ok(())
        }
        pub fn set_insertion_point(&mut self, insertion_point: &str) -> Result<()> {
            self.0
                .field_mut(Self::INSERTION_POINT_FIELD_NUMBER)
                .push_string(insertion_point);
            Ok(())
        }
        pub fn set_content(&mut self, content: &str) -> Result<()> {
            self.0
                .field_mut(Self::CONTENT_FIELD_NUMBER)
                .push_string(content);
            Ok(())
        }
    }

    impl_message_trait_for_trivial_types! {
        pub trait FileTrait {
            fn name(&self) -> Option<&str>;
            fn insertion_point(&self) -> Option<&str>;
            fn content(&self) -> Option<&str>;
        }
    }
    impl_message_mut_trait_for_trivial_types! {
        pub trait FileMutTrait: FileTrait {
            fn set_name(&mut self, name: &str);
            fn set_insertion_point(&mut self, insertion_point: &str);
            fn set_content(&mut self, content: &str);
        }
    }
}

#[derive(Deref, DerefMut, From, Into, Default, Debug, RefCast, Clone)]
#[repr(transparent)]
pub struct CodeGeneratorResponse<A: Allocator = Global>(GenericMessage<A>);
impl<A: Allocator> CodeGeneratorResponse<A> {
    pub const ERROR_FIELD_NUMBER: i32 = 1;
    pub const SUPPORTED_FEATURES_FIELD_NUMBER: i32 = 2;
    pub const MINIMUM_EDITION_FIELD_NUMBER: i32 = 3;
    pub const MAXIMUM_EDITION_FIELD_NUMBER: i32 = 4;
    pub const FILE_FIELD_NUMBER: i32 = 15;
}
impl<A: Allocator + Clone> CodeGeneratorResponse<A> {
    pub fn error(&self) -> Option<&str> {
        self.as_scalar_string(Self::ERROR_FIELD_NUMBER)
    }
    pub fn supported_features(&self) -> u64 {
        self.0
            .field(Self::SUPPORTED_FEATURES_FIELD_NUMBER)
            .map(|f| f.as_scalar_variant::<UInt64>(false))
            .unwrap_or_default()
    }
    pub fn minimum_edition(&self) -> Option<i32> {
        self.as_scalar_int32(Self::MINIMUM_EDITION_FIELD_NUMBER)
    }
    pub fn maximum_edition(&self) -> Option<i32> {
        self.as_scalar_int32(Self::MAXIMUM_EDITION_FIELD_NUMBER)
    }
    pub fn file(&self) -> impl Iterator<Item = &code_generator_response::File<A>> {
        self.as_repeated_message(Self::FILE_FIELD_NUMBER)
    }

    pub fn set_error(&mut self, error: &str) -> Result<()> {
        self.0
            .field_mut(Self::ERROR_FIELD_NUMBER)
            .push_string(error);
        Ok(())
    }
    pub fn set_supported_features(&mut self, features: u64) -> Result<()> {
        self.0
            .field_mut(Self::SUPPORTED_FEATURES_FIELD_NUMBER)
            .push_variant_from::<UInt64>(features);
        Ok(())
    }
    pub fn set_minimum_edition(&mut self, edition: i32) -> Result<()> {
        self.0
            .field_mut(Self::MINIMUM_EDITION_FIELD_NUMBER)
            .push_variant_from::<Int32>(edition);
        Ok(())
    }
    pub fn set_maximum_edition(&mut self, edition: i32) -> Result<()> {
        self.0
            .field_mut(Self::MAXIMUM_EDITION_FIELD_NUMBER)
            .push_variant_from::<Int32>(edition);
        Ok(())
    }
    pub fn push_file(&mut self, file: code_generator_response::File<A>) -> Result<()> {
        self.0
            .field_mut(Self::FILE_FIELD_NUMBER)
            .push_message(file.into());
        Ok(())
    }
}

impl_message_trait_for_trivial_types! {
    pub trait VersionTrait {
        fn major(&self) -> Option<i32>;
        fn minor(&self) -> Option<i32>;
        fn patch(&self) -> Option<i32>;
        fn suffix(&self) -> Option<&str>;
    }
    pub trait CodeGeneratorRequestTrait {
        fn file_to_generate(&self) -> impl Iterator<Item = &str>;
        fn parameter(&self) -> Option<&str>;
        fn proto_file(&self) -> impl Iterator<Item = impl FileDescriptorProtoTrait>;
        fn source_file_descriptors(&self) -> impl Iterator<Item = impl FileDescriptorProtoTrait>;
        fn compiler_version(&self) -> Option<impl VersionTrait>;
    }
    pub trait CodeGeneratorResponseTrait {
        fn error(&self) -> Option<&str>;
        fn supported_features(&self) -> u64;
        fn minimum_edition(&self) -> Option<i32>;
        fn maximum_edition(&self) -> Option<i32>;
        fn file(&self) -> impl Iterator<Item = impl code_generator_response::FileTrait>;
    }
}
impl_message_mut_trait_for_trivial_types! {
    pub trait VersionMutTrait: VersionTrait {
        fn major_mut(&mut self) -> impl ::std::ops::DerefMut<Target = i32>;
        fn minor_mut(&mut self) -> impl ::std::ops::DerefMut<Target = i32>;
        fn patch_mut(&mut self) -> impl ::std::ops::DerefMut<Target = i32>;
        fn suffix_mut(&mut self) -> impl ::std::ops::DerefMut<Target = String>;
    }
}
