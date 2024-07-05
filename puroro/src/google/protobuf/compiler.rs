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

use crate::google::protobuf::FileDescriptorProto;
use crate::untyped_message::UntypedMessage;
use crate::variant::variant_types::{Int32, UInt64};
use crate::variant::VariantIntegerType;
use crate::{ErrorKind, Result};
use ::derive_more::{Deref as DDeref, DerefMut as DDerefMut, From as DFrom, Into as DInto};

#[derive(DDeref, DDerefMut, DFrom, DInto, Default, Debug)]
pub struct Version<'a>(UntypedMessage<'a>);
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
        Ok(self.0.field(4).as_scalar_string()?.unwrap_or(""))
    }
}

#[derive(DDeref, DDerefMut, DFrom, DInto, Default, Debug)]
pub struct CodeGeneratorRequest<'a>(UntypedMessage<'a>);
impl<'a> CodeGeneratorRequest<'a> {
    pub fn file_to_generate(&self) -> impl IntoIterator<Item = Result<&str>> {
        self.0.field(1).as_repeated_string()
    }
    pub fn parameter(&self) -> Result<Option<&str>> {
        self.0.field(2).as_scalar_string()
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
        type Error = ErrorKind;
        fn try_from(value: i32) -> Result<Self> {
            match value {
                0 => Ok(Self::FeatureNone),
                1 => Ok(Self::FeatureProto3Optional),
                2 => Ok(Self::FeatureSupportsEditions),
                _ => Err(ErrorKind::TryFromIntIntoEnumError(value)),
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
    pub struct File<'a>(UntypedMessage<'a>);
    impl<'a> File<'a> {
        pub fn name(&self) -> Result<Option<&str>> {
            self.0.field(1).as_scalar_string()
        }
        pub fn insertion_point(&self) -> Result<Option<&str>> {
            self.0.field(2).as_scalar_string()
        }
        pub fn content(&self) -> Result<&str> {
            self.0
                .field(15)
                .as_scalar_string()
                .map(|opt| opt.unwrap_or(""))
        }

        pub fn set_name(&mut self, name: &str) -> Result<()> {
            self.0.field_mut(1).push_string(name)
        }
        pub fn set_insertion_point(&mut self, insertion_point: &str) -> Result<()> {
            self.0.field_mut(2).push_string(insertion_point)
        }
        pub fn set_content(&mut self, content: &str) -> Result<()> {
            self.0.field_mut(15).push_string(content)
        }
    }
}

#[derive(DDeref, DDerefMut, DFrom, DInto, Default, Debug)]
pub struct CodeGeneratorResponse<'a>(UntypedMessage<'a>);
impl<'a> CodeGeneratorResponse<'a> {
    pub fn error(&self) -> Result<Option<&str>> {
        self.0.field(1).as_scalar_string()
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
        self.0.field_mut(1).push_string(error)
    }
    pub fn set_supported_features(&mut self, features: u64) -> Result<()> {
        self.0.field_mut(2).push_variant(features.into())
    }
    pub fn set_minimum_edition(&mut self, edition: i32) -> Result<()> {
        self.0
            .field_mut(3)
            .push_variant(Int32::try_into_variant(edition)?)
    }
    pub fn set_maximum_edition(&mut self, edition: i32) -> Result<()> {
        self.0
            .field_mut(4)
            .push_variant(Int32::try_into_variant(edition)?)
    }
    pub fn push_file(&mut self, file: code_generator_response::File<'a>) -> Result<()> {
        self.0.field_mut(15).push_message(file.into())
    }
}
