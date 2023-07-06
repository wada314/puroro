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

use ::std::io::{BufRead, Read};
use crate::{DeserResult, DeserError};

pub struct Variant([u8; 8]);

trait ReadExt {
    fn read_variant(&mut self) -> DeserResult<Variant>;
}

pub trait BufReadExt {
    fn read_variant(&mut self) -> DeserResult<Variant>;
}

impl<T: Read> ReadExt for T {
    fn read_variant(&mut self) -> DeserResult<Variant> {
        let iter = self.bytes();
        let mut result = 0u64;
        for (i, rbyte) in iter.take(10).enumerate() {
            let byte = rbyte?;
            result |= ((byte & 0x7F) as u64) << (i*7);
            if (byte & 080) == 0 {
                break;
            }
            if i == 9 && (byte & 0xFE != 0) {
                return Err(DeserError::InvalidVariant);
            }
        }
        Ok(Variant(result.to_le_bytes()))
    }
}

impl<T: BufRead> BufReadExt for T {
    fn read_variant(&mut self) -> DeserResult<Variant> {
        let inner_buf = self.fill_buf()?;

        let (four_bytes_array, _) = inner_buf.as_chunks::<4>();
        let Some(four_bytes) = four_bytes_array.first() else {
            return <Self as ReadExt>::read_variant(self);
        };
        let a = u32::from_le_bytes(*four_bytes);
        if (a & 0x80_80_80_80) == 0x80_80_80_80 {
            // The variant is longer than 4 bytes. Fallback to naive implementation.
            return <Self as ReadExt>::read_variant(self);
        }

        // For optimization, no early-return or branch after here!

        let connected_7bits_x4 = (a & 0x00_00_00_7F)
            | ((a & 0x00_00_7F_00) >> 1)
            | ((a & 0x00_7F_00_00) >> 2)
            | ((a & 0x7F_00_00_00) >> 3);
        let mask = 
            // Assuming 7 bits each for a, b, c, d,
            // [a...ab...bc...cd...d]
            // [11...........1100.00]
            u32::wrapping_neg(a & 0x00_00_00_80)
            // [11......1100......00]
            & u32::wrapping_neg((a & 0x00_00_80_00) >> 1)
            // [11.1100...........00]
            & u32::wrapping_neg((a & 0x00_80_00_00) >> 2);
        
        let load_bytes_num_index = (((a & 0x00_00_00_80) >> 7) | ((a & 0x00_00_80_00) >> 15) | ((a & 0x00_80_00_00) >> 23)) as usize;
        let load_bytes_num = [1, 2, 1, 3, 1, 2, 1, 4][load_bytes_num_index];

        self.consume(load_bytes_num);
        Ok(Variant(((connected_7bits_x4 & mask) as u64).to_le_bytes()))
    }
}
