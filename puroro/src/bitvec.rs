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

//! A thin wrapper over the bitvec crate.

use ::bitvec::field::BitField;
use ::std::ops::{Deref, DerefMut};

pub type BitArray<const LEN32: usize> = ::bitvec::array::BitArray<[u32; LEN32]>;

pub trait BitSlice {
    fn get<const INDEX: usize>(&self) -> bool;
    fn get_range<const BEGIN: usize, const END: usize>(&self) -> u32;
    fn set<const INDEX: usize>(&mut self, value: bool);
}

impl<const LEN32: usize> BitSlice for BitArray<LEN32> {
    fn get<const INDEX: usize>(&self) -> bool {
        *<::bitvec::slice::BitSlice<u32>>::get(self.deref(), INDEX).unwrap()
    }

    fn get_range<const BEGIN: usize, const END: usize>(&self) -> u32 {
        <::bitvec::slice::BitSlice<u32>>::get(self.deref(), BEGIN..END)
            .unwrap()
            .load::<u32>()
    }

    fn set<const INDEX: usize>(&mut self, value: bool) {
        <::bitvec::slice::BitSlice<u32>>::set(self.deref_mut(), INDEX, value)
    }
}
