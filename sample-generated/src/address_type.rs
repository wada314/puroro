//! Sample of the code puroro generates for message `example.Address`
//! (from `example.proto`). The public type is re-exported at the crate root so
//! it sits beside companion module [`crate::address`].

use allocator_api2::alloc::{Allocator, Global};
use bitvec::array::BitArray;
use bitvec::order::Lsb0;
use bytes::{Buf, BufMut};
use core::fmt;
use core::mem;
use core::ops::ControlFlow;
use core::ops::DerefMut;

use puroro::{DecodeBuf, DecodeError, Message};
use puroro_rt::decode::{decode_tag, skip_field_and_save};
use puroro_rt::{
    CloneFieldsVisitor, CloneIn, DebugStructVisitor, EncodeCtx, EncodeRawVisitor,
    EncodedLenVisitor, Explicit, FieldDeallocVisitor, FieldEqVisitor, FieldPairVisitor,
    FieldPairVisitorMut, FieldVisitor, FieldVisitorMut, InlineOrHeap, MessageCommon, MessageEncode,
    MessageMerge, ProtoDouble, ProtoFixed32, ProtoString, SingularField,
};

use crate::address::{
    BIT_CITY, BIT_CITY_SSO, BIT_LATITUDE, BIT_POSTAL_CODE, BIT_STREET, BIT_STREET_SSO, FIELD_CITY,
    FIELD_LATITUDE, FIELD_POSTAL_CODE, FIELD_STREET,
};

// ---------------------------------------------------------------------------
// Message struct
// ---------------------------------------------------------------------------

pub struct Address<A: Allocator = Global> {
    _common: MessageCommon<BitArray<[u8; 1], Lsb0>, A>,
    street: SingularField<
        ProtoString,
        Explicit<{ BIT_STREET }>,
        { FIELD_STREET },
        A,
        InlineOrHeap<{ BIT_STREET_SSO }>,
    >, // proto: string street = 1;
    city: SingularField<
        ProtoString,
        Explicit<{ BIT_CITY }>,
        { FIELD_CITY },
        A,
        InlineOrHeap<{ BIT_CITY_SSO }>,
    >, // proto: string city = 2;
    postal_code:
        SingularField<ProtoFixed32, Explicit<{ BIT_POSTAL_CODE }>, { FIELD_POSTAL_CODE }, A>, // proto: fixed32 postal_code = 3;
    latitude: SingularField<ProtoDouble, Explicit<{ BIT_LATITUDE }>, { FIELD_LATITUDE }, A>, // proto: double latitude = 4;
}

impl<A: Allocator> Address<A> {
    pub fn street<'a>(&'a self) -> ::puroro::Optional<&'a str, impl ::puroro::HasDefault<&'a str>>
    where
        A: 'a,
    {
        self.street.bind(&self._common).optional()
    }

    pub fn city<'a>(&'a self) -> ::puroro::Optional<&'a str, impl ::puroro::HasDefault<&'a str>>
    where
        A: 'a,
    {
        self.city.bind(&self._common).optional()
    }

    pub fn postal_code<'a>(&'a self) -> ::puroro::Optional<u32, impl ::puroro::HasDefault<u32>>
    where
        A: 'a,
    {
        self.postal_code.bind(&self._common).optional()
    }

    pub fn latitude<'a>(&'a self) -> ::puroro::Optional<f64, impl ::puroro::HasDefault<f64>>
    where
        A: 'a,
    {
        self.latitude.bind(&self._common).optional()
    }

    fn visit_fields<V: FieldVisitor<MessageCommon<BitArray<[u8; 1], Lsb0>, A>>>(
        &self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("street", &self.street)?;
        v.visit("city", &self.city)?;
        v.visit("postal_code", &self.postal_code)?;
        v.visit("latitude", &self.latitude)?;
        ControlFlow::Continue(())
    }

    /// Pair / shared.
    fn visit_field_pairs<V: FieldPairVisitor<MessageCommon<BitArray<[u8; 1], Lsb0>, A>>>(
        &self,
        other: &Self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("street", &self.street, &other.street)?;
        v.visit("city", &self.city, &other.city)?;
        v.visit("postal_code", &self.postal_code, &other.postal_code)?;
        v.visit("latitude", &self.latitude, &other.latitude)?;
        ControlFlow::Continue(())
    }

    /// Pair / mut.
    fn visit_field_pairs_mut<V: FieldPairVisitorMut<MessageCommon<BitArray<[u8; 1], Lsb0>, A>>>(
        &self,
        dst: &mut Self,
        v: &mut V,
    ) -> ControlFlow<V::Break>
    where
        A: Clone,
    {
        v.visit("street", &self.street, &mut dst.street)?;
        v.visit("city", &self.city, &mut dst.city)?;
        v.visit("postal_code", &self.postal_code, &mut dst.postal_code)?;
        v.visit("latitude", &self.latitude, &mut dst.latitude)?;
        ControlFlow::Continue(())
    }

    /// Scalar / mut.
    fn visit_fields_mut<V: FieldVisitorMut<MessageCommon<BitArray<[u8; 1], Lsb0>, A>>>(
        &mut self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("street", &mut self.street)?;
        v.visit("city", &mut self.city)?;
        v.visit("postal_code", &mut self.postal_code)?;
        v.visit("latitude", &mut self.latitude)?;
        ControlFlow::Continue(())
    }
}

impl<A: Allocator + Clone> Address<A> {
    pub fn new_in(alloc: A) -> Self {
        // Each field initializer gets its own clone of the allocator; the last
        // heap field takes the original by move.
        Self {
            _common: MessageCommon::new_in(BitArray::ZERO, alloc.clone()),
            street: SingularField::new_in(alloc.clone()),
            city: SingularField::new_in(alloc.clone()),
            postal_code: SingularField::new_in(alloc.clone()),
            latitude: SingularField::new_in(alloc),
        }
    }

    // -- street (EXPLICIT string, proto field 1) ----------------------------

    pub fn street_mut(&mut self) -> impl ::puroro::StringMut<A> + '_ {
        self.street.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_street(&mut self) {
        self.street.bind_mut(&mut self._common).clear();
    }

    // -- city (EXPLICIT string, proto field 2) ------------------------------

    pub fn city_mut(&mut self) -> impl ::puroro::StringMut<A> + '_ {
        self.city.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_city(&mut self) {
        self.city.bind_mut(&mut self._common).clear();
    }

    // -- postal_code (EXPLICIT fixed32, proto field 3) ----------------------

    pub fn postal_code_mut(&mut self) -> impl DerefMut<Target = u32> + '_ {
        self.postal_code.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_postal_code(&mut self) {
        self.postal_code.bind_mut(&mut self._common).clear();
    }

    // -- latitude (EXPLICIT double, proto field 4) --------------------------

    pub fn latitude_mut(&mut self) -> impl DerefMut<Target = f64> + '_ {
        self.latitude.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_latitude(&mut self) {
        self.latitude.bind_mut(&mut self._common).clear();
    }
}

impl Address<Global> {
    pub fn new() -> Self {
        <Self as Default>::default()
    }
}

impl<A: Allocator + Clone + Default> Default for Address<A> {
    fn default() -> Self {
        Self::new_in(A::default())
    }
}

// ---------------------------------------------------------------------------
// Clone / PartialEq / Debug
// ---------------------------------------------------------------------------

impl<A: Allocator + Clone> ::puroro_rt::CloneIn<A> for Address<A> {
    fn clone_in(&self, alloc: A) -> Self {
        let mut dst = Self::new_in(alloc.clone());
        let mut v = CloneFieldsVisitor::new(&self._common, &dst._common);
        let _ = self.visit_field_pairs_mut(&mut dst, &mut v);
        let mut old = mem::replace(&mut dst._common, self._common.clone_in(alloc));
        old.deallocate();
        dst
    }
}

impl<A: Allocator + Clone> Clone for Address<A> {
    #[inline]
    fn clone(&self) -> Self {
        self.clone_in(self._common.alloc.clone())
    }
}

impl<A: Allocator> PartialEq for Address<A> {
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

impl<A: Allocator> fmt::Debug for Address<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut v = DebugStructVisitor::new(f.debug_struct("Address"), &self._common);
        let _ = self.visit_fields(&mut v);
        v.finish()
    }
}

// ---------------------------------------------------------------------------
// Drop — releases every unmanaged field through the single allocator
// ---------------------------------------------------------------------------

impl<A: Allocator> Drop for Address<A> {
    fn drop(&mut self) {
        let mut v = FieldDeallocVisitor::new(&self._common);
        let _ = self.visit_fields_mut(&mut v);
        self._common.deallocate();
    }
}

// ---------------------------------------------------------------------------
// DeallocateIn — required for nested `UnmanagedBox` / catalog bounds
// ---------------------------------------------------------------------------

impl<A: Allocator> ::puroro_rt::DeallocateIn<A> for Address<A> {
    #[inline]
    unsafe fn deallocate_in(self, _alloc: &A) {
        // Heap is owned by `self._common.alloc`; parent-passed `alloc` is only
        // needed when freeing an enclosing `UnmanagedBox` slot.
        drop(self);
    }
}

// ---------------------------------------------------------------------------
// Message
// ---------------------------------------------------------------------------

impl<A: Allocator> MessageEncode for Address<A> {
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

impl<A: Allocator + Clone> MessageMerge for Address<A> {
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
                FIELD_STREET => {
                    // street = 1, EXPLICIT string
                    self.street
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                FIELD_CITY => {
                    // city = 2, EXPLICIT string
                    self.city
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                FIELD_POSTAL_CODE => {
                    // postal_code = 3, EXPLICIT fixed32
                    self.postal_code
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                FIELD_LATITUDE => {
                    // latitude = 4, EXPLICIT double
                    self.latitude
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                _ => {
                    // unknown field — preserve in _common.unknown_fields
                    skip_field_and_save(
                        field_number,
                        wire_type,
                        buf,
                        &mut self._common.unknown_fields,
                        self._common.alloc.clone(),
                    )?
                }
            }
        }
        Ok(())
    }
}

impl<A: Allocator + Clone> ::puroro_rt::DefaultIn<A> for Address<A> {
    #[inline]
    fn default_in(alloc: A) -> Self {
        Self::new_in(alloc)
    }
}

impl<A: Allocator + Clone> Message for Address<A> {
    type Alloc = A;

    fn new_in(alloc: A) -> Self {
        Self::new_in(alloc)
    }

    fn encode<B: BufMut>(&self, buf: &mut B) {
        ::puroro_rt::encode_message(self, buf)
    }

    fn encode_to_vec(&self) -> Vec<u8> {
        ::puroro_rt::encode_message_to_vec(self)
    }

    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError> {
        ::puroro_rt::merge_message(self, buf)
    }

    fn unknown_fields(&self) -> impl Iterator<Item = ::puroro::UnknownField<'_>> + '_ {
        self._common.iter_unknown_fields()
    }

    fn validate(&self) -> Result<(), DecodeError> {
        Ok(())
    }
}
