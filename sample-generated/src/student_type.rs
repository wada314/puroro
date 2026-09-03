//! Sample mid-level nested message: an EXPLICIT scalar plus inlined
//! [`Point`](crate::Point) and [`Address`](crate::Address) (each a full `M`).
//!
//! Used by [`School`](crate::School) as a two-hop inline chain. Not in DESIGN.md.

use allocator_api2::alloc::{Allocator, Global};
use bitvec::array::BitArray;
use bitvec::order::Lsb0;
use bytes::{Buf, BufMut};
use core::fmt;
use core::mem;
use core::ops::ControlFlow;
use core::ops::DerefMut;

use puroro::{DecodeBuf, DecodeError, HasDefault, Message, Optional};
use puroro_rt::decode::{decode_tag, skip_field_and_save};
use puroro_rt::{
    CloneFieldsVisitor, CloneIn, DebugStructVisitor, EncodeCtx, EncodeRawVisitor,
    EncodedLenVisitor, Explicit, FieldDeallocVisitor, FieldEqVisitor, FieldPairVisitor,
    FieldPairVisitorMut, FieldVisitor, FieldVisitorMut, MessageCommon, MessageEncode, MessageMerge,
    ProtoInt32, ProtoMessage, SingularField,
};

use crate::Address;
use crate::Point;
use crate::student::{BIT_HOME, BIT_LOCATION, BIT_YEAR, FIELD_HOME, FIELD_LOCATION, FIELD_YEAR};

pub struct Student<A: Allocator = Global> {
    _common: MessageCommon<BitArray<[u8; 1], Lsb0>, A>,
    year: SingularField<ProtoInt32, Explicit<{ BIT_YEAR }>, { FIELD_YEAR }, A>,
    location:
        SingularField<ProtoMessage<Point<A>>, Explicit<{ BIT_LOCATION }>, { FIELD_LOCATION }, A>,
    home: SingularField<ProtoMessage<Address<A>>, Explicit<{ BIT_HOME }>, { FIELD_HOME }, A>,
}

impl<A: Allocator> Student<A> {
    pub fn year<'a>(&'a self) -> Optional<i32, impl HasDefault<i32>>
    where
        A: 'a,
    {
        self.year.bind(&self._common).optional()
    }

    pub fn location(&self) -> Option<&Point<A>> {
        self.location.bind(&self._common).get()
    }

    pub fn home(&self) -> Option<&Address<A>> {
        self.home.bind(&self._common).get()
    }

    fn visit_fields<V: FieldVisitor<MessageCommon<BitArray<[u8; 1], Lsb0>, A>>>(
        &self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("year", &self.year)?;
        v.visit("location", &self.location)?;
        v.visit("home", &self.home)?;
        ControlFlow::Continue(())
    }

    fn visit_field_pairs<V: FieldPairVisitor<MessageCommon<BitArray<[u8; 1], Lsb0>, A>>>(
        &self,
        other: &Self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("year", &self.year, &other.year)?;
        v.visit("location", &self.location, &other.location)?;
        v.visit("home", &self.home, &other.home)?;
        ControlFlow::Continue(())
    }

    fn visit_field_pairs_mut<V: FieldPairVisitorMut<MessageCommon<BitArray<[u8; 1], Lsb0>, A>>>(
        &self,
        dst: &mut Self,
        v: &mut V,
    ) -> ControlFlow<V::Break>
    where
        A: Clone,
    {
        v.visit("year", &self.year, &mut dst.year)?;
        v.visit("location", &self.location, &mut dst.location)?;
        v.visit("home", &self.home, &mut dst.home)?;
        ControlFlow::Continue(())
    }

    fn visit_fields_mut<V: FieldVisitorMut<MessageCommon<BitArray<[u8; 1], Lsb0>, A>>>(
        &mut self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("year", &mut self.year)?;
        v.visit("location", &mut self.location)?;
        v.visit("home", &mut self.home)?;
        ControlFlow::Continue(())
    }
}

impl<A: Allocator + Clone> Student<A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            _common: MessageCommon::new_in(BitArray::ZERO, alloc.clone()),
            year: SingularField::new_in(alloc.clone()),
            location: SingularField::new_in(alloc.clone()),
            home: SingularField::new_in(alloc),
        }
    }

    pub fn year_mut(&mut self) -> impl DerefMut<Target = i32> + '_ {
        self.year.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_year(&mut self) {
        self.year.bind_mut(&mut self._common).clear();
    }

    pub fn location_mut(&mut self) -> &mut Point<A> {
        self.location.bind_mut(&mut self._common).get_mut()
    }

    pub fn set_location(&mut self, src: Point<A>) {
        *self.location_mut() = src;
    }

    pub fn clear_location(&mut self) {
        self.location.bind_mut(&mut self._common).clear();
    }

    pub fn home_mut(&mut self) -> &mut Address<A> {
        self.home.bind_mut(&mut self._common).get_mut()
    }

    pub fn set_home(&mut self, src: Address<A>) {
        *self.home_mut() = src;
    }

    pub fn clear_home(&mut self) {
        self.home.bind_mut(&mut self._common).clear();
    }

    /// Replaces `self` with a clone of `src` (same allocator family).
    pub fn copy_from(&mut self, src: &Self) {
        *self = src.clone();
    }
}

impl Student<Global> {
    pub fn new() -> Self {
        <Self as Default>::default()
    }
}

impl<A: Allocator + Clone + Default> Default for Student<A> {
    fn default() -> Self {
        Self::new_in(A::default())
    }
}

impl<A: Allocator + Clone> CloneIn<A> for Student<A> {
    fn clone_in(&self, alloc: A) -> Self {
        let mut dst = Self::new_in(alloc.clone());
        let mut v = CloneFieldsVisitor::new(&self._common, &dst._common);
        let _ = self.visit_field_pairs_mut(&mut dst, &mut v);
        let mut old = mem::replace(&mut dst._common, self._common.clone_in(alloc));
        old.deallocate();
        dst
    }
}

impl<A: Allocator + Clone> Clone for Student<A> {
    #[inline]
    fn clone(&self) -> Self {
        self.clone_in(self._common.alloc.clone())
    }
}

impl<A: Allocator> PartialEq for Student<A> {
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

impl<A: Allocator> fmt::Debug for Student<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut v = DebugStructVisitor::new(f.debug_struct("Student"), &self._common);
        let _ = self.visit_fields(&mut v);
        v.finish()
    }
}

impl<A: Allocator> Drop for Student<A> {
    fn drop(&mut self) {
        let mut v = FieldDeallocVisitor::new(&self._common);
        let _ = self.visit_fields_mut(&mut v);
        self._common.deallocate();
    }
}

impl<A: Allocator> ::puroro_rt::DeallocateIn<A> for Student<A> {
    #[inline]
    unsafe fn deallocate_in(self, _alloc: &A) {
        drop(self);
    }
}

::puroro_rt::impl_owned_slot_bounds!(Student);

impl<A: Allocator> MessageEncode for Student<A> {
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

impl<A: Allocator + Clone> MessageMerge for Student<A> {
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
                FIELD_YEAR => {
                    self.year
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                FIELD_LOCATION => {
                    self.location
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                FIELD_HOME => {
                    self.home
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                _ => skip_field_and_save(
                    field_number,
                    wire_type,
                    buf,
                    &mut self._common.unknown_fields,
                    self._common.alloc.clone(),
                )?,
            }
        }
        Ok(())
    }
}

impl<A: Allocator + Clone> ::puroro_rt::DefaultIn<A> for Student<A> {
    #[inline]
    fn default_in(alloc: A) -> Self {
        Self::new_in(alloc)
    }
}

impl<A: Allocator> Message for Student<A> {
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
