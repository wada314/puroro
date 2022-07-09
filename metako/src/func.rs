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

use crate::Bool;
use ::std::marker::PhantomData;

pub trait Func<T> {
    type Type;
}
pub trait UsizeFunc<const X: usize> {
    const VALUE: usize;
}
pub trait Pred<T> {
    type Type: Bool;
}
impl<T, U> Pred<U> for T
where
    T: Func<U>,
    T::Type: Bool,
{
    type Type = T::Type;
}

pub struct Const<T>(PhantomData<T>);
impl<T, _U> Func<_U> for Const<T> {
    type Type = T;
}
