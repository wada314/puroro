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

//! Typetags for Proto field types.

use crate::Result;
use ::std::marker::PhantomData;

// Variants
pub struct Int32;
pub struct Int64;
pub struct UInt32;
pub struct UInt64;
pub struct SInt32;
pub struct SInt64;
pub struct Bool;
pub struct Enum2<E>(PhantomData<E>);
pub struct Enum3<E>(PhantomData<E>);

// Length delimited types
pub struct Bytes;
pub struct String;
pub struct Message<M>(PhantomData<M>);

// Fixed 32 / 64 bit types
pub struct Float;
pub struct Fixed32;
pub struct SFixed32;
pub struct Double;
pub struct Fixed64;
pub struct SFixed64;

// Traits
pub trait NumericalType {
    type RustType;
}
pub trait VariantType: NumericalType {
    #[allow(unused)]
    fn from_bytes(bytes: [u8; 8]) -> Result<Self::RustType> {
        todo!()
    }
}

// Trait impls
impl NumericalType for Int32 {
    type RustType = i32;
}
impl NumericalType for Int64 {
    type RustType = i64;
}
impl NumericalType for UInt32 {
    type RustType = u32;
}
impl NumericalType for UInt64 {
    type RustType = u64;
}
impl NumericalType for SInt32 {
    type RustType = i32;
}
impl NumericalType for SInt64 {
    type RustType = i64;
}
impl NumericalType for Bool {
    type RustType = bool;
}
impl<E> NumericalType for Enum2<E> {
    type RustType = E;
}
impl<E> NumericalType for Enum3<E> {
    type RustType = ::std::result::Result<E, i32>;
}
impl NumericalType for Float {
    type RustType = f32;
}
impl NumericalType for Fixed32 {
    type RustType = u32;
}
impl NumericalType for SFixed32 {
    type RustType = i32;
}
impl NumericalType for Double {
    type RustType = f64;
}
impl NumericalType for Fixed64 {
    type RustType = u64;
}
impl NumericalType for SFixed64 {
    type RustType = i64;
}

impl VariantType for Int32 {
    fn from_bytes(bytes: [u8; 8]) -> Result<Self::RustType> {
        let val_u32: u32 = u64::from_le_bytes(bytes).try_into()?;
        Ok(i32::from_le_bytes(val_u32.to_le_bytes()))
    }
}
impl VariantType for Int64 {}
impl VariantType for UInt32 {
    fn from_bytes(bytes: [u8; 8]) -> Result<Self::RustType> {
        Ok(u64::from_le_bytes(bytes).try_into()?)
    }
}
impl VariantType for UInt64 {}
impl VariantType for SInt32 {}
impl VariantType for SInt64 {}
impl VariantType for Bool {}
impl<E> VariantType for Enum2<E> {}
impl<E> VariantType for Enum3<E> {}
