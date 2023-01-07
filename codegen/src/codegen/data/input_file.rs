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
use super::{
    DataTypeBase, Enum, EnumImpl, Message, MessageImpl, Package, PackageOrMessage, Syntax,
};
use crate::Result;
use ::once_cell::unsync::OnceCell;
use ::puroro_protobuf_compiled::google::protobuf::{
    DescriptorProto, EnumDescriptorProto, FileDescriptorProto,
};
use ::std::fmt::Debug;
#[cfg(test)]
use ::std::iter;
use ::std::rc::{Rc, Weak};

pub trait InputFile: DataTypeBase + DataTypeBase + Debug {
    fn syntax(&self) -> Result<Syntax>;
    fn package(&self) -> Result<Rc<dyn Package>>;
    fn messages(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn Message>>>>;
    fn enums(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn Enum>>>>;
}

#[derive(Debug)]
pub struct InputFileImpl {
    cache: AnonymousCache,
    name: String,
    syntax: String,
    syntax_cell: OnceCell<Syntax>,
    package: Weak<dyn Package>,
    messages: Vec<Rc<dyn Message>>,
    enums: Vec<Rc<dyn Enum>>,
}

impl InputFileImpl {
    pub fn new(proto: &FileDescriptorProto, package: Weak<dyn Package>) -> Rc<Self> {
        Self::new_with(proto, package, MessageImpl::new, EnumImpl::new)
    }
    fn new_with<FM, M, FE, E>(
        proto: &FileDescriptorProto,
        package: Weak<dyn Package>,
        fm: FM,
        fe: FE,
    ) -> Rc<Self>
    where
        FM: Fn(&DescriptorProto, Weak<dyn InputFile>, Weak<dyn PackageOrMessage>) -> Rc<M>,
        FE: Fn(&EnumDescriptorProto, Weak<dyn InputFile>, Weak<dyn PackageOrMessage>) -> Rc<E>,
        M: 'static + Message,
        E: 'static + Enum,
    {
        Rc::new_cyclic(|weak| Self {
            cache: Default::default(),
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
                        Weak::clone(weak) as Weak<dyn InputFile>,
                        Weak::clone(&package) as Weak<dyn PackageOrMessage>,
                    ) as Rc<dyn Message>
                })
                .collect(),
            enums: proto
                .enum_type()
                .into_iter()
                .map(|e| {
                    fe(
                        e,
                        Weak::clone(weak) as Weak<dyn InputFile>,
                        Weak::clone(&package) as Weak<dyn PackageOrMessage>,
                    ) as Rc<dyn Enum>
                })
                .collect(),
        })
    }
}

impl DataTypeBase for InputFileImpl {
    fn cache(&self) -> &AnonymousCache {
        &self.cache
    }
    fn name(&self) -> Result<&str> {
        Ok(&self.name)
    }
}

impl InputFile for InputFileImpl {
    fn syntax(&self) -> Result<Syntax> {
        self.syntax_cell
            .get_or_try_init(|| self.syntax.as_str().try_into())
            .cloned()
    }
    fn package(&self) -> Result<Rc<dyn Package>> {
        self.package.try_upgrade()
    }
    fn messages(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn Message>>>> {
        Ok(Box::new(self.messages.iter().cloned()))
    }
    fn enums(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn Enum>>>> {
        Ok(Box::new(self.enums.iter().cloned()))
    }
}

#[cfg(test)]
#[derive(Debug)]
pub struct InputFileFake {
    name: String,
    package: Weak<dyn Package>,
}

#[cfg(test)]
impl InputFileFake {
    pub fn new(proto: &FileDescriptorProto, package: Weak<dyn Package>) -> Rc<Self> {
        Rc::new(Self {
            name: proto.name().to_string(),
            package,
        })
    }
}

#[cfg(test)]
impl DataTypeBase for InputFileFake {
    fn cache(&self) -> &AnonymousCache {
        unimplemented!()
    }
    fn name(&self) -> Result<&str> {
        Ok(&self.name)
    }
}

#[cfg(test)]
impl InputFile for InputFileFake {
    fn syntax(&self) -> Result<Syntax> {
        unimplemented!()
    }
    fn package(&self) -> Result<Rc<dyn Package>> {
        self.package.try_upgrade()
    }
    fn messages(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn Message>>>> {
        Ok(Box::new(iter::empty()))
    }
    fn enums(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn Enum>>>> {
        Ok(Box::new(iter::empty()))
    }
}
