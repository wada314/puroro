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

use crate::{ErrorKind, Result};
use ::std::marker::PhantomData;

// Variants
#[derive(Default, Clone)]
pub struct Int32;
#[derive(Default, Clone)]
pub struct Int64;
#[derive(Default, Clone)]
pub struct UInt32;
#[derive(Default, Clone)]
pub struct UInt64;
#[derive(Default, Clone)]
pub struct SInt32;
#[derive(Default, Clone)]
pub struct SInt64;
#[derive(Default, Clone)]
pub struct Bool;
#[derive(Default, Clone)]
pub struct Enum2<E>(PhantomData<E>);
#[derive(Default, Clone)]
pub struct Enum3<E>(PhantomData<E>);

// Length delimited types
#[derive(Default, Clone)]
pub struct Bytes;
#[derive(Default, Clone)]
pub struct String;
#[derive(Default, Clone)]
pub struct Message<M>(PhantomData<M>);

// Fixed 32 / 64 bit types
#[derive(Default, Clone)]
pub struct Float;
#[derive(Default, Clone)]
pub struct Fixed32;
#[derive(Default, Clone)]
pub struct SFixed32;
#[derive(Default, Clone)]
pub struct Double;
#[derive(Default, Clone)]
pub struct Fixed64;
#[derive(Default, Clone)]
pub struct SFixed64;

// Traits
pub trait NumericalType {
    type RustType;
    fn from_variant(_bytes: [u8; 8]) -> Result<Self::RustType> {
        Err(ErrorKind::UnexpectedWireType)?
    }
    fn from_bits32(_bytes: [u8; 4]) -> Result<Self::RustType> {
        Err(ErrorKind::UnexpectedWireType)?
    }
    fn from_bits64(_bytes: [u8; 8]) -> Result<Self::RustType> {
        Err(ErrorKind::UnexpectedWireType)?
    }
    fn to_variant(_val: Self::RustType) -> Result<[u8; 8]> {
        Err(ErrorKind::UnexpectedWireType)?
    }
    fn to_wire_type(_val: Self::RustType) -> Result<NumericalWireType> {
        unimplemented!()
    }
}
pub enum NumericalWireType {
    Variant([u8; 8]),
    Bits32([u8; 4]),
    Bits64([u8; 8]),
}

// Trait impls
impl NumericalType for Int32 {
    type RustType = i32;
    fn from_variant(bytes: [u8; 8]) -> Result<Self::RustType> {
        let val_u32: u32 = u64::from_le_bytes(bytes).try_into()?;
        Ok(i32::from_le_bytes(val_u32.to_le_bytes()))
    }
    fn to_variant(val: Self::RustType) -> Result<[u8; 8]> {
        Ok(i64::to_le_bytes(val.into()))
    }
}
impl NumericalType for Int64 {
    type RustType = i64;
}
impl NumericalType for UInt32 {
    type RustType = u32;
    fn from_variant(bytes: [u8; 8]) -> Result<Self::RustType> {
        Ok(u64::from_le_bytes(bytes).try_into()?)
    }
    fn to_variant(val: Self::RustType) -> Result<[u8; 8]> {
        Ok(u64::to_le_bytes(val.into()))
    }
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
    fn from_bits32(bytes: [u8; 4]) -> Result<Self::RustType> {
        Ok(f32::from_le_bytes(bytes))
    }
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
