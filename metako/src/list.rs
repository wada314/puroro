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

use crate::Func;
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

pub struct Map<F>(PhantomData<F>);
impl<F> Func<()> for Map<F> {
    type Type = ();
}
impl<Car, Cdr, F> Func<(Car, Cdr)> for Map<F>
where
    F: Func<Car>,
    Self: Func<Cdr>,
{
    type Type = (<F as Func<Car>>::Type, <Self as Func<Cdr>>::Type);
}

pub struct Fold<B, F>(PhantomData<(B, F)>);
impl<B, F> Func<()> for Fold<B, F> {
    type Type = B;
}
impl<Car, Cdr, B, F, NewB, Result> Func<(Car, Cdr)> for Fold<B, F>
where
    F: Func<(B, Car), Type = NewB>,
    Fold<NewB, F>: Func<Cdr, Type = Result>,
{
    type Type = Result;
}

pub struct Zip;
impl Func<((), ())> for Zip {
    type Type = ();
}
impl<ACar, ACdr, BCar, BCdr> Func<((ACar, ACdr), (BCar, BCdr))> for Zip
where
    Zip: Func<(ACdr, BCdr)>,
{
    type Type = ((ACar, BCar), <Zip as Func<(ACdr, BCdr)>>::Type);
}

pub struct Transpose;
impl Func<((), ())> for Transpose {
    type Type = ();
}
impl<Caar, Cadr, CadrTr> Func<((Caar, Cadr), ())> for Transpose
where
    Transpose: Func<(Cadr, ()), Type = CadrTr>,
{
    type Type = ((Caar, ()), CadrTr);
}
impl<Car, Cdar, Cddr, CdrTr, Result> Func<(Car, (Cdar, Cddr))> for Transpose
where
    Transpose: Func<(Cdar, Cddr), Type = CdrTr>,
    Zip: Func<(Car, CdrTr), Type = Result>,
{
    type Type = Result;
}

pub struct Scan<F>(PhantomData<F>);
impl<F, State> Func<((), State)> for Scan<F> {
    type Type = ((), State);
}
impl<F, Car, Cdr, State, NextState, Mapped> Func<((Car, Cdr), State)> for Scan<F>
where
    F: Func<(State, Car), Type = (NextState, Mapped)>,
    Self: Func<(Cdr, NextState)>,
{
    type Type = (Mapped, <Self as Func<(Cdr, NextState)>>::Type); 
}
