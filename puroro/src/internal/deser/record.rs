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
use crate::internal::WireType;
use crate::{ErrorKind, Result};
use ::std::io::{Read, Take};
use std::default;

#[derive(Debug)]
pub struct Record<T> {
    pub number: u32,
    pub payload: Payload<T>,
}
#[derive(Debug)]
pub enum Payload<T> {
    Variant(Variant),
    I32([u8; 4]),
    I64([u8; 8]),
    Len(T),
}

pub trait SliceExtReadRecord<'a> {
    fn read_record<'b>(&'b mut self) -> Result<Record<&'a [u8]>>;
}
impl<'a> SliceExtReadRecord<'a> for &'a [u8] {
    fn read_record<'b>(&'b mut self) -> Result<Record<&'a [u8]>> {
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

pub trait ReadExtReadRecord: Sized {
    fn read_record(&mut self) -> Result<Record<Take<&mut Self>>>;
}
impl<T: Read> ReadExtReadRecord for T {
    fn read_record(&mut self) -> Result<Record<Take<&mut Self>>> {
        use crate::internal::variant::ReadExtVariant;
        let tag = self.read_variant()?.try_as_uint32()?;
        let wire_type: WireType = (tag & 0x7).try_into()?;
        let number = tag >> 3;
        let payload = match wire_type {
            WireType::Variant => Payload::Variant(self.read_variant()?),
            WireType::I32 => {
                let mut buf = <[u8; 4]>::default();
                self.read_exact(&mut buf)?;
                Payload::I32(buf)
            }
            WireType::I64 => {
                let mut buf = <[u8; 8]>::default();
                self.read_exact(&mut buf)?;
                Payload::I64(buf)
            }
            WireType::Len => {
                let length: u64 = self.read_variant()?.try_as_int32()?.try_into()?;
                Payload::Len(self.take(length))
            }
        };
        Ok(Record { number, payload })
    }
}

trait SliceExtTrySplitAt<T> {
    fn try_split_at(&self, at: usize) -> Option<(&[T], &[T])>;
}
impl<T> SliceExtTrySplitAt<T> for [T] {
    fn try_split_at(&self, at: usize) -> Option<(&[T], &[T])> {
        if at <= self.len() {
            Some(self.split_at(at))
        } else {
            None
        }
    }
}
