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
use ::puroro_protobuf_compiled::google::protobuf::{
    DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, FileDescriptorProto,
    OneofDescriptorProto,
};
use ::std::borrow::Cow;
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

    pub fn parent(&self) -> Result<RcFileOrMessage> {
        self.parent.upgrade()
    }

    pub fn package_opt(&self) -> Result<Option<Cow<str>>> {
        Ok(self.parent()?.package_opt()?.map(|s| s.into_owned().into()))
    }

    pub fn enclosing_messages(&self) -> Result<Option<Cow<str>>> {
        todo!()
    }

    pub fn fqtn(&self) -> Result<Cow<str>> {
        let package = self.package_opt()?;
        todo!()
    }

    pub fn nested_type(&self) -> &[Rc<DescriptorExt>] {
        &self.nested_type
    }

    pub fn enum_type(&self) -> &[Rc<EnumDescriptorExt>] {
        &self.enum_type
    }
}

impl EnumDescriptorExt {
    pub fn new(source: &EnumDescriptorProto, parent: WeakFileOrMessage) -> Rc<Self> {
        Rc::new(Self {
            proto: source.clone(),
            parent: parent.clone(),
        })
    }

    pub fn parent(&self) -> Result<RcFileOrMessage> {
        self.parent.upgrade()
    }

    pub fn package_opt(&self) -> Result<Option<Cow<str>>> {
        Ok(self.parent()?.package_opt()?.map(|s| s.into_owned().into()))
    }

    pub fn fqtn(&self) -> Result<Cow<str>> {
        let package = self.package_opt()?;
        todo!()
    }
}

impl FieldDescriptorExt {
    pub fn new(source: &FieldDescriptorProto, parent: Weak<DescriptorExt>) -> Rc<Self> {
        Rc::new(Self {
            proto: source.clone(),
            parent: parent.clone(),
        })
    }

    pub fn parent(&self) -> Result<Rc<DescriptorExt>> {
        Ok(self.parent.upgrade().ok_or(ErrorKind::WeakUpgradeFailure)?)
    }
}

impl WeakFileOrMessage {
    pub fn upgrade(&self) -> Result<RcFileOrMessage> {
        Ok(match self {
            WeakFileOrMessage::File(f) => {
                RcFileOrMessage::File(Weak::upgrade(f).ok_or(ErrorKind::WeakUpgradeFailure)?)
            }
            WeakFileOrMessage::Message(m) => {
                RcFileOrMessage::Message(Weak::upgrade(m).ok_or(ErrorKind::WeakUpgradeFailure)?)
            }
        })
    }
}

impl RcFileOrMessage {
    pub fn package_opt(&self) -> Result<Option<Cow<str>>> {
        Ok(match self {
            RcFileOrMessage::File(f) => f.package_opt().map(|s| s.into()),
            RcFileOrMessage::Message(m) => m.package_opt()?.map(|s| s.into()),
        })
    }

    pub fn enclosing_messages(&self) -> Result<Option<Cow<str>>> {
        todo!()
    }
}
