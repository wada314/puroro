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

// pub struct Person<M>(M);
mod dynamic {
    use crate::reflection::dynamic::desc::*;
    static FILED: FileDescriptor = FileDescriptor { messages: &[] };
    static MD_PERSON: MessageDescriptor = MessageDescriptor {
        parent: &FILED,
        name: "Person",
        fields: &[&FD_NAME, &FD_AGE, &FD_PARTNER],
    };
    static FD_NAME: FieldDescriptor = FieldDescriptor {
        parent: &MD_PERSON,
        name: "name",
        number: 1,
        r#type: FieldType::String,
        label: FieldLabel::Optional,
    };
    static FD_AGE: FieldDescriptor = FieldDescriptor {
        parent: &MD_PERSON,
        name: "age",
        number: 2,
        r#type: FieldType::UInt32,
        label: FieldLabel::Optional,
    };
    static FD_PARTNER: FieldDescriptor = FieldDescriptor {
        parent: &MD_PERSON,
        name: "partner",
        number: 4,
        r#type: FieldType::Message(&MD_PERSON),
        label: FieldLabel::Optional,
    };
}

mod r#static {
    use crate::reflection::r#static::desc::*;
    use crate::reflection::r#static::Reflection;
    use crate::tags;
    use crate::ErrorKind;
    use ::typenum::{U1, U2, U3};

    use super::PersonMessageImpl;
    use super::PersonTrait;

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
        type Number = U3;
        type FieldType = tags::Message<MdPerson>;
    }

    impl Reflection for PersonMessageImpl {
        fn has_field<FD: FieldDescriptor>(&self) -> crate::Result<bool> {
            match <FD::Number as typenum::ToInt<i32>>::to_int() {
                1 | 2 => Ok(true),
                4 => Ok(self.partner.is_some()),
                _ => Err(ErrorKind::ReflectionError)?,
            }
        }

        fn get_uint32<FD: FieldDescriptor>(&self) -> crate::Result<u32> {
            todo!()
        }

        fn get_string<FD: FieldDescriptor>(&self) -> crate::Result<&str> {
            todo!()
        }

        type ChildReflection<'a, FD>
        where
            Self: 'a,
            FD: FieldDescriptor,
        = &'a PersonMessageImpl; // TBD

        fn get_message<FD: FieldDescriptor>(&self) -> crate::Result<Self::ChildReflection<'_, FD>> {
            if 4 == <FD::Number as typenum::ToInt<i32>>::to_int() {
                Ok(self.partner.as_deref().unwrap()) // TODO
            } else {
                Err(ErrorKind::ReflectionError)?
            }
        }
    }
}

impl<T> PersonTrait for T
where
    T: crate::reflection::r#static::Reflection,
{
    fn name(&self) -> &str {
        self.get_string::<r#static::FdName>().unwrap()
    }
    fn age(&self) -> u32 {
        self.get_uint32::<r#static::FdAge>().unwrap()
    }

    type PartnerType<'a>
    where
        Self: 'a,
    = <T as crate::reflection::r#static::Reflection>::ChildReflection<'a, r#static::FdPartner>;

    fn partner(&self) -> Self::PartnerType<'_> {
        self.get_message::<r#static::FdPartner>().unwrap()
    }
}

////////////////////////////////////////////
