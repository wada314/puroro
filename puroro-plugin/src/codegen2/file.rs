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

use super::*;
use crate::Result;
use ::puroro_protobuf_compiled::google::protobuf::FileDescriptorProto;

pub trait File: Sized {
    fn try_new(proto: &FileDescriptorProto) -> Result<Self>;
}

#[cfg(test)]
pub struct FileFake {
    proto: FileDescriptorProto,
}

#[cfg(test)]
impl File for FileFake {
    fn try_new(proto: &FileDescriptorProto) -> Result<Self> {
        Ok(FileFake {
            proto: proto.clone(),
        })
    }
}

#[derive(Debug)]
pub struct FileImpl {
    syntax: Syntax,
    messages: Vec<Message>,
    enums: Vec<Enum>,
}

impl File for FileImpl {
    fn try_new(proto: &FileDescriptorProto) -> Result<Self> {
        Ok(Self {
            syntax: proto.syntax().try_into()?,
            messages: proto
                .message_type()
                .into_iter()
                .map(|m| Message::try_new(m))
                .collect::<Result<Vec<_>>>()?,
            enums: proto
                .enum_type()
                .into_iter()
                .map(|e| Enum::try_new(e))
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

impl FileImpl {
    pub fn messages(&self) -> Result<&[Message]> {
        Ok(&self.messages)
    }
    pub fn enums(&self) -> Result<&[Enum]> {
        Ok(&self.enums)
    }
}
