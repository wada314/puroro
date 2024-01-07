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

use crate::internal::variant::Variant;
use crate::{ErrorKind, Result};
use ::futures::io::AsyncRead;

pub struct Record<T> {
    number: i32,
    payload: Payload<T>,
}
pub enum Payload<T> {
    Variant(Variant),
    I32([u8; 4]),
    I64([u8; 8]),
    Len(T),
}

trait ReadExtRecord {
    type LenPayloadType;
    fn read_record(&mut self) -> Result<Payload<Self::LenPayloadType>>;
}
impl<'a> ReadExtRecord for &'a [u8] {
    type LenPayloadType = &'a [u8];
    fn read_record(&mut self) -> Result<Payload<Self::LenPayloadType>> {
        use crate::internal::variant::ReadExtVariant;
        let tag = self.read_variant()?.try_as_uint32()?;
        let wire_type = tag & 0x7;
        let number = tag >> 3;
        todo!()
    }
}

pub trait DeseringMessage {
    fn finish(&mut self) -> Result<()>;
    fn deser_record(&mut self, record: Record<&[u8]>) -> Result<Option<&mut dyn DeseringMessage>>;
}
pub trait AsyncDeseringMessage {
    fn finish(&mut self) -> Result<()>;
}

pub fn deser_from_slice(root_msg: &mut dyn DeseringMessage, input: &[u8]) -> Result<()> {
    let mut stack = Vec::new();
    stack.push((root_msg, input));
    while let Some((msg, mut remain)) = stack.pop() {
        if remain.is_empty() {
            msg.finish()?;
            continue;
        }
    }
    todo!()
}
