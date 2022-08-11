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

//! Extend the raw protobuf descriptors to add a pointer to the parent descriptor.

use crate::utils::Package;
use crate::{ErrorKind, Result};
use ::puroro_protobuf_compiled::google::protobuf::{
    DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, FileDescriptorProto,
};
use ::std::fmt::Debug;
use itertools::Either;

pub trait FileDescriptorExt {
    fn package_ext(&self) -> Package<&str>;

    fn try_syntax(&self) -> Result<Syntax>;
}

impl FileDescriptorExt for FileDescriptorProto {
    fn package_ext(&self) -> Package<&str> {
        Package::new(FileDescriptorProto::package(self))
    }

    fn try_syntax(&self) -> Result<Syntax> {
        Ok(match self.syntax() {
            "proto2" | "" => Syntax::Proto2,
            "proto3" => Syntax::Proto3,
            _ => Err(ErrorKind::UnknownProtoSyntax {
                name: self.syntax().to_string(),
            })?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Syntax {
    Proto2,
    Proto3,
}

pub trait DescriptorExt {}

impl DescriptorExt for DescriptorProto {}

pub trait EnumDescriptorExt {}

impl EnumDescriptorExt for EnumDescriptorProto {}

pub trait FieldDescriptorExt {}

impl FieldDescriptorExt for FieldDescriptorProto {}

pub trait FileOrMessage: Debug {
    fn messages(&self) -> &[DescriptorProto];
    fn enums(&self) -> &[EnumDescriptorProto];
}

impl FileOrMessage for FileDescriptorProto {
    fn messages(&self) -> &[DescriptorProto] {
        self.message_type()
    }
    fn enums(&self) -> &[EnumDescriptorProto] {
        self.enum_type()
    }
}

impl FileOrMessage for DescriptorProto {
    fn messages(&self) -> &[DescriptorProto] {
        self.nested_type()
    }
    fn enums(&self) -> &[EnumDescriptorProto] {
        self.enum_type()
    }
}

pub trait MessageOrEnum: Debug {
    fn name(&self) -> &str;
    fn to_either(&self) -> Either<&DescriptorProto, &EnumDescriptorProto>;
    fn is_message(&self) -> bool {
        matches!(self.to_either(), Either::Left(_))
    }
    fn is_enum(&self) -> bool {
        matches!(self.to_either(), Either::Right(_))
    }
    fn as_message(&self) -> Result<&DescriptorProto> {
        match self.to_either() {
            Either::Left(msg) => Ok(msg),
            Either::Right(_) => Err(ErrorKind::InternalError {
                detail: "enum has found where message is expected".to_string(),
            })?,
        }
    }
    fn as_enum(&self) -> Result<&EnumDescriptorProto> {
        match self.to_either() {
            Either::Left(_) => Err(ErrorKind::InternalError {
                detail: "enum has found where message is expected".to_string(),
            })?,
            Either::Right(e) => Ok(e),
        }
    }
}

impl MessageOrEnum for DescriptorProto {
    fn name(&self) -> &str {
        DescriptorProto::name(self)
    }
    fn to_either(&self) -> Either<&DescriptorProto, &EnumDescriptorProto> {
        Either::Left(self)
    }
}

impl MessageOrEnum for EnumDescriptorProto {
    fn name(&self) -> &str {
        EnumDescriptorProto::name(self)
    }
    fn to_either(&self) -> Either<&DescriptorProto, &EnumDescriptorProto> {
        Either::Right(self)
    }
}
