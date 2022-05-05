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

use ::std::marker::PhantomData;
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
use ::typenum::{UInt, UTerm};

// We need this trait so that we can compare the
// type equality with ANY M:Number type.
// typenum's Cmp does not fulfill this condition...
pub trait Number {
    type Lsb: If;
    type Remains: Number;
    type IsUTerm: If;
    type Eq<M: Number>;
}
impl<U: Number> Number for UInt<U, B0> {
    type Lsb = B0;
    type Remains = U;
    type IsUTerm = B0;
    type Eq<M: Number> = <M::Lsb as If>::Type<B0, <U::Remains as Number>::Eq<M::Remains>>;
}
impl<U: Number> Number for UInt<U, B1> {
    type Lsb = B1;
    type Remains = U;
    type IsUTerm = B0;
    type Eq<M: Number> = <M::Lsb as If>::Type<<U::Remains as Number>::Eq<M::Remains>, B0>;
}
impl Number for UTerm {
    type Lsb = B0;
    type Remains = UTerm;
    type IsUTerm = B1;
    type Eq<M: Number> = <M::IsUTerm as If>::Type<B1, B0>;
}

pub trait Pred<T> {
    type Type: If;
}
pub struct IsNumberEqual<N>(PhantomData<N>);
impl<N, M> Pred<M> for IsNumberEqual<N>
where
    M: Number,
    N: Number,
    M::Eq<N>: If,
{
    type Type = M::Eq<N>;
}

pub trait Func<T> {
    type Type;
}
pub struct Ident<T>(PhantomData<T>);
impl<T, _U> Func<_U> for Ident<T> {
    type Type = T;
}

pub struct FdIntoOwnedType<FD>(PhantomData<FD>);
impl<FD> Func<FD> for FdIntoOwnedType<FD> {
    type Type = ();
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
