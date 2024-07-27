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

use crate::generic_message::GenericMessage;
use crate::google::protobuf::{FileDescriptorProto, FileDescriptorTrait};
use crate::internal::{
    impl_message_mut_trait_for_trivial_types, impl_message_trait_for_trivial_types,
};
use crate::variant::variant_types::{Int32, UInt64};
use crate::variant::VariantIntegerType;
use crate::Result;
use ::derive_more::{Deref as DDeref, DerefMut as DDerefMut, From as DFrom, Into as DInto};

#[derive(DDeref, DDerefMut, DFrom, DInto, Default, Debug)]
pub struct Version<'a>(GenericMessage<'a>);
impl<'a> Version<'a> {
    pub fn major(&self) -> Result<i32> {
        Ok(self.0.scalar_variant_field::<Int32>(1)?.unwrap_or(0))
    }
    pub fn minor(&self) -> Result<i32> {
        Ok(self.0.scalar_variant_field::<Int32>(2)?.unwrap_or(0))
    }
    pub fn patch(&self) -> Result<i32> {
        Ok(self.0.scalar_variant_field::<Int32>(3)?.unwrap_or(0))
    }
    pub fn suffix(&self) -> Result<&str> {
        Ok(self.0.field(4).try_as_scalar_string_opt()?.unwrap_or(""))
    }
}

#[derive(DDeref, DDerefMut, DFrom, DInto, Default, Debug)]
pub struct CodeGeneratorRequest<'a>(GenericMessage<'a>);
impl<'a> CodeGeneratorRequest<'a> {
    pub fn file_to_generate(&self) -> impl IntoIterator<Item = Result<&str>> {
        self.0.field(1).try_as_repeated_string()
    }
    pub fn parameter(&self) -> Result<Option<&str>> {
        self.0.field(2).try_as_scalar_string_opt()
    }
    pub fn proto_file(&self) -> impl Iterator<Item = Result<FileDescriptorProto>> {
        self.0.repeated_message_field(15, Into::into)
    }
    pub fn source_file_descriptors(&self) -> impl Iterator<Item = Result<FileDescriptorProto>> {
        self.0.repeated_message_field(17, Into::into)
    }
    pub fn compiler_version(&self) -> Result<Option<Version>> {
        self.0.scalar_message_field(3, Version)
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

    #[derive(DDeref, DDerefMut, DFrom, DInto, Default, Debug)]
    pub struct File<'a>(GenericMessage<'a>);
    impl<'a> File<'a> {
        pub fn name(&self) -> Result<Option<&str>> {
            self.0.field(1).try_as_scalar_string_opt()
        }
        pub fn insertion_point(&self) -> Result<Option<&str>> {
            self.0.field(2).try_as_scalar_string_opt()
        }
        pub fn content(&self) -> Result<&str> {
            self.0
                .field(15)
                .try_as_scalar_string_opt()
                .map(|opt| opt.unwrap_or(""))
        }

        pub fn set_name(&mut self, name: &str) -> Result<()> {
            self.0.field_mut(1).push_string(name);
            Ok(())
        }
        pub fn set_insertion_point(&mut self, insertion_point: &str) -> Result<()> {
            self.0.field_mut(2).push_string(insertion_point);
            Ok(())
        }
        pub fn set_content(&mut self, content: &str) -> Result<()> {
            self.0.field_mut(15).push_string(content);
            Ok(())
        }
    }

    impl_message_trait_for_trivial_types! {
        pub trait FileTrait {
            fn name(&self) -> &str;
            fn insertion_point(&self) -> &str;
            fn content(&self) -> &str;
        }
    }
    impl_message_mut_trait_for_trivial_types! {
        pub trait FileMutTrait: FileTrait {
            fn set_name(&mut self, name: &str);
            fn set_insertion_point(&mut self, insertion_point: &str);
            fn set_content(&mut self, content: &str);
        }
    }
    impl FileTrait for GenericMessage<'_> {
        fn name(&self) -> &str {
            self.field(1).as_scalar_string()
        }
        fn insertion_point(&self) -> &str {
            self.field(2).as_scalar_string()
        }
        fn content(&self) -> &str {
            self.field(15).as_scalar_string()
        }
    }
    impl FileMutTrait for GenericMessage<'_> {
        fn set_name(&mut self, name: &str) {
            self.field_mut(1).set_string(name)
        }
        fn set_insertion_point(&mut self, insertion_point: &str) {
            self.field_mut(2).set_string(insertion_point)
        }
        fn set_content(&mut self, content: &str) {
            self.field_mut(15).set_string(content)
        }
    }
}

#[derive(DDeref, DDerefMut, DFrom, DInto, Default, Debug)]
pub struct CodeGeneratorResponse<'a>(GenericMessage<'a>);
impl<'a> CodeGeneratorResponse<'a> {
    pub fn error(&self) -> Result<Option<&str>> {
        self.0.field(1).try_as_scalar_string_opt()
    }
    pub fn supported_features(&self) -> Result<Option<u64>> {
        self.0.scalar_variant_field::<UInt64>(2)
    }
    pub fn minimum_edition(&self) -> Result<Option<i32>> {
        self.0.scalar_variant_field::<Int32>(3)
    }
    pub fn maximum_edition(&self) -> Result<Option<i32>> {
        self.0.scalar_variant_field::<Int32>(4)
    }
    pub fn file(&self) -> impl Iterator<Item = Result<code_generator_response::File>> {
        self.0.repeated_message_field(15, Into::into)
    }

    pub fn set_error(&mut self, error: &str) -> Result<()> {
        self.0.field_mut(1).push_string(error);
        Ok(())
    }
    pub fn set_supported_features(&mut self, features: u64) -> Result<()> {
        self.0.field_mut(2).push_variant::<UInt64>(features);
        Ok(())
    }
    pub fn set_minimum_edition(&mut self, edition: i32) -> Result<()> {
        self.0.field_mut(3).push_variant::<Int32>(edition);
        Ok(())
    }
    pub fn set_maximum_edition(&mut self, edition: i32) -> Result<()> {
        self.0.field_mut(4).push_variant::<Int32>(edition);
        Ok(())
    }
    pub fn push_file(&mut self, file: code_generator_response::File<'a>) -> Result<()> {
        self.0.field_mut(15).push_message(&file);
        Ok(())
    }
}

impl_message_trait_for_trivial_types! {
    pub trait VersionTrait {
        fn major(&self) -> i32;
        fn minor(&self) -> i32;
        fn patch(&self) -> i32;
        fn suffix(&self) -> &str;
    }
    pub trait CodeGeneratorRequestTrait {
        fn file_to_generate(&self) -> impl Iterator<Item = &str>;
        fn parameter(&self) -> &str;
        fn proto_file(&self) -> impl Iterator<Item = impl FileDescriptorTrait>;
        fn source_file_descriptors(&self) -> impl Iterator<Item = impl FileDescriptorTrait>;
        fn compiler_version(&self) -> Option<impl VersionTrait>;
    }
    pub trait CodeGeneratorResponseTrait {
        fn error(&self) -> &str;
        fn supported_features(&self) -> u64;
        fn minimum_edition(&self) -> i32;
        fn maximum_edition(&self) -> i32;
        fn file(&self) -> impl Iterator<Item = code_generator_response::File>;
    }
}
impl_message_mut_trait_for_trivial_types! {
    pub trait VersionMutTrait: VersionTrait {
        fn set_major(&mut self, major: i32);
        fn set_minor(&mut self, minor: i32);
        fn set_patch(&mut self, patch: i32);
        fn set_suffix(&mut self, suffix: &str);
    }
    pub trait CodeGeneratorRequestMutTrait: CodeGeneratorRequestTrait {
        fn set_parameter(&mut self, parameter: &str);
        fn push_proto_file(&mut self, proto_file: &impl FileDescriptorTrait);
        fn push_source_file_descriptor(&mut self, source_file_descriptor: &impl FileDescriptorTrait);
        fn set_compiler_version(&mut self, compiler_version: impl VersionTrait);
    }
    pub trait CodeGeneratorResponseMutTrait: CodeGeneratorResponseTrait {
        fn set_error(&mut self, error: &str);
        fn set_supported_features(&mut self, features: u64);
        fn set_minimum_edition(&mut self, edition: i32);
        fn set_maximum_edition(&mut self, edition: i32);
        fn push_file(&mut self, file: code_generator_response::File);
    }
}

impl VersionTrait for GenericMessage<'_> {
    fn major(&self) -> i32 {
        self.field(1).as_scalar_variant::<Int32>(false)
    }
    fn minor(&self) -> i32 {
        self.field(2).as_scalar_variant::<Int32>(false)
    }
    fn patch(&self) -> i32 {
        self.field(3).as_scalar_variant::<Int32>(false)
    }
    fn suffix(&self) -> &str {
        self.field(4).as_scalar_string()
    }
}
impl VersionMutTrait for GenericMessage<'_> {
    fn set_major(&mut self, major: i32) {
        self.field_mut(1).set_variant::<Int32>(major)
    }
    fn set_minor(&mut self, minor: i32) {
        self.field_mut(2).set_variant::<Int32>(minor)
    }
    fn set_patch(&mut self, patch: i32) {
        self.field_mut(3).set_variant::<Int32>(patch)
    }
    fn set_suffix(&mut self, suffix: &str) {
        self.field_mut(4).set_string(suffix)
    }
}

impl CodeGeneratorRequestTrait for GenericMessage<'_> {
    fn file_to_generate(&self) -> impl Iterator<Item = &str> {
        self.field(1).as_repeated_string()
    }
    fn parameter(&self) -> &str {
        self.field(2).as_scalar_string()
    }
    fn proto_file(&self) -> impl Iterator<Item = impl FileDescriptorTrait> {
        self.field(15).as_repeated_message()
    }
    fn source_file_descriptors(&self) -> impl Iterator<Item = impl FileDescriptorTrait> {
        self.field(17).as_repeated_message()
    }
    fn compiler_version(&self) -> Option<impl VersionTrait> {
        self.field(3).as_scalar_message()
    }
}
impl CodeGeneratorRequestMutTrait for GenericMessage<'_> {
    fn set_parameter(&mut self, parameter: &str) {
        self.field_mut(2).set_string(parameter)
    }
    fn push_proto_file(&mut self, proto_file: &GenericMessage) {
        self.field_mut(15).push_message(proto_file)
    }
    fn push_source_file_descriptor(&mut self, source_file_descriptor: &GenericMessage) {
        self.field_mut(17).push_message(source_file_descriptor)
    }
    fn set_compiler_version(&mut self, compiler_version: &GenericMessage) {
        self.field_mut(3).set_message(&compiler_version)
    }
}
