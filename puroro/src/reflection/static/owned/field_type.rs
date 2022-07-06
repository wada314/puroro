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

use super::OwnedField;
use crate::Result;
use ::std::ops::Index;

pub struct U32ScalarOwnedField<const BITFIELD_START_INDEX: usize>(u32);

impl<const BITFIELD_START_INDEX: usize> OwnedField for U32ScalarOwnedField<BITFIELD_START_INDEX> {
    fn has_field<B: Index<usize, Output = bool>>(&self, _bitfield: &B) -> Result<bool> {
        Ok(self.0 != Default::default())
    }

    fn get_uint32<B: Index<usize, Output = bool>>(&self, _bitfield: &B) -> Result<u32> {
        Ok(self.0)
    }

    const BITFIELD_START_INDEX: usize = BITFIELD_START_INDEX;

    const BITFIELD_NEXT_INDEX: usize = BITFIELD_START_INDEX + 0;
}

pub struct U32OptionalOwnedField<const BITFIELD_START_INDEX: usize>(u32);

impl<const BITFIELD_START_INDEX: usize> OwnedField for U32OptionalOwnedField<BITFIELD_START_INDEX> {
    fn has_field<B: Index<usize, Output = bool>>(&self, bitfield: &B) -> Result<bool> {
        Ok(bitfield[Self::BITFIELD_START_INDEX])
    }

    fn get_uint32<B: Index<usize, Output = bool>>(&self, _bitfield: &B) -> Result<u32> {
        Ok(self.0)
    }

    const BITFIELD_START_INDEX: usize = BITFIELD_START_INDEX;

    const BITFIELD_NEXT_INDEX: usize = BITFIELD_START_INDEX + 0;
}
