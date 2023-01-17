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

use super::ser::{ser_numerical_shared, ser_wire_and_number, FieldData, WireType};
use crate::internal::tags;
use crate::Result;
use ::std::collections::HashMap;
use ::std::fmt::{DebugStruct, Result as FmtResult};
use ::std::io::{Result as IoResult, Write};

pub trait UnknownFields {
    fn debug_struct_fields<'a, 'b>(&self, debug_struct: &mut DebugStruct<'a, 'b>) -> FmtResult;
    fn push<I: Iterator<Item = IoResult<u8>>>(
        &mut self,
        number: i32,
        field_data: FieldData<I>,
    ) -> Result<()>;
    fn ser_to_write<W: Write>(&self, out: &mut W) -> Result<()>;
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct UnknownFieldsImpl {
    fields: HashMap<i32, Vec<FieldData<Vec<u8>>>>,
}

impl UnknownFields for UnknownFieldsImpl {
    fn debug_struct_fields<'a, 'b>(&self, debug_struct: &mut DebugStruct<'a, 'b>) -> FmtResult {
        for (&number, fields) in &self.fields {
            for field in fields {
                debug_struct.field(&number.to_string(), field);
            }
        }
        Ok(())
    }

    fn push<I: Iterator<Item = IoResult<u8>>>(
        &mut self,
        number: i32,
        field_data: FieldData<I>,
    ) -> Result<()> {
        let owned_field_data = field_data
            .map(|iter| iter.collect::<IoResult<Vec<_>>>())
            .transpose()?;
        self.fields
            .entry(number)
            .or_default()
            .push(owned_field_data);
        Ok(())
    }

    fn ser_to_write<W: Write>(&self, out: &mut W) -> Result<()> {
        for (&number, fds_vec) in &self.fields {
            for field_data in fds_vec {
                match field_data {
                    FieldData::Variant(v) => {
                        // We do not do the packed variant output here to respect the
                        // original input wire format.
                        ser_wire_and_number(WireType::Variant, number, out)?;
                        v.encode_bytes(out)?;
                    }
                    FieldData::LengthDelimited(vec) => {
                        ser_wire_and_number(WireType::LengthDelimited, number, out)?;
                        ser_numerical_shared::<i32, tags::Int32, _>(
                            vec.len().try_into()?,
                            number,
                            out,
                        )?;
                        out.write_all(vec)?;
                    }
                    FieldData::Bits32(b) => {
                        ser_wire_and_number(WireType::Bits32, number, out)?;
                        out.write_all(b)?;
                    }
                    FieldData::Bits64(b) => {
                        ser_wire_and_number(WireType::Bits64, number, out)?;
                        out.write_all(b)?;
                    }
                }
            }
        }
        Ok(())
    }
}
