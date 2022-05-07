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

use crate::tags;
use ::metako::*;
use ::typenum;

pub trait MessageDescriptor {
    type Fields;
}

pub trait FieldDescriptor {
    type Number: typenum::ToInt<i32> + Number;
    type FieldType: tags::FieldTypeTag;
}

struct IsFdNumberEqualTo<N>(::std::marker::PhantomData<N>);
impl<N, T> Func<T> for IsFdNumberEqualTo<N>
where
    T: FieldDescriptor,
    T::Number: Number,
    N: Number,
{
    type Type = N::Eq<T::Number>;
}

trait MdGetFieldExt<N> {
    type GetField: FieldDescriptor;
}
impl<N, MD> MdGetFieldExt<N> for MD
where
    MD: MessageDescriptor,
    list::Find<IsFdNumberEqualTo<N>>: Func<MD::Fields>,
    <list::Find<IsFdNumberEqualTo<N>> as Func<MD::Fields>>::Type: FieldDescriptor,
{
    type GetField = <list::Find<IsFdNumberEqualTo<N>> as Func<MD::Fields>>::Type;
}

pub struct GetSupplementalDescriptor;
impl<T: tags::FieldTypeTag> Func<T> for GetSupplementalDescriptor {
    type Type = T::MaybeSupplementalDescriptor;
}

pub struct MdIntoOptBoxOwnedFieldList;
impl<MD> Func<MD> for MdIntoOptBoxOwnedFieldList
where
    MD: MessageDescriptor,
    MdIntoOwnedFieldList: Func<MD>,
{
    type Type = Option<Box<<MdIntoOwnedFieldList as Func<MD>>::Type>>;
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
    type Type =
        <<TypeTagIntoOwnedTypeGen as Func<T>>::Type as Func<T::MaybeSupplementalDescriptor>>::Type;
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
