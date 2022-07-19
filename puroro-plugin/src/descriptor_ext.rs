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
use ::itertools::{Either, Itertools};
use ::puroro_protobuf_compiled::google::protobuf::{
    DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, FileDescriptorProto,
};
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
        self.try_get_parent()?.try_get_file()
    }

    pub fn try_get_package_path_opt(&self) -> Result<Option<String>> {
        Ok(self.try_get_parent()?.try_get_package_path_opt()?)
    }

    pub fn try_get_enclosing_messages_path_opt(&self) -> Result<Option<String>> {
        let vec_msgs = self
            .try_traverse_enclosing_messages()
            .collect::<Result<Vec<_>>>()?;
        if vec_msgs.is_empty() {
            Ok(None)
        } else {
            Ok(Some(
                vec_msgs
                    .into_iter()
                    .rev()
                    .map(|m| m.name().to_string())
                    .join("."),
            ))
        }
    }

    // returns in inner message to outer message order
    pub fn try_traverse_enclosing_messages(
        &self,
    ) -> impl Iterator<Item = Result<Rc<DescriptorExt>>> {
        let either: Either<_, _> = self.try_get_parent().into();
        either
            .map_left(|e| ::std::iter::once(Err(e)))
            .map_right(|parent| parent.try_traverse_self_and_enclosing_messages())
            .into_iter()
    }

    pub fn try_fqtn(&self) -> Result<String> {
        Ok(self
            .try_get_package_path_opt()?
            .into_iter()
            .chain(self.try_get_enclosing_messages_path_opt()?.into_iter())
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

    pub fn try_get_parent(&self) -> Result<RcFileOrMessage> {
        self.parent.try_upgrade()
    }

    pub fn try_get_package_path_opt(&self) -> Result<Option<String>> {
        Ok(self.try_get_parent()?.try_get_package_path_opt()?)
    }

    pub fn try_get_enclosing_messages_path_opt(&self) -> Result<Option<String>> {
        let vec_msgs = self
            .try_traverse_enclosing_messages()
            .collect::<Result<Vec<_>>>()?;
        if vec_msgs.is_empty() {
            Ok(None)
        } else {
            Ok(Some(
                vec_msgs
                    .into_iter()
                    .rev()
                    .map(|m| m.name().to_string())
                    .join("."),
            ))
        }
    }

    // returns in inner message to outer message order
    pub fn try_traverse_enclosing_messages(
        &self,
    ) -> impl Iterator<Item = Result<Rc<DescriptorExt>>> {
        let either: Either<_, _> = self.try_get_parent().into();
        either
            .map_left(|e| ::std::iter::once(Err(e)))
            .map_right(|parent| parent.try_traverse_self_and_enclosing_messages())
            .into_iter()
    }

    pub fn try_fqtn(&self) -> Result<String> {
        let package = self.try_get_package_path_opt()?;
        let enclosing_messages = self.try_get_enclosing_messages_path_opt()?;
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

#[derive(Debug, Clone)]
enum FileOrMessage<F, M> {
    File(F),
    Message(M),
}

impl<F, M> FileOrMessage<F, M> {
    pub fn map<G: FnOnce(F) -> F2, H: FnOnce(M) -> M2, F2, M2>(
        self,
        g: G,
        h: H,
    ) -> FileOrMessage<F2, M2> {
        use FileOrMessage::*;
        match self {
            File(f) => File(g(f)),
            Message(m) => Message(h(m)),
        }
    }
    pub fn either<G: FnOnce(F) -> T, H: FnOnce(M) -> T, T>(self, g: G, h: H) -> T {
        use FileOrMessage::*;
        match self {
            File(f) => g(f),
            Message(m) => h(m),
        }
    }
}

impl FileOrMessage<Weak<FileDescriptorExt>, Weak<DescriptorExt>> {
    pub fn try_upgrade(&self) -> Result<FileOrMessage<Rc<FileDescriptorExt>, Rc<DescriptorExt>>> {
        Ok(match self {
            FileOrMessage::File(f) => {
                FileOrMessage::File(Weak::upgrade(f).ok_or(ErrorKind::WeakUpgradeFailure)?)
            }
            FileOrMessage::Message(m) => {
                FileOrMessage::Message(Weak::upgrade(m).ok_or(ErrorKind::WeakUpgradeFailure)?)
            }
        })
    }
}

impl FileOrMessage<Rc<FileDescriptorExt>, Rc<DescriptorExt>> {
    pub fn try_get_file(self) -> Result<Rc<FileDescriptorExt>> {
        use FileOrMessage::*;
        let mut f_or_m = self;
        loop {
            match f_or_m {
                File(f) => break Ok(f),
                Message(m) => f_or_m = m.try_get_parent()?,
            }
        }
    }
}

impl<F, M> FileOrMessage<F, M>
where
    F: Deref<Target = FileDescriptorExt>,
    M: Deref<Target = DescriptorExt>,
{
}

impl<F, M> FileOrMessage<F, M>
where
    F: Deref,
    M: Deref,
{
    pub fn as_deref(&self) -> FileOrMessage<&<F as Deref>::Target, &<M as Deref>::Target> {
        self.map(|f| f.deref(), |m| m.deref())
    }
}

type RcFileOrMessage = FileOrMessage<Rc<FileDescriptorExt>, Rc<DescriptorExt>>;
type WeakFileOrMessage = FileOrMessage<Weak<FileDescriptorExt>, Weak<DescriptorExt>>;

#[derive(Debug, Clone)]
enum MessageOrEnum<M, E> {
    Message(M),
    Enum(E),
}

impl<M, E> MessageOrEnum<M, E>
where
    M: Deref<Target = DescriptorExt>,
    E: Deref<Target = EnumDescriptorExt>,
{
    pub fn try_get_parent(&self) -> Result<RcFileOrMessage> {
        match self {
            MessageOrEnum::Message(m) => m.try_get_parent(),
            MessageOrEnum::Enum(e) => e.try_get_parent(),
        }
    }

    pub fn try_traverse_enclosing_messages(
        &self,
    ) -> impl Iterator<Item = Result<Rc<DescriptorExt>>> {
        let opt_try_parent = match self.try_get_parent() {
            Ok(RcFileOrMessage::File(_)) => None,
            Ok(RcFileOrMessage::Message(parent)) => Some(Ok(parent.clone())),
            Err(e) => Some(Err(e)),
        };
        ::std::iter::successors(opt_try_parent, |try_m| {
            try_m.as_ref().ok().and_then(|m| match m.try_get_parent() {
                Ok(RcFileOrMessage::File(_)) => None,
                Ok(RcFileOrMessage::Message(parent)) => Some(Ok(parent)),
                Err(e) => Some(Err(e)),
            })
        })
    }
}
