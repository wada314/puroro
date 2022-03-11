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
use crate::{ErrorKind, MessageImpl, Result};
use ::std::io::Read;
use ::std::io::Result as IoResult;

enum Task<'a> {
    Pop,
    Parse(&'a [u8]),
}

pub trait DeserFromSlice: MatchFieldNumber<GetMaybeLastMutMessageHandler> {
    fn deser_from_slice(&mut self, slice: &[u8]) -> Result<()>;
}

impl<MP, FieldsType, SharedType> DeserFromSlice
    for MessageImpl<MP, tags::OwnedImpl, FieldsType, SharedType>
where
    Self: MatchFieldNumber<GetMaybeLastMutMessageHandler>,
{
    fn deser_from_slice(&mut self, root_slice: &[u8]) -> Result<()> {
        let mut tasks = vec![Task::Parse(root_slice)];
        let mut msg_stack: Vec<&mut dyn DeserFromSlice> = vec![self];
        while let Some(task) = tasks.pop() {
            match task {
                Task::Pop => {
                    msg_stack.pop();
                }
                Task::Parse(slice) => {
                    if slice.is_empty() {
                        continue;
                    }
                    let msg = *msg_stack
                        .last()
                        .ok_or(ErrorKind::DeserInternalError("msg_stack is empty"))?;
                    let (wire_type, field_number) = try_get_wire_type_and_field_number(
                        slice.as_ref().bytes(),
                    )?
                    .ok_or(ErrorKind::DeserInternalError(
                        "we have already checked if the slice is empty",
                    ))?;

                    // If the field is message type, need to go deeper
                    if matches!(wire_type, WireType::LengthDelimited) {
                        if let Some(submsg) = msg.match_field_number_mut(
                            field_number,
                            GetMaybeLastMutMessageHandler::default(),
                        )? {
                            let submsg_len =
                                Variant::decode_bytes(slice.as_ref().bytes())?.to_u32()? as usize;

                            // reserve the succeeding field parse for the current msg
                            tasks.push(Task::Parse(&slice[submsg_len..]));

                            // Push the child message into the msg_stack, and reserve it to pop
                            // after the parse is done
                            tasks.push(Task::Pop);
                            msg_stack.push(submsg);

                            // do the current message field parse next
                            tasks.push(Task::Parse(&slice[0..submsg_len]));
                            continue;
                        }
                    }

                    // If the field is non-message type, just process the field and
                    // then reserve the succeeding field parse right next
                }
            }
        }
        Ok(())
    }
}
