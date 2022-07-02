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

use std::marker::PhantomData;

use super::owned::MdFdIntoOwnedTypeFunctor;
use crate::tags;
use ::metako::*;
use ::typenum;
use ::typenum::U0;

pub trait MessageDescriptor: Sized {
    type Fields: GetFieldListAsMdFd<Self> + GetOwnedFieldList<Self>;
    type Syntax: tags::ProtoSyntaxTag;
}
impl MessageDescriptor for () {
    type Fields = ();
    type Syntax = ();
}
pub trait MessageDescriptorExt {
    type Fields;
    type Syntax: tags::ProtoSyntaxTag;
    type GetOwnedFieldList;
    type GetFieldByNumber;
}
// Implementation note: Do not introduce any additional bounds except
// `MD: MessageDescriptor`!
impl<MD: MessageDescriptor> MessageDescriptorExt for MD {
    type Fields = MD::Fields;
    type Syntax = MD::Syntax;
    type GetOwnedFieldList = <MD::Fields as GetOwnedFieldList<MD>>::Type;
    type GetFieldByNumber = <GetGetFieldByNumber as Functor<MD>>::Type;
}

pub trait FieldDescriptor {
    type Number: typenum::ToInt<i32> + Number;
    type Label: tags::FieldLabelTag;
    type Type: tags::FieldTypeTag;
    type HasOneofIndex: If;
    type OneofIndex: Number;
    type IsProto3Optional: If;
}
impl FieldDescriptor for () {
    type Number = U0;
    type Label = ();
    type Type = ();
    type HasOneofIndex = B0;
    type OneofIndex = U0;
    type IsProto3Optional = B0;
}
pub trait FieldDescriptorExt {
    type Number: typenum::ToInt<i32> + Number;
    type Label: tags::FieldLabelTag;
    type Type: tags::FieldTypeTag;
    type HasOneofIndex: If;
    type OneofIndex: Number;
    type IsProto3Optional: If;
    type MaybeFieldMessageDescriptor: MessageDescriptorExt;
}
impl<FD: FieldDescriptor> FieldDescriptorExt for FD {
    type Number = FD::Number;
    type Label = FD::Label;
    type Type = FD::Type;
    type HasOneofIndex = FD::HasOneofIndex;
    type OneofIndex = FD::OneofIndex;
    type IsProto3Optional = FD::IsProto3Optional;
    type MaybeFieldMessageDescriptor = <FD::Type as tags::FieldTypeTag>::MessageDescriptor;
}

pub struct IsFdNumberEqualTo<N>(::std::marker::PhantomData<N>);
impl<N, T> Functor<T> for IsFdNumberEqualTo<N>
where
    T: FieldDescriptor,
    T::Number: Number,
    N: Number,
{
    type Type = N::Eq<T::Number>;
}

pub struct GetGetFieldByNumber;
impl<MD: MessageDescriptor> Functor<MD> for GetGetFieldByNumber {
    type Type = GetFieldByNumber<MD>;
}

pub struct GetFieldByNumber<MD>(PhantomData<MD>);
impl<MD: MessageDescriptor, N: Number, FD> Functor<N> for GetFieldByNumber<MD>
where
    list::Find<IsFdNumberEqualTo<N>>: Functor<MD::Fields, Type = FD>,
    FD: FieldDescriptor,
{
    type Type = FD;
}

pub struct GetFieldListAsMdFdFunctor;
impl<MD: MessageDescriptor> Functor<MD> for GetFieldListAsMdFdFunctor
where
    list::MapFunctor<GetFieldListAsMdFdHelper<MD>>: Functor<MD::Fields>,
{
    type Type = <list::MapFunctor<GetFieldListAsMdFdHelper<MD>> as Functor<MD::Fields>>::Type;
}
pub trait GetFieldListAsMdFd<MD> {
    type Type;
}
impl<MD, Fields> GetFieldListAsMdFd<MD> for Fields
where
    list::MapFunctor<GetFieldListAsMdFdHelper<MD>>: Functor<Fields>,
{
    type Type = <list::MapFunctor<GetFieldListAsMdFdHelper<MD>> as Functor<Fields>>::Type;
}

pub struct GetFieldListAsMdFdHelper<MD>(PhantomData<MD>);
impl<MD, FD> Functor<FD> for GetFieldListAsMdFdHelper<MD> {
    type Type = (MD, FD);
}

pub trait GetOwnedFieldList<MD> {
    type Type;
}
impl<MD, Fields> GetOwnedFieldList<MD> for Fields
where
    Fields: GetFieldListAsMdFd<MD>,
    <Fields as GetFieldListAsMdFd<MD>>::Type: list::Map<MdFdIntoOwnedTypeFunctor>,
{
    type Type =
        <<Fields as GetFieldListAsMdFd<MD>>::Type as list::Map<MdFdIntoOwnedTypeFunctor>>::Type;
}

pub struct GetSupplementalDescriptor;
impl<T: tags::FieldTypeTag> Functor<T> for GetSupplementalDescriptor {
    type Type = T::MessageDescriptor;
}

pub trait GetFieldsMDExt {
    type GetFieldsMD: MessageDescriptor;
}
impl<FD: FieldDescriptor, FieldMD> GetFieldsMDExt for FD
where
    FD::Type: tags::FieldTypeTag<MessageDescriptor = FieldMD>,
    FieldMD: MessageDescriptor,
{
    type GetFieldsMD = FieldMD;
}
