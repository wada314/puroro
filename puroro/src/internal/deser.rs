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
    number: u32,
    payload: Payload<T>,
}
pub enum Payload<T> {
    Variant(Variant),
    I32([u8; 4]),
    I64([u8; 8]),
    Len(T),
}

enum WireType {
    Variant = 0,
    I64 = 1,
    Len = 2,
    I32 = 5,
}
impl TryFrom<u32> for WireType {
    type Error = ErrorKind;
    fn try_from(value: u32) -> Result<WireType> {
        Ok(match value {
            0 => WireType::Variant,
            1 => WireType::I64,
            2 => WireType::Len,
            5 => WireType::I32,
            _ => Err(ErrorKind::UnknownWireType)?,
        })
    }
}

trait ReadExtRecord {
    type LenPayloadType;
    fn read_record(&mut self) -> Result<Record<Self::LenPayloadType>>;
}
impl<'a> ReadExtRecord for &'a [u8] {
    type LenPayloadType = &'a [u8];
    fn read_record(&mut self) -> Result<Record<Self::LenPayloadType>> {
        use crate::internal::variant::ReadExtVariant;
        let tag = self.read_variant()?.try_as_uint32()?;
        let wire_type: WireType = (tag & 0x7).try_into()?;
        let number = tag >> 3;
        let payload = match wire_type {
            WireType::Variant => Payload::Variant(self.read_variant()?),
            WireType::I32 => {
                let Some((chunk, remain)) = self.split_first_chunk::<4>() else {
                    Err(ErrorKind::DeserUnexpectedEof)?
                };
                *self = remain;
                Payload::I32(chunk.clone())
            }
            WireType::I64 => {
                let Some((chunk, remain)) = self.split_first_chunk::<8>() else {
                    Err(ErrorKind::DeserUnexpectedEof)?
                };
                *self = remain;
                Payload::I64(chunk.clone())
            }
            WireType::Len => {
                let length: usize = self.read_variant()?.try_as_int32()?.try_into()?;
                let Some((chunk, remain)) = self.try_split_at(length) else {
                    Err(ErrorKind::DeserUnexpectedEof)?
                };
                *self = remain;
                Payload::Len(chunk)
            }
        };
        Ok(Record { number, payload })
    }
}

pub trait DeseringMessage {
    fn finish(&mut self) -> Result<()>;
    fn deser_record<'a, 'b>(
        &'a mut self,
        record: Record<&'b [u8]>,
    ) -> Result<Option<(&'a mut dyn DeseringMessage, &'b [u8])>>;
}
pub trait AsyncDeseringMessage {
    fn finish(&mut self) -> Result<()>;
}

pub fn deser_from_slice(root_msg: &mut dyn DeseringMessage, mut input: &[u8]) -> Result<()> {
    let mut stack = Vec::new();
    stack.push((root_msg, input));
    while let Some((msg, mut remain)) = stack.pop() {
        if remain.is_empty() {
            msg.finish()?;
            continue;
        }
        let record = remain.read_record()?;
        if let Some((child_message, child_record)) = msg.deser_record(record)? {
            stack.push((child_message, child_record));
        };
        stack.push((msg, remain));
    }
    todo!()
}

trait SliceExt<T> {
    fn try_split_at(&self, at: usize) -> Option<(&[T], &[T])>;
}
impl<T> SliceExt<T> for [T] {
    fn try_split_at(&self, at: usize) -> Option<(&[T], &[T])> {
        if at <= self.len() {
            Some(self.split_at(at))
        } else {
            None
        }
    }
}
