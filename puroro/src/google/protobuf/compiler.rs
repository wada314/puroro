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
use crate::variant::{Int32, UInt64};
use crate::Result;
use ::derive_more::{Deref as DDeref, From as DFrom};

#[derive(DDeref, DFrom)]
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

#[derive(DDeref, DFrom)]
pub struct CodeGeneratorRequest<'a>(UntypedMessage<'a>);
impl<'a> CodeGeneratorRequest<'a> {
    pub fn file_to_generate(&self) -> impl IntoIterator<Item = Result<&str>> {
        self.0.field(1).as_repeated_string()
    }
    pub fn parameter(&self) -> Result<Option<&str>> {
        self.0.field(2).as_scalar_string()
    }
    pub fn proto_file(&self) -> impl Iterator<Item = Result<FileDescriptorProto>> {
        self.0.repeated_message_field(15, FileDescriptorProto)
    }
    pub fn source_file_descriptors(&self) -> impl Iterator<Item = Result<FileDescriptorProto>> {
        self.0.repeated_message_field(17, FileDescriptorProto)
    }
    pub fn compiler_version(&self) -> Result<Option<Version>> {
        self.0.scalar_message_field(3, Version)
    }
}

pub mod code_generator_response {
    use crate::untyped_message::UntypedMessage;
    use crate::{ErrorKind, Result};

    pub enum Feature {
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

    pub struct File<'a>(pub(crate) UntypedMessage<'a>);
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
    }
}

#[derive(DDeref, DFrom)]
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
        self.0
            .repeated_message_field(15, code_generator_response::File)
    }
}
