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

use super::DynamicMessage;
use crate::internal::utils::{
    boxed_fn_converter_with_context, BoxedFnConverterWithContext, ConverterForOnceList1, OnceList1,
    PairWithOnceList1, PairWithOnceList1Ext,
};
use crate::internal::WireType;
use crate::message::MessageMut;
use crate::variant::{ReadExtVariant, Variant, WriteExtVariant};
use crate::{ErrorKind, Result};
use ::cached_pair::{EitherOrBoth, Pair};
use ::derive_more::{Debug, Deref, DerefMut, TryUnwrap};
use ::std::alloc::{Allocator, Global};
use ::std::cell::Cell;
use ::std::convert::Infallible;

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct DynamicLenPayload<A: Allocator = Global> {
    payload:
        PairWithOnceList1<Vec<u8, A>, LenCustomPayloadView<A>, A, DynamicLenPayloadConverter<A>>,
}

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

#[derive(Clone, Debug, TryUnwrap, ::derive_more::From, ::derive_more::TryInto)]
#[try_unwrap(ref, ref_mut)]
#[try_into(owned, ref)]
pub enum LenCustomPayloadView<A: Allocator = Global> {
    Message(DynamicMessage<A>),
    PackedVariants(Vec<Variant, A>),
}

#[derive(Clone, Copy, Debug)]
enum LenCustomPayloadViewCase {
    Message,
    PackedVariants,
}

type DynamicLenPayloadConverter<A> = BoxedFnConverterWithContext<
    Cell<LenCustomPayloadViewCase>,
    Vec<u8, A>,
    LenCustomPayloadView<A>,
    Infallible,
    ErrorKind,
>;
fn dynamic_len_payload_converter<A: Allocator + Clone>() -> DynamicLenPayloadConverter<A> {
    boxed_fn_converter_with_context(
        Cell::new(LenCustomPayloadViewCase::Message),
        |view: &LenCustomPayloadView<A>, _| Ok(view.to_buf()),
        |buf: &Vec<u8, A>, case| match case.get() {
            LenCustomPayloadViewCase::Message => Ok(LenCustomPayloadView::Message({
                let mut msg = DynamicMessage::new_in(buf.allocator().clone());
                msg.merge_from_read(buf.as_slice())?;
                msg
            })),
            LenCustomPayloadViewCase::PackedVariants => Ok(LenCustomPayloadView::PackedVariants({
                let mut variants = Vec::new_in(buf.allocator().clone());
                for v in buf.into_variant_iter() {
                    variants.push(v?);
                }
                variants
            })),
        },
    )
}

impl<A: Allocator + Clone> LenCustomPayloadView<A> {
    pub(crate) fn to_buf(&self) -> Vec<u8, A> {
        match self {
            LenCustomPayloadView::Message(msg) => {
                let mut buf = Vec::new_in(msg.allocator().clone());
                msg.write_to_vec(&mut buf);
                buf
            }
            LenCustomPayloadView::PackedVariants(variants) => {
                let mut buf = Vec::new_in(variants.allocator().clone());
                for v in variants {
                    buf.write_variant(v.clone()).unwrap();
                }
                buf
            }
        }
    }
}

impl<A: Allocator + Clone> DynamicLenPayload<A> {
    pub(crate) fn from_buf(buf: Vec<u8, A>) -> Self {
        let alloc = buf.allocator().clone();
        Self {
            payload: Pair::from_left_conv(
                buf,
                ConverterForOnceList1::new_in(dynamic_len_payload_converter::<A>(), alloc),
            ),
        }
    }

    pub(crate) fn from_message(msg: DynamicMessage<A>, alloc: &A) -> Self {
        Self {
            payload: Pair::from_right_conv(
                OnceList1::new_in(LenCustomPayloadView::Message(msg), alloc.clone()),
                ConverterForOnceList1::new_in(dynamic_len_payload_converter::<A>(), alloc.clone()),
            ),
        }
    }

    pub(crate) fn from_variant(variant: Variant, alloc: &A) -> Self {
        let mut vec = Vec::new_in(alloc.clone());
        vec.push(variant);
        Self {
            payload: Pair::from_right_conv(
                OnceList1::new_in(LenCustomPayloadView::PackedVariants(vec), alloc.clone()),
                ConverterForOnceList1::new_in(dynamic_len_payload_converter::<A>(), alloc.clone()),
            ),
        }
    }

    pub(crate) fn as_buf(&self) -> &Vec<u8, A> {
        unsafe { self.payload.left_with(|list| list.first().to_buf()) }
    }

    pub(crate) fn as_buf_mut(&mut self) -> &mut Vec<u8, A> {
        unsafe { self.payload.left_mut_with(|list| list.first().to_buf()) }
    }

    pub(crate) fn as_message(&self) -> Result<&DynamicMessage<A>> {
        self.set_context(LenCustomPayloadViewCase::Message);
        let LenCustomPayloadView::Message(msg) =
            self.payload.try_get_or_insert_into_right(|view| {
                matches!(view, LenCustomPayloadView::Message(_))
            })?
        else {
            unreachable!()
        };
        Ok(msg)
    }

    pub(crate) fn as_packed_variants(&self) -> Result<&Vec<Variant, A>> {
        self.set_context(LenCustomPayloadViewCase::PackedVariants);
        let LenCustomPayloadView::PackedVariants(variants) =
            self.payload.try_get_or_insert_into_right(|view| {
                matches!(view, LenCustomPayloadView::PackedVariants(_))
            })?
        else {
            unreachable!()
        };
        Ok(variants)
    }

    fn set_context(&self, case: LenCustomPayloadViewCase) {
        self.payload.converter().inner().context().set(case);
    }
}

impl<A: Allocator> DynamicLenPayload<A> {
    pub(crate) fn allocator(&self) -> &A {
        match self.payload.as_ref() {
            EitherOrBoth::Left(vec) | EitherOrBoth::Both(vec, _) => vec.allocator(),
            EitherOrBoth::Right(list) => list.allocator(),
        }
    }
}
