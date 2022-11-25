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
use super::super::{
    Enum, EnumImpl, Field, FieldImpl, InputFile, Package, PackageOrMessage, RootPackage,
};
use crate::Result;
use ::puroro_protobuf_compiled::google::protobuf::{
    DescriptorProto, EnumDescriptorProto, FieldDescriptorProto,
};
use ::std::fmt::Debug;
use ::std::iter;
use ::std::rc::{Rc, Weak};

pub trait Message: Debug + PackageOrMessage {
    fn cache(&self) -> &AnonymousCache;
    fn as_dyn_rc(self: Rc<Self>) -> Rc<dyn Message>;
    fn input_file(&self) -> Result<Rc<dyn InputFile>>;
    fn parent(&self) -> Result<Rc<dyn PackageOrMessage>>;
    fn fields(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn Field>>>>;

    fn should_generate_module_file(&self) -> Result<bool> {
        let has_submessages = self.messages()?.next().is_some();
        let has_subenums = self.enums()?.next().is_some();
        let has_oneofs = false; // TODO!
        Ok(has_submessages || has_subenums || has_oneofs)
    }
}

#[derive(Debug)]
pub struct MessageImpl {
    cache1: AnonymousCache,
    cache2: AnonymousCache,
    name: String,
    fields: Vec<Rc<dyn Field>>,
    messages: Vec<Rc<dyn Message>>,
    enums: Vec<Rc<dyn Enum>>,
    input_file: Weak<dyn InputFile>,
    parent: Weak<dyn PackageOrMessage>,
}

impl MessageImpl {
    pub fn new(
        proto: &DescriptorProto,
        input_file: Weak<dyn InputFile>,
        parent: Weak<dyn PackageOrMessage>,
    ) -> Rc<Self> {
        Self::new_with(
            proto,
            input_file,
            parent,
            FieldImpl::new,
            MessageImpl::new,
            EnumImpl::new,
        )
    }

    pub fn new_with<FF, F, FM, M, FE, E>(
        proto: &DescriptorProto,
        input_file: Weak<dyn InputFile>,
        parent: Weak<dyn PackageOrMessage>,
        ff: FF,
        fm: FM,
        fe: FE,
    ) -> Rc<Self>
    where
        FF: Fn(&FieldDescriptorProto, Weak<dyn Message>) -> Rc<F>,
        FM: Fn(&DescriptorProto, Weak<dyn InputFile>, Weak<dyn PackageOrMessage>) -> Rc<M>,
        FE: Fn(&EnumDescriptorProto, Weak<dyn InputFile>, Weak<dyn PackageOrMessage>) -> Rc<E>,
        F: 'static + Field,
        M: 'static + Message,
        E: 'static + Enum,
    {
        let name = proto.name().to_string();
        Rc::new_cyclic(|weak_message| MessageImpl {
            cache1: Default::default(),
            cache2: Default::default(),
            name,
            input_file: Weak::clone(&input_file),
            parent,
            fields: proto
                .field()
                .into_iter()
                .filter(|f| !f.has_oneof_index() || f.has_proto3_optional())
                .map(|f| ff(f, Weak::clone(weak_message) as Weak<dyn Message>) as Rc<dyn Field>)
                .collect(),
            messages: proto
                .nested_type()
                .into_iter()
                .map(|m| {
                    fm(
                        m,
                        Weak::clone(&input_file),
                        Weak::clone(weak_message) as Weak<dyn PackageOrMessage>,
                    ) as Rc<dyn Message>
                })
                .collect(),
            enums: proto
                .enum_type()
                .into_iter()
                .map(|e| {
                    fe(
                        e,
                        Weak::clone(&input_file),
                        Weak::clone(weak_message) as Weak<dyn PackageOrMessage>,
                    ) as Rc<dyn Enum>
                })
                .collect(),
        })
    }
}

impl PackageOrMessage for MessageImpl {
    fn cache(&self) -> &AnonymousCache {
        &self.cache1
    }
    fn name(&self) -> Result<&str> {
        Ok(&self.name)
    }
    fn messages(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn Message>>>> {
        Ok(Box::new(self.messages.iter().cloned()))
    }
    fn enums(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn Enum>>>> {
        Ok(Box::new(self.enums.iter().cloned()))
    }
    fn subpackages(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn Package>>>> {
        Ok(Box::new(iter::empty()))
    }
    fn root_package(&self) -> Result<Rc<RootPackage>> {
        self.parent.try_upgrade()?.root_package()
    }
    fn parent(&self) -> Result<Option<Rc<dyn PackageOrMessage>>> {
        Ok(Some(self.parent.try_upgrade()?))
    }
}

impl Message for MessageImpl {
    fn cache(&self) -> &AnonymousCache {
        &self.cache2
    }
    fn as_dyn_rc(self: Rc<Self>) -> Rc<dyn Message> {
        self
    }
    fn input_file(&self) -> Result<Rc<dyn InputFile>> {
        Ok(self.input_file.try_upgrade()?)
    }
    fn parent(&self) -> Result<Rc<dyn PackageOrMessage>> {
        Ok(self.parent.try_upgrade()?)
    }
    fn fields(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn Field>>>> {
        Ok(Box::new(self.fields.iter().cloned()))
    }
}
