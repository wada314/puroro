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

use super::deser::try_get_wire_type_and_field_number;
use crate::internal::impls::owned::deser2::GetMaybeLastMutMessageHandler;
use crate::internal::types::WireType;
use crate::internal::variant::Variant;
use crate::internal::MatchFieldNumber;
use crate::tags;
use crate::{MessageImpl, Result};
use ::std::io::Read;
use ::std::io::Result as IoResult;

pub trait DeserFromSlice: MatchFieldNumber<GetMaybeLastMutMessageHandler> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> Result<()>;
}

impl<MP, FieldsType, SharedType> DeserFromSlice
    for MessageImpl<MP, tags::OwnedImpl, FieldsType, SharedType>
where
    Self: MatchFieldNumber<GetMaybeLastMutMessageHandler>,
{
    fn deser_from_slice(&mut self, root_slice: &[u8]) -> Result<()> {
        let mut tasks: Vec<(&mut dyn DeserFromSlice, &[u8])> = vec![(self, root_slice)];
        while let Some((msg, slice)) = tasks.pop() {
            if slice.is_empty() {
                continue;
            }
            if let Some((wire_type, field_number)) =
                try_get_wire_type_and_field_number(slice.as_ref().bytes())?
            {
                // If the field is message type, need to go deeper
                if matches!(wire_type, WireType::LengthDelimited) {
                    if let Some(submsg) = msg.match_field_number_mut(
                        field_number,
                        GetMaybeLastMutMessageHandler::default(),
                    )? {
                        let submsg_len =
                            Variant::decode_bytes(slice.as_ref().bytes())?.to_u32()? as usize;
                        tasks.push((msg, &slice[submsg_len..])); // reserve the succeeding field parse
                        tasks.push((submsg, &slice[0..submsg_len])); // do the current message field parse next
                        continue;
                    }
                }

                // If the field is non-message type, just process the field and
                // then reserve the succeeding field parse right next
            }
        }
        Ok(())
    }
}
