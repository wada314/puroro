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

use ::std::io::{BufRead, Read, Result as IoResult};

pub struct Variant([u8; 8]);

trait ReadExt {
    fn read_variant(&mut self) -> IoResult<Variant>;
}

pub trait BufReadExt {
    fn read_variant(&mut self) -> IoResult<Variant>;
}

impl<T: Read> ReadExt for T {
    fn read_variant(&mut self) -> IoResult<Variant> {
        todo!()
    }
}

impl<T: BufRead> BufReadExt for T {
    fn read_variant(&mut self) -> IoResult<Variant> {
        let inner_buf = self.fill_buf()?;

        // Speculative load 4 bytes.
        // Assuming the most usecases of the variant are <= 4 bytes.
        let (four_bytes_array, _) = inner_buf.as_chunks::<4>();
        let Some(four_bytes) = four_bytes_array.first() else {
            return <Self as ReadExt>::read_variant(self);
        };
        let a = u32::from_le_bytes(*four_bytes);
        if (a & 0x8888) == 0x8888 {
            // The variant is longer than 4 bytes. Fallback to naive implementation.
            return <Self as ReadExt>::read_variant(self);
        }

        // No early-return or branch after here for optimization!

        let connected_7bits_x4 = (a & 0x00_00_00_7F)
            | ((a & 0x00_00_7F_00) >> 1)
            | ((a & 0x00_7F_00_00) >> 2)
            | ((a & 0x7F_00_00_00) >> 3);
        let mask = 0x00_00_00_7F
            | u32::wrapping_neg(a & 0x00_00_00_80)
            | u32::wrapping_neg((a & 0x00_00_80_00) >> 1)
            | u32::wrapping_neg((a & 0x00_80_00_00) >> 2);

        todo!()
    }
}
