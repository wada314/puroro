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
use crate::internal::bitvec::BitSlice;
use crate::internal::tags;
use crate::Message;

pub trait RepeatedFieldType: FieldType {
    type ScalarType;
    fn get_field<B: BitSlice>(&self, bitvec: &B) -> &[Self::ScalarType];
    type ContainerType;
    fn mut_field<B: BitSlice>(&mut self, bitvec: &mut B) -> &mut Self::ContainerType;
    fn clear<B: BitSlice>(&mut self, bitvec: &mut B);
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

    type ContainerType = Vec<Self::ScalarType>;

    fn mut_field<B: BitSlice>(&mut self, _bitvec: &mut B) -> &mut Self::ContainerType {
        &mut self.0
    }

    fn clear<B: BitSlice>(&mut self, _bitvec: &mut B) {
        self.0.clear()
    }
}

impl RepeatedFieldType for RepeatedStringField {
    type ScalarType = String;
    fn get_field<B: BitSlice>(&self, _bitvec: &B) -> &[Self::ScalarType] {
        self.0.as_slice()
    }
    type ContainerType = Vec<Self::ScalarType>;
    fn mut_field<B: BitSlice>(&mut self, _bitvec: &mut B) -> &mut Self::ContainerType {
        &mut self.0
    }
    fn clear<B: BitSlice>(&mut self, _bitvec: &mut B) {
        self.0.clear()
    }
}

impl RepeatedFieldType for RepeatedBytesField {
    type ScalarType = Vec<u8>;
    fn get_field<B: BitSlice>(&self, _bitvec: &B) -> &[Self::ScalarType] {
        self.0.as_slice()
    }
    type ContainerType = Vec<Self::ScalarType>;
    fn mut_field<B: BitSlice>(&mut self, _bitvec: &mut B) -> &mut Self::ContainerType {
        &mut self.0
    }
    fn clear<B: BitSlice>(&mut self, _bitvec: &mut B) {
        self.0.clear()
    }
}

impl<M: Message + Default + Clone> RepeatedFieldType for RepeatedMessageField<M> {
    type ScalarType = M;
    fn get_field<B: BitSlice>(&self, _bitvec: &B) -> &[Self::ScalarType] {
        self.0.as_slice()
    }

    type ContainerType = Vec<Self::ScalarType>;

    fn mut_field<B: BitSlice>(&mut self, _bitvec: &mut B) -> &mut Self::ContainerType {
        &mut self.0
    }

    fn clear<B: BitSlice>(&mut self, _bitvec: &mut B) {
        self.0.clear()
    }
}
