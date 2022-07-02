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

use crate::{Functor, If, Pred};
use ::std::marker::PhantomData;

#[macro_export]
macro_rules! make_list {
    () => {
        ()
    };
    ($a:ty$(, $rest:ty)* $(,)?) => {
        ($a, make_list!($($rest),*))
    };
}

pub trait List {
    type Car;
    type Cdr: List;
}
impl List for () {
    type Car = ();
    type Cdr = ();
}
impl<T, U: List> List for (T, U) {
    type Car = T;
    type Cdr = U;
}

pub struct FindOrDefault<P, D>(PhantomData<(P, D)>);
impl<P, D> Functor<()> for FindOrDefault<P, D> {
    type Type = D;
}
impl<T, U, P, D> Functor<(T, U)> for FindOrDefault<P, D>
where
    P: Pred<T>,
    FindOrDefault<P, D>: Functor<U>,
{
    type Type = <P::Type as If>::Then<T, <FindOrDefault<P, D> as Functor<U>>::Type>;
}
pub struct Find<P>(PhantomData<P>);
impl<P, L> Functor<L> for Find<P>
where
    FindOrDefault<P, ()>: Functor<L>,
{
    type Type = <FindOrDefault<P, ()> as Functor<L>>::Type;
}

pub struct MapFunctor<F>(PhantomData<F>);
impl<F> Functor<()> for MapFunctor<F> {
    type Type = ();
}
impl<T, U, F> Functor<(T, U)> for MapFunctor<F>
where
    F: Functor<T>,
    MapFunctor<F>: Functor<U>,
{
    type Type = (<F as Functor<T>>::Type, <MapFunctor<F> as Functor<U>>::Type);
}

pub trait Map<F> {
    type Type;
}
impl<F> Map<F> for () {
    type Type = ();
}
impl<T, U, F> Map<F> for (T, U)
where
    F: Functor<T>,
    U: Map<F>,
{
    type Type = (<F as Functor<T>>::Type, <U as Map<F>>::Type);
}

pub struct Map2<L, F>(PhantomData<(L, F)>);
impl<L, F> List for Map2<L, F>
where
    L: List,
    F: Functor<L::Car>,
    Map2<L::Cdr, F>: List,
{
    type Car = <F as Functor<L::Car>>::Type;
    type Cdr = Map2<L::Cdr, F>;
}
