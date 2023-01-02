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
use ::std::ops::{Deref, DerefMut, Range};

pub type BitArray<const LEN32: usize> = ::bitvec::array::BitArray<[u32; LEN32]>;

pub trait BitSlice {
    fn get(&self, index: usize) -> bool;
    fn get_range(&self, range: Range<usize>) -> u32;
    fn set(&mut self, index: usize, value: bool);
    fn set_range(&mut self, range: Range<usize>, value: u32);
}

impl<const LEN32: usize> BitSlice for BitArray<LEN32> {
    #[inline]
    fn get(&self, index: usize) -> bool {
        *<::bitvec::slice::BitSlice<u32>>::get(self.deref(), index).unwrap()
    }

    #[inline]
    fn get_range(&self, range: Range<usize>) -> u32 {
        <::bitvec::slice::BitSlice<u32>>::get(self.deref(), range)
            .unwrap()
            .load::<u32>()
    }

    #[inline]
    fn set(&mut self, index: usize, value: bool) {
        <::bitvec::slice::BitSlice<u32>>::set(self.deref_mut(), index, value)
    }

    #[inline]
    fn set_range(&mut self, range: Range<usize>, value: u32) {
        <::bitvec::slice::BitSlice<u32>>::get_mut(self.deref_mut(), range)
            .unwrap()
            .store::<u32>(value)
    }
}
