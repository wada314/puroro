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

use crate::internal::impls::owned::deser2::GetMaybeLastMutMessageHandler;
use crate::internal::types::WireType;
use crate::internal::variant::Variant;
use crate::internal::MatchFieldNumber;
use crate::tags;
use crate::{ErrorKind, MessageImpl, Result};
use ::std::io::Read;
use ::std::io::Result as IoResult;

#[derive(Clone, Default)]
struct DeserFromSliceOptions {
    recursion_limit: Option<usize>,
}

pub trait DeserFromSlice: MatchFieldNumber<GetMaybeLastMutMessageHandler> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> Result<()> {
        self.deser_from_slice_with_options(self, slice, DeserFromSliceOptions::default())
    }
    fn deser_from_slice_with_options(
        &mut self,
        slice: &[u8],
        options: DeserFromSliceOptions,
    ) -> Result<()>;
}

impl<MP, FieldsType, SharedType> DeserFromSlice
    for MessageImpl<MP, tags::OwnedImpl, FieldsType, SharedType>
where
    Self: MatchFieldNumber<GetMaybeLastMutMessageHandler>,
{
    fn deser_from_slice_with_options(
        &mut self,
        mut slice: &[u8],
        options: DeserFromSliceOptions,
    ) -> Result<()> {
        while !slice.is_empty() {
            let (wire_type, number) = try_get_wire_type_and_field_number(&mut slice)?;
            
        }
        todo!()
    }
}

fn try_get_wire_type_and_field_number(slice: &mut &[u8]) -> Result<(WireType, i32)> {
    let key = Variant::decode_bytes(slice.bytes())?.to_u32()?;
    Ok((
        WireType::try_from((key & 0x07) as i32)?,
        (key >> 3)
            .try_into()
            .map_err(|_| ErrorKind::InvalidFieldNumber)?,
    ))
}
