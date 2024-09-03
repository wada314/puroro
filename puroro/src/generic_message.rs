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
use crate::internal::utils::{BaseAndDerived, Derived, EnumOfDeriveds, EnumVariant};
use crate::internal::WireType;
use crate::message::MessageLite;
use crate::variant::{
    variant_types::UInt32, ReadExtVariant, Variant, VariantIntegerType, WriteExtVariant,
};
use crate::{ErrorKind, Result};
use ::derive_more::{Debug, TryUnwrap};
use ::hashbrown::hash_map::{DefaultHashBuilder, Entry};
use ::hashbrown::HashMap;
use ::itertools::Either;
use ::ref_cast::RefCast;
use ::std::alloc::{Allocator, Global};
use ::std::io::{BufRead, Read, Write};

#[derive(Default, Clone)]
pub struct GenericMessage<A: Allocator = Global> {
    fields: HashMap<
        i32,
        BaseAndDerived<Vec<WireTypeAndPayload<A>, A>, FieldCustomView<A>, A>,
        DefaultHashBuilder,
        A,
    >,
    alloc: A,
}

#[derive(Clone, Debug)]
pub enum WireTypeAndPayload<A: Allocator = Global> {
    Variant(Variant),
    I64([u8; 8]),
    I32([u8; 4]),
    Len(BaseAndDerived<Vec<u8, A>, LenCustomPayloadView<A>, A>),
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
    Message(GenericMessage<A>),
}
impl<A: Allocator + Clone> EnumOfDeriveds<Vec<u8, A>> for LenCustomPayloadView<A> {
    type Error = ErrorKind;
    fn as_ref(&self) -> &dyn Derived<Vec<u8, A>, Error = Self::Error> {
        match self {
            Self::Message(msg) => msg,
        }
    }
}
impl<A: Allocator> EnumVariant<LenCustomPayloadView<A>> for GenericMessage<A> {
    fn from_enum(
        e: LenCustomPayloadView<A>,
    ) -> ::std::result::Result<Self, LenCustomPayloadView<A>> {
        e.try_unwrap_message().map_err(|e| e.input)
    }
    fn into_enum(self) -> LenCustomPayloadView<A> {
        LenCustomPayloadView::Message(self)
    }
    fn from_enum_ref(e: &LenCustomPayloadView<A>) -> Option<&Self> {
        e.try_unwrap_message_ref().ok()
    }
    fn from_enum_mut(e: &mut LenCustomPayloadView<A>) -> Option<&mut Self> {
        e.try_unwrap_message_mut().ok()
    }
}

#[derive(Clone, Debug, TryUnwrap)]
#[try_unwrap(ref, ref_mut)]
pub enum FieldCustomView<A: Allocator = Global> {
    #[debug("{:?}", _0.0)] // Ignore allocator
    ScalarMessage((Option<GenericMessage<A>>, A)),
}
impl<A: Allocator + Clone> EnumOfDeriveds<Vec<WireTypeAndPayload<A>, A>> for FieldCustomView<A> {
    type Error = ErrorKind;
    fn as_ref(&self) -> &dyn Derived<Vec<WireTypeAndPayload<A>, A>, Error = Self::Error> {
        match self {
            Self::ScalarMessage(msg) => msg,
        }
    }
}
impl<A: Allocator> EnumVariant<FieldCustomView<A>> for (Option<GenericMessage<A>>, A) {
    fn from_enum(e: FieldCustomView<A>) -> ::std::result::Result<Self, FieldCustomView<A>> {
        e.try_unwrap_scalar_message().map_err(|e| e.input)
    }
    fn into_enum(self) -> FieldCustomView<A> {
        FieldCustomView::ScalarMessage(self)
    }
    fn from_enum_ref(e: &FieldCustomView<A>) -> Option<&Self> {
        e.try_unwrap_scalar_message_ref().ok()
    }
    fn from_enum_mut(e: &mut FieldCustomView<A>) -> Option<&mut Self> {
        e.try_unwrap_scalar_message_mut().ok()
    }
}

impl GenericMessage<Global> {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            alloc: Global,
        }
    }
}
impl<A: Allocator + Clone> GenericMessage<A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            fields: HashMap::new_in(alloc.clone()),
            alloc: alloc.clone(),
        }
    }
    pub fn field(&self, number: i32) -> Option<&Field<A>> {
        let field_base = self.fields.get(&number)?;
        Some(Field::ref_cast(field_base))
    }
    pub fn field_mut(&mut self, number: i32) -> &mut Field<A> {
        let field_base = self.fields.entry(number).or_insert_with(|| {
            BaseAndDerived::<_, _, A>::from_base(
                Vec::new_in(self.alloc.clone()),
                self.alloc.clone(),
            )
        });
        Field::ref_cast_mut(field_base)
    }
}
impl<A: Allocator + Clone> GenericMessage<A> {
    pub fn merge(&mut self, other: Self) -> Result<()> {
        for (number, other_field_base) in other.fields {
            let other_payloads = other_field_base.try_take_base()?;
            match self.fields.entry(number) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().try_as_base_mut()?.extend(other_payloads);
                }
                Entry::Vacant(entry) => {
                    entry.insert(BaseAndDerived::from_base(
                        other_payloads,
                        self.alloc.clone(),
                    ));
                }
            }
        }
        Ok(())
    }
}

impl<A: Allocator> Debug for GenericMessage<A> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut ds = f.debug_struct("GenericMessage");
        for (number, field_base) in &self.fields {
            ds.field(&format!("field{}", number), field_base);
        }
        ds.finish()
    }
}

// Corresponds to a repeated message field.
impl<A: Allocator + Clone> Derived<Vec<u8, A>> for GenericMessage<A> {
    type Error = ErrorKind;
    fn from_base(base: &Vec<u8, A>) -> Result<Self>
    where
        Self: Sized,
    {
        let mut msg = GenericMessage::new_in(base.allocator().clone());
        msg.merge_from_bufread(base.as_slice())?;
        Ok(msg)
    }

    fn to_base(&self) -> Result<Vec<u8, A>> {
        let mut buf = Vec::new_in(self.alloc.clone());
        self.write(&mut buf)?;
        Ok(buf)
    }
}

// Corresponds to a scalar message field -- A message can be consist of multiple LEN payloads.
impl<A: Allocator + Clone> Derived<Vec<WireTypeAndPayload<A>, A>>
    for (Option<GenericMessage<A>>, A)
{
    type Error = ErrorKind;
    fn from_base(base: &Vec<WireTypeAndPayload<A>, A>) -> Result<Self>
    where
        Self: Sized,
    {
        let mut msg_opt = None;
        let allocator = base.allocator().clone();
        for wire_and_payload in base {
            let WireTypeAndPayload::Len(bytes_or_msg) = wire_and_payload else {
                // found a non-message field. What is the expected behavior here?
                return Err(ErrorKind::GenericMessageFieldTypeError)?;
            };
            let msg = msg_opt.get_or_insert_with(|| GenericMessage::new_in(allocator.clone()));
            msg.merge_from_bufread(bytes_or_msg.try_as_base()?.as_slice())?;
        }
        Ok((msg_opt, allocator))
    }

    fn to_base(&self) -> Result<Vec<WireTypeAndPayload<A>, A>> {
        match self {
            (Some(msg), alloc) => {
                let mut buf = Vec::new_in(alloc.clone());
                msg.write(&mut buf)?;
                let payload =
                    WireTypeAndPayload::Len(BaseAndDerived::from_base(buf, alloc.clone()));
                let mut vec = Vec::with_capacity_in(1, alloc.clone());
                vec.push(payload);
                Ok(vec)
            }
            (None, alloc) => Ok(Vec::new_in(alloc.clone())),
        }
    }
}

impl<A: Allocator + Clone> MessageLite for GenericMessage<A> {
    fn merge_from_bufread<R: BufRead>(&mut self, read: R) -> Result<()> {
        deser_from_bufread(read, self)
    }

    fn write<W: Write>(&self, mut write: W) -> Result<usize> {
        let mut total_bytes = 0;
        for (number, field_base) in &self.fields {
            for wire_and_payload in field_base.try_as_base()? {
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
                    WireTypeAndPayload::Len(bytes_or_msg) => {
                        // If bytes buffer is vacant, generate it from msg.
                        let bytes = bytes_or_msg.try_as_base()?;
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

impl<A: Allocator + Clone> DeserMessageHandlerBase for GenericMessage<A> {
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
impl<A: Allocator + Clone, R: Read> DeserMessageHandlerForRead<R> for GenericMessage<A> {
    fn parse_len(&mut self, num: i32, read: &mut R) -> Result<usize> {
        let alloc = self.alloc.clone();
        // Facepalm
        let mut buf = Vec::new();
        let len = read.read_to_end(&mut buf)?;
        self.field_mut(num)
            .0
            .try_as_base_mut()?
            .push(WireTypeAndPayload::Len(BaseAndDerived::from_base(
                buf.as_slice().to_vec_in(alloc.clone()),
                alloc,
            )));
        Ok(len)
    }
}

#[repr(transparent)]
#[derive(RefCast)]
pub struct Field<A: Allocator = Global>(
    BaseAndDerived<Vec<WireTypeAndPayload<A>, A>, FieldCustomView<A>, A>,
);

impl<A: Allocator> Field<A> {
    fn allocator(&self) -> &A {
        self.0.allocator()
    }
}

impl<A: Allocator + Clone> Field<A> {
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
    pub fn as_scalar_message(&self) -> Option<&GenericMessage<A>> {
        self.try_as_scalar_message().ok().flatten()
    }
    pub fn as_repeated_message(&self) -> impl '_ + Iterator<Item = &GenericMessage<A>> {
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
            .try_as_base()?
            .iter()
            .flat_map(move |record| match (allow_packed, record) {
                (_, WireTypeAndPayload::Variant(variant)) => {
                    Either::Left(Some(Ok(variant.clone())).into_iter())
                }
                (true, WireTypeAndPayload::Len(bytes_or_msg)) => match bytes_or_msg.try_as_base() {
                    Ok(bytes) => Either::Right(bytes.into_variant_iter()),
                    Err(e) => Either::Left(Some(Err(e)).into_iter()),
                },
                _ => Either::Left(Some(Err(ErrorKind::GenericMessageFieldTypeError)).into_iter()),
            })
            .map(|rv| rv.and_then(T::try_from_variant)))
    }
    pub fn try_as_scalar_i32_opt(&self) -> Result<Option<[u8; 4]>> {
        self.try_as_repeated_i32()?.last().transpose()
    }
    pub fn try_as_repeated_i32(&self) -> Result<impl '_ + Iterator<Item = Result<[u8; 4]>>> {
        Ok(self.0.try_as_base()?.iter().map(|record| match record {
            WireTypeAndPayload::I32(buf) => Ok(*buf),
            _ => Err(ErrorKind::GenericMessageFieldTypeError),
        }))
    }
    pub fn try_as_scalar_i64_opt(&self) -> Result<Option<[u8; 8]>> {
        self.try_as_repeated_i64()?.last().transpose()
    }
    pub fn try_as_repeated_i64(&self) -> Result<impl '_ + Iterator<Item = Result<[u8; 8]>>> {
        Ok(self.0.try_as_base()?.iter().map(|record| match record {
            WireTypeAndPayload::I64(buf) => Ok(*buf),
            _ => Err(ErrorKind::GenericMessageFieldTypeError),
        }))
    }
    pub fn try_as_scalar_string_opt(&self) -> Result<Option<&str>> {
        self.try_as_repeated_string()?.last().transpose()
    }
    pub fn try_as_repeated_string(&self) -> Result<impl '_ + Iterator<Item = Result<&str>>> {
        Ok(self.0.try_as_base()?.iter().map(|record| match record {
            WireTypeAndPayload::Len(bytes_or_msg) => {
                Ok(::std::str::from_utf8(bytes_or_msg.try_as_base()?)?)
            }
            _ => Err(ErrorKind::GenericMessageFieldTypeError),
        }))
    }
    pub fn try_as_scalar_bytes_opt(&self) -> Result<Option<&[u8]>> {
        self.try_as_repeated_bytes()?.last().transpose()
    }
    pub fn try_as_repeated_bytes(&self) -> Result<impl '_ + Iterator<Item = Result<&[u8]>>> {
        Ok(self.0.try_as_base()?.iter().map(|record| match record {
            WireTypeAndPayload::Len(bytes_or_msg) => Ok(bytes_or_msg.try_as_base()?.as_slice()),
            _ => Err(ErrorKind::GenericMessageFieldTypeError),
        }))
    }
    pub fn try_as_scalar_message(&self) -> Result<Option<&GenericMessage<A>>> {
        let (opt_msg, _) = self.0.try_as_derived::<(Option<GenericMessage<A>>, A)>()?;
        Ok(opt_msg.as_ref())
    }
    pub fn try_as_repeated_message(
        &self,
    ) -> Result<impl Iterator<Item = Result<&GenericMessage<A>>>> {
        Ok(self.0.try_as_base()?.iter().map(|wire_and_payload| {
            let WireTypeAndPayload::Len(bytes_or_msg) = wire_and_payload else {
                Err(ErrorKind::GenericMessageFieldTypeError)?
            };
            Ok(bytes_or_msg.try_as_derived()?)
        }))
    }

    pub fn clear(&mut self) {
        self.0.try_as_base_mut().unwrap().clear();
    }

    pub fn push_variant(&mut self, val: Variant) {
        self.0
            .try_as_base_mut()
            .unwrap()
            .push(WireTypeAndPayload::Variant(val));
    }
    pub fn push_variant_from<T: VariantIntegerType>(&mut self, val: T::RustType) {
        self.0
            .try_as_base_mut()
            .unwrap()
            .push(WireTypeAndPayload::Variant(Variant::from::<T>(val)));
    }
    pub fn push_i32(&mut self, val: [u8; 4]) {
        self.0
            .try_as_base_mut()
            .unwrap()
            .push(WireTypeAndPayload::I32(val));
    }
    pub fn push_i64(&mut self, val: [u8; 8]) {
        self.0
            .try_as_base_mut()
            .unwrap()
            .push(WireTypeAndPayload::I64(val));
    }
    pub fn push_string(&mut self, val: &str) {
        let alloc = self.allocator().clone();
        self.0
            .try_as_base_mut()
            .unwrap()
            .push(WireTypeAndPayload::Len(BaseAndDerived::from_base(
                val.as_bytes().to_vec_in(alloc.clone()),
                alloc.clone(),
            )));
    }
    pub fn push_bytes(&mut self, val: &[u8]) {
        let alloc = self.allocator().clone();
        self.0
            .try_as_base_mut()
            .unwrap()
            .push(WireTypeAndPayload::Len(BaseAndDerived::from_base(
                val.to_vec_in(alloc.clone()),
                alloc.clone(),
            )));
    }
    pub fn push_message(&mut self, val: GenericMessage<A>) {
        let alloc = self.allocator().clone();
        self.0
            .try_as_base_mut()
            .unwrap()
            .push(WireTypeAndPayload::Len(BaseAndDerived::from_derived(
                val, alloc,
            )));
    }
}
