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
use ::once_cell::unsync::OnceCell;
use ::puroro_protobuf_compiled::google::protobuf::{DescriptorProto, FileDescriptorProto};
use ::quote::quote;
use ::std::fmt::Debug;
use ::std::ops::Deref;
use ::std::rc::{Rc, Weak};

pub(super) trait InputFileTrait: Debug {
    fn syntax(&self) -> Result<Syntax>;
    fn gen_structs_for_messages(&self) -> Result<TokenStream>;
    fn messages(&self) -> Box<dyn '_ + Iterator<Item = Weak<dyn MessageTrait>>>;
    fn enums(&self) -> Box<dyn '_ + Iterator<Item = Weak<dyn EnumTrait>>>;
}

#[derive(Debug)]
pub(super) struct InputFile {
    syntax: String,
    syntax_cell: OnceCell<Syntax>,
    messages: Vec<Rc<dyn MessageTrait>>,
}

impl InputFile {
    pub(super) fn new(proto: &FileDescriptorProto) -> Rc<dyn InputFileTrait> {
        Self::new_with(proto, |m, weak| Message::new(m, &weak))
    }
    fn new_with<FM>(proto: &FileDescriptorProto, mut fm: FM) -> Rc<dyn InputFileTrait>
    where
        FM: FnMut(&DescriptorProto, Weak<InputFile>) -> Rc<dyn MessageTrait>,
    {
        Rc::new_cyclic(|weak| Self {
            syntax: proto.syntax().to_string(),
            syntax_cell: OnceCell::new(),
            messages: proto
                .message_type()
                .into_iter()
                .map(|m| fm(m, Weak::clone(weak)))
                .collect(),
        }) as Rc<dyn InputFileTrait>
    }
}

impl InputFileTrait for InputFile {
    fn syntax(&self) -> Result<Syntax> {
        self.syntax_cell
            .get_or_try_init(|| self.syntax.as_str().try_into())
            .cloned()
    }

    fn gen_structs_for_messages(&self) -> Result<TokenStream> {
        let message_structs = self
            .messages
            .iter()
            .map(|m| m.gen_struct())
            .collect::<Result<Vec<_>>>()?;
        Ok(quote! {
            #(#message_structs)*
        })
    }

    fn messages(&self) -> Box<dyn '_ + Iterator<Item = Weak<dyn MessageTrait>>> {
        Box::new(self.messages.iter().map(|m| Rc::downgrade(m)))
    }

    fn enums(&self) -> Box<dyn '_ + Iterator<Item = Weak<dyn EnumTrait>>> {
        Box::new(::std::iter::empty())
    }
}
