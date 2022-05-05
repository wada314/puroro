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

use crate::{ErrorKind, Result};

#[derive(Default)]
pub struct PersonMessageImpl {
    name: String,
    age: u32,
    partner: Option<Box<PersonMessageImpl>>,
}

pub trait PersonTrait {
    fn name(&self) -> &str;
    fn age(&self) -> u32;
    type PartnerType<'a>: PersonTrait
    where
        Self: 'a;
    fn partner(&self) -> Self::PartnerType<'_>;
}

use crate::reflection::r#static::desc::*;
use crate::reflection::r#static::Reflection;
use crate::tags;
use crate::util::{IsNumberEqual, MapGet};
use ::typenum::{U1, U2, U4};

pub struct MdPerson;
pub struct FdName;
pub struct FdAge;
pub struct FdPartner;
impl MessageDescriptor for MdPerson {
    type Fields = (FdName, (FdAge, (FdPartner, ())));
}
impl FieldDescriptor for FdName {
    type Number = U1;
    type FieldType = tags::String;
}
impl FieldDescriptor for FdAge {
    type Number = U2;
    type FieldType = tags::UInt32;
}
impl FieldDescriptor for FdPartner {
    type Number = U4;
    type FieldType = tags::Message<MdPerson>;
}

impl<T> PersonTrait for T
where
    T: Reflection,
{
    fn name(&self) -> &str {
        self.get_string::<FdName>().unwrap()
    }
    fn age(&self) -> u32 {
        self.get_uint32::<FdAge>().unwrap()
    }

    type PartnerType<'a>
    = <T as Reflection>::ChildReflection<'a, FdPartner>
    where
        Self: 'a;

    fn partner(&self) -> Self::PartnerType<'_> {
        self.get_message::<FdPartner>().unwrap()
    }
}

////////////////////////////////////////////
