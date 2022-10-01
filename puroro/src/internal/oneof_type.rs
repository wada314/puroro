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

use crate::bitvec::BitSlice;

pub trait OneofUnion {
    type CaseRef<'a>: OneofCaseRef<'a, Union = Self>
    where
        Self: 'a;
    fn case_ref<B: BitSlice>(&self, bits: &B) -> Option<Self::CaseRef<'_>>;
    fn clear<B: BitSlice>(&mut self, bits: &mut B);
    fn clone<B: BitSlice>(&self, bits: &B) -> Self;
}

pub trait OneofCase: Sized {
    const BITFIELD_BEGIN: usize;
    const BITFIELD_END: usize;
    fn from_u32(x: u32) -> Option<Self>;
    fn from_bitslice<B: BitSlice>(b: &B) -> Option<Self> {
        Self::from_u32(b.get_range(Self::BITFIELD_BEGIN..Self::BITFIELD_END))
    }
}

pub trait OneofCaseRef<'a>: Sized {
    type Case: OneofCase;
    type Union;
    fn from_union_and_case(u: &'a Self::Union, case: Self::Case) -> Self;
}
