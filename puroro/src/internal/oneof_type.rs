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

use crate::internal::bitvec::BitSlice;
use crate::internal::ser::{FieldData, ScopedIter};
use crate::Result;
use ::std::io::{Result as IoResult, Write};

pub trait OneofUnion {
    type Case: OneofCase;
    type CaseRef<'a>
    where
        Self: 'a;
    fn case_ref<B: BitSlice>(&self, bits: &B) -> Option<Self::CaseRef<'_>>;
    fn clear<B: BitSlice>(&mut self, bits: &mut B);
    fn clone<B: BitSlice>(&self, bits: &B) -> Self;

    fn deser_from_field_data<'a, I: Iterator<Item = IoResult<u8>>, B: BitSlice>(
        &mut self,
        bitvec: &mut B,
        field_data: FieldData<ScopedIter<'a, I>>,
        case: Self::Case,
    ) -> Result<()>;

    fn ser_to_write<W: Write, B: BitSlice>(&self, bitvec: &B, out: &mut W) -> Result<()>;
}

pub trait OneofCase: Sized {
    const BITFIELD_BEGIN: usize;
    const BITFIELD_END: usize;
    // Intentionall not using ::std::convert::From trait.
    // This conversion is puroro internal only and should not be exposed to the
    // library user.
    fn from_u32(x: u32) -> Option<Self>;
    fn from_bitslice<B: BitSlice>(b: &B) -> Option<Self> {
        Self::from_u32(b.get_range(Self::BITFIELD_BEGIN..Self::BITFIELD_END))
    }
    fn into_u32(self) -> u32;
}
