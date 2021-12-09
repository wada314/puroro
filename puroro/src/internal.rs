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

pub mod bool;
pub mod de;
pub mod fixed_bits;
pub mod impls;
pub mod se;
pub mod types;
pub mod utils;
pub mod variant;

pub use impls::bumpalo::NoAllocBox as NoAllocBumpBox;
pub use impls::bumpalo::NoAllocString as NoAllocBumpString;
pub use impls::bumpalo::NoAllocVec as NoAllocBumpVec;
pub use impls::bumpalo::RefMutString as RefMutBumpString;
pub use impls::bumpalo::RefMutVec as RefMutBumpVec;
pub use impls::bumpalo::{AddBumpVecView, BumpDefault};

use ::bitvec::order::BitOrder;
use ::bitvec::slice::BitSlice;
use ::bitvec::store::BitStore;
use ::std::ops::{Deref, DerefMut};

pub fn get_bitvec_bit<O, T>(slice: &BitSlice<O, T>, index: usize) -> bool
where
    O: BitOrder,
    T: BitStore,
{
    *slice.get(index).expect("bitvec index out of bound.")
}

pub trait IsDefault {
    fn is_default(&self) -> bool;
}
impl<T: Default + PartialEq> IsDefault for T {
    fn is_default(&self) -> bool {
        *self == Default::default()
    }
}
impl IsDefault for NoAllocBumpString {
    fn is_default(&self) -> bool {
        self.is_empty()
    }
}
impl<T> IsDefault for NoAllocBumpVec<T> {
    fn is_default(&self) -> bool {
        self.is_empty()
    }
}

pub struct Bare<T>(T);
impl<T> Bare<T> {
    pub fn new(val: T) -> Self {
        Self(val)
    }
    pub fn inner(self) -> T {
        self.0
    }
}
impl<T> Deref for Bare<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for Bare<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T> From<T> for Bare<T> {
    fn from(from: T) -> Self {
        Self(from)
    }
}
impl<T: Default> Default for Bare<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}
impl<T: PartialEq> PartialEq for Bare<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
