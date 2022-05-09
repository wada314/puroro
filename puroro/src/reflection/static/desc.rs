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
