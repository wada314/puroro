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

use super::owned::{MdFdIntoOwnedType, OwnedMessage};
use crate::tags;
use ::metako::*;
use ::typenum;
use ::typenum::U0;

pub trait MessageDescriptor {
    type Fields: FieldDescriptorExt;
    type Syntax: tags::ProtoSyntaxTag;
}
impl MessageDescriptor for () {
    type Fields = ();
    type Syntax = ();
}
pub trait MessageDescriptorExt {
    type Fields;
    type Syntax: tags::ProtoSyntaxTag;
    type GetFieldListAsMdFd;
    type GetOwnedFieldList;
    type GetFieldByNumber;
}
impl<MD: MessageDescriptor> MessageDescriptorExt for MD
where
// self::GetFieldListAsMdFd: Func<MD, Type = GetFieldListAsMdFd>,
// list::Map<MdFdIntoOwnedType>: Func<GetFieldListAsMdFd, Type = GetOwnedFieldList>,
// FdToField: Func<MD::Fields>,
// MD::Fields: FieldDescriptorExt,
// <MD::Fields as FieldDescriptorExt>::MaybeFieldMessageDescriptor: MessageDescriptorExt,
{
    type Fields = MD::Fields;
    type Syntax = MD::Syntax;
    type GetFieldListAsMdFd = ();
    type GetOwnedFieldList = <FdToField as Func<MD::Fields>>::Type;
    type GetFieldByNumber = <GetGetFieldByNumber as Func<MD>>::Type;
}

pub struct FdToField;
impl<FD: FieldDescriptorExt> Func<FD> for FdToField
where
    FD::MaybeFieldMessageDescriptor: MessageDescriptorExt,
{
    type Type = Option<Box<OwnedMessage<FD::MaybeFieldMessageDescriptor>>>;
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
impl<FD: FieldDescriptor> FieldDescriptorExt for FD
where
    <FD::Type as tags::FieldTypeTag>::MessageDescriptor: MessageDescriptorExt,
{
    type Number = FD::Number;
    type Label = FD::Label;
    type Type = FD::Type;
    type HasOneofIndex = FD::HasOneofIndex;
    type OneofIndex = FD::OneofIndex;
    type IsProto3Optional = FD::IsProto3Optional;
    type MaybeFieldMessageDescriptor = <FD::Type as tags::FieldTypeTag>::MessageDescriptor;
}

pub struct IsFdNumberEqualTo<N>(::std::marker::PhantomData<N>);
impl<N, T> Func<T> for IsFdNumberEqualTo<N>
where
    T: FieldDescriptor,
    T::Number: Number,
    N: Number,
{
    type Type = N::Eq<T::Number>;
}

pub struct GetGetFieldByNumber;
impl<MD: MessageDescriptor> Func<MD> for GetGetFieldByNumber {
    type Type = GetFieldByNumber<MD>;
}

pub struct GetFieldByNumber<MD>(PhantomData<MD>);
impl<MD: MessageDescriptor, N: Number, FD> Func<N> for GetFieldByNumber<MD>
where
    list::Find<IsFdNumberEqualTo<N>>: Func<MD::Fields, Type = FD>,
    FD: FieldDescriptor,
{
    type Type = FD;
}

pub struct GetFieldListAsMdFd;
impl<MD: MessageDescriptor> Func<MD> for GetFieldListAsMdFd
where
    list::Map<GetFieldListAsMdFdHelper<MD>>: Func<MD::Fields>,
{
    type Type = <list::Map<GetFieldListAsMdFdHelper<MD>> as Func<MD::Fields>>::Type;
}

pub struct GetFieldListAsMdFdHelper<MD>(PhantomData<MD>);
impl<MD, FD> Func<FD> for GetFieldListAsMdFdHelper<MD> {
    type Type = (MD, FD);
}

pub struct GetSupplementalDescriptor;
impl<T: tags::FieldTypeTag> Func<T> for GetSupplementalDescriptor {
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
