//! Sample of the code puroro generates for message `example.Address`
//! (from `example.proto`).

mod defaults;

use ::allocator_api2::alloc::{Allocator, Global};
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;
use ::bitvec::ptr::{BitRef, Mut};
use ::bytes::{Buf, BufMut};
use ::core::fmt;
use ::core::ops::ControlFlow;
use ::core::ops::DerefMut;

use ::puroro::{DecodeError, Message};
use ::puroro_rt::decode::{decode_tag, skip_field_and_save};
use ::puroro_rt::{
    DebugStructVisitor, Explicit, FieldDeallocVisitor, FieldEqVisitor, FieldPairVisitor,
    FieldVisitor, FieldVisitorMut, MessageCommon, PresenceBits, ProtoDouble, ProtoFixed32,
    ProtoString, SingularField,
};
use ::unmanaged::CloneIn;

// ---------------------------------------------------------------------------
// Presence bitfield (4 tracked singular fields)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AddressPresence(BitArray<[u8; 1], Lsb0>);

impl AddressPresence {
    pub const ZERO: Self = Self(BitArray::ZERO);
}

impl PresenceBits for AddressPresence {
    fn is_set(&self, bit: usize) -> bool {
        self.0[bit]
    }

    fn set(&mut self, bit: usize, present: bool) {
        self.0.set(bit, present);
    }

    fn bit_mut(&mut self, bit: usize) -> BitRef<'_, Mut, u8, Lsb0> {
        self.0.get_mut(bit).expect("presence bit index in range")
    }
}

// ---------------------------------------------------------------------------
// Presence bit indices (4 tracked singular fields)
// ---------------------------------------------------------------------------

pub const BIT_STREET: usize = 0; // street (EXPLICIT)
pub const BIT_CITY: usize = 1; // city (EXPLICIT)
pub const BIT_POSTAL_CODE: usize = 2; // postal_code (EXPLICIT fixed32)
pub const BIT_LATITUDE: usize = 3; // latitude (EXPLICIT double)

// ---------------------------------------------------------------------------
// Proto field numbers
// ---------------------------------------------------------------------------

pub const FIELD_STREET: u32 = 1; // street
pub const FIELD_CITY: u32 = 2; // city
pub const FIELD_POSTAL_CODE: u32 = 3; // postal_code
pub const FIELD_LATITUDE: u32 = 4; // latitude

// ---------------------------------------------------------------------------
// Message struct
// ---------------------------------------------------------------------------

pub struct Address<A: Allocator + Clone = Global> {
    _common: MessageCommon<AddressPresence, A>,
    street: SingularField<ProtoString, Explicit<{ BIT_STREET }>, { FIELD_STREET }, A>, // proto: string street = 1;
    city: SingularField<ProtoString, Explicit<{ BIT_CITY }>, { FIELD_CITY }, A>, // proto: string city = 2;
    postal_code:
        SingularField<ProtoFixed32, Explicit<{ BIT_POSTAL_CODE }>, { FIELD_POSTAL_CODE }, A>, // proto: fixed32 postal_code = 3;
    latitude: SingularField<ProtoDouble, Explicit<{ BIT_LATITUDE }>, { FIELD_LATITUDE }, A>, // proto: double latitude = 4;
}

impl<A: Allocator + Clone> Address<A> {
    pub fn new_in(alloc: A) -> Self {
        // Each field initializer gets its own clone of the allocator; the last
        // heap field takes the original by move.
        Self {
            _common: MessageCommon::new_in(AddressPresence::ZERO, alloc.clone()),
            street: SingularField::new_in(alloc.clone()),
            city: SingularField::new_in(alloc.clone()),
            postal_code: SingularField::new_in(alloc.clone()),
            latitude: SingularField::new_in(alloc),
        }
    }

    // -- street (EXPLICIT string, proto field 1) ----------------------------

    pub fn street<'a>(&'a self) -> ::puroro::Optional<&'a str, impl ::puroro::HasDefault<&'a str>>
    where
        A: 'a,
    {
        self.street.bind(&self._common).optional()
    }

    pub fn street_mut<'s>(&'s mut self) -> impl DerefMut<Target = ::unmanaged::String<A>> + 's {
        self.street.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_street(&mut self) {
        self.street.bind_mut(&mut self._common).clear();
    }

    // -- city (EXPLICIT string, proto field 2) ------------------------------

    pub fn city<'a>(&'a self) -> ::puroro::Optional<&'a str, impl ::puroro::HasDefault<&'a str>>
    where
        A: 'a,
    {
        self.city.bind(&self._common).optional()
    }

    pub fn city_mut<'s>(&'s mut self) -> impl DerefMut<Target = ::unmanaged::String<A>> + 's {
        self.city.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_city(&mut self) {
        self.city.bind_mut(&mut self._common).clear();
    }

    // -- postal_code (EXPLICIT fixed32, proto field 3) ----------------------

    pub fn postal_code<'a>(&'a self) -> ::puroro::Optional<u32, impl ::puroro::HasDefault<u32>>
    where
        A: 'a,
    {
        self.postal_code.bind(&self._common).optional()
    }

    pub fn postal_code_mut(&mut self) -> impl DerefMut<Target = u32> + '_ {
        self.postal_code.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_postal_code(&mut self) {
        self.postal_code.bind_mut(&mut self._common).clear();
    }

    // -- latitude (EXPLICIT double, proto field 4) --------------------------

    pub fn latitude<'a>(&'a self) -> ::puroro::Optional<f64, impl ::puroro::HasDefault<f64>>
    where
        A: 'a,
    {
        self.latitude.bind(&self._common).optional()
    }

    pub fn latitude_mut(&mut self) -> impl DerefMut<Target = f64> + '_ {
        self.latitude.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_latitude(&mut self) {
        self.latitude.bind_mut(&mut self._common).clear();
    }

    // -- field visitors (single enumeration for Eq / Debug / Drop) ----------

    pub fn visit_fields<V: FieldVisitor<AddressPresence, A>>(
        &self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        let c = &self._common;
        v.visit("street", c, &self.street)?;
        v.visit("city", c, &self.city)?;
        v.visit("postal_code", c, &self.postal_code)?;
        v.visit("latitude", c, &self.latitude)?;
        ControlFlow::Continue(())
    }

    pub fn visit_fields_with<V: FieldPairVisitor<AddressPresence, A>>(
        &self,
        other: &Self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        let c = &self._common;
        let o = &other._common;
        v.visit("street", c, &self.street, o, &other.street)?;
        v.visit("city", c, &self.city, o, &other.city)?;
        v.visit("postal_code", c, &self.postal_code, o, &other.postal_code)?;
        v.visit("latitude", c, &self.latitude, o, &other.latitude)?;
        ControlFlow::Continue(())
    }

    pub fn visit_fields_mut<V: FieldVisitorMut<AddressPresence, A>>(
        &mut self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        let c = &self._common;
        v.visit_mut("street", c, &mut self.street)?;
        v.visit_mut("city", c, &mut self.city)?;
        v.visit_mut("postal_code", c, &mut self.postal_code)?;
        v.visit_mut("latitude", c, &mut self.latitude)?;
        ControlFlow::Continue(())
    }
}

impl Address<Global> {
    pub fn new() -> Self {
        Self::new_in(Global)
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

impl<A: Allocator + Clone> ::unmanaged::CloneIn<A> for Address<A> {
    fn clone_in(&self, alloc: A) -> Self {
        Self {
            _common: self._common.clone_in(alloc.clone()),
            street: self.street.clone_in(&self._common, alloc.clone()),
            city: self.city.clone_in(&self._common, alloc.clone()),
            postal_code: self.postal_code.clone_in(&self._common, alloc.clone()),
            latitude: self.latitude.clone_in(&self._common, alloc),
        }
    }
}

impl<A: Allocator + Clone> Clone for Address<A> {
    #[inline]
    fn clone(&self) -> Self {
        self.clone_in(self._common.alloc.clone())
    }
}

impl<A: Allocator + Clone> PartialEq for Address<A> {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            self.visit_fields_with(other, &mut FieldEqVisitor),
            ControlFlow::Continue(())
        ) && self._common.unknown_fields_eq(&other._common)
    }
}

impl<A: Allocator + Clone> fmt::Debug for Address<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut v = DebugStructVisitor::new(f.debug_struct("Address"));
        let _ = self.visit_fields(&mut v);
        v.finish()
    }
}

// ---------------------------------------------------------------------------
// Drop — releases every unmanaged field through the single allocator
// ---------------------------------------------------------------------------

impl<A: Allocator + Clone> Drop for Address<A> {
    fn drop(&mut self) {
        let _ = self.visit_fields_mut(&mut FieldDeallocVisitor);
        self._common.deallocate();
    }
}

// ---------------------------------------------------------------------------
// unmanaged::DeallocateIn — required for nested `UnmanagedBox` / catalog bounds
// ---------------------------------------------------------------------------

impl<A: Allocator + Clone> ::unmanaged::DeallocateIn<A> for Address<A> {
    #[inline]
    unsafe fn deallocate_in(self, _alloc: A) {
        // Heap is owned by `self._common.alloc`; parent-passed `alloc` is only
        // needed when freeing an enclosing `UnmanagedBox` slot.
        drop(self);
    }
}

// ---------------------------------------------------------------------------
// Message
// ---------------------------------------------------------------------------

impl<A: Allocator + Clone> Message for Address<A> {
    type Alloc = A;

    fn new_in(alloc: A) -> Self {
        Self::new_in(alloc)
    }

    fn encoded_len(&self) -> usize {
        let c = &self._common;
        self.street.encoded_len(c)
            + self.city.encoded_len(c)
            + self.postal_code.encoded_len(c)
            + self.latitude.encoded_len(c)
            + c.unknown_fields.len()
    }

    fn encode_raw<B: BufMut>(&self, buf: &mut B) {
        let c = &self._common;
        self.street.encode_raw(c, buf);
        self.city.encode_raw(c, buf);
        self.postal_code.encode_raw(c, buf);
        self.latitude.encode_raw(c, buf);
        let unknown: &[u8] = &c.unknown_fields;
        buf.put_slice(unknown);
    }

    fn merge_from_with_depth<B: Buf>(
        &mut self,
        buf: &mut B,
        depth: usize,
    ) -> Result<(), DecodeError> {
        if depth >= ::puroro::RECURSION_LIMIT {
            return Err(DecodeError::RecursionLimitExceeded);
        }
        while buf.has_remaining() {
            let (field_number, wire_type) = decode_tag(buf)?;
            match field_number {
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

    fn unknown_fields(&self) -> impl Iterator<Item = ::puroro::UnknownField<'_>> + '_ {
        self._common.iter_unknown_fields()
    }

    fn validate(&self) -> Result<(), DecodeError> {
        Ok(())
    }
}
