//! Sample of a message that **bakes** [`DiscardUnknowns`] into `Marker<A>`.
//!
//! Impls are `impl<A> Marker<A>` only — no `U:` bound on the message type.
//! Unknowns are consumed on decode and omitted on encode. Not part of the
//! DESIGN.md reference schema.

use allocator_api2::alloc::{Allocator, Global};
use bitvec::array::BitArray;
use bitvec::order::Lsb0;
use bytes::{Buf, BufMut};
use core::fmt;
use core::mem;
use core::ops::ControlFlow;
use core::ops::DerefMut;

use puroro::{DecodeBuf, DecodeError, Message};
use puroro_rt::decode::{decode_tag, skip_field_and_save_in};
use puroro_rt::{
    CloneFieldsVisitor, CloneIn, DebugStructVisitor, DiscardUnknowns, EncodeCtx, EncodeRawVisitor,
    EncodedLenVisitor, FieldDeallocVisitor, FieldEqVisitor, FieldPairVisitor, FieldPairVisitorMut,
    FieldVisitor, FieldVisitorMut, Implicit, MessageCommon, MessageEncode, MessageMerge,
    ProtoInt32, SingularField,
};

use crate::marker::FIELD_N;

type Common<A> = MessageCommon<BitArray<[u8; 1], Lsb0>, A, DiscardUnknowns>;

pub struct Marker<A: Allocator = Global> {
    _common: Common<A>,
    n: SingularField<ProtoInt32, Implicit, { FIELD_N }, A>,
}

impl<A: Allocator> Marker<A> {
    pub fn n(&self) -> i32 {
        self.n.bind(&self._common).value()
    }

    fn visit_fields<V: FieldVisitor<Common<A>>>(&self, v: &mut V) -> ControlFlow<V::Break> {
        v.visit("n", &self.n)?;
        ControlFlow::Continue(())
    }

    fn visit_field_pairs<V: FieldPairVisitor<Common<A>>>(
        &self,
        other: &Self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("n", &self.n, &other.n)?;
        ControlFlow::Continue(())
    }

    fn visit_field_pairs_mut<V: FieldPairVisitorMut<Common<A>>>(
        &self,
        dst: &mut Self,
        v: &mut V,
    ) -> ControlFlow<V::Break>
    where
        A: Clone,
    {
        v.visit("n", &self.n, &mut dst.n)?;
        ControlFlow::Continue(())
    }

    fn visit_fields_mut<V: FieldVisitorMut<Common<A>>>(
        &mut self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("n", &mut self.n)?;
        ControlFlow::Continue(())
    }
}

impl<A: Allocator + Clone> Marker<A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            _common: MessageCommon::new_in(BitArray::ZERO, alloc.clone()),
            n: SingularField::new_in(alloc),
        }
    }

    pub fn n_mut(&mut self) -> impl DerefMut<Target = i32> + '_ {
        self.n.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_n(&mut self) {
        self.n.bind_mut(&mut self._common).clear();
    }

    /// Replaces `self` with a clone of `src` (same allocator family).
    pub fn copy_from(&mut self, src: &Self) {
        *self = src.clone();
    }
}

impl Marker<Global> {
    pub fn new() -> Self {
        <Self as Default>::default()
    }
}

impl<A: Allocator + Clone + Default> Default for Marker<A> {
    fn default() -> Self {
        Self::new_in(A::default())
    }
}

impl<A: Allocator + Clone> CloneIn<A> for Marker<A> {
    fn clone_in(&self, alloc: A) -> Self {
        let mut dst = Self::new_in(alloc.clone());
        let mut v = CloneFieldsVisitor::new(&self._common, &dst._common);
        let _ = self.visit_field_pairs_mut(&mut dst, &mut v);
        let mut old = mem::replace(&mut dst._common, self._common.clone_in(alloc));
        old.deallocate();
        dst
    }
}

impl<A: Allocator + Clone> Clone for Marker<A> {
    #[inline]
    fn clone(&self) -> Self {
        self.clone_in(self._common.alloc.clone())
    }
}

impl<A: Allocator> PartialEq for Marker<A> {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            self.visit_field_pairs(
                other,
                &mut FieldEqVisitor::new(&self._common, &other._common)
            ),
            ControlFlow::Continue(())
        ) && self._common.unknown_fields_eq(&other._common)
    }
}

impl<A: Allocator> fmt::Debug for Marker<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut v = DebugStructVisitor::new(f.debug_struct("Marker"), &self._common);
        let _ = self.visit_fields(&mut v);
        v.finish()
    }
}

impl<A: Allocator> Drop for Marker<A> {
    fn drop(&mut self) {
        let mut v = FieldDeallocVisitor::new(&self._common);
        let _ = self.visit_fields_mut(&mut v);
        self._common.deallocate();
    }
}

impl<A: Allocator> ::puroro_rt::DeallocateIn<A> for Marker<A> {
    #[inline]
    unsafe fn deallocate_in(self, _alloc: &A) {
        drop(self);
    }
}

::puroro_rt::impl_owned_slot_bounds!(Marker);

impl<A: Allocator> MessageEncode for Marker<A> {
    fn encoded_len(&self, ctx: &mut EncodeCtx) -> usize {
        let mut v = EncodedLenVisitor::new(&self._common, ctx);
        let _ = self.visit_fields(&mut v);
        v.len + self._common.unknown_fields.len()
    }

    fn encode_raw<B: BufMut>(&self, ctx: &mut EncodeCtx, buf: &mut B) {
        let _ = self.visit_fields(&mut EncodeRawVisitor::new(&self._common, ctx, buf));
        let unknown: &[u8] = &self._common.unknown_fields;
        buf.put_slice(unknown);
    }
}

impl<A: Allocator + Clone> MessageMerge for Marker<A> {
    fn merge_from_with_depth<B: DecodeBuf>(
        &mut self,
        buf: &mut B,
        depth: usize,
    ) -> Result<(), DecodeError> {
        if depth >= ::puroro::RECURSION_LIMIT {
            return Err(DecodeError::RecursionLimitExceeded);
        }
        while buf.has_remaining() {
            let (field_number, wire_type) = decode_tag(buf)?;
            match field_number.as_u32() {
                FIELD_N => {
                    self.n
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                _ => skip_field_and_save_in(
                    field_number,
                    wire_type,
                    buf,
                    &mut *self._common.unknown_fields,
                    self._common.alloc.clone(),
                )?,
            }
        }
        Ok(())
    }
}

impl<A: Allocator + Clone> ::puroro_rt::DefaultIn<A> for Marker<A> {
    #[inline]
    fn default_in(alloc: A) -> Self {
        Self::new_in(alloc)
    }
}

impl<A: Allocator> Message for Marker<A> {
    type Alloc = A;

    fn new_in(alloc: A) -> Self
    where
        A: Clone,
    {
        Self::new_in(alloc)
    }

    fn encode<B: BufMut>(&self, buf: &mut B) {
        ::puroro_rt::encode_message(self, buf)
    }

    fn encode_to_vec(&self) -> Vec<u8> {
        ::puroro_rt::encode_message_to_vec(self)
    }

    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError>
    where
        A: Clone,
    {
        ::puroro_rt::merge_message(self, buf)
    }

    fn unknown_fields(&self) -> impl Iterator<Item = ::puroro::UnknownField<'_>> + '_ {
        self._common.iter_unknown_fields()
    }

    fn validate(&self) -> Result<(), DecodeError> {
        Ok(())
    }
}
