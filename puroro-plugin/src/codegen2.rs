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
mod file;
mod message;
mod oneof;
mod package;
use r#enum::Enum;
use file::File;
use message::Message;
use oneof::Oneof;
use package::Package;

use crate::{ErrorKind, GeneratorError, Result};

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
