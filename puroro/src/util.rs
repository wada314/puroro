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

use ::typenum::{B0, B1};

pub trait If {
    type Type<T, F>;
}
impl If for B0 {
    type Type<T, F> = F;
}
impl If for B1 {
    type Type<T, F> = T;
}
pub trait If2: If {
    type Type2<T: If2, F: If2>: If2;
}
impl If2 for B0 {
    type Type2<T: If2, F: If2> = F;
}
impl If2 for B1 {
    type Type2<T: If2, F: If2> = T;
}

use ::typenum::{UInt, UTerm};

pub trait FieldNumber {
    type Lsb: If2;
    type Remains: FieldNumber;
    type IsUTerm: If2;
    type Eq<M: FieldNumber>: If2;
}
impl<U: FieldNumber> FieldNumber for UInt<U, B0> {
    type Lsb = B0;
    type Remains = U;
    type IsUTerm = B0;
    type Eq<M: FieldNumber> =
        <M::Lsb as If2>::Type2<B0, <U::Remains as FieldNumber>::Eq<M::Remains>>;
}
impl<U: FieldNumber> FieldNumber for UInt<U, B1> {
    type Lsb = B1;
    type Remains = U;
    type IsUTerm = B0;
    type Eq<M: FieldNumber> =
        <M::Lsb as If2>::Type2<<U::Remains as FieldNumber>::Eq<M::Remains>, B0>;
}
impl FieldNumber for UTerm {
    type Lsb = B0;
    type Remains = UTerm;
    type IsUTerm = B1;
    type Eq<M: FieldNumber> = <M::IsUTerm as If2>::Type2<B1, B0>;
}

pub trait Pred<T> {
    type Type: If;
}
pub struct IsTypeNumEqual<N>(::std::marker::PhantomData<N>);
impl<N, M> Pred<M> for IsTypeNumEqual<N>
where
    N: ::typenum::IsEqual<M>,
    typenum::Eq<N, M>: If,
{
    type Type = typenum::Eq<N, M>;
}

pub trait ListFind<P> {
    type Type;
}
impl<P> ListFind<P> for () {
    type Type = ();
}
impl<T, U, P: Pred<T>> ListFind<P> for (T, U)
where
    U: ListFind<P>,
{
    type Type = <P::Type as If>::Type<T, <U as ListFind<P>>::Type>;
}

pub trait MapGet<P> {
    type Type;
}
impl<P> MapGet<P> for () {
    type Type = ();
}
impl<K, V, U, P: Pred<K>> MapGet<P> for ((K, V), U)
where
    U: MapGet<P>,
{
    type Type = <P::Type as If>::Type<V, <U as MapGet<P>>::Type>;
}
