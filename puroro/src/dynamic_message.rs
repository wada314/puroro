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

use crate::internal::deser::{
    deser_from_bufread, DeserMessageHandlerBase, DeserMessageHandlerForRead,
};
use crate::internal::WireType;
use crate::message::{Field, GFResult, Message, MessageMut};
use crate::repeated::RepeatedView;
use crate::variant::variant_types::Int32;
use crate::variant::{
    variant_types::UInt32, ReadExtVariant, Variant, VariantIntegerType, WriteExtVariant,
};
use crate::{ErrorKind, Result};
use ::allocator_api2::alloc::{Allocator, Global};
use ::allocator_api2::vec::Vec;
use ::cached_pair::Pair;
use ::derive_more::{Debug, Deref, DerefMut, TryUnwrap};
use ::hashbrown::hash_map::{DefaultHashBuilder, Entry};
use ::hashbrown::HashMap;
use ::itertools::Either;
use ::once_list2::OnceList;
use ::ref_cast::RefCast;
use ::std::io::{BufRead, Read, Write};
use ::std::sync::Once;

#[derive(Default, Clone)]
pub struct DynamicMessage<A: Allocator = Global> {
    fields: HashMap<i32, DynamicField<A>, DefaultHashBuilder, A>,
    alloc: A,
}

#[repr(transparent)]
#[derive(RefCast, Clone, Debug, Deref, DerefMut)]
pub struct DynamicField<A: Allocator = Global>(
    Pair<Vec<WireTypeAndPayload<A>, A>, OnceList1<FieldCustomView<A>, A>>,
);

#[repr(transparent)]
#[derive(RefCast, Clone, Debug, Deref, DerefMut)]
struct DynamicLenPayload<A: Allocator = Global>(
    Pair<Vec<u8, A>, OnceList1<LenCustomPayloadView<A>, A>>,
);

#[derive(Clone, Debug)]
pub enum WireTypeAndPayload<A: Allocator = Global> {
    Variant(Variant),
    I64([u8; 8]),
    I32([u8; 4]),
    Len(DynamicLenPayload<A>),
}
impl<A: Allocator> WireTypeAndPayload<A> {
    pub(crate) fn wire_type(&self) -> WireType {
        match self {
            WireTypeAndPayload::Variant(_) => WireType::Variant,
            WireTypeAndPayload::I64(_) => WireType::I64,
            WireTypeAndPayload::I32(_) => WireType::I32,
            WireTypeAndPayload::Len(_) => WireType::Len,
        }
    }
}

#[derive(Clone, Debug, TryUnwrap)]
#[try_unwrap(ref, ref_mut)]
pub enum LenCustomPayloadView<A: Allocator = Global> {
    Message(DynamicMessage<A>),
}
#[derive(Clone, Debug, TryUnwrap)]
#[try_unwrap(ref, ref_mut)]
pub enum FieldCustomView<A: Allocator = Global> {
    #[debug("{:?}", _0)] // Ignore allocator
    ScalarMessage(Option<DynamicMessage<A>>),
}

impl DynamicMessage<Global> {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            alloc: Global,
        }
    }
}
impl<A: Allocator> DynamicMessage<A> {
    pub fn field(&self, number: i32) -> Option<&DynamicField<A>> {
        self.fields.get(&number)
    }
}
impl<A: Allocator + Clone> DynamicMessage<A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            fields: HashMap::new_in(alloc.clone()),
            alloc: alloc.clone(),
        }
    }

    pub fn field_mut(&mut self, number: i32) -> &mut DynamicField<A> {
        self.fields
            .entry(number)
            .or_insert_with(|| DynamicField::default_in(self.alloc.clone()))
    }

    pub fn merge(&mut self, other: Self) {
        for (number, other_field) in other.fields {
            match self.fields.entry(number) {
                Entry::Occupied(mut entry) => {
                    todo!()
                }
                Entry::Vacant(entry) => {
                    todo!()
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
        for (number, field_base) in &self.fields {
            ds.field(&format!("field{}", number), field_base);
        }
        ds.finish()
    }
}

impl<A: Allocator> Message for DynamicMessage<A> {
    fn write<W: Write>(&self, mut write: W) -> Result<usize> {
        let mut total_bytes = 0;
        for (number, field_base) in &self.fields {
            for wire_and_payload in field_base.as_base() {
                let tag = (TryInto::<u32>::try_into(*number)? << 3)
                    | Into::<u32>::into(wire_and_payload.wire_type());
                total_bytes += write.write_variant(UInt32::into_variant(tag))?;
                total_bytes += match wire_and_payload {
                    WireTypeAndPayload::Variant(variant) => write.write_variant(variant.clone())?,
                    WireTypeAndPayload::I64(buf) => {
                        write.write_all(&buf)?;
                        4usize
                    }
                    WireTypeAndPayload::I32(buf) => {
                        write.write_all(&buf)?;
                        8usize
                    }
                    WireTypeAndPayload::Len(bytes_or_msg) => {
                        let bytes = bytes_or_msg.as_base();
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

    fn field(&self, number: i32) -> impl Field {
        <DynamicMessage<_>>::field(self, number)
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
        let alloc = self.alloc.clone();
        // Facepalm
        let mut buf = Vec::new();
        let len = read.read_to_end(&mut buf)?;
        self.field_mut(num)
            .0
            .as_base_mut()
            .push(WireTypeAndPayload::Len(BaseAndDerived::from_base(
                buf.as_slice().to_vec_in(alloc.clone()),
                alloc,
            )));
        Ok(len)
    }
}

impl<A: Allocator + Clone> DynamicField<A> {
    pub fn as_scalar_variant<T: VariantIntegerType>(&self, allow_packed: bool) -> T::RustType {
        self.as_repeated_variant::<T>(allow_packed)
            .last()
            .unwrap_or_default()
    }
    pub fn as_repeated_variant<T: VariantIntegerType>(
        &self,
        allow_packed: bool,
    ) -> impl '_ + Iterator<Item = T::RustType> {
        self.try_as_repeated_variant::<T>(allow_packed)
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
    }
    pub fn as_scalar_i32(&self) -> [u8; 4] {
        self.as_repeated_i32().last().unwrap_or_default()
    }
    pub fn as_repeated_i32(&self) -> impl '_ + Iterator<Item = [u8; 4]> {
        self.try_as_repeated_i32()
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
    }
    pub fn as_scalar_i64(&self) -> [u8; 8] {
        self.as_repeated_i64().last().unwrap_or_default()
    }
    pub fn as_repeated_i64(&self) -> impl '_ + Iterator<Item = [u8; 8]> {
        self.try_as_repeated_i64()
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
    }
    pub fn as_scalar_string(&self) -> &str {
        self.as_repeated_string().last().unwrap_or_default()
    }
    pub fn as_repeated_string(&self) -> impl '_ + Iterator<Item = &str> {
        self.try_as_repeated_string()
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
    }
    pub fn as_scalar_bytes(&self) -> &[u8] {
        self.as_repeated_bytes().last().unwrap_or_default()
    }
    pub fn as_repeated_bytes(&self) -> impl '_ + Iterator<Item = &[u8]> {
        self.try_as_repeated_bytes()
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
    }
    pub fn as_scalar_message(&self) -> Option<&DynamicMessage<A>> {
        self.try_as_scalar_message().ok().flatten()
    }
    pub fn as_repeated_message(&self) -> impl '_ + Iterator<Item = &DynamicMessage<A>> {
        self.try_as_repeated_message()
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
    }
    pub fn try_as_scalar_variant_opt<T: VariantIntegerType>(
        &self,
        allow_packed: bool,
    ) -> Result<Option<T::RustType>> {
        self.try_as_repeated_variant::<T>(allow_packed)?
            .last()
            .transpose()
    }
    pub fn try_as_repeated_variant<T: VariantIntegerType>(
        &self,
        allow_packed: bool,
    ) -> Result<impl '_ + Iterator<Item = Result<T::RustType>>> {
        Ok(self
            .0
            .as_base()
            .iter()
            .flat_map(move |record| match (allow_packed, record) {
                (_, WireTypeAndPayload::Variant(variant)) => {
                    Either::Left(Some(Ok(variant.clone())).into_iter())
                }
                (true, WireTypeAndPayload::Len(bytes_or_msg)) => {
                    Either::Right(bytes_or_msg.as_base().into_variant_iter())
                }
                _ => Either::Left(Some(Err(ErrorKind::DynamicMessageFieldTypeError)).into_iter()),
            })
            .map(|rv| rv.and_then(T::try_from_variant)))
    }
    pub fn try_as_scalar_i32_opt(&self) -> Result<Option<[u8; 4]>> {
        self.try_as_repeated_i32()?.last().transpose()
    }
    pub fn try_as_repeated_i32(&self) -> Result<impl '_ + Iterator<Item = Result<[u8; 4]>>> {
        Ok(self.0.as_base().iter().map(|record| match record {
            WireTypeAndPayload::I32(buf) => Ok(*buf),
            _ => Err(ErrorKind::DynamicMessageFieldTypeError),
        }))
    }
    pub fn try_as_scalar_i64_opt(&self) -> Result<Option<[u8; 8]>> {
        self.try_as_repeated_i64()?.last().transpose()
    }
    pub fn try_as_repeated_i64(&self) -> Result<impl '_ + Iterator<Item = Result<[u8; 8]>>> {
        Ok(self.0.as_base().iter().map(|record| match record {
            WireTypeAndPayload::I64(buf) => Ok(*buf),
            _ => Err(ErrorKind::DynamicMessageFieldTypeError),
        }))
    }
    pub fn try_as_scalar_string_opt(&self) -> Result<Option<&str>> {
        self.try_as_repeated_string()?.last().transpose()
    }
    pub fn try_as_repeated_string(&self) -> Result<impl '_ + Iterator<Item = Result<&str>>> {
        Ok(self.0.as_base().iter().map(|record| match record {
            WireTypeAndPayload::Len(bytes_or_msg) => {
                Ok(::std::str::from_utf8(bytes_or_msg.as_base())?)
            }
            _ => Err(ErrorKind::DynamicMessageFieldTypeError),
        }))
    }
    pub fn try_as_scalar_bytes_opt(&self) -> Result<Option<&[u8]>> {
        self.try_as_repeated_bytes()?.last().transpose()
    }
    pub fn try_as_repeated_bytes(&self) -> Result<impl '_ + Iterator<Item = Result<&[u8]>>> {
        Ok(self.0.as_base().iter().map(|record| match record {
            WireTypeAndPayload::Len(bytes_or_msg) => Ok(bytes_or_msg.as_base().as_slice()),
            _ => Err(ErrorKind::DynamicMessageFieldTypeError),
        }))
    }
    pub fn try_as_scalar_message(&self) -> Result<Option<&DynamicMessage<A>>> {
        let (opt_msg, _) = self.0.try_as_derived::<(Option<DynamicMessage<A>>, A)>()?;
        Ok(opt_msg.as_ref())
    }
    pub fn try_as_repeated_message(
        &self,
    ) -> Result<impl Iterator<Item = Result<&DynamicMessage<A>>>> {
        Ok(self.0.as_base().iter().map(|wire_and_payload| {
            let WireTypeAndPayload::Len(bytes_or_msg) = wire_and_payload else {
                Err(ErrorKind::DynamicMessageFieldTypeError)?
            };
            Ok(bytes_or_msg.try_as_derived()?)
        }))
    }

    pub fn clear(&mut self) {
        self.0.as_base_mut().clear();
    }

    pub fn push_variant(&mut self, val: Variant) {
        self.0.as_base_mut().push(WireTypeAndPayload::Variant(val));
    }
    pub fn push_variant_from<T: VariantIntegerType>(&mut self, val: T::RustType) {
        self.0
            .as_base_mut()
            .push(WireTypeAndPayload::Variant(Variant::from::<T>(val)));
    }
    pub fn push_i32(&mut self, val: [u8; 4]) {
        self.0.as_base_mut().push(WireTypeAndPayload::I32(val));
    }
    pub fn push_i64(&mut self, val: [u8; 8]) {
        self.0.as_base_mut().push(WireTypeAndPayload::I64(val));
    }
    pub fn push_string(&mut self, val: &str) {
        let alloc = self.allocator().clone();
        self.0
            .as_base_mut()
            .push(WireTypeAndPayload::Len(BaseAndDerived::from_base(
                val.as_bytes().to_vec_in(alloc.clone()),
                alloc.clone(),
            )));
    }
    pub fn push_bytes(&mut self, val: &[u8]) {
        let alloc = self.allocator().clone();
        self.0
            .as_base_mut()
            .push(WireTypeAndPayload::Len(BaseAndDerived::from_base(
                val.to_vec_in(alloc.clone()),
                alloc.clone(),
            )));
    }
    pub fn push_message(&mut self, val: DynamicMessage<A>) {
        let alloc = self.allocator().clone();
        self.0
            .as_base_mut()
            .push(WireTypeAndPayload::Len(BaseAndDerived::from_derived(
                val, alloc,
            )));
    }

    fn default_in(alloc: A) -> Self {
        Self(Pair::from_left(Vec::new_in(alloc)))
    }
}

impl<'a, A: Allocator> Field<'a> for Option<&'a DynamicField<A>> {
    fn has_field(&self) -> GFResult<bool> {
        todo!()
    }
    fn as_scalar_variant(&self) -> GFResult<Variant> {
        todo!()
    }
    fn as_repeated_variant(&self) -> GFResult<impl RepeatedView<Item = Variant>> {
        Ok(::std::iter::empty())
    }
}

#[derive(Clone)]
struct OnceList1<T, A: Allocator>(T, OnceList<T, A>);
impl<T, A: Allocator> OnceList1<T, A> {
    fn first(&self) -> &T {
        &self.0
    }
}

impl<T: ::std::fmt::Debug, A: Allocator> ::std::fmt::Debug for OnceList1<T, A> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_list()
            .entry(&self.0)
            .entries(self.1.iter())
            .finish()
    }
}

impl<A: Allocator + Clone> FieldCustomView<A> {
    fn try_to_field(&self, alloc: &A) -> Result<Vec<WireTypeAndPayload<A>, A>> {
        let encoded_bytes = match self {
            FieldCustomView::ScalarMessage(Some(msg)) => {
                let mut buf = Vec::new_in(A::clone(alloc));
                msg.write_to_vec(&mut buf);
                buf
            }
            FieldCustomView::ScalarMessage(None) => Vec::new_in(A::clone(alloc)),
        };
        let mut payload_vec = Vec::with_capacity_in(1, A::clone(alloc));
        payload_vec.push(WireTypeAndPayload::Len(encoded_bytes.into()));
        Ok(payload_vec)
    }
}

impl<A: Allocator> From<Vec<u8, A>> for DynamicLenPayload<A> {
    fn from(value: Vec<u8, A>) -> Self {
        Self(Pair::from_left(value))
    }
}
