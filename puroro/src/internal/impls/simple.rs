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

#![doc = include_str!("simple.md")]

pub mod de;
mod getter;
mod getter_opt;
use super::super::SharedObjects;

pub struct SimpleFields;

#[derive(Default, Clone, Debug)]
pub struct SimpleShared<const BITFIELD_U32_LEN: usize> {
    bitfield: crate::bitvec::array::BitArray<crate::bitvec::order::Lsb0, [u32; BITFIELD_U32_LEN]>,
}

impl<const BITFIELD_U32_LEN: usize> SharedObjects for SimpleShared<BITFIELD_U32_LEN> {
    type AllocatorType = ();
    type BitfieldType =
        crate::bitvec::array::BitArray<crate::bitvec::order::Lsb0, [u32; BITFIELD_U32_LEN]>;
    fn alloc(&self) -> &Self::AllocatorType {
        &()
    }
    fn bitfield(&self) -> &Self::BitfieldType {
        &self.bitfield
    }
    fn bitfield_mut(&mut self) -> &mut Self::BitfieldType {
        &mut self.bitfield
    }
}
