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

use crate::internal::types::WireType;
use crate::internal::variant::Variant;
use crate::ErrorKind;
use crate::Result;
use ::std::convert::TryInto;
use ::std::io::Write;

pub fn write_field_number_and_wire_type<W: Write>(
    out: &mut W,
    field_number: i32,
    wire_type: WireType,
) -> Result<()> {
    let encoding_val = <i32 as TryInto<u32>>::try_into(field_number)
        .map_err(|_| ErrorKind::InvalidFieldNumber)?
        .checked_shl(3)
        .ok_or(ErrorKind::InvalidFieldNumber)?
        | (wire_type as u32);
    let variant = Variant::from_u32(encoding_val)?;
    variant.encode_bytes(out)?;
    Ok(())
}
