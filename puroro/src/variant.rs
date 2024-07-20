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

use crate::{ErrorKind, Result};
use ::std::io::{BufRead, Read, Write};

#[derive(Debug, PartialEq, Eq, Default, Copy, Clone)]
pub struct Variant([u8; 8]);

// region: Traits for variant int types of proto
pub trait VariantIntegerType {
    type RustType;
    fn try_from_variant(var: Variant) -> Result<Self::RustType>;
    fn try_into_variant(x: Self::RustType) -> Result<Variant>;
}

pub mod variant_types {
    pub type Int32 = i32;
    pub type Int64 = i64;
    pub type UInt32 = u32;
    pub type UInt64 = u64;
    pub type Bool = bool;
    pub struct SInt32;
    pub struct SInt64;
    pub struct Enum<E>(::std::marker::PhantomData<E>);
}
use variant_types as vt;

impl VariantIntegerType for vt::Int32 {
    type RustType = i32;
    #[inline]
    fn try_from_variant(var: Variant) -> Result<Self::RustType> {
        Ok(i64::try_from_variant(var)?.try_into()?)
    }
    #[inline]
    fn try_into_variant(x: Self::RustType) -> Result<Variant> {
        i64::try_into_variant(x.into())
    }
}
impl VariantIntegerType for vt::Int64 {
    type RustType = i64;
    #[inline]
    fn try_from_variant(var: Variant) -> Result<Self::RustType> {
        Ok(i64::from_le_bytes(var.0))
    }
    #[inline]
    fn try_into_variant(x: Self::RustType) -> Result<Variant> {
        Ok(Variant(x.to_le_bytes()))
    }
}
impl VariantIntegerType for vt::UInt32 {
    type RustType = u32;
    #[inline]
    fn try_from_variant(var: Variant) -> Result<Self::RustType> {
        Ok(u64::try_from_variant(var)?.try_into()?)
    }
    #[inline]
    fn try_into_variant(x: Self::RustType) -> Result<Variant> {
        u64::try_into_variant(x as u64)
    }
}
impl VariantIntegerType for vt::UInt64 {
    type RustType = u64;
    #[inline]
    fn try_from_variant(var: Variant) -> Result<Self::RustType> {
        Ok(u64::from_le_bytes(var.0))
    }
    #[inline]
    fn try_into_variant(x: Self::RustType) -> Result<Variant> {
        Ok(Variant(x.to_le_bytes()))
    }
}
impl VariantIntegerType for vt::Bool {
    type RustType = bool;
    #[inline]
    fn try_from_variant(var: Variant) -> Result<Self::RustType> {
        match u64::try_from_variant(var)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(ErrorKind::IntegerToBoolError),
        }
    }
    #[inline]
    fn try_into_variant(x: Self::RustType) -> Result<Variant> {
        u64::try_into_variant(x as u64)
    }
}
impl VariantIntegerType for vt::SInt32 {
    type RustType = i32;
    #[inline]
    fn try_from_variant(var: Variant) -> Result<Self::RustType> {
        Ok(vt::SInt64::try_from_variant(var)?.try_into()?)
    }
    #[inline]
    fn try_into_variant(x: Self::RustType) -> Result<Variant> {
        Ok(vt::SInt64::try_into_variant(x as i64)?)
    }
}
impl VariantIntegerType for vt::SInt64 {
    type RustType = i64;
    #[inline]
    fn try_from_variant(var: Variant) -> Result<Self::RustType> {
        #![allow(unused)]
        todo!()
    }
    #[inline]
    fn try_into_variant(x: Self::RustType) -> Result<Variant> {
        #![allow(unused)]
        todo!()
    }
}
impl<E> VariantIntegerType for vt::Enum<E>
where
    E: TryFrom<i32, Error = i32>,
    i32: From<E>,
{
    type RustType = E;
    #[inline]
    fn try_from_variant(var: Variant) -> Result<Self::RustType> {
        Ok(E::try_from(vt::Int32::try_from_variant(var)?)
            .map_err(|i| ErrorKind::TryFromIntIntoEnumError(i))?)
    }
    #[inline]
    fn try_into_variant(e: Self::RustType) -> Result<Variant> {
        i32::try_into_variant(e.into())
    }
}

// endregion

// region: From unsigned
// To and from unsigned integers are unique (where signed integers are not)
impl From<u32> for Variant {
    fn from(value: u32) -> Self {
        (value as u64).into()
    }
}
impl From<u64> for Variant {
    fn from(value: u64) -> Self {
        Variant(value.to_le_bytes())
    }
}
impl From<Variant> for u64 {
    fn from(variant: Variant) -> Self {
        u64::from_le_bytes(variant.0)
    }
}
impl TryFrom<Variant> for u32 {
    type Error = ErrorKind;
    fn try_from(variant: Variant) -> Result<Self> {
        Ok(Into::<u64>::into(variant).try_into()?)
    }
}

// endregion

// region: Variant readers

pub trait ReadExtVariant {
    fn read_variant(&mut self) -> Result<Variant>;
    fn read_variant_or_eof(&mut self) -> Result<Option<Variant>>;
    fn into_variant_iter(self) -> impl Iterator<Item = Result<Variant>>;
}

pub trait BufReadExtVariant: ReadExtVariant {
    fn read_variant(&mut self) -> Result<Variant> {
        <Self as ReadExtVariant>::read_variant(self)
    }
    fn read_variant_peek_10(&mut self) -> Result<Variant>;
    fn read_variant_assume_4(&mut self) -> Result<Variant>;
    fn read_variant_assume_2(&mut self) -> Result<Variant>;
}

pub trait WriteExtVariant {
    fn write_variant(&mut self, variant: Variant) -> Result<usize>;
}

impl<T: Read> ReadExtVariant for T {
    #[inline]
    fn read_variant(&mut self) -> Result<Variant> {
        let iter = self.bytes();
        let mut result = 0u64;
        for (i, rbyte) in iter.take(10).enumerate() {
            let byte = rbyte?;
            result |= ((byte & 0x7F) as u64) << (i * 7);
            if (byte & 0x80) == 0 {
                break;
            }
            if i == 9 && ((byte & 0xFE) != 0) {
                return Err(ErrorKind::TooLongEncodedVariant);
            }
        }
        Ok(Variant(result.to_le_bytes()))
    }
    #[inline]
    fn read_variant_or_eof(&mut self) -> Result<Option<Variant>> {
        let mut iter = self.bytes().peekable();
        if iter.peek().is_none() {
            return Ok(None);
        }
        let mut result = 0u64;
        for (i, rbyte) in iter.take(10).enumerate() {
            let byte = rbyte?;
            result |= ((byte & 0x7F) as u64) << (i * 7);
            if (byte & 0x80) == 0 {
                break;
            }
            if i == 9 && ((byte & 0xFE) != 0) {
                return Err(ErrorKind::TooLongEncodedVariant);
            }
        }
        Ok(Some(Variant(result.to_le_bytes())))
    }
    fn into_variant_iter(self) -> impl Iterator<Item = Result<Variant>> {
        struct Iter<T> {
            reader: T,
        }
        impl<T: Read> Iterator for Iter<T> {
            type Item = Result<Variant>;
            fn next(&mut self) -> Option<Self::Item> {
                self.reader.read_variant_or_eof().transpose()
            }
        }
        Iter { reader: self }
    }
}

impl<T: BufRead> BufReadExtVariant for T {
    #[inline]
    fn read_variant_peek_10(&mut self) -> Result<Variant> {
        let inner_buf = self.fill_buf()?;
        let (ten_bytes_buf, _) = inner_buf.as_chunks::<10>();
        let Some(ten_bytes) = ten_bytes_buf.first() else {
            return <Self as ReadExtVariant>::read_variant(self);
        };

        let mut result = 0u64;
        for i in 0usize..10 {
            let byte = ten_bytes[i];
            result |= ((byte & 0b_01111111) as u64) << (i * 7);
            if (byte & 0b_10000000) == 0 {
                self.consume(i + 1);
                break;
            }
            if i == 9 && ((byte & 0b_11111110) != 0) {
                return Err(ErrorKind::TooLongEncodedVariant);
            }
        }

        Ok(result.into())
    }

    #[inline]
    fn read_variant_assume_4(&mut self) -> Result<Variant> {
        let inner_buf = self.fill_buf()?;

        let (four_bytes_array, _) = inner_buf.as_chunks::<4>();
        let Some(four_bytes) = four_bytes_array.first() else {
            return <Self as ReadExtVariant>::read_variant(self);
        };
        let a = u32::from_le_bytes(*four_bytes);
        if (a & 0b_10000000_10000000_10000000_10000000) == 0b_10000000_10000000_10000000_10000000 {
            // The variant is longer than 4 bytes. Fallback to naive implementation.
            return <Self as ReadExtVariant>::read_variant(self);
        }

        // For optimization, no early-return or branch after here!

        let connected_7bits_x4 = (a & 0b_00000000_00000000_00000000_01111111)
            | ((a & 0b_00000000_00000000_01111111_00000000) >> 1)
            | ((a & 0b_00000000_01111111_00000000_00000000) >> 2)
            | ((a & 0b_01111111_00000000_00000000_00000000) >> 3);
        let mask = {
            // Assuming 7 bits each for a, b, c, d,
            // [a....ab....bc....cd....d]: 28 bits
            // [11....................11]
            let mask1 = 0b_0000_1111111_1111111_1111111_1111111;
            // [00..............0011..11] or [11...11]
            let mask2 = !u32::wrapping_neg((!a & 0b_00000000_00000000_00000000_10000000) >> 0);
            // [00.......0011.........11] or [11...11]
            let mask3 = !u32::wrapping_neg((!a & 0b_00000000_00000000_10000000_00000000) >> 1);
            // [00..0011..............11] or [11...11]
            let mask4 = !u32::wrapping_neg((!a & 0b_00000000_10000000_00000000_00000000) >> 2);
            mask1 & mask2 & mask3 & mask4
        };

        let load_bytes_num_index = (((a & 0x00_00_00_80) >> 7)
            | ((a & 0x00_00_80_00) >> 14)
            | ((a & 0x00_80_00_00) >> 21)) as usize;
        let load_bytes_num = [1, 2, 1, 3, 1, 2, 1, 4][load_bytes_num_index];

        self.consume(load_bytes_num);
        Ok(((connected_7bits_x4 & mask) as u64).into())
    }

    #[inline]
    fn read_variant_assume_2(&mut self) -> Result<Variant> {
        let inner_buf = self.fill_buf()?;

        let (two_bytes_array, _) = inner_buf.as_chunks::<2>();
        let Some(two_bytes) = two_bytes_array.first() else {
            return <Self as ReadExtVariant>::read_variant(self);
        };
        let a = u16::from_le_bytes(*two_bytes);
        if (a & 0b_10000000_10000000) == 0b_10000000_10000000 {
            // The variant is longer than 2 bytes. Fallback to naive implementation.
            return <Self as ReadExtVariant>::read_variant(self);
        }

        // For optimization, no early-return or branch after here!

        let connected_7bits_x2 = (a & 0b_00000000_01111111) | ((a & 0b_01111111_00000000) >> 1);
        let mask = !u16::wrapping_neg(!a & 0b_00000000_10000000) & 0b_00_1111111_1111111;

        let load_bytes_num = ((a & 0x00_80) >> 7) as usize + 1;

        self.consume(load_bytes_num);
        Ok(((connected_7bits_x2 & mask) as u64).into())
    }
}

// endregion:

// region: Variant writers

impl<T: Write> WriteExtVariant for T {
    fn write_variant(&mut self, variant: Variant) -> Result<usize> {
        let mut v = u64::from_le_bytes(variant.0);
        let mut buffer = <[u8; 10]>::default();
        let mut byte_len = 0;
        for i in 0..10 {
            buffer[i] = (v & 0x7F) as u8;
            if (v & !0x7F) == 0 {
                byte_len = i + 1;
                break;
            }
            buffer[i] |= 0x80;
            v >>= 7;
        }
        let out_slice = &buffer[0..byte_len];
        self.write_all(out_slice)?;
        Ok(byte_len)
    }
}

// endregion:

#[cfg(test)]
mod test {
    use super::{BufReadExtVariant, ReadExtVariant, WriteExtVariant};
    use crate::{ErrorKind, Result};
    use ::std::assert_matches::assert_matches;

    const BASIC_TEST_CASES: &[(u64, &[u8])] = &[
        (0, &[0]),
        (1, &[1]),
        (100, &[100]),
        (127, &[0x7f]),
        (128, &[0x80, 0x01]),
        (1_000, &[0xe8, 0x07]),
        (0x3FFF, &[0xFF, 0x7F]),
        (0x4000, &[0x80, 0x80, 0x01]),
        (1_000_000, &[0xc0, 0x84, 0x3d]),
        (100_000_000, &[0x80, 0xc2, 0xd7, 0x2f]),
        (1_000_000_000, &[0x80, 0x94, 0xeb, 0xdc, 0x03]),
        (
            0x7FFF_FFFF_FFFF_FFFF,
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F],
        ),
        (
            0x8000_0000_0000_0000,
            &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01],
        ),
        (
            0xFFFF_FFFF_FFFF_FFFF,
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01],
        ),
    ];

    #[test]
    fn test_write_variant() -> Result<()> {
        for &(value, expected) in BASIC_TEST_CASES {
            let mut buf = Vec::<u8>::new();
            buf.write_variant(value.into())?;
            assert_eq!(expected, &buf, "Failed for value={}.", value);
        }

        Ok(())
    }

    #[test]
    fn test_read_variant() -> Result<()> {
        for &(expected, mut input) in BASIC_TEST_CASES {
            let var = <&[u8] as ReadExtVariant>::read_variant(&mut input)?;
            assert_eq!(
                0,
                input.len(),
                "The input buffer is not read until the end. value={}.",
                expected
            );
            assert_eq!(expected, var.into());
        }

        Ok(())
    }

    #[test]
    fn test_read_variant_too_long() -> Result<()> {
        let mut input: &[u8] = &[0x80u8; 10];
        assert_matches!(
            <&[u8] as ReadExtVariant>::read_variant(&mut input),
            Err(ErrorKind::TooLongEncodedVariant)
        );
        Ok(())
    }

    #[test]
    fn test_buf_read_peek_10() -> Result<()> {
        for &(expected, mut input) in BASIC_TEST_CASES {
            let var = <&[u8] as BufReadExtVariant>::read_variant_peek_10(&mut input)?;
            assert_eq!(
                0,
                input.len(),
                "The input buffer is not read until the end. value={}.",
                expected
            );
            assert_eq!(expected, var.into());
        }

        Ok(())
    }

    #[test]
    fn test_buf_read_variant_4() -> Result<()> {
        for &(expected, mut input) in BASIC_TEST_CASES {
            let var = <&[u8] as BufReadExtVariant>::read_variant_assume_4(&mut input)?;
            assert_eq!(
                0,
                input.len(),
                "The input buffer is not read until the end. value={}.",
                expected
            );
            assert_eq!(expected, var.into());
        }

        Ok(())
    }

    #[test]
    fn test_buf_read_variant_2() -> Result<()> {
        for &(expected, mut input) in BASIC_TEST_CASES {
            let var = <&[u8] as BufReadExtVariant>::read_variant_assume_2(&mut input)?;
            assert_eq!(
                0,
                input.len(),
                "The input buffer is not read until the end. value={}.",
                expected
            );
            assert_eq!(expected, var.into());
        }

        Ok(())
    }
}
