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

//! A type for the message's field-wise shared items.
//! e.g. Allocator, unknown field storage

use crate::internal::bitvec::{BitArray, BitSlice};
use crate::internal::unknown_fields::{UnknownFields, UnknownFieldsImpl};

pub struct SharedItems<const BITFIELD_LEN32: usize> {
    bitfield: BitArray<BITFIELD_LEN32>,
    unknown_fields: UnknownFieldsImpl,
}

pub trait SharedItemsTrait {
    type BitSliceType: BitSlice;
    fn bitfield(&self) -> &Self::BitSliceType;
    fn bitfield_mut(&mut self) -> &mut Self::BitSliceType;
    type UnknownFieldsType: UnknownFields;
    fn unknown_fields(&self) -> &Self::UnknownFieldsType;
    fn unknown_fields_mut(&mut self) -> &mut Self::UnknownFieldsType;
}

impl<const BITFIELD_LEN32: usize> SharedItemsTrait for SharedItems<BITFIELD_LEN32> {
    type BitSliceType = BitArray<BITFIELD_LEN32>;
    fn bitfield(&self) -> &Self::BitSliceType {
        &self.bitfield
    }
    fn bitfield_mut(&mut self) -> &mut Self::BitSliceType {
        &mut self.bitfield
    }
    type UnknownFieldsType = UnknownFieldsImpl;
    fn unknown_fields(&self) -> &Self::UnknownFieldsType {
        &self.unknown_fields
    }
    fn unknown_fields_mut(&mut self) -> &mut Self::UnknownFieldsType {
        &mut self.unknown_fields
    }
}
