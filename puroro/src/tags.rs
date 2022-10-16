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

use crate::{ErrorKind, PuroroError, Result};
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
        Ok(i64::from_le_bytes(bytes).try_into()?)
    }
    fn to_variant(val: Self::RustType) -> Result<[u8; 8]> {
        Ok(i64::to_le_bytes(val.into()))
    }
    fn to_wire_type(val: Self::RustType) -> Result<NumericalWireType> {
        Ok(NumericalWireType::Variant(i64::to_le_bytes(val.into())))
    }
}
impl NumericalType for Int64 {
    type RustType = i64;
    fn from_variant(bytes: [u8; 8]) -> Result<Self::RustType> {
        Ok(i64::from_le_bytes(bytes))
    }
    fn to_variant(val: Self::RustType) -> Result<[u8; 8]> {
        Ok(i64::to_le_bytes(val))
    }
    fn to_wire_type(val: Self::RustType) -> Result<NumericalWireType> {
        Ok(NumericalWireType::Variant(i64::to_le_bytes(val)))
    }
}
impl NumericalType for UInt32 {
    type RustType = u32;
    fn from_variant(bytes: [u8; 8]) -> Result<Self::RustType> {
        Ok(u64::from_le_bytes(bytes).try_into()?)
    }
    fn to_variant(val: Self::RustType) -> Result<[u8; 8]> {
        Ok(u64::to_le_bytes(val.into()))
    }
    fn to_wire_type(val: Self::RustType) -> Result<NumericalWireType> {
        Ok(NumericalWireType::Variant(u64::to_le_bytes(val.into())))
    }
}
impl NumericalType for UInt64 {
    type RustType = u64;
    fn from_variant(bytes: [u8; 8]) -> Result<Self::RustType> {
        Ok(u64::from_le_bytes(bytes))
    }
    fn to_variant(val: Self::RustType) -> Result<[u8; 8]> {
        Ok(u64::to_le_bytes(val))
    }
    fn to_wire_type(val: Self::RustType) -> Result<NumericalWireType> {
        Ok(NumericalWireType::Variant(u64::to_le_bytes(val)))
    }
}
impl NumericalType for SInt32 {
    type RustType = i32;
    fn from_variant(bytes: [u8; 8]) -> Result<Self::RustType> {
        let val_u32: u32 = u64::from_le_bytes(bytes).try_into()?;
        Ok(decode_sint32(val_u32))
    }
    fn to_variant(val: Self::RustType) -> Result<[u8; 8]> {
        Ok(u64::to_le_bytes(encode_sint32(val).into()))
    }
    fn to_wire_type(val: Self::RustType) -> Result<NumericalWireType> {
        Ok(NumericalWireType::Variant(u64::to_le_bytes(
            encode_sint32(val).into(),
        )))
    }
}
impl NumericalType for SInt64 {
    type RustType = i64;
    fn from_variant(bytes: [u8; 8]) -> Result<Self::RustType> {
        Ok(decode_sint64(u64::from_le_bytes(bytes)))
    }
    fn to_variant(val: Self::RustType) -> Result<[u8; 8]> {
        Ok(u64::to_le_bytes(encode_sint64(val)))
    }
    fn to_wire_type(val: Self::RustType) -> Result<NumericalWireType> {
        Ok(NumericalWireType::Variant(u64::to_le_bytes(encode_sint64(
            val,
        ))))
    }
}
impl NumericalType for Bool {
    type RustType = bool;
    fn from_variant(bytes: [u8; 8]) -> Result<Self::RustType> {
        Ok(u64::from_le_bytes(bytes) != 0)
    }
    fn to_variant(val: Self::RustType) -> Result<[u8; 8]> {
        Ok(u64::to_le_bytes(val.into()))
    }
    fn to_wire_type(val: Self::RustType) -> Result<NumericalWireType> {
        Ok(NumericalWireType::Variant(u64::to_le_bytes(val.into())))
    }
}
impl<E> NumericalType for Enum2<E>
where
    i32: TryInto<E, Error = PuroroError>,
    E: Into<i32>,
{
    type RustType = E;
    fn from_variant(bytes: [u8; 8]) -> Result<Self::RustType> {
        let int_value: i32 = i64::from_le_bytes(bytes).try_into()?;
        Ok(int_value.try_into()?)
    }
    fn to_variant(val: Self::RustType) -> Result<[u8; 8]> {
        Ok(i64::to_le_bytes(val.into() as i64))
    }
    fn to_wire_type(val: Self::RustType) -> Result<NumericalWireType> {
        Ok(NumericalWireType::Variant(i64::to_le_bytes(
            val.into() as i64
        )))
    }
}
impl<E> NumericalType for Enum3<E>
where
    i32: Into<E>,
    E: Into<i32>,
{
    type RustType = E;
    fn from_variant(bytes: [u8; 8]) -> Result<Self::RustType> {
        let int_value: i32 = i64::from_le_bytes(bytes).try_into()?;
        Ok(int_value.into())
    }
    fn to_variant(val: Self::RustType) -> Result<[u8; 8]> {
        Ok(i64::to_le_bytes(val.into() as i64))
    }
    fn to_wire_type(val: Self::RustType) -> Result<NumericalWireType> {
        Ok(NumericalWireType::Variant(i64::to_le_bytes(
            val.into() as i64
        )))
    }
}
impl NumericalType for Float {
    type RustType = f32;
    fn from_bits32(bytes: [u8; 4]) -> Result<Self::RustType> {
        Ok(f32::from_le_bytes(bytes))
    }
    fn to_wire_type(val: Self::RustType) -> Result<NumericalWireType> {
        Ok(NumericalWireType::Bits32(f32::to_le_bytes(val)))
    }
}
impl NumericalType for Fixed32 {
    type RustType = u32;
    fn from_bits32(bytes: [u8; 4]) -> Result<Self::RustType> {
        Ok(u32::from_le_bytes(bytes))
    }
    fn to_wire_type(val: Self::RustType) -> Result<NumericalWireType> {
        Ok(NumericalWireType::Bits32(u32::to_le_bytes(val)))
    }
}
impl NumericalType for SFixed32 {
    type RustType = i32;
    fn from_bits32(bytes: [u8; 4]) -> Result<Self::RustType> {
        Ok(i32::from_le_bytes(bytes))
    }
    fn to_wire_type(val: Self::RustType) -> Result<NumericalWireType> {
        Ok(NumericalWireType::Bits32(i32::to_le_bytes(val)))
    }
}
impl NumericalType for Double {
    type RustType = f64;
    fn from_bits64(bytes: [u8; 8]) -> Result<Self::RustType> {
        Ok(f64::from_le_bytes(bytes))
    }
    fn to_wire_type(val: Self::RustType) -> Result<NumericalWireType> {
        Ok(NumericalWireType::Bits64(f64::to_le_bytes(val)))
    }
}
impl NumericalType for Fixed64 {
    type RustType = u64;
    fn from_bits64(bytes: [u8; 8]) -> Result<Self::RustType> {
        Ok(u64::from_le_bytes(bytes))
    }
    fn to_wire_type(val: Self::RustType) -> Result<NumericalWireType> {
        Ok(NumericalWireType::Bits64(u64::to_le_bytes(val)))
    }
}
impl NumericalType for SFixed64 {
    type RustType = i64;
    fn from_bits64(bytes: [u8; 8]) -> Result<Self::RustType> {
        Ok(i64::from_le_bytes(bytes))
    }
    fn to_wire_type(val: Self::RustType) -> Result<NumericalWireType> {
        Ok(NumericalWireType::Bits64(i64::to_le_bytes(val)))
    }
}

fn encode_sint32(s: i32) -> u32 {
    u32::from_le_bytes(((s << 1) ^ (s >> 31)).to_le_bytes())
}
fn encode_sint64(s: i64) -> u64 {
    u64::from_le_bytes(((s << 1) ^ (s >> 63)).to_le_bytes())
}
fn decode_sint32(i: u32) -> i32 {
    i32::from_le_bytes(((i >> 1) ^ (0u32.wrapping_sub(i & 1))).to_le_bytes())
}
fn decode_sint64(i: u64) -> i64 {
    i64::from_le_bytes(((i >> 1) ^ (0u64.wrapping_sub(i & 1))).to_le_bytes())
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sint32() {
        fn check(s: i32) {
            assert_eq!(s, decode_sint32(encode_sint32(s)))
        }
        check(0);
        check(1);
        check(2);
        check(3);
        check(i32::MIN);
        check(i32::MIN + 1);
        check(i32::MAX);
        check(i32::MAX - 1);
    }

    #[test]
    fn test_sint64() {
        fn check(s: i64) {
            assert_eq!(s, decode_sint64(encode_sint64(s)))
        }
        check(0);
        check(1);
        check(2);
        check(3);
        check(i64::MIN);
        check(i64::MIN + 1);
        check(i64::MAX);
        check(i64::MAX - 1);
    }
}
