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

use crate::{Func, If, Pred};
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

pub trait ListBase {
    type Car;
    type Cdr: List;
}
impl<T, U: List> ListBase for (T, U) {
    type Car = T;
    type Cdr = U;
}
pub trait List {
    type Car;
    type Cdr: List;
}
impl List for () {
    type Car = ();
    type Cdr = ();
}
impl<L: ListBase> List for L {
    type Car = L::Car;
    type Cdr = L::Cdr;
}

pub struct FindOrDefault<P, D>(PhantomData<(P, D)>);
impl<P, D> Func<()> for FindOrDefault<P, D> {
    type Type = D;
}
impl<T, U, P, D> Func<(T, U)> for FindOrDefault<P, D>
where
    P: Pred<T>,
    FindOrDefault<P, D>: Func<U>,
{
    type Type = <P::Type as If>::Then<T, <FindOrDefault<P, D> as Func<U>>::Type>;
}
pub struct Find<P>(PhantomData<P>);
impl<P, L> Func<L> for Find<P>
where
    FindOrDefault<P, ()>: Func<L>,
{
    type Type = <FindOrDefault<P, ()> as Func<L>>::Type;
}

pub struct MapFunctor<F>(PhantomData<F>);
impl<F> Func<()> for MapFunctor<F> {
    type Type = ();
}
impl<T, U, F> Func<(T, U)> for MapFunctor<F>
where
    F: Func<T>,
    MapFunctor<F>: Func<U>,
{
    type Type = (<F as Func<T>>::Type, <MapFunctor<F> as Func<U>>::Type);
}

pub trait Map<F> {
    type Type;
}
impl<F> Map<F> for () {
    type Type = ();
}
impl<T, U, F> Map<F> for (T, U)
where
    F: Func<T>,
    U: Map<F>,
{
    type Type = (<F as Func<T>>::Type, <U as Map<F>>::Type);
}
