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
use crate::internal::types::WireType;
use crate::internal::variant::Variant;
use crate::internal::MatchFieldNumber;
use crate::tags;
use crate::{ErrorKind, MessageImpl, Result};
use ::std::io::Read;
use ::std::io::Result as IoResult;

pub trait DeserFromSlice {
    fn deser_from_slice(&mut self, slice: &[u8]) -> Result<()>;
}

impl<MP, FieldsType, SharedType> DeserFromSlice
    for MessageImpl<MP, tags::OwnedImpl, FieldsType, SharedType>
{
    fn deser_from_slice(&mut self, mut slice: &[u8]) -> Result<()> {
        let mut stack: Vec<&mut dyn DeserFromSlice> = vec![self];
        loop {
            if slice.is_empty() {
                break Ok(());
            }
            if let Some((wire_type, field_number)) =
                try_get_wire_type_and_field_number(slice.bytes())?
            {}
        }
    }
}
