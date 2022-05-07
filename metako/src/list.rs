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

pub struct Find<P>(PhantomData<P>);
impl<P> Func<()> for Find<P> {
    type Type = ();
}
impl<T, U, P> Func<(T, U)> for Find<P>
where
    P: Pred<T>,
    Find<P>: Func<U>,
{
    type Type = <P::Type as If>::Type<T, <Find<P> as Func<U>>::Type>;
}

pub struct Map<F>(PhantomData<F>);
impl<F> Func<()> for Map<F> {
    type Type = ();
}
impl<T, U, F> Func<(T, U)> for Map<F>
where
    F: Func<T>,
    Map<F>: Func<U>,
{
    type Type = (<F as Func<T>>::Type, <Map<F> as Func<U>>::Type);
}
