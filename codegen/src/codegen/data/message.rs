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
    DataTypeBase, Enum, Field, FieldOrOneof, InputFile, Oneof, Package, PackageOrMessage,
    PackageOrMessageCase, MESSAGE_FIELD_NUMBER_IN_FILE_DESCRIPTOR,
    MESSAGE_FIELD_NUMBER_IN_MESSAGE_DESCRIPTOR,
};
use crate::Result;
use ::puroro_protobuf_compiled::google::protobuf::DescriptorProto;
use ::std::fmt::Debug;
use ::std::iter;
use ::std::rc::{Rc, Weak};

#[derive(Debug)]
pub(crate) struct Message {
    cache: AnonymousCache,
    name: String,
    fields: Vec<Rc<Field>>,
    messages: Vec<Rc<Message>>,
    enums: Vec<Rc<Enum>>,
    oneofs: Vec<Rc<Oneof>>,
    input_file: Weak<InputFile>,
    parent: Weak<dyn PackageOrMessage>,
    index_in_parent: usize,
}

impl Message {
    pub(crate) fn new(
        proto: &DescriptorProto,
        input_file: Weak<InputFile>,
        parent: Weak<dyn PackageOrMessage>,
        index_in_parent: usize,
    ) -> Rc<Self> {
        let name = proto.name().to_string();
        Rc::new_cyclic(|weak_message| {
            let fields = proto
                .field()
                .into_iter()
                .filter(|f| !f.has_oneof_index() || f.has_proto3_optional())
                .map(|f| Field::new(f, Weak::clone(weak_message) as Weak<Message>) as Rc<Field>)
                .collect();
            let messages = proto
                .nested_type()
                .into_iter()
                .enumerate()
                .map(|(i, m)| {
                    Message::new(
                        m,
                        Weak::clone(&input_file),
                        Weak::clone(weak_message) as Weak<dyn PackageOrMessage>,
                        i,
                    )
                })
                .collect();
            let enums = proto
                .enum_type()
                .into_iter()
                .enumerate()
                .map(|(i, e)| {
                    Enum::new(
                        e,
                        Weak::clone(&input_file),
                        Weak::clone(weak_message) as Weak<dyn PackageOrMessage>,
                        i,
                    )
                })
                .collect();
            let oneof_num = proto
                .field()
                .iter()
                .filter_map(|f| match (f.oneof_index_opt(), f.proto3_optional()) {
                    (Some(i), false) => Some(i as usize),
                    _ => None,
                })
                .max()
                .map_or(0, |i| i + 1);
            let oneofs = (0..oneof_num)
                .into_iter()
                .map(|i| {
                    Oneof::new(
                        proto,
                        &proto.oneof_decl()[i],
                        i,
                        Weak::clone(weak_message) as Weak<Message>,
                    )
                })
                .collect();
            Message {
                cache: Default::default(),
                name,
                input_file: Weak::clone(&input_file),
                parent,
                index_in_parent,
                fields,
                messages,
                enums,
                oneofs,
            }
        })
    }
}

impl DataTypeBase for Message {
    fn cache(&self) -> &AnonymousCache {
        &self.cache
    }
    fn name(&self) -> Result<&str> {
        Ok(&self.name)
    }
}

impl PackageOrMessage for Message {
    fn either(&self) -> PackageOrMessageCase<&Package, &Message> {
        PackageOrMessageCase::Message(self)
    }

    fn messages(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<Message>>>> {
        Ok(Box::new(self.messages.iter().cloned()))
    }
    fn enums(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<Enum>>>> {
        Ok(Box::new(self.enums.iter().cloned()))
    }
    fn oneofs(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<Oneof>>>> {
        Ok(Box::new(self.oneofs.iter().cloned()))
    }
    fn subpackages(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<Package>>>> {
        Ok(Box::new(iter::empty()))
    }
    fn root_package(&self) -> Result<Rc<Package>> {
        self.parent.try_upgrade()?.root_package()
    }
    fn parent(&self) -> Result<Option<Rc<dyn PackageOrMessage>>> {
        Ok(Some(self.parent.try_upgrade()?))
    }
}

impl Message {
    pub(crate) fn input_file(&self) -> Result<Rc<InputFile>> {
        Ok(self.input_file.try_upgrade()?)
    }
    pub(crate) fn parent(&self) -> Result<Rc<dyn PackageOrMessage>> {
        Ok(self.parent.try_upgrade()?)
    }
    pub(crate) fn fields(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<Field>>>> {
        Ok(Box::new(self.fields.iter().cloned()))
    }
    pub(crate) fn fields_or_oneofs(
        &self,
    ) -> Result<Box<dyn '_ + Iterator<Item = Rc<dyn FieldOrOneof>>>> {
        let fields = self
            .fields
            .iter()
            .map(|f| f.clone() as Rc<dyn FieldOrOneof>);
        let oneofs = self
            .oneofs
            .iter()
            .map(|o| o.clone() as Rc<dyn FieldOrOneof>);
        Ok(Box::new(fields.chain(oneofs)))
    }
    pub(crate) fn location_path(&self) -> Result<Box<dyn Iterator<Item = i32>>> {
        Ok(match self.parent()?.either() {
            PackageOrMessageCase::Package(_) => {
                // This message is a direct item under the input file.
                let this_path = [
                    MESSAGE_FIELD_NUMBER_IN_FILE_DESCRIPTOR,
                    self.index_in_parent.try_into()?,
                ];
                Box::new(this_path.into_iter())
            }
            PackageOrMessageCase::Message(m) => {
                let parent_path = m.location_path()?;
                let this_path = [
                    MESSAGE_FIELD_NUMBER_IN_MESSAGE_DESCRIPTOR,
                    self.index_in_parent.try_into()?,
                ];
                Box::new(parent_path.chain(this_path.into_iter()))
            }
        })
    }
    pub(crate) fn should_generate_module_file(&self) -> Result<bool> {
        let has_submessages = self.messages()?.next().is_some();
        let has_subenums = self.enums()?.next().is_some();
        let has_oneofs = self.oneofs()?.next().is_some();
        Ok(has_submessages || has_subenums || has_oneofs)
    }
}
