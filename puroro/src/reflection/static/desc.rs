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
use ::typenum;
use ::typenum::{B0, B1};

pub trait MessageDescriptor {
    type Fields;
}

pub trait FieldDescriptor {
    type Number: typenum::ToInt<i32>;
    type FieldType: tags::FieldTypeTag;
}

pub trait If<T, F> {
    type Type;
}
impl<T, F> If<T, F> for B0 {
    type Type = F;
}
impl<T, F> If<T, F> for B1 {
    type Type = T;
}

trait GetFieldFromTuple<N> {
    type Type;
}
impl<N> GetFieldFromTuple<N> for () {
    type Type = ();
}
impl<T, U, N> GetFieldFromTuple<N> for (T, U)
where
    T: FieldDescriptor,
    T::Number: typenum::IsEqual<N>,
    typenum::Eq<T::Number, N>: If<T, U::Type>,
    U: GetFieldFromTuple<N>,
{
    type Type = <typenum::Eq<T::Number, N> as If<T, U::Type>>::Type;
}

trait MdGetFieldExt<N> {
    type GetField;
}
impl<N, MD> MdGetFieldExt<N> for MD
where
    MD: MessageDescriptor,
    MD::Fields: GetFieldFromTuple<N>,
{
    type GetField = <MD::Fields as GetFieldFromTuple<N>>::Type;
}
