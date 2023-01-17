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

//! An extention to typenum crate.

use ::std::marker::PhantomData;
pub use ::typenum::{UTerm, B0, B1};

// A sample external trait that the result type need to be bound
pub trait Message {}

pub trait MessageGen {
    type Type: Message;
}

pub trait BoolGen {
    type Type;
}
impl BoolGen for B0 {
    type Type = B0;
}
impl BoolGen for B1 {
    type Type = B1;
}

pub struct If<B, T, F>(PhantomData<(B, T, F)>);
pub struct IfGen<B, T, F>(PhantomData<(B, T, F)>);

impl<B: BoolGen, T: MessageGen, F: MessageGen> MessageGen for If<B, T, F> {
    type Type = <IfGen<B::Type, T, F> as MessageGen>::Type;
}

impl<T: MessageGen, F: MessageGen> MessageGen for IfGen<B0, T, F> {
    type Type = F::Type;
}
impl<T: MessageGen, F: MessageGen> MessageGen for IfGen<B1, T, F> {
    type Type = T::Type;
}
