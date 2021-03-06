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

use crate::tags;

pub trait Bits32TypeTag: tags::NumericalTypeTag {
    fn from_array(array: [u8; 4]) -> Self::NativeType;
    fn into_array(val: Self::NativeType) -> [u8; 4];
}
impl Bits32TypeTag for tags::Float {
    fn from_array(array: [u8; 4]) -> Self::NativeType {
        f32::from_le_bytes(array)
    }
    fn into_array(val: Self::NativeType) -> [u8; 4] {
        f32::to_le_bytes(val)
    }
}
impl Bits32TypeTag for tags::Fixed32 {
    fn from_array(array: [u8; 4]) -> Self::NativeType {
        u32::from_le_bytes(array)
    }
    fn into_array(val: Self::NativeType) -> [u8; 4] {
        u32::to_le_bytes(val)
    }
}
impl Bits32TypeTag for tags::SFixed32 {
    fn from_array(array: [u8; 4]) -> Self::NativeType {
        i32::from_le_bytes(array)
    }
    fn into_array(val: Self::NativeType) -> [u8; 4] {
        i32::to_le_bytes(val)
    }
}

pub trait Bits64TypeTag: tags::NumericalTypeTag {
    fn from_array(array: [u8; 8]) -> Self::NativeType;
    fn into_array(val: Self::NativeType) -> [u8; 8];
}
impl Bits64TypeTag for tags::Double {
    fn from_array(array: [u8; 8]) -> Self::NativeType {
        f64::from_le_bytes(array)
    }
    fn into_array(val: Self::NativeType) -> [u8; 8] {
        f64::to_le_bytes(val)
    }
}
impl Bits64TypeTag for tags::Fixed64 {
    fn from_array(array: [u8; 8]) -> Self::NativeType {
        u64::from_le_bytes(array)
    }
    fn into_array(val: Self::NativeType) -> [u8; 8] {
        u64::to_le_bytes(val)
    }
}
impl Bits64TypeTag for tags::SFixed64 {
    fn from_array(array: [u8; 8]) -> Self::NativeType {
        i64::from_le_bytes(array)
    }
    fn into_array(val: Self::NativeType) -> [u8; 8] {
        i64::to_le_bytes(val)
    }
}
