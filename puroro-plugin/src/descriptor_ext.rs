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

//! Extend the raw protobuf descriptors to add a pointer to the parent descriptor.

use crate::{ErrorKind, Result};
use ::itertools::Itertools;
use ::puroro_protobuf_compiled::google::protobuf::{
    DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, FileDescriptorProto,
};
use ::std::borrow::Cow;
use ::std::iter;
use ::std::ops::Deref;
use ::std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct FileDescriptorExt {
    proto: FileDescriptorProto,
    message_type: Vec<Rc<DescriptorExt>>,
    enum_type: Vec<Rc<EnumDescriptorExt>>,
}

#[derive(Debug)]
pub struct DescriptorExt {
    proto: DescriptorProto,
    parent: WeakFileOrMessage,
    nested_type: Vec<Rc<DescriptorExt>>,
    enum_type: Vec<Rc<EnumDescriptorExt>>,
    field: Vec<Rc<FieldDescriptorExt>>,
}

#[derive(Debug)]
pub struct EnumDescriptorExt {
    proto: EnumDescriptorProto,
    parent: WeakFileOrMessage,
}

#[derive(Debug)]
pub struct FieldDescriptorExt {
    proto: FieldDescriptorProto,
    #[allow(unused)]
    parent: Weak<DescriptorExt>,
}

#[derive(Debug, Clone)]
pub enum WeakFileOrMessage {
    File(Weak<FileDescriptorExt>),
    Message(Weak<DescriptorExt>),
}

#[derive(Debug, Clone)]
pub enum RcFileOrMessage {
    File(Rc<FileDescriptorExt>),
    Message(Rc<DescriptorExt>),
}

impl Deref for FileDescriptorExt {
    type Target = FileDescriptorProto;
    fn deref(&self) -> &Self::Target {
        &self.proto
    }
}
impl Deref for DescriptorExt {
    type Target = DescriptorProto;
    fn deref(&self) -> &Self::Target {
        &self.proto
    }
}
impl Deref for EnumDescriptorExt {
    type Target = EnumDescriptorProto;
    fn deref(&self) -> &Self::Target {
        &self.proto
    }
}
impl Deref for FieldDescriptorExt {
    type Target = FieldDescriptorProto;
    fn deref(&self) -> &Self::Target {
        &self.proto
    }
}

impl FileDescriptorExt {
    pub fn new(source: &FileDescriptorProto) -> Rc<Self> {
        Rc::new_cyclic(|this| Self {
            proto: source.clone(),
            message_type: source
                .message_type()
                .iter()
                .map(|m| DescriptorExt::new(m, WeakFileOrMessage::File(this.clone())))
                .collect(),
            enum_type: source
                .enum_type()
                .iter()
                .map(|e| EnumDescriptorExt::new(e, WeakFileOrMessage::File(this.clone())))
                .collect(),
        })
    }

    pub fn message_type(&self) -> &[Rc<DescriptorExt>] {
        &self.message_type
    }

    pub fn enum_type(&self) -> &[Rc<EnumDescriptorExt>] {
        &self.enum_type
    }

    pub fn for_each_message<F: FnMut(Rc<DescriptorExt>)>(&self, mut f: F) {
        fn inner<F: FnMut(Rc<DescriptorExt>)>(m: Rc<DescriptorExt>, mut f: F) -> F {
            f(Rc::clone(&m));
            for mm in m.nested_type().iter() {
                f = inner(Rc::clone(mm), f);
            }
            f
        }
        for m in self.message_type().iter() {
            f = inner(Rc::clone(m), f);
        }
    }

    pub fn for_each_enum<F: FnMut(Rc<EnumDescriptorExt>)>(&self, mut f: F) {
        fn inner<F: FnMut(Rc<EnumDescriptorExt>)>(m: Rc<DescriptorExt>, mut f: F) -> F {
            for e in m.enum_type().iter() {
                f(Rc::clone(e));
            }
            for mm in m.nested_type().iter() {
                f = inner(Rc::clone(mm), f);
            }
            f
        }
        for e in self.enum_type().iter() {
            f(Rc::clone(e));
        }
        for m in self.message_type().iter() {
            f = inner(Rc::clone(m), f);
        }
    }
}

impl DescriptorExt {
    pub fn new(source: &DescriptorProto, parent: WeakFileOrMessage) -> Rc<Self> {
        Rc::new_cyclic(|this| Self {
            proto: source.clone(),
            parent: parent.clone(),
            nested_type: source
                .nested_type()
                .iter()
                .map(|nm| DescriptorExt::new(nm, WeakFileOrMessage::Message(this.clone())))
                .collect(),
            enum_type: source
                .enum_type()
                .iter()
                .map(|e| EnumDescriptorExt::new(e, WeakFileOrMessage::Message(this.clone())))
                .collect(),
            field: source
                .field()
                .iter()
                .map(|f| FieldDescriptorExt::new(f, this.clone()))
                .collect(),
        })
    }

    pub fn try_get_parent(&self) -> Result<RcFileOrMessage> {
        self.parent.try_upgrade()
    }

    pub fn try_get_file(&self) -> Result<Rc<FileDescriptorExt>> {
        self.parent.try_get_file()
    }

    pub fn try_package_path_opt(&self) -> Result<Option<Cow<str>>> {
        Ok(self
            .try_get_parent()?
            .try_package_opt()?
            .map(|s| s.into_owned().into()))
    }

    pub fn try_enclosing_messages_path_opt(&self) -> Result<Option<Cow<str>>> {
        Ok(self
            .try_get_parent()?
            .try_enclosing_messages_opt()?
            .map(|s| s.into_owned().into()))
    }

    // returns in inner message to outer message order
    pub fn try_traverse_enclosing_messages(
        &self,
    ) -> impl Iterator<Item = Result<Rc<DescriptorExt>>> {
        let parent = match self.try_parent() {
            Ok(RcFileOrMessage::File(_)) => None,
            Ok(RcFileOrMessage::Message(parent)) => Some(Ok(parent)),
            Err(e) => Some(Err(e)),
        };
        ::std::iter::successors(parent, |m| match m.try_parent() {
            Ok(RcFileOrMessage::File(_)) => None,
            Ok(RcFileOrMessage::Message(parent)) => Some(Ok(parent)),
            Err(e) => Some(Err(e)),
        })
    }

    pub fn try_fqtn(&self) -> Result<Cow<str>> {
        Ok(self
            .try_package_path_opt()?
            .into_iter()
            .chain(self.try_enclosing_messages_path_opt()?.into_iter())
            .chain(iter::once(self.name().into()))
            .join(".")
            .into())
    }

    pub fn nested_type(&self) -> &[Rc<DescriptorExt>] {
        &self.nested_type
    }

    pub fn enum_type(&self) -> &[Rc<EnumDescriptorExt>] {
        &self.enum_type
    }

    pub fn field(&self) -> &[Rc<FieldDescriptorExt>] {
        &self.field
    }
}

impl EnumDescriptorExt {
    pub fn new(source: &EnumDescriptorProto, parent: WeakFileOrMessage) -> Rc<Self> {
        Rc::new(Self {
            proto: source.clone(),
            parent: parent.clone(),
        })
    }

    pub fn try_parent(&self) -> Result<RcFileOrMessage> {
        self.parent.try_upgrade()
    }

    pub fn try_package_path_opt(&self) -> Result<Option<Cow<str>>> {
        Ok(self
            .try_parent()?
            .try_package_opt()?
            .map(|s| s.into_owned().into()))
    }

    pub fn try_enclosing_messages_path_opt(&self) -> Result<Option<Cow<str>>> {
        Ok(self
            .try_parent()?
            .try_enclosing_messages_opt()?
            .map(|s| s.into_owned().into()))
    }

    // returns in inner message to outer message order
    pub fn try_traverse_enclosing_messages(
        &self,
    ) -> impl Iterator<Item = Result<Rc<DescriptorExt>>> {
        let parent = match self.try_parent() {
            Ok(RcFileOrMessage::File(_)) => None,
            Ok(RcFileOrMessage::Message(parent)) => Some(Ok(parent)),
            Err(e) => Some(Err(e)),
        };
        ::std::iter::successors(parent, |m| match m.try_parent() {
            Ok(RcFileOrMessage::File(_)) => None,
            Ok(RcFileOrMessage::Message(parent)) => Some(Ok(parent)),
            Err(e) => Some(Err(e)),
        })
    }

    pub fn try_fqtn(&self) -> Result<Cow<str>> {
        let package = self.try_package_path_opt()?;
        let enclosing_messages = self.try_enclosing_messages_path_opt()?;
        Ok(package
            .into_iter()
            .chain(enclosing_messages.into_iter())
            .chain(iter::once(self.name().into()))
            .join(".")
            .into())
    }
}

impl FieldDescriptorExt {
    pub fn new(source: &FieldDescriptorProto, parent: Weak<DescriptorExt>) -> Rc<Self> {
        Rc::new(Self {
            proto: source.clone(),
            parent: parent.clone(),
        })
    }

    #[allow(unused)]
    pub fn try_parent(&self) -> Result<Rc<DescriptorExt>> {
        Ok(self.parent.upgrade().ok_or(ErrorKind::WeakUpgradeFailure)?)
    }
}

impl WeakFileOrMessage {
    pub fn try_upgrade(&self) -> Result<RcFileOrMessage> {
        Ok(match self {
            WeakFileOrMessage::File(f) => {
                RcFileOrMessage::File(Weak::upgrade(f).ok_or(ErrorKind::WeakUpgradeFailure)?)
            }
            WeakFileOrMessage::Message(m) => {
                RcFileOrMessage::Message(Weak::upgrade(m).ok_or(ErrorKind::WeakUpgradeFailure)?)
            }
        })
    }

    pub fn try_get_file(&self) -> Result<Rc<FileDescriptorExt>> {
        let mut f_or_m = self.try_upgrade()?;
        loop {
            match f_or_m {
                RcFileOrMessage::File(f) => break Ok(f.clone()),
                RcFileOrMessage::Message(m) => f_or_m = m.try_get_parent()?,
            }
        }
    }
}

impl RcFileOrMessage {
    pub fn try_package_opt(&self) -> Result<Option<Cow<str>>> {
        Ok(match self {
            RcFileOrMessage::File(f) => f.package_opt().map(|s| s.into()),
            RcFileOrMessage::Message(m) => m.try_package_path_opt()?.map(|s| s.into()),
        })
    }

    pub fn try_enclosing_messages_opt(&self) -> Result<Option<Cow<str>>> {
        Ok(match self {
            RcFileOrMessage::File(_) => None,
            RcFileOrMessage::Message(m) => Some(
                m.try_enclosing_messages_path_opt()?
                    .into_iter()
                    .chain(iter::once(m.name().into()))
                    .join(".")
                    .into(),
            ),
        })
    }
}
