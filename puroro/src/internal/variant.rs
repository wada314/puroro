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
use crate::{ErrorKind, Result};
use std::io::Result as IoResult;
use std::io::Write;

/// A representative type of variant-encoded types.
#[derive(Debug, Default, Clone)]
pub struct Variant([u8; 8]);

impl Variant {
    #[allow(unused)]
    pub fn new(bytes: [u8; 8]) -> Self {
        Variant(bytes)
    }
    pub fn decode_bytes<I>(bytes: &mut I) -> Result<Option<Self>>
    where
        I: Iterator<Item = IoResult<u8>>,
    {
        let mut x = 0u64;
        for i in 0..9 {
            match (i, bytes.next()) {
                (_, Some(maybe_byte)) => {
                    let byte = maybe_byte?;
                    x |= ((byte & 0x7F) as u64) << (i * 7);
                    if byte < 0x80 {
                        return Ok(Some(Variant(x.to_ne_bytes())));
                    }
                }
                (0, None) => return Ok(None),
                (_, None) => Err(ErrorKind::UnexpectedInputTermination)?,
            }
        }
        // i == 9, so now checking a last MSBit.
        match bytes.next() {
            Some(maybe_byte) => {
                let byte = maybe_byte?;
                x |= ((byte & 0x01) as u64) << 63;
                if byte & 0xFE != 0 {
                    Err(ErrorKind::TooLargeVariant)?
                } else {
                    return Ok(Some(Variant(x.to_ne_bytes())));
                }
            }
            None => Err(ErrorKind::UnexpectedInputTermination)?,
        }
    }
    pub fn encode_bytes<W>(&self, write: &mut W) -> Result<()>
    where
        W: Write,
    {
        let mut x = u64::from_le_bytes(self.0);
        if x == 0 {
            write.write_all(&[0])?;
            return Ok(());
        }
        let mut length = 0;
        let mut buf: [u8; 10] = Default::default();
        for i in 0..11 {
            if x == 0 {
                length = i;
                break;
            }
            debug_assert!(i != 10);
            buf[i] = ((x as u8) & 0x7F) | 0x80;
            x = x >> 7;
        }
        buf[length - 1] = buf[length - 1] & 0x7F;
        write.write_all(buf.split_at(length).0)?;
        Ok(())
    }

    pub fn zero() -> Self {
        Self([0; 8])
    }
    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|x| *x == 0)
    }

    pub fn get<T: tags::NumericalType>(&self) -> Result<T::RustType> {
        Ok(T::from_variant(self.0)?)
    }
    pub fn get_i32(&self) -> Result<i32> {
        Ok(self.get::<tags::Int32>()?)
    }
    pub fn get_u32(&self) -> Result<u32> {
        Ok(self.get::<tags::UInt32>()?)
    }

    pub fn from_i32(val: i32) -> Self {
        Self::new(i64::to_le_bytes(val.into()))
    }
    pub fn from_u32(val: u32) -> Self {
        Self::new(u64::to_le_bytes(val.into()))
    }
}
