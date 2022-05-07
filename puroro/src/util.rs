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
