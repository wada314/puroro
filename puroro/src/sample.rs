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

pub trait PersonTrait {
    type NameType<'a>: AsRef<str>
    where
        Self: 'a;
    fn name(&self) -> Self::NameType<'_>;
    fn name_str(&self) -> &str
    where
        for<'a> Self: PersonTrait<NameType<'a> = &'a str>,
    {
        self.name()
    }
    fn age(&self) -> u32;
}

use crate::reflection::r#static::desc::*;
use crate::reflection::r#static::owned::OwnedMessage;
use crate::reflection::r#static::Reflection;
use crate::tags;
use ::metako::make_list;
use ::typenum::{B0, U0, U1, U2, U4};

pub struct MdPerson;
pub struct FdName;
pub struct FdAge;
pub struct FdPartner;
impl MessageDescriptor for MdPerson {
    type Fields = make_list!(FdName, FdAge, FdPartner);
    type Syntax = tags::Proto3;
}
impl FieldDescriptor for FdName {
    type Number = U1;
    type Label = tags::Optional;
    type Type = tags::String;
    type HasOneofIndex = B0;
    type OneofIndex = U0;
    type IsProto3Optional = B0;
}
impl FieldDescriptor for FdAge {
    type Number = U2;
    type Label = tags::Optional;
    type Type = tags::UInt32;
    type HasOneofIndex = B0;
    type OneofIndex = U0;
    type IsProto3Optional = B0;
}
impl FieldDescriptor for FdPartner {
    type Number = U4;
    type Label = tags::Optional;
    type Type = tags::Message<MdPerson>;
    type HasOneofIndex = B0;
    type OneofIndex = U0;
    type IsProto3Optional = B0;
}

pub type PersonOwned = OwnedMessage<MdPerson>;

impl<T> PersonTrait for T
where
    T: Reflection,
{
    type NameType<'a> = T::StringFieldType<'a, FdName> where Self: 'a;
    fn name(&self) -> Self::NameType<'_> {
        self.get_string::<FdName>().unwrap()
    }
    fn age(&self) -> u32 {
        self.get_uint32::<FdAge>().unwrap()
    }
}

////////////////////////////////////////////
////
#[allow(unused)]
mod test {
    use super::*;
    use crate::reflection::r#static::desc::*;
    use ::metako::*;

    // fn test(v: <TypeTagIntoOwnedType as Func<tags::Message<MdPerson>>>::Type) {}

    fn foo() {
        let mut t = PersonOwned::default();
        let p = t.get_message::<FdPartner>();
        // let f: i32 = t.fields;
        // t.1.1.0 = Some(BoxedMessage::default());
        // test(10)
    }
}

mod test2 {
    trait MessageDescriptor {
        type FieldsType;
    }

    struct Message<MD: MessageDescriptor> {
        fields: MD::FieldsType,
    }

    struct MdPerson;
    impl MessageDescriptor for MdPerson {
        type FieldsType = Box<Message<MdPerson>>;
    }

    type Person = Message<MdPerson>;

    fn test(_p: Person) {}
}
