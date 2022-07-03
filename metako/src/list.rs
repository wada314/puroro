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

type Sample = ((i8, (i16, ())), ((u8, (u16, ())), ()));
pub struct Transpose;
impl Func<((), ())> for Transpose {
    type Type = ();
}

// ((A, ()), ()) => ((A, ()), ())
//
// ( (A, (B, ())), () )
// => ( (A, ()), ((B, ()), ()))
impl<Caar, Cadr, CadrTr> Func<((Caar, Cadr), ())> for Transpose
where
    Transpose: Func<(Cadr, ()), Type = CadrTr>,
{
    type Type = ((Caar, ()), CadrTr);
}
impl<Caar, Cdr, CdrTr> Func<((Caar, ()), Cdr)> for Transpose
where
    Transpose: Func<Cdr, Type = CdrTr>,
{
    type Type = ((Caar, CdrTr), ());
}

fn hoge() {
    type T1 = make_list![i8, i16, i32];
    type T2 = make_list![u8, u16, u32];
    type T3 = make_list![f32, f64, bool];
    type TT = make_list![T1, T2, T3];
    type T31 = make_list![make_list![i8], make_list![i16], make_list![i32]];
    type T13 = <Transpose as Func<T31>>::Type;
    let mut x: T31;
    x = 0;
}
