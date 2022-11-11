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
use ::quote::quote;
use ::std::fmt::Debug;
use ::std::rc::{Rc, Weak};

pub trait InputFileTrait: Debug {}

#[cfg(test)]
pub struct InputFileFake {
    pub proto: FileDescriptorProto,
}

#[cfg(test)]
impl InputFileFake {
    fn try_new(proto: &FileDescriptorProto) -> Result<Self> {
        Ok(InputFileFake {
            proto: proto.clone(),
        })
    }
}

#[derive(Debug)]
pub struct InputFileImpl {
    syntax: Syntax,
    messages: Vec<Rc<dyn MessageTrait>>,
    enums: Vec<Rc<dyn EnumTrait>>,
}

pub type InputFile = InputFileImpl;

impl InputFileTrait for InputFileImpl {
    fn try_new(proto: &FileDescriptorProto) -> Result<Self> {
        Ok(Self {
            syntax: proto.syntax().try_into()?,
            messages: proto
                .message_type()
                .into_iter()
                .map(|m| MessageType::try_new(m))
                .collect::<Result<Vec<_>>>()?,
            enums: proto
                .enum_type()
                .into_iter()
                .map(|e| EnumType::try_new(e))
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

impl InputFileImpl {
    pub fn gen_structs_for_messages(&self) -> Result<TokenStream> {
        let message_structs = self
            .messages
            .iter()
            .map(|m| m.gen_struct())
            .collect::<Result<Vec<_>>>()?;
        Ok(quote! {
            #(#message_structs)*
        })
    }
}
