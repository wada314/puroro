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

use super::super::util::*;
use super::{DataTypeBase, Enum, Message, Package, PackageOrMessage, SourceCodeInfo, Syntax};
use crate::Result;
use ::once_cell::unsync::OnceCell;
use ::puroro::protobuf::google::protobuf::FileDescriptorProto;
use ::std::collections::HashMap;
use ::std::fmt::Debug;
use ::std::rc::{Rc, Weak};

#[derive(Debug)]
pub(crate) struct InputFile {
    cache: AnonymousCache,
    name: String,
    #[cfg(test)]
    package: Weak<Package>,
    syntax: String,
    syntax_cell: OnceCell<Syntax>,
    messages: Vec<Rc<Message>>,
    enums: Vec<Rc<Enum>>,
    source_code_info: HashMap<Vec<i32>, SourceCodeInfo>,
}

impl InputFile {
    pub(crate) fn new(proto: &FileDescriptorProto, package: Weak<Package>) -> Rc<Self> {
        Rc::new_cyclic(|weak| Self {
            cache: Default::default(),
            name: proto.name().to_string(),
            #[cfg(test)]
            package: Weak::clone(&package),
            syntax: proto.syntax().to_string(),
            syntax_cell: OnceCell::new(),
            messages: proto
                .message_type()
                .into_iter()
                .enumerate()
                .map(|(i, m)| {
                    Message::new(
                        m,
                        Weak::clone(weak) as Weak<InputFile>,
                        Weak::clone(&package) as Weak<dyn PackageOrMessage>,
                        i,
                    )
                })
                .collect(),
            enums: proto
                .enum_type()
                .into_iter()
                .enumerate()
                .map(|(i, e)| {
                    Enum::new(
                        e,
                        Weak::clone(weak) as Weak<InputFile>,
                        Weak::clone(&package) as Weak<dyn PackageOrMessage>,
                        i,
                    )
                })
                .collect(),
            source_code_info: proto
                .source_code_info()
                .into_iter()
                .flat_map(|sci| {
                    sci.location()
                        .into_iter()
                        .map(|loc| (loc.path().to_vec(), loc.into()))
                })
                .collect(),
        })
    }
}

impl DataTypeBase for InputFile {
    fn cache(&self) -> &AnonymousCache {
        &self.cache
    }
    fn name(&self) -> Result<&str> {
        Ok(&self.name)
    }
}

impl InputFile {
    pub(crate) fn syntax(&self) -> Result<Syntax> {
        self.syntax_cell
            .get_or_try_init(|| self.syntax.as_str().try_into())
            .cloned()
    }
    pub(crate) fn messages(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<Message>>>> {
        Ok(Box::new(self.messages.iter().cloned()))
    }
    pub(crate) fn enums(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<Enum>>>> {
        Ok(Box::new(self.enums.iter().cloned()))
    }
    #[cfg(test)]
    pub(crate) fn package(&self) -> Result<Rc<Package>> {
        self.package.try_upgrade()
    }
    pub(crate) fn source_code_info(
        &self,
        path: impl Iterator<Item = i32>,
    ) -> Result<Option<&SourceCodeInfo>> {
        let path_vec: Vec<i32> = path.collect();
        Ok(self.source_code_info.get(&path_vec))
    }
}
