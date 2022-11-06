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
mod file;
mod message;
mod oneof;
mod package;
mod util;
use self::r#enum::*;
use self::field::*;
use self::file::*;
use self::message::*;
use self::oneof::*;
use self::package::*;
use self::util::*;
use crate::{ErrorKind, GeneratorError, Result};
use ::puroro_protobuf_compiled::google::protobuf::FileDescriptorProto;
use ::std::rc::Rc;

#[derive(Debug, Clone, Copy)]
enum Syntax {
    Proto2,
    Proto3,
}
impl TryFrom<&str> for Syntax {
    type Error = GeneratorError;
    fn try_from(value: &str) -> Result<Self> {
        Ok(match value {
            "" | "proto2" => Syntax::Proto2,
            "proto3" => Syntax::Proto3,
            _ => Err(ErrorKind::UnknownProtoSyntax {
                name: value.to_string(),
            })?,
        })
    }
}

pub fn generate_root_package<'a>(
    files: impl Iterator<Item = &'a FileDescriptorProto>,
) -> Result<Rc<RootPackage<File>>> {
    RootPackage::try_new_from_files(files)
}
