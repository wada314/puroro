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

mod r#enum;
mod field;
mod field_or_oneof;
mod field_rule;
mod field_type;
mod input_file;
mod message;
mod oneof;
mod oneof_field;
mod package;
mod package_or_message;

pub(crate) use self::r#enum::*;
pub(crate) use self::field::*;
pub(crate) use self::field_or_oneof::*;
pub(crate) use self::field_rule::*;
pub(crate) use self::field_type::*;
pub(crate) use self::input_file::*;
pub(crate) use self::message::*;
pub(crate) use self::oneof::*;
pub(crate) use self::oneof_field::*;
pub(crate) use self::package::*;
pub(crate) use self::package_or_message::*;

use super::util::AnonymousCache;
use crate::{FatalErrorKind, GeneratorError, Result};
use ::puroro_protobuf_compiled::google::protobuf::source_code_info;

const MESSAGE_FIELD_NUMBER_IN_FILE_DESCRIPTOR: i32 = 4;
const MESSAGE_FIELD_NUMBER_IN_MESSAGE_DESCRIPTOR: i32 = 3;
const ENUM_FIELD_NUMBER_IN_FILE_DESCRIPTOR: i32 = 5;
const ENUM_FIELD_NUMBER_IN_MESSAGE_DESCRIPTOR: i32 = 4;

pub(crate) trait DataTypeBase {
    fn cache(&self) -> &AnonymousCache;
    fn name(&self) -> Result<&str>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Syntax {
    Proto2,
    Proto3,
}
impl TryFrom<&str> for Syntax {
    type Error = GeneratorError;
    fn try_from(value: &str) -> Result<Self> {
        Ok(match value {
            "" | "proto2" => Syntax::Proto2,
            "proto3" => Syntax::Proto3,
            _ => Err(FatalErrorKind::UnknownProtoSyntax {
                name: value.to_string(),
            })?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum PackageOrMessageCase<P, M> {
    Package(P),
    Message(M),
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum MessageOrEnumCase<M, E> {
    Message(M),
    Enum(E),
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum FieldOrOneofCase<F, O> {
    Field(F),
    Oneof(O),
}

#[derive(Debug, Clone)]
pub(crate) struct SourceCodeInfo {
    pub(crate) leading_comments: String,
    pub(crate) trailing_comments: String,
}
impl From<&source_code_info::Location> for SourceCodeInfo {
    fn from(info: &source_code_info::Location) -> Self {
        Self {
            leading_comments: info.leading_comments().to_string(),
            trailing_comments: info.trailing_comments().to_string(),
        }
    }
}
impl From<&SourceCodeInfo> for Option<String> {
    fn from(sci: &SourceCodeInfo) -> Option<String> {
        match (sci.leading_comments.len(), sci.trailing_comments.len()) {
            (0, 0) => None,
            (_, 0) => Some(sci.leading_comments.clone()),
            (0, _) => Some(sci.trailing_comments.clone()),
            _ => Some(format!(
                "{}\n\n{}",
                &sci.leading_comments, &sci.trailing_comments,
            )),
        }
    }
}
