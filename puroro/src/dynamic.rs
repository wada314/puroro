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

pub mod fields;
pub mod payload;

pub use self::fields::FieldReducingErrorStrategy;

use self::fields::DynamicField;
use self::payload::{DynamicLenPayload, WireTypeAndPayload};
use crate::internal::deser::{
    deser_from_bufread, DeserMessageHandlerBase, DeserMessageHandlerForRead,
};
use crate::message::{Message, MessageMut};
use crate::variant::variant_types::{Int32, UInt32};
use crate::variant::{Variant, VariantIntegerType, WriteExtVariant};
use crate::Result;
use ::hashbrown::hash_map::Entry;
use ::hashbrown::DefaultHashBuilder;
use ::hashbrown::HashMap;
use ::std::alloc::{Allocator, Global};
use ::std::fmt::Debug;
use ::std::io::{BufRead, Read, Write};
use ::std::vec::Vec;

/// A Message implementation which is expected to be used in the situation where
/// the protocol buffer schema is not known at compile time.
///
/// This implementation's interface is designed to be wrapped by a higher-level
/// interface which knows about the schema, so it misses some of the common methods:
/// - No "Unknown field"s. By definition, unknown fields requires the schema.
/// - TBD
#[derive(Default, Clone)]
pub struct DynamicMessage<A: Allocator = Global> {
    fields: HashMap<i32, DynamicField<A>, DefaultHashBuilder, A>,
}

impl DynamicMessage<Global> {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }
}

impl<A: Allocator> DynamicMessage<A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            fields: HashMap::new_in(alloc),
        }
    }

    pub fn field(&self, number: i32) -> Option<&DynamicField<A>> {
        self.fields.get(&number)
    }

    pub fn allocator(&self) -> &A {
        self.fields.allocator()
    }
}

impl<A: Allocator + Clone> DynamicMessage<A> {
    pub fn field_mut(&mut self, number: i32) -> &mut DynamicField<A> {
        let alloc = self.allocator().clone();
        self.fields
            .entry(number)
            .or_insert_with(|| DynamicField::default_in(alloc))
    }

    pub fn merge(&mut self, other: DynamicMessage<A>) {
        for (number, other_field) in other.fields {
            match self.fields.entry(number) {
                Entry::Occupied(mut entry) => {
                    let payloads = entry.get_mut().as_payloads_mut();
                    payloads.extend(other_field.into_payloads().into_iter());
                }
                Entry::Vacant(entry) => {
                    entry.insert(other_field);
                }
            }
        }
    }

    fn write_to_vec<A2: Allocator>(&self, buf: &mut Vec<u8, A2>) -> usize {
        // Writing to a Vec<u8> is always successful.
        unsafe { self.write(buf).unwrap_unchecked() }
    }
}

impl<A: Allocator> Debug for DynamicMessage<A> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut ds = f.debug_struct("DynamicMessage");
        for (number, field) in &self.fields {
            ds.field(&format!("field{}", number), field);
        }
        ds.finish()
    }
}

impl<A: Allocator + Clone> Message for DynamicMessage<A> {
    fn write<W: Write>(&self, mut write: W) -> Result<usize> {
        let mut total_bytes = 0;
        for (number, field) in &self.fields {
            for wire_and_payload in field.as_payloads() {
                let tag = (TryInto::<u32>::try_into(*number)? << 3)
                    | Into::<u32>::into(wire_and_payload.wire_type());
                total_bytes += write.write_variant(UInt32::into_variant(tag))?;
                total_bytes += match wire_and_payload {
                    WireTypeAndPayload::Variant(variant) => write.write_variant(variant.clone())?,
                    WireTypeAndPayload::I64(buf) => {
                        write.write_all(buf)?;
                        4usize
                    }
                    WireTypeAndPayload::I32(buf) => {
                        write.write_all(buf)?;
                        8usize
                    }
                    WireTypeAndPayload::Len(len_payload) => {
                        let bytes = len_payload.as_buf();
                        let len = bytes.len();
                        write.write_variant(Variant::from::<Int32>(len.try_into()?))?;
                        write.write_all(bytes)?;
                        len
                    }
                };
            }
        }
        Ok(total_bytes)
    }
}

impl<A: Allocator + Clone> MessageMut<A> for DynamicMessage<A> {
    fn merge_from_bufread<R: BufRead>(&mut self, read: R) -> Result<()> {
        deser_from_bufread(read, self)
    }
}

impl<A: Allocator + Clone> DeserMessageHandlerBase for DynamicMessage<A> {
    fn parse_variant(&mut self, num: i32, var: Variant) -> Result<()> {
        self.field_mut(num).push_variant(var);
        Ok(())
    }
    fn parse_i32(&mut self, num: i32, val: [u8; 4]) -> Result<()> {
        self.field_mut(num).push_i32(val);
        Ok(())
    }
    fn parse_i64(&mut self, num: i32, val: [u8; 8]) -> Result<()> {
        self.field_mut(num).push_i64(val);
        Ok(())
    }
    fn is_message_field(&self, _num: i32) -> bool {
        false
    }
    fn start_message(&mut self, _num: i32) -> Result<()> {
        // Every message fields are deserialized as a bytes initially,
        // and then might be deserialized on demand.
        unreachable!()
    }
    fn end_message(&mut self) -> Result<()> {
        unreachable!()
    }
}

impl<A: Allocator + Clone, R: Read> DeserMessageHandlerForRead<R> for DynamicMessage<A> {
    fn parse_len(&mut self, num: i32, read: &mut R) -> Result<usize> {
        let alloc = self.allocator().clone();
        // Facepalm
        let (buf, len) = {
            let mut std_buf = ::std::vec::Vec::new();
            let len = read.read_to_end(&mut std_buf)?;
            let mut buf = Vec::with_capacity_in(len, A::clone(&alloc));
            buf.extend_from_slice(&std_buf);
            (buf, len)
        };
        self.field_mut(num)
            .as_payloads_mut()
            .push(WireTypeAndPayload::Len(DynamicLenPayload::from_buf(buf)));
        Ok(len)
    }
}
