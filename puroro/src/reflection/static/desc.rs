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
use crate::tuple_list;
use crate::util::{Func, Ident, If, IsNumberEqual, ListFind, MapGet, Number, Pred};
use ::typenum;

pub trait MessageDescriptor {
    type Fields;
}

pub trait FieldDescriptor {
    type Number: typenum::ToInt<i32> + Number;
    type FieldType: tags::FieldTypeTag;
}

struct IsFdNumberEqualTo<N>(::std::marker::PhantomData<N>);
impl<N, T> Pred<T> for IsFdNumberEqualTo<N>
where
    T: FieldDescriptor,
    T::Number: Number,
    N: Number,
    N::Eq<T::Number>: If,
{
    type Type = N::Eq<T::Number>;
}

trait MdGetFieldExt<N> {
    type GetField: FieldDescriptor;
}
impl<N, MD> MdGetFieldExt<N> for MD
where
    MD: MessageDescriptor,
    MD::Fields: ListFind<IsFdNumberEqualTo<N>>,
    <MD::Fields as ListFind<IsFdNumberEqualTo<N>>>::Type: FieldDescriptor,
{
    type GetField = <MD::Fields as ListFind<IsFdNumberEqualTo<N>>>::Type;
}

pub struct GetSupplementalDescriptor;
impl<T: tags::FieldTypeTag> Func<T> for GetSupplementalDescriptor {
    type Type = T::MaybeSupplementalDescriptor;
}

pub struct TypeTagIntoOwnedTypeGen;
type TypeTagIntoOwnedTypeGenMap = tuple_list!(
    (<tags::UInt32 as tags::FieldTypeTag>::Id, Ident<u32>),
    (<tags::String as tags::FieldTypeTag>::Id, Ident<String>),
    (
        <tags::Message<()> as tags::FieldTypeTag>::Id,
        GetSupplementalDescriptor
    ),
);
impl<T: Number> Func<T> for TypeTagIntoOwnedTypeGen {
    type Type = <TypeTagIntoOwnedTypeGenMap as MapGet<IsNumberEqual<T>>>::Type;
}
