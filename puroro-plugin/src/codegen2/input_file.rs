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
use ::puroro_protobuf_compiled::google::protobuf::{
    DescriptorProto, EnumDescriptorProto, FileDescriptorProto,
};
use ::quote::quote;
use ::std::fmt::Debug;
use ::std::rc::{Rc, Weak};

pub(super) trait InputFileTrait: Debug {
    fn name(&self) -> Result<&str>;
    fn syntax(&self) -> Result<Syntax>;
    fn package(&self) -> Result<Rc<dyn PackageTrait>>;
    fn messages(&self) -> Result<&[Rc<dyn MessageTrait>]>;
    fn enums(&self) -> Result<&[Rc<dyn EnumTrait>]>;
    fn gen_structs_for_messages(&self) -> Result<TokenStream>;
}

#[derive(Debug)]
pub(super) struct InputFile {
    name: String,
    syntax: String,
    syntax_cell: OnceCell<Syntax>,
    package: Weak<dyn PackageTrait>,
    messages: Vec<Rc<dyn MessageTrait>>,
    enums: Vec<Rc<dyn EnumTrait>>,
}

impl InputFile {
    pub(super) fn new(proto: &FileDescriptorProto, package: Weak<dyn PackageTrait>) -> Rc<Self> {
        Self::new_with(proto, package, Message::new, Enum::new)
    }
    fn new_with<FM, M, FE, E>(
        proto: &FileDescriptorProto,
        package: Weak<dyn PackageTrait>,
        fm: FM,
        fe: FE,
    ) -> Rc<Self>
    where
        FM: Fn(
            &DescriptorProto,
            Weak<dyn InputFileTrait>,
            PackageOrMessage<Weak<dyn PackageTrait>, Weak<dyn MessageTrait>>,
        ) -> Rc<M>,
        FE: Fn(
            &EnumDescriptorProto,
            Weak<dyn InputFileTrait>,
            PackageOrMessage<Weak<dyn PackageTrait>, Weak<dyn MessageTrait>>,
        ) -> Rc<E>,
        M: 'static + MessageTrait,
        E: 'static + EnumTrait,
    {
        Rc::new_cyclic(|weak| Self {
            name: proto.name().to_string(),
            syntax: proto.syntax().to_string(),
            syntax_cell: OnceCell::new(),
            package: Weak::clone(&package),
            messages: proto
                .message_type()
                .into_iter()
                .map(|m| {
                    fm(
                        m,
                        Weak::clone(weak) as Weak<dyn InputFileTrait>,
                        PackageOrMessage::Package(Weak::clone(&package)),
                    ) as Rc<dyn MessageTrait>
                })
                .collect(),
            enums: proto
                .enum_type()
                .into_iter()
                .map(|e| {
                    fe(
                        e,
                        Weak::clone(weak) as Weak<dyn InputFileTrait>,
                        PackageOrMessage::Package(Weak::clone(&package)),
                    ) as Rc<dyn EnumTrait>
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

    fn messages(&self) -> Result<&[Rc<dyn MessageTrait>]> {
        Ok(&self.messages)
    }

    fn enums(&self) -> Result<&[Rc<dyn EnumTrait>]> {
        Ok(&self.enums)
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

#[cfg(test)]
#[derive(Debug)]
pub struct InputFileFake {
    name: String,
    package: Weak<dyn PackageTrait>,
}

#[cfg(test)]
impl InputFileFake {
    pub(super) fn new(proto: &FileDescriptorProto, package: Weak<dyn PackageTrait>) -> Rc<Self> {
        Rc::new(Self {
            name: proto.name().to_string(),
            package,
        })
    }
}

#[cfg(test)]
impl InputFileTrait for InputFileFake {
    fn name(&self) -> Result<&str> {
        Ok(&self.name)
    }
    fn syntax(&self) -> Result<Syntax> {
        unimplemented!()
    }
    fn package(&self) -> Result<Rc<dyn PackageTrait>> {
        self.package.try_upgrade()
    }
    fn gen_structs_for_messages(&self) -> Result<TokenStream> {
        Ok(TokenStream::new())
    }
    fn messages(&self) -> Result<&[Rc<dyn crate::codegen2::message::MessageTrait>]> {
        Ok(&[])
    }
    fn enums(&self) -> Result<&[Rc<dyn crate::codegen2::r#enum::EnumTrait>]> {
        Ok(&[])
    }
}
