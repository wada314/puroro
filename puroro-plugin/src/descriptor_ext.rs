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
use ::std::ops::Deref;
use ::std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct FileDescriptorExt {
    proto: FileDescriptorProto,
    message_type: Vec<Rc<DescriptorExt>>,
}

#[derive(Debug)]
pub struct DescriptorExt {
    proto: DescriptorProto,
    parent: WeakFileOrMessage,
    nested_type: Vec<Rc<DescriptorExt>>,
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
    fn new(source: &FileDescriptorProto) -> Rc<Self> {
        Rc::new_cyclic(|this| Self {
            proto: source.clone(),
            message_type: source
                .message_type()
                .iter()
                .map(|m| DescriptorExt::new(m, WeakFileOrMessage::File(this.clone())))
                .collect(),
        })
    }
}

impl DescriptorExt {
    fn new(source: &DescriptorProto, parent: WeakFileOrMessage) -> Rc<Self> {
        Rc::new_cyclic(|this| Self {
            proto: source.clone(),
            parent: parent.clone(),
            nested_type: source
                .nested_type()
                .iter()
                .map(|nm| DescriptorExt::new(nm, WeakFileOrMessage::Message(this.clone())))
                .collect(),
            field: source
                .field()
                .iter()
                .map(|f| FieldDescriptorExt::new(f, this.clone()))
                .collect(),
        })
    }

    fn parent(&self) -> Result<RcFileOrMessage> {
        self.parent.upgrade()
    }
}

impl EnumDescriptorExt {
    fn new(source: &EnumDescriptorProto, parent: WeakFileOrMessage) -> Rc<Self> {
        Rc::new(Self {
            proto: source.clone(),
            parent: parent.clone(),
        })
    }
}

impl FieldDescriptorExt {
    fn new(source: &FieldDescriptorProto, parent: Weak<DescriptorExt>) -> Rc<Self> {
        Rc::new(Self {
            proto: source.clone(),
            parent: parent.clone(),
        })
    }
}

impl WeakFileOrMessage {
    fn upgrade(&self) -> Result<RcFileOrMessage> {
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
