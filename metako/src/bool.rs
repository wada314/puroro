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

use crate::func::Functor;
use crate::func::Pred;
pub use ::typenum::{B0, B1};

pub trait If {
    type Then<T, F>;
    type Not: If;
    type And<T: If>: If;
    type Or<T: If>: If;
}
impl If for B0 {
    type Then<T, F> = F;
    type Not = B1;
    type And<T: If> = B0;
    type Or<T: If> = T;
}
impl If for B1 {
    type Then<T, F> = T;
    type Not = B0;
    type And<T: If> = T;
    type Or<T: If> = B1;
}

pub trait AnyOf {
    type Type: If;
}
impl AnyOf for () {
    type Type = B0;
}
impl<T, U> AnyOf for (T, U)
where
    T: If,
    U: AnyOf,
{
    type Type = T::Or<<U as AnyOf>::Type>;
}
pub struct AnyOfFunctor;
impl Functor<()> for AnyOfFunctor {
    type Type = B0;
}
impl<T, U> Functor<(T, U)> for AnyOfFunctor
where
    T: If,
    AnyOfFunctor: Pred<U>,
{
    type Type = T::Or<<AnyOfFunctor as Pred<U>>::Type>;
}

pub trait AllOf {
    type Type: If;
}
impl AllOf for () {
    type Type = B1;
}
impl<T, U> AllOf for (T, U)
where
    T: If,
    U: AllOf,
{
    type Type = T::And<<U as AllOf>::Type>;
}
pub struct AllOfFunctor;
impl Functor<()> for AllOfFunctor {
    type Type = B1;
}
impl<T, U> Functor<(T, U)> for AllOfFunctor
where
    T: If,
    AllOfFunctor: Pred<U>,
{
    type Type = T::And<<AllOfFunctor as Pred<U>>::Type>;
}

pub trait Switch<PredAndValueList> {
    type Type;
}
impl<X> Switch<()> for X {
    type Type = ();
}
impl<P, T, U, X> Switch<((P, T), U)> for X
where
    P: Pred<X>,
    X: Switch<U>,
{
    type Type = <<P as Pred<X>>::Type as If>::Then<T, <X as Switch<U>>::Type>;
}

pub struct SwitchFunctor;
impl<X> Functor<(X, ())> for SwitchFunctor {
    type Type = ();
}
impl<X, P: Pred<X>, T, U> Functor<(X, ((P, T), U))> for SwitchFunctor
where
    SwitchFunctor: Functor<(X, U)>,
{
    type Type = <<P as Pred<X>>::Type as If>::Then<T, <SwitchFunctor as Functor<(X, U)>>::Type>;
}
