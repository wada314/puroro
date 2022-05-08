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
    fn name(&self) -> &str;
    fn age(&self) -> u32;
    type PartnerType<'a>: PersonTrait
    where
        Self: 'a;
    fn partner(&self) -> Self::PartnerType<'_>;
}

use crate::reflection::r#static::desc::*;
use crate::reflection::r#static::{OwnedFields, Reflection};
use crate::tags;
use ::metako::make_list;
use ::typenum::{U1, U2, U4};

pub struct MdPerson;
pub struct FdName;
pub struct FdAge;
pub struct FdPartner;
impl MessageDescriptor for MdPerson {
    type Fields = make_list!(FdName, FdAge, FdPartner);
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

type PersonFields = OwnedFields<MdPerson>;

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

mod test {
    use super::*;
    use ::metako::*;

    struct MdPerson2;
    impl MessageDescriptor for MdPerson2 {
        type Fields = make_list!(FdName, FdAge, FdPartner);
    }

    pub struct OwnedMessage<MD: MessageDescriptor>
    where
        MdIntoOwnedFieldList: Func<MD>,
    {
        pub fields: <MdIntoOwnedFieldList as Func<MD>>::Type,
    }

    pub struct MdIntoOptBoxOwnedFieldList;
    impl<MD> Func<MD> for MdIntoOptBoxOwnedFieldList
    where
        MD: MessageDescriptor,
        MdIntoOwnedFieldList: Func<MD>,
    {
        type Type = Option<Box<OwnedMessage<MD>>>;
    }

    pub struct TypeTagIntoOwnedTypeGen;
    type TypeTagIntoOwnedTypeGenMap = make_list!(
        (<tags::UInt32 as tags::FieldTypeTag>::Id, Ident<u32>),
        (<tags::String as tags::FieldTypeTag>::Id, Ident<String>),
        (
            <tags::Message<()> as tags::FieldTypeTag>::Id,
            MdIntoOptBoxOwnedFieldList
        ),
    );
    impl<T: tags::FieldTypeTag> Func<T> for TypeTagIntoOwnedTypeGen {
        type Type = <map::Get<IsNumberEqual<T::Id>> as Func<TypeTagIntoOwnedTypeGenMap>>::Type;
    }

    pub struct TypeTagIntoOwnedType;
    impl<T> Func<T> for TypeTagIntoOwnedType
    where
        T: tags::FieldTypeTag,
        TypeTagIntoOwnedTypeGen: Func<T>,
        <TypeTagIntoOwnedTypeGen as Func<T>>::Type: Func<T::MaybeSupplementalDescriptor>,
    {
        type Type = <<TypeTagIntoOwnedTypeGen as Func<T>>::Type as Func<
            T::MaybeSupplementalDescriptor,
        >>::Type;
    }

    pub struct FdIntoOwnedType;
    impl<FD> Func<FD> for FdIntoOwnedType
    where
        FD: FieldDescriptor,
        TypeTagIntoOwnedType: Func<FD::FieldType>,
    {
        type Type = <TypeTagIntoOwnedType as Func<FD::FieldType>>::Type;
    }

    pub struct MdIntoOwnedFieldList;
    impl<MD> Func<MD> for MdIntoOwnedFieldList
    where
        MD: MessageDescriptor,
        list::Map<FdIntoOwnedType>: Func<MD::Fields>,
    {
        type Type = <list::Map<FdIntoOwnedType> as Func<MD::Fields>>::Type;
    }

    // fn test(v: <TypeTagIntoOwnedType as Func<tags::Message<MdPerson>>>::Type) {}
    fn test(v: <MdIntoOwnedFieldList as Func<MdPerson2>>::Type) {}
    fn foo() {
        test(10)
    }
    struct MD;
    trait MDT {
        type Field;
    }
    impl MDT for MD {
        type Field = MD;
    }
    struct P<F: Func<MD>, MD>(Box<F::Type>);
    struct Gen;
    impl Func<MD> for Gen {
        type Type = P<Gen, <MD as MDT>::Field>;
    }
    fn hoge(p: P<Gen, MD>) {
        let P(q) = p;
    }
}
