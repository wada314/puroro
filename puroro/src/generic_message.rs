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

use crate::internal::deser::record::{Payload, Record};
use crate::internal::deser::{
    deser_from_bufread, DeserMessageHandlerBase, DeserMessageHandlerForRead,
};
use crate::internal::WireType;
use crate::message::MessageLite;
use crate::variant::{
    variant_types::UInt32, ReadExtVariant, Variant, VariantIntegerType, WriteExtVariant,
};
use crate::{ErrorKind, Result};
use ::hashbrown::hash_map::DefaultHashBuilder;
use ::hashbrown::HashMap as HashMap2;
use ::itertools::Either;
use ::std::alloc::{Allocator, Global};
use ::std::borrow::Cow;
use ::std::cell::OnceCell;
use ::std::collections::HashMap;
use ::std::io::{BufRead, Read, Write};

/// Assuming proto2 syntax.
#[derive(Clone, Debug, Default)]
pub struct GenericMessage<'a> {
    fields: HashMap<i32, Vec<WireTypeAndPayload<'a>>>,
}

#[derive(Debug, Clone)]
pub enum WireTypeAndPayload<'a> {
    Variant(Variant),
    I64([u8; 8]),
    I32([u8; 4]),
    Len(Cow<'a, [u8]>),
    // StartGroup,
    // EndGroup,
}
impl WireTypeAndPayload<'_> {
    pub(crate) fn wire_type(&self) -> WireType {
        match self {
            WireTypeAndPayload::Variant(_) => WireType::Variant,
            WireTypeAndPayload::I64(_) => WireType::I64,
            WireTypeAndPayload::I32(_) => WireType::I32,
            WireTypeAndPayload::Len(_) => WireType::Len,
        }
    }
}

impl<'a> GenericMessage<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_buffer(buf: &'a [u8]) -> Result<Self> {
        let mut message = GenericMessage::default();
        message.merge_from_buffer(buf)?;
        Ok(message)
    }

    pub fn merge_from_buffer(&mut self, buf: &'a [u8]) -> Result<()> {
        deser_from_bufread(buf, self)?;
        Ok(())
    }

    pub fn field(&self, number: i32) -> Field {
        self.fields.get(&number).map_or_else(
            || Field {
                number,
                wire_and_payloads: &[],
            },
            |wire_and_payloads| Field {
                number,
                wire_and_payloads,
            },
        )
    }

    pub fn field_mut(&mut self, number: i32) -> FieldMut<'_, 'a> {
        FieldMut {
            number,
            wire_and_payloads: self.fields.entry(number).or_default(),
        }
    }

    pub fn fields(&self) -> impl '_ + Iterator<Item = Field> {
        self.fields.iter().map(|(number, wire_and_payloads)| Field {
            number: *number,
            wire_and_payloads,
        })
    }

    pub fn payloads_for_field(&self, number: i32) -> Option<&[WireTypeAndPayload]> {
        self.fields.get(&number).map(|v| v.as_slice())
    }
    pub fn payloads_for_field_mut<'this>(
        &'this mut self,
        number: i32,
    ) -> &'this mut Vec<WireTypeAndPayload<'a>> {
        self.fields.entry(number).or_default()
    }
}

impl MessageLite for GenericMessage<'_> {
    fn merge_from_bufread<R: BufRead>(&mut self, read: R) -> Result<()> {
        deser_from_bufread(read, self)
    }
    fn write<W: Write>(&self, mut write: W) -> Result<usize> {
        let mut total_bytes = 0;
        for (number, wire_and_payloads) in &self.fields {
            for wire_and_payload in wire_and_payloads {
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
                    WireTypeAndPayload::Len(ld) => {
                        let len_len =
                            write.write_variant(UInt32::into_variant(ld.len().try_into()?))?;
                        write.write_all(ld)?;
                        len_len + ld.len()
                    }
                };
            }
        }
        Ok(total_bytes)
    }
}

#[derive(Debug, Clone)]
pub struct Field<'a> {
    number: i32,
    wire_and_payloads: &'a [WireTypeAndPayload<'a>],
}

impl<'a> Field<'a> {
    pub fn number(&self) -> i32 {
        self.number
    }

    pub fn as_scalar_variant<T: VariantIntegerType>(&self, allow_packed: bool) -> T::RustType {
        self.as_repeated_variant::<T>(allow_packed)
            .last()
            .unwrap_or_default()
    }
    pub fn as_repeated_variant<T: VariantIntegerType>(
        &self,
        allow_packed: bool,
    ) -> impl 'a + Iterator<Item = T::RustType> {
        self.try_as_repeated_variant::<T>(allow_packed)
            .filter_map(Result::ok)
    }
    pub fn as_scalar_i32(&self) -> [u8; 4] {
        self.as_repeated_i32().last().unwrap_or_default()
    }
    pub fn as_repeated_i32(&self) -> impl 'a + Iterator<Item = [u8; 4]> {
        self.try_as_repeated_i32().filter_map(Result::ok)
    }
    pub fn as_scalar_i64(&self) -> [u8; 8] {
        self.as_repeated_i64().last().unwrap_or_default()
    }
    pub fn as_repeated_i64(&self) -> impl 'a + Iterator<Item = [u8; 8]> {
        self.try_as_repeated_i64().filter_map(Result::ok)
    }
    pub fn as_scalar_string(&self) -> &'a str {
        self.as_repeated_string().last().unwrap_or_default()
    }
    pub fn as_repeated_string(&self) -> impl 'a + Iterator<Item = &'a str> {
        self.try_as_repeated_string().filter_map(Result::ok)
    }
    pub fn as_scalar_bytes(&self) -> &'a [u8] {
        self.as_repeated_bytes().last().unwrap_or_default()
    }
    pub fn as_repeated_bytes(&self) -> impl 'a + Iterator<Item = &'a [u8]> {
        self.try_as_repeated_bytes().filter_map(Result::ok)
    }
    pub fn as_scalar_message(&self) -> Option<GenericMessage<'a>> {
        self.as_repeated_message().last()
    }
    pub fn as_repeated_message(&self) -> impl 'a + Iterator<Item = GenericMessage<'a>> {
        self.try_as_repeated_message().filter_map(Result::ok)
    }

    pub fn try_as_scalar_variant_opt<T: VariantIntegerType>(
        &self,
        allow_packed: bool,
    ) -> Result<Option<T::RustType>> {
        self.try_as_repeated_variant::<T>(allow_packed)
            .last()
            .transpose()
    }
    pub fn try_as_repeated_variant<T: VariantIntegerType>(
        &self,
        allow_packed: bool,
    ) -> impl 'a + Iterator<Item = Result<T::RustType>> {
        self.wire_and_payloads
            .iter()
            .flat_map(move |record| match (allow_packed, record) {
                (_, WireTypeAndPayload::Variant(variant)) => {
                    Either::Left(Some(Ok(variant.clone())).into_iter())
                }
                (true, WireTypeAndPayload::Len(ld)) => Either::Right(ld.into_variant_iter()),
                _ => Either::Left(Some(Err(ErrorKind::GenericMessageFieldTypeError)).into_iter()),
            })
            .map(|rv| rv.and_then(T::try_from_variant))
    }
    pub fn try_as_scalar_i32_opt(&self) -> Result<Option<[u8; 4]>> {
        self.try_as_repeated_i32().last().transpose()
    }
    pub fn try_as_repeated_i32(&self) -> impl 'a + Iterator<Item = Result<[u8; 4]>> {
        self.wire_and_payloads.iter().map(|record| match record {
            WireTypeAndPayload::I32(buf) => Ok(*buf),
            _ => Err(ErrorKind::GenericMessageFieldTypeError),
        })
    }
    pub fn try_as_scalar_i64_opt(&self) -> Result<Option<[u8; 8]>> {
        self.try_as_repeated_i64().last().transpose()
    }
    pub fn try_as_repeated_i64(&self) -> impl 'a + Iterator<Item = Result<[u8; 8]>> {
        self.wire_and_payloads.iter().map(|record| match record {
            WireTypeAndPayload::I64(buf) => Ok(*buf),
            _ => Err(ErrorKind::GenericMessageFieldTypeError),
        })
    }
    pub fn try_as_scalar_string_opt(&self) -> Result<Option<&'a str>> {
        self.try_as_repeated_string().last().transpose()
    }
    pub fn try_as_repeated_string(&self) -> impl 'a + Iterator<Item = Result<&'a str>> {
        self.wire_and_payloads.iter().map(|record| match record {
            WireTypeAndPayload::Len(ld) => {
                ::std::str::from_utf8(&ld).map_err(|_| ErrorKind::GenericMessageFieldTypeError)
            }
            _ => Err(ErrorKind::GenericMessageFieldTypeError),
        })
    }
    pub fn try_as_scalar_bytes_opt(&self) -> Result<Option<&'a [u8]>> {
        self.try_as_repeated_bytes().last().transpose()
    }
    pub fn try_as_repeated_bytes(&self) -> impl 'a + Iterator<Item = Result<&'a [u8]>> {
        self.wire_and_payloads.iter().map(|record| match record {
            WireTypeAndPayload::Len(ld) => Ok(ld.as_ref()),
            _ => Err(ErrorKind::GenericMessageFieldTypeError),
        })
    }
    pub fn try_as_scalar_message(&self) -> Result<Option<GenericMessage<'a>>> {
        let mut message_opt: Option<GenericMessage> = None;
        for wire_and_payload in self.wire_and_payloads {
            let WireTypeAndPayload::Len(buf) = wire_and_payload else {
                Err(ErrorKind::GenericMessageFieldTypeError)?
            };
            message_opt
                .get_or_insert_with(GenericMessage::default)
                .merge_from_buffer(buf)?;
        }
        Ok(message_opt)
    }
    pub fn try_as_repeated_message(&self) -> impl 'a + Iterator<Item = Result<GenericMessage<'a>>> {
        self.wire_and_payloads.iter().map(|wire_and_payload| {
            let WireTypeAndPayload::Len(buf) = wire_and_payload else {
                Err(ErrorKind::GenericMessageFieldTypeError)?
            };
            GenericMessage::from_buffer(buf)
        })
    }
}

#[derive(Debug)]
pub struct FieldMut<'msg, 'a> {
    number: i32,
    wire_and_payloads: &'msg mut Vec<WireTypeAndPayload<'a>>,
}

impl<'msg, 'a> FieldMut<'msg, 'a> {
    pub fn number(&self) -> i32 {
        self.number
    }

    pub fn set_variant<T: VariantIntegerType>(&mut self, val: T::RustType) {
        self.wire_and_payloads.clear();
        self.push_variant::<T>(val);
    }
    pub fn set_i32(&mut self, val: [u8; 4]) {
        self.wire_and_payloads.clear();
        self.push_i32(val);
    }
    pub fn set_i64(&mut self, val: [u8; 8]) {
        self.wire_and_payloads.clear();
        self.push_i64(val);
    }
    pub fn set_string(&mut self, val: &str) {
        self.wire_and_payloads.clear();
        self.push_string(val);
    }
    pub fn set_bytes(&mut self, val: &'a [u8]) {
        self.wire_and_payloads.clear();
        self.push_bytes(val);
    }
    pub fn set_message(&mut self, message: &GenericMessage<'a>) {
        self.wire_and_payloads.clear();
        self.push_message(message);
    }

    pub fn push_variant<T: VariantIntegerType>(&mut self, val: T::RustType) {
        self.wire_and_payloads
            .push(WireTypeAndPayload::Variant(Variant::from::<T>(val)));
    }
    pub fn push_i32(&mut self, val: [u8; 4]) {
        self.wire_and_payloads.push(WireTypeAndPayload::I32(val));
    }
    pub fn push_i64(&mut self, val: [u8; 8]) {
        self.wire_and_payloads.push(WireTypeAndPayload::I64(val));
    }
    pub fn push_string(&mut self, val: &str) {
        self.wire_and_payloads
            .push(WireTypeAndPayload::Len(val.to_string().into_bytes().into()));
    }
    pub fn push_bytes(&mut self, val: &'a [u8]) {
        self.wire_and_payloads
            .push(WireTypeAndPayload::Len(val.into()));
    }
    pub fn push_message(&mut self, message: &GenericMessage) {
        let mut buf = Vec::new();
        if let Ok(_) = message.write(&mut buf) {
            self.wire_and_payloads
                .push(WireTypeAndPayload::Len(buf.into()));
        }
    }
}

impl<'a, T> From<Record<T>> for WireTypeAndPayload<'a>
where
    T: 'a,
    Cow<'a, [u8]>: From<T>,
{
    fn from(record: Record<T>) -> Self {
        match record.payload {
            Payload::Variant(variant) => WireTypeAndPayload::Variant(variant),
            Payload::I64(buf) => WireTypeAndPayload::I64(buf),
            Payload::I32(buf) => WireTypeAndPayload::I32(buf),
            Payload::Len(buf) => WireTypeAndPayload::Len(buf.into()),
        }
    }
}

impl DeserMessageHandlerBase for GenericMessage<'_> {
    fn start_message(&mut self, #[allow(unused)] num: i32) -> Result<()> {
        unimplemented!()
    }
    fn end_message(&mut self) -> Result<()> {
        unimplemented!()
    }
    fn parse_variant(&mut self, num: i32, var: Variant) -> Result<()> {
        self.payloads_for_field_mut(num)
            .push(WireTypeAndPayload::Variant(var));
        Ok(())
    }
    fn parse_i32(&mut self, num: i32, val: [u8; 4]) -> Result<()> {
        self.payloads_for_field_mut(num)
            .push(WireTypeAndPayload::I32(val));
        Ok(())
    }
    fn parse_i64(&mut self, num: i32, val: [u8; 8]) -> Result<()> {
        self.payloads_for_field_mut(num)
            .push(WireTypeAndPayload::I64(val));
        Ok(())
    }
    fn is_message_field(&self, #[allow(unused)] num: i32) -> bool {
        false
    }
}
impl<R: Read> DeserMessageHandlerForRead<R> for GenericMessage<'_> {
    fn parse_len(&mut self, num: i32, val: &mut R) -> Result<usize> {
        let mut buf = Vec::new();
        let len = val.read_to_end(&mut buf)?;
        self.payloads_for_field_mut(num)
            .push(WireTypeAndPayload::Len(buf.into()));
        Ok(len)
    }
}

#[derive(Default)]
pub struct GenericMessage2<A: Allocator = Global> {
    fields: HashMap2<i32, Vec<WireTypeAndPayload2<A>, A>, DefaultHashBuilder, A>,
    alloc: A,
}
pub enum WireTypeAndPayload2<A: Allocator = Global> {
    Variant(Variant),
    I64([u8; 8]),
    I32([u8; 4]),
    Len(InnerMutableBytesOrMsg<A>),
}
pub struct InnerMutableBytesOrMsg<A: Allocator> {
    bytes_cell: OnceCell<Vec<u8, A>>,
    msg_cell: OnceCell<GenericMessage2<A>>,
}
impl<A: Allocator> WireTypeAndPayload2<A> {
    pub(crate) fn wire_type(&self) -> WireType {
        match self {
            WireTypeAndPayload2::Variant(_) => WireType::Variant,
            WireTypeAndPayload2::I64(_) => WireType::I64,
            WireTypeAndPayload2::I32(_) => WireType::I32,
            WireTypeAndPayload2::Len(_) => WireType::Len,
        }
    }
}

impl GenericMessage2<Global> {
    pub fn new() -> Self {
        Self {
            fields: HashMap2::new(),
            alloc: Global,
        }
    }
}
impl<A: Allocator + Clone> GenericMessage2<A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            fields: HashMap2::new_in(alloc.clone()),
            alloc: alloc.clone(),
        }
    }
    pub fn field_mut(&mut self, number: i32) -> FieldMut2<A> {
        FieldMut2(
            self.fields
                .entry(number)
                .or_insert_with(|| Vec::new_in(self.alloc.clone())),
        )
    }
}
impl<A: Allocator> GenericMessage2<A> {
    pub fn field(&self, number: i32) -> FieldRef2<A> {
        FieldRef2(
            self.fields
                .get(&number)
                .map(Vec::as_slice)
                .unwrap_or_default(),
        )
    }
}

impl<A: Allocator + Clone> MessageLite for GenericMessage2<A> {
    fn merge_from_bufread<R: BufRead>(&mut self, read: R) -> Result<()> {
        deser_from_bufread(read, self)
    }

    fn write<W: Write>(&self, mut write: W) -> Result<usize> {
        let mut total_bytes = 0;
        for (number, wire_and_payloads) in &self.fields {
            for wire_and_payload in wire_and_payloads {
                let tag = (TryInto::<u32>::try_into(*number)? << 3)
                    | Into::<u32>::into(wire_and_payload.wire_type());
                total_bytes += write.write_variant(UInt32::into_variant(tag))?;
                total_bytes += match wire_and_payload {
                    WireTypeAndPayload2::Variant(variant) => {
                        write.write_variant(variant.clone())?
                    }
                    WireTypeAndPayload2::I64(buf) => {
                        write.write_all(buf)?;
                        4usize
                    }
                    WireTypeAndPayload2::I32(buf) => {
                        write.write_all(buf)?;
                        8usize
                    }
                    WireTypeAndPayload2::Len(bytes_or_msg) => {
                        // If bytes buffer is vacant, generate it from msg.
                        let bytes = bytes_or_msg.as_bytes()?;
                        let len = bytes.len();
                        write.write_all(bytes)?;
                        len
                    }
                };
            }
        }
        Ok(total_bytes)
    }
}

impl<A: Allocator + Clone> DeserMessageHandlerBase for GenericMessage2<A> {
    fn parse_variant(&mut self, num: i32, var: Variant) -> Result<()> {
        self.field_mut(num)
            .0
            .push(WireTypeAndPayload2::Variant(var));
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
impl<A: Allocator + Clone, R: Read> DeserMessageHandlerForRead<R> for GenericMessage2<A> {
    fn parse_len(&mut self, num: i32, read: &mut R) -> Result<usize> {
        let alloc = self.alloc.clone();
        // Facepalm
        let mut buf = Vec::new();
        let len = read.read_to_end(&mut buf)?;
        self.field_mut(num).0.push(WireTypeAndPayload2::Len(
            InnerMutableBytesOrMsg::from_bytes(buf.as_slice().to_vec_in(alloc)),
        ));
        Ok(len)
    }
}

impl<A: Allocator + Clone> InnerMutableBytesOrMsg<A> {
    pub fn from_bytes(bytes: Vec<u8, A>) -> Self {
        let s = Self {
            bytes_cell: OnceCell::new(),
            msg_cell: OnceCell::new(),
        };
        s.bytes_cell.set(bytes);
        s
    }
    pub fn from_msg(msg: GenericMessage2<A>) -> Self {
        let s = Self {
            bytes_cell: OnceCell::new(),
            msg_cell: OnceCell::new(),
        };
        s.msg_cell.set(msg);
        s
    }

    pub fn as_bytes(&self) -> Result<&[u8]> {
        let v = self.bytes_cell.get_or_try_init(|| -> Result<_> {
            let Some(msg) = self.msg_cell.get() else {
                unreachable!("either bytes_cell or msg_cell must be initialized");
            };
            let mut buf = Vec::new_in(msg.alloc.clone());
            msg.write(&mut buf)?;
            Ok(buf)
        })?;
        Ok(v)
    }
    pub fn as_msg(&self) -> Result<&GenericMessage2<A>> {
        self.msg_cell.get_or_try_init(|| -> Result<_> {
            let Some(bytes) = self.bytes_cell.get() else {
                unreachable!("either bytes_cell or msg_cell must be initialized");
            };
            let mut msg = GenericMessage2::new_in(bytes.allocator().clone());
            msg.merge_from_read(&mut bytes.as_slice())?;
            Ok(msg)
        })
    }
    pub fn as_bytes_mut(&mut self) -> Result<&mut Vec<u8, A>> {
        // Make sure the bytes_cell is initialized.
        self.as_bytes()?;
        // Dispose msg_cell.
        self.msg_cell.take();
        let Some(mut_bytes) = self.bytes_cell.get_mut() else {
            unreachable!("bytes_cell must be already initialized in as_bytes() call");
        };
        Ok(mut_bytes)
    }
    pub fn as_msg_mut(&mut self) -> Result<&mut GenericMessage2<A>> {
        // Make sure the msg_cell is initialized.
        self.as_msg()?;
        // Dispose bytes_cell.
        self.bytes_cell.take();
        let Some(mut_msg) = self.msg_cell.get_mut() else {
            unreachable!("msg_cell must be already initialized in as_msg() call");
        };
        Ok(mut_msg)
    }
}

pub struct FieldRef2<'a, A: Allocator = Global>(&'a [WireTypeAndPayload2<A>]);
pub struct FieldMut2<'a, A: Allocator = Global>(&'a mut Vec<WireTypeAndPayload2<A>, A>);

impl<'a, A: Allocator + Clone> FieldRef2<'a, A> {
    pub fn as_scalar_variant<T: VariantIntegerType>(&self, allow_packed: bool) -> T::RustType {
        self.as_repeated_variant::<T>(allow_packed)
            .last()
            .unwrap_or_default()
    }
    pub fn as_repeated_variant<T: VariantIntegerType>(
        &self,
        allow_packed: bool,
    ) -> impl 'a + Iterator<Item = T::RustType> {
        self.try_as_repeated_variant::<T>(allow_packed)
            .filter_map(Result::ok)
    }
    pub fn as_scalar_i32(&self) -> [u8; 4] {
        self.as_repeated_i32().last().unwrap_or_default()
    }
    pub fn as_repeated_i32(&self) -> impl 'a + Iterator<Item = [u8; 4]> {
        self.try_as_repeated_i32().filter_map(Result::ok)
    }
    pub fn as_scalar_i64(&self) -> [u8; 8] {
        self.as_repeated_i64().last().unwrap_or_default()
    }
    pub fn as_repeated_i64(&self) -> impl 'a + Iterator<Item = [u8; 8]> {
        self.try_as_repeated_i64().filter_map(Result::ok)
    }
    pub fn as_scalar_string(&self) -> &'a str {
        self.as_repeated_string().last().unwrap_or_default()
    }
    pub fn as_repeated_string(&self) -> impl 'a + Iterator<Item = &'a str> {
        self.try_as_repeated_string().filter_map(Result::ok)
    }
    pub fn as_scalar_bytes(&self) -> &'a [u8] {
        self.as_repeated_bytes().last().unwrap_or_default()
    }
    pub fn as_repeated_bytes(&self) -> impl 'a + Iterator<Item = &'a [u8]> {
        self.try_as_repeated_bytes().filter_map(Result::ok)
    }
    pub fn as_scalar_message(&self) -> Option<&'a GenericMessage2<A>> {
        self.as_repeated_message().last()
    }
    pub fn as_repeated_message(&self) -> impl 'a + Iterator<Item = &'a GenericMessage2<A>> {
        self.try_as_repeated_message().filter_map(Result::ok)
    }
    pub fn try_as_scalar_variant_opt<T: VariantIntegerType>(
        &self,
        allow_packed: bool,
    ) -> Result<Option<T::RustType>> {
        self.try_as_repeated_variant::<T>(allow_packed)
            .last()
            .transpose()
    }
    pub fn try_as_repeated_variant<T: VariantIntegerType>(
        &self,
        allow_packed: bool,
    ) -> impl 'a + Iterator<Item = Result<T::RustType>> {
        self.0
            .iter()
            .flat_map(move |record| match (allow_packed, record) {
                (_, WireTypeAndPayload2::Variant(variant)) => {
                    Either::Left(Some(Ok(variant.clone())).into_iter())
                }
                (true, WireTypeAndPayload2::Len(bytes_or_msg)) => match bytes_or_msg.as_bytes() {
                    Ok(bytes) => Either::Right(bytes.into_variant_iter()),
                    Err(e) => Either::Left(Some(Err(e)).into_iter()),
                },
                _ => Either::Left(Some(Err(ErrorKind::GenericMessageFieldTypeError)).into_iter()),
            })
            .map(|rv| rv.and_then(T::try_from_variant))
    }
    pub fn try_as_scalar_i32_opt(&self) -> Result<Option<[u8; 4]>> {
        self.try_as_repeated_i32().last().transpose()
    }
    pub fn try_as_repeated_i32(&self) -> impl 'a + Iterator<Item = Result<[u8; 4]>> {
        self.0.iter().map(|record| match record {
            WireTypeAndPayload2::I32(buf) => Ok(*buf),
            _ => Err(ErrorKind::GenericMessageFieldTypeError),
        })
    }
    pub fn try_as_scalar_i64_opt(&self) -> Result<Option<[u8; 8]>> {
        self.try_as_repeated_i64().last().transpose()
    }
    pub fn try_as_repeated_i64(&self) -> impl 'a + Iterator<Item = Result<[u8; 8]>> {
        self.0.iter().map(|record| match record {
            WireTypeAndPayload2::I64(buf) => Ok(*buf),
            _ => Err(ErrorKind::GenericMessageFieldTypeError),
        })
    }
    pub fn try_as_scalar_string_opt(&self) -> Result<Option<&'a str>> {
        self.try_as_repeated_string().last().transpose()
    }
    pub fn try_as_repeated_string(&self) -> impl 'a + Iterator<Item = Result<&'a str>> {
        self.0.iter().map(|record| match record {
            WireTypeAndPayload2::Len(bytes_or_msg) => bytes_or_msg
                .as_bytes()?
                .map(|bytes| ::std::str::from_utf8(bytes)),
            _ => Err(ErrorKind::GenericMessageFieldTypeError),
        })
    }
    pub fn try_as_scalar_bytes_opt(&self) -> Result<Option<&'a [u8]>> {
        self.try_as_repeated_bytes().last().transpose()
    }
    pub fn try_as_repeated_bytes(&self) -> impl 'a + Iterator<Item = Result<&'a [u8]>> {
        self.0.iter().map(|record| match record {
            WireTypeAndPayload2::Len(bytes_or_msg) => Ok(bytes_or_msg.as_bytes()?),
            _ => Err(ErrorKind::GenericMessageFieldTypeError),
        })
    }
    pub fn try_as_scalar_message(&self) -> Result<Option<GenericMessage2<A>>> {
        let mut message_opt = None;
        for wire_and_payload in self.0 {
            let WireTypeAndPayload2::Len(bytes_or_msg) = wire_and_payload else {
                Err(ErrorKind::GenericMessageFieldTypeError)?
            };
            let msg = bytes_or_msg.as_msg()?;
            message_opt
                .get_or_insert_with(|| GenericMessage2::new_in(msg.alloc.clone()))
                .merge()?;
        }
        Ok(message_opt)
    }
    pub fn try_as_repeated_message(&self) -> impl Iterator<Item = Result<&'a GenericMessage2<A>>> {
        self.0.iter().map(|wire_and_payload| {
            let WireTypeAndPayload2::Len(buf) = wire_and_payload else {
                Err(ErrorKind::GenericMessageFieldTypeError)?
            };
            GenericMessage2::from_buffer(buf.as_ref())
        })
    }
}

impl<A: Allocator + Clone> FieldMut2<'_, A> {
    pub fn clear(&mut self) {
        self.0.clear();
    }
    pub fn push_variant<T: VariantIntegerType>(&mut self, val: T::RustType) {
        self.0
            .push(WireTypeAndPayload2::Variant(Variant::from::<T>(val)));
    }
    pub fn push_i32(&mut self, val: [u8; 4]) {
        self.0.push(WireTypeAndPayload2::I32(val));
    }
    pub fn push_i64(&mut self, val: [u8; 8]) {
        self.0.push(WireTypeAndPayload2::I64(val));
    }
    pub fn push_string(&mut self, val: &str) {
        self.0.push(WireTypeAndPayload2::Len(
            InnerMutableBytesOrMsg::from_bytes(
                val.as_bytes().to_vec_in(self.0.allocator().clone()),
            ),
        ));
    }
    pub fn push_bytes(&mut self, val: &[u8]) {
        self.0.push(WireTypeAndPayload2::Len(
            InnerMutableBytesOrMsg::from_bytes(val.to_vec_in(self.0.allocator().clone())),
        ));
    }
    pub fn push_message(&mut self, val: GenericMessage2<A>) {
        self.0
            .push(WireTypeAndPayload2::Len(InnerMutableBytesOrMsg::from_msg(
                val,
            )));
    }
}
