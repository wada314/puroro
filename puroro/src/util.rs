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

pub trait Pred<T> {
    type Type: If;
}

pub trait Find<P> {
    type Type;
}
impl<P: Pred<()>> Find<P> for () {
    type Type = ();
}
impl<T, U, P: Pred<T>> Find<P> for (T, U)
where
    U: Find<P>,
{
    type Type = <P::Type as If>::Type<T, <U as Find<P>>::Type>;
}
