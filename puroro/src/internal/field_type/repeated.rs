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

use super::*;
use crate::bitvec::BitSlice;
use crate::tags;
use crate::Message;

pub trait RepeatedFieldType: FieldType {
    type ScalarType;
    fn get_field<B: BitSlice>(&self, bitvec: &B) -> &[Self::ScalarType];
}

impl<RustType, ProtoType> RepeatedFieldType for RepeatedNumericalField<RustType, ProtoType>
where
    RustType: PartialEq + Default + Clone,
    ProtoType: tags::NumericalType<RustType = RustType>,
{
    type ScalarType = RustType;

    fn get_field<B: BitSlice>(&self, _bitvec: &B) -> &[Self::ScalarType] {
        self.0.as_slice()
    }
}

impl RepeatedFieldType for RepeatedStringField {
    type ScalarType = String;
    fn get_field<B: BitSlice>(&self, _bitvec: &B) -> &[Self::ScalarType] {
        self.0.as_slice()
    }
}

impl<M: Message + Default + Clone> RepeatedFieldType for RepeatedMessageField<M> {
    type ScalarType = M;
    fn get_field<B: BitSlice>(&self, _bitvec: &B) -> &[Self::ScalarType] {
        self.0.as_slice()
    }
}
