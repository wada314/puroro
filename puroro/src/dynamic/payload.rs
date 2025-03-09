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
use crate::internal::WireType;
use crate::message::MessageMut;
use crate::variant::{ReadExtVariant, Variant, WriteExtVariant};
use crate::{ErrorKind, Result};
use ::cached_pair::multi_pair::collections::std::VecCollection;
use ::cached_pair::multi_pair::{Case, MultiPair, MultiPairConverter};
use ::derive_more::{Debug, TryUnwrap};
use ::std::alloc::{Allocator, Global};
use ::std::convert::Infallible;

#[derive(Clone, Debug)]
pub struct DynamicLenPayload<A: Allocator = Global> {
    payload: MultiPair<
        Vec<u8, A>,
        LenCustomPayloadView<A>,
        VecCollection<LenCustomPayloadView<A>, A>,
        DynamicLenPayloadConverter<A>,
        A,
    >,
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

#[derive(Clone, Copy, Debug, Default)]
enum LenCustomPayloadViewCase {
    #[default]
    Message,
    PackedVariants,
}

#[derive(Clone, Copy, Debug)]
struct DynamicLenPayloadConverter<A> {
    #[debug(skip)]
    allocator: A,
}

impl<A: Allocator + Clone>
    MultiPairConverter<
        Vec<u8, A>,
        LenCustomPayloadView<A>,
        VecCollection<LenCustomPayloadView<A>, A>,
    > for DynamicLenPayloadConverter<A>
{
    type ToLeftError = Infallible;
    type ToRightError = ErrorKind;
    type Case = LenCustomPayloadViewCase;

    fn rights_to_left<'a>(
        &self,
        rights: impl IntoIterator<Item = &'a LenCustomPayloadView<A>>,
    ) -> ::std::result::Result<Vec<u8, A>, Self::ToLeftError>
    where
        LenCustomPayloadView<A>: 'a,
    {
        let right = rights.into_iter().next().unwrap();
        Ok(right.to_buf())
    }

    fn left_to_right(
        &self,
        left: &Vec<u8, A>,
        context: &Self::Case,
    ) -> ::std::result::Result<LenCustomPayloadView<A>, Self::ToRightError> {
        match context {
            LenCustomPayloadViewCase::Message => Ok(LenCustomPayloadView::Message({
                let mut msg = DynamicMessage::new_in(self.allocator.clone());
                msg.merge_from_read(left.as_slice())?;
                msg
            })),
            LenCustomPayloadViewCase::PackedVariants => Ok(LenCustomPayloadView::PackedVariants({
                let mut variants = Vec::new_in(self.allocator.clone());
                for v in left.into_variant_iter() {
                    variants.push(v?);
                }
                variants
            })),
        }
    }
}

impl<A: Allocator> Case<LenCustomPayloadView<A>> for LenCustomPayloadViewCase {
    fn matches(&self, right: &LenCustomPayloadView<A>) -> bool {
        match (self, right) {
            (LenCustomPayloadViewCase::Message, LenCustomPayloadView::Message(_)) => true,
            (LenCustomPayloadViewCase::PackedVariants, LenCustomPayloadView::PackedVariants(_)) => {
                true
            }
            _ => false,
        }
    }
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
            payload: MultiPair::from_left_conv_in(
                buf,
                DynamicLenPayloadConverter {
                    allocator: alloc.clone(),
                },
                alloc,
            ),
        }
    }

    pub(crate) fn from_message(msg: DynamicMessage<A>) -> Self {
        let alloc = msg.allocator().clone();
        Self {
            payload: MultiPair::from_right_conv_in(
                LenCustomPayloadView::Message(msg),
                DynamicLenPayloadConverter {
                    allocator: alloc.clone(),
                },
                alloc,
            ),
        }
    }

    #[allow(unused)]
    pub(crate) fn from_variant(variant: Variant, alloc: &A) -> Self {
        let mut vec = Vec::new_in(alloc.clone());
        vec.push(variant);
        Self {
            payload: MultiPair::from_right_conv_in(
                LenCustomPayloadView::PackedVariants(vec),
                DynamicLenPayloadConverter {
                    allocator: alloc.clone(),
                },
                alloc.clone(),
            ),
        }
    }

    pub(crate) fn as_buf(&self) -> &Vec<u8, A> {
        self.payload.left()
    }

    #[allow(unused)]
    pub(crate) fn as_buf_mut(&mut self) -> &mut Vec<u8, A> {
        self.payload.left_mut()
    }

    pub(crate) fn as_message(&self) -> Result<&DynamicMessage<A>> {
        let LenCustomPayloadView::Message(msg) = self
            .payload
            .try_right::<ErrorKind>(&LenCustomPayloadViewCase::Message)?
        else {
            unreachable!()
        };
        Ok(msg)
    }

    #[allow(unused)]
    pub(crate) fn as_packed_variants(&self) -> Result<&Vec<Variant, A>> {
        let LenCustomPayloadView::PackedVariants(variants) = self
            .payload
            .try_right::<ErrorKind>(&LenCustomPayloadViewCase::PackedVariants)?
        else {
            unreachable!()
        };
        Ok(variants)
    }

    pub(crate) fn as_packed_variants_mut(&mut self) -> Result<&mut Vec<Variant, A>> {
        let LenCustomPayloadView::PackedVariants(variants) = self
            .payload
            .try_right_mut::<ErrorKind>(&LenCustomPayloadViewCase::PackedVariants)?
        else {
            unreachable!()
        };
        Ok(variants)
    }
}

impl<A: Allocator> DynamicLenPayload<A> {
    pub(crate) fn allocator(&self) -> &A {
        self.payload.allocator()
    }
}
