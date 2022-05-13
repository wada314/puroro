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

use crate::tags;
use ::metako::*;
use ::typenum;

pub trait MessageDescriptor {
    type Fields;
    type Syntax: tags::ProtoSyntaxTag;
}

pub trait FieldDescriptor {
    type Number: typenum::ToInt<i32> + Number;
    type Label: tags::FieldLabelTag;
    type Type: tags::FieldTypeTag;
    type HasOneofIndex: If;
    type OneofIndex: Number;
    type IsProto3Optional: If;
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

pub trait GetFieldExt<N> {
    type GetField: FieldDescriptor;
}
impl<N, MD> GetFieldExt<N> for MD
where
    MD: MessageDescriptor,
    list::Find<IsFdNumberEqualTo<N>>: Func<MD::Fields>,
    <list::Find<IsFdNumberEqualTo<N>> as Func<MD::Fields>>::Type: FieldDescriptor,
{
    type GetField = <list::Find<IsFdNumberEqualTo<N>> as Func<MD::Fields>>::Type;
}

pub trait GetFieldListAsMdFdExt {
    type GetFieldListAsMdFd;
}
impl<MD: MessageDescriptor> GetFieldListAsMdFdExt for MD
where
    GetFieldListAsMdFd: Func<MD>,
{
    type GetFieldListAsMdFd = <GetFieldListAsMdFd as Func<MD>>::Type;
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
