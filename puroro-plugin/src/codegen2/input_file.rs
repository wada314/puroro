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
    fn name(&self) -> Result<&str>;
    fn syntax(&self) -> Result<Syntax>;
    fn package(&self) -> Result<Rc<dyn PackageTrait>>;
    fn gen_structs_for_messages(&self) -> Result<TokenStream>;
}

#[derive(Debug)]
pub(super) struct InputFile {
    name: String,
    syntax: String,
    syntax_cell: OnceCell<Syntax>,
    package: Weak<dyn PackageTrait>,
    messages: Vec<Rc<dyn MessageTrait>>,
}

impl InputFile {
    pub(super) fn new(proto: &FileDescriptorProto, package: Weak<dyn PackageTrait>) -> Rc<Self> {
        Self::new_with(proto, package, Message::new)
    }
    fn new_with<FM, M>(
        proto: &FileDescriptorProto,
        package: Weak<dyn PackageTrait>,
        mut fm: FM,
    ) -> Rc<Self>
    where
        FM: FnMut(&DescriptorProto, Weak<dyn InputFileTrait>) -> Rc<M>,
        M: 'static + MessageTrait,
    {
        Rc::new_cyclic(|weak| Self {
            name: proto.name().to_string(),
            syntax: proto.syntax().to_string(),
            syntax_cell: OnceCell::new(),
            package,
            messages: proto
                .message_type()
                .into_iter()
                .map(|m| {
                    fm(m, Weak::clone(weak) as Weak<dyn InputFileTrait>) as Rc<dyn MessageTrait>
                })
                .collect(),
        })
    }
}

impl InputFileTrait for InputFile {
    fn name(&self) -> Result<&str> {
        Ok(&self.name)
    }

    fn syntax(&self) -> Result<Syntax> {
        self.syntax_cell
            .get_or_try_init(|| self.syntax.as_str().try_into())
            .cloned()
    }

    fn package(&self) -> Result<Rc<dyn PackageTrait>> {
        self.package.try_upgrade()
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
}
