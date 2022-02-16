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

#![doc = include_str!("owned.md")]

mod getter;
mod getter_mut;
mod getter_opt;
mod getter_slice;
use crate::internal::SharedAllocator;
use crate::DefaultIn;

use super::super::SharedBitfield;

pub struct OwnedFields;

#[derive(Default, Clone, Debug)]
pub struct OwnedShared<const BITFIELD_U32_LEN: usize> {
    bitfield: crate::bitvec::array::BitArray<crate::bitvec::order::Lsb0, [u32; BITFIELD_U32_LEN]>,
}

impl<const BITFIELD_U32_LEN: usize> SharedBitfield for OwnedShared<BITFIELD_U32_LEN> {
    type BitfieldType =
        crate::bitvec::array::BitArray<crate::bitvec::order::Lsb0, [u32; BITFIELD_U32_LEN]>;
    fn bitfield(&self) -> &Self::BitfieldType {
        &self.bitfield
    }
    fn bitfield_mut(&mut self) -> &mut Self::BitfieldType {
        &mut self.bitfield
    }
}

impl<const BITFIELD_U32_LEN: usize> SharedAllocator for OwnedShared<BITFIELD_U32_LEN> {
    type AllocatorType = ();
    fn alloc(&self) -> &Self::AllocatorType {
        &()
    }
}

impl<const BITFIELD_U32_LEN: usize> DefaultIn for OwnedShared<BITFIELD_U32_LEN> {
    type AllocatorType = ();
    fn default_in(_: ()) -> Self {
        Default::default()
    }
}

impl<const BITFIELD_U32_LEN: usize> From<&()> for OwnedShared<BITFIELD_U32_LEN> {
    fn from(_: &()) -> Self {
        Default::default()
    }
}
