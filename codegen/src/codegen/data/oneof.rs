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
    DataTypeBase, Field, FieldOrOneof, FieldOrOneofCase, Message, OneofField,
    ONEOF_FIELD_NUMBER_IN_MESSAGE_DESCRIPTOR,
};
use crate::Result;
use ::puroro_protobuf_compiled::google::protobuf::{DescriptorProto, OneofDescriptorProto};
use ::std::fmt::Debug;
use ::std::rc::{Rc, Weak};

#[derive(Debug)]
pub(crate) struct Oneof {
    cache: AnonymousCache,
    message: Weak<Message>,
    oneof_index: usize,
    name: String,
    fields: Vec<Rc<OneofField>>,
}

impl Oneof {
    pub(crate) fn new(
        message_proto: &DescriptorProto,
        oneof_proto: &OneofDescriptorProto,
        oneof_index: usize,
        message: Weak<Message>,
    ) -> Rc<Oneof> {
        Rc::new_cyclic(|weak| {
            let fields = message_proto
                .field()
                .iter()
                .enumerate()
                .filter(|(_, f)| f.oneof_index() as usize == oneof_index)
                .map(|(i, f)| OneofField::new(f, Weak::clone(weak), i))
                .collect::<Vec<_>>();
            Oneof {
                cache: Default::default(),
                message,
                oneof_index,
                name: oneof_proto.name().to_string(),
                fields,
            }
        })
    }
}

impl DataTypeBase for Oneof {
    fn cache(&self) -> &AnonymousCache {
        &self.cache
    }
    fn name(&self) -> Result<&str> {
        Ok(&self.name)
    }
}

impl FieldOrOneof for Oneof {
    fn either(&self) -> FieldOrOneofCase<&Field, &Oneof> {
        FieldOrOneofCase::Oneof(self)
    }
}

impl Oneof {
    pub(crate) fn fields(&self) -> Result<Box<dyn '_ + Iterator<Item = Rc<OneofField>>>> {
        Ok(Box::new(self.fields.iter().cloned()))
    }
    pub(crate) fn message(&self) -> Result<Rc<Message>> {
        Ok(self.message.try_upgrade()?)
    }
    pub(crate) fn location_path(&self) -> Result<impl Iterator<Item = i32>> {
        let this_path = [
            ONEOF_FIELD_NUMBER_IN_MESSAGE_DESCRIPTOR,
            self.oneof_index.try_into()?,
        ];
        Ok(self
            .message()?
            .location_path()?
            .chain(this_path.into_iter()))
    }
}
