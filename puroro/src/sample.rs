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

//////////////////////////////////////////////////////////////

// assume a proto like this as input:
// message Person {
//     optional string name = 1;
//     optional uint32 age = 2;
//     optional Person partner = 4;
//     repeated string nicknames = 5;
//     repeated uint32 scores = 6;
//     repeated Person children = 3;
// }

use crate::desc::*;
use crate::message::Message;
use crate::{ErrorKind, Result};

pub struct Person<M>(M);
impl<M: Message> Person<M> {
    pub fn age(&self) -> u32 {
        self.0.get_uint32(&PersonAgeFD).unwrap()
    }
}

static FD: FileDescriptor = FileDescriptor { messages: &[] };
static PersonMD: MessageDescriptor = MessageDescriptor {
    parent: &FD,
    name: "Person",
    fields: &[&PersonAgeFD],
};
static PersonAgeFD: FieldDescriptor = FieldDescriptor {
    parent: &PersonMD,
    name: "age",
    number: 2,
    r#type: FieldType::UInt32,
    label: FieldLabel::Optional,
};

#[derive(Default)]
pub struct PersonMessageImpl {
    name: String,
    age: u32,
    partner: Option<Box<PersonMessageImpl>>,
}
impl Message for PersonMessageImpl {
    fn has_field(&self, fd: &FieldDescriptor) -> Result<bool> {
        match fd.number {
            1 | 2 | 4 => Ok(true),
            _ => Err(ErrorKind::ReflectionError)?,
        }
    }
    fn get_uint32(&self, fd: &FieldDescriptor) -> Result<u32> {
        match fd.number {
            2 => Ok(self.age),
            _ => Err(ErrorKind::ReflectionError)?,
        }
    }
    fn get_string(&self, fd: &FieldDescriptor) -> Result<&str> {
        match fd.number {
            1 => Ok(&self.name),
            _ => Err(ErrorKind::ReflectionError)?,
        }
    }
    fn get_message(&self, fd: &FieldDescriptor) -> Result<&dyn Message> {
        match fd.number {
            4 => Ok(self.partner.as_deref().map(|m| m as &dyn Message).unwrap()),
            _ => Err(ErrorKind::ReflectionError)?,
        }
    }
}

////////////////////////////////////////////

#[inline(never)]
pub fn hoge<M: Message>(person: &Person<M>) -> u32 {
    person.age() + 10
}

#[test]
fn testhoge() {
    let person = Person(PersonMessageImpl { age: 10 });
    assert_eq!(20, hoge(&person));
}
