//! Sample of the code puroro generates for message `example.Address`
//! (from `example.proto`).

mod defaults;

use ::allocator_api2::alloc::{Allocator, Global};
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;
use ::bytes::{Buf, BufMut};
use ::core::ops::DerefMut;

use ::puroro::{DecodeError, MessageDecode, MessageEncode};
use ::puroro_rt::{
    Explicit, FieldDeallocate, MessageCommon, NestedMessage, PresenceBits, ProtoString,
    SingularAccess, SingularLenField,
};

// ---------------------------------------------------------------------------
// Presence bitfield (2 tracked singular fields)
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

    fn bit_mut(
        &mut self,
        bit: usize,
    ) -> ::bitvec::ptr::BitRef<'_, ::bitvec::ptr::Mut, u8, Lsb0> {
        self.0
            .get_mut(bit)
            .expect("presence bit index in range")
    }
}

// ---------------------------------------------------------------------------
// Presence bit indices (2 tracked singular fields)
// ---------------------------------------------------------------------------

pub const BIT_STREET: usize = 0; // street (EXPLICIT)
pub const BIT_CITY: usize = 1; // city (EXPLICIT)

// ---------------------------------------------------------------------------
// Proto field numbers
// ---------------------------------------------------------------------------

pub const FIELD_STREET: u32 = 1; // street
pub const FIELD_CITY: u32 = 2; // city

// ---------------------------------------------------------------------------
// Message struct
// ---------------------------------------------------------------------------

pub struct Address<A: Allocator + Clone = Global> {
    _common: MessageCommon<AddressPresence, A>,
    street: SingularLenField<ProtoString, Explicit<{ BIT_STREET }>, { FIELD_STREET }>, // proto: string street = 1;
    city: SingularLenField<ProtoString, Explicit<{ BIT_CITY }>, { FIELD_CITY }>, // proto: string city = 2;
}

impl<A: Allocator + Clone> Address<A> {
    pub fn new_in(alloc: A) -> Self {
        // Each field initializer gets its own clone of the allocator; the last
        // heap field (`city`) takes the original by move.
        Self {
            _common: MessageCommon::new_in(AddressPresence::ZERO, alloc.clone()),
            street: SingularLenField::new_in(alloc.clone()),
            city: SingularLenField::new_in(alloc),
        }
    }

    // -- street (EXPLICIT string, proto field 1) ----------------------------

    pub fn street<'a>(&'a self) -> ::puroro::Optional<&'a str, impl ::puroro::HasDefault<&'a str>> {
        self.street.bind(&self._common).optional()
    }

    pub fn street_mut<'s>(
        &'s mut self,
    ) -> impl DerefMut<Target = ::unmanaged::String<A>> + 's {
        self.street.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_street(&mut self) {
        self.street.bind_mut(&mut self._common).clear();
    }

    // -- city (EXPLICIT string, proto field 2) ------------------------------

    pub fn city<'a>(&'a self) -> ::puroro::Optional<&'a str, impl ::puroro::HasDefault<&'a str>> {
        self.city.bind(&self._common).optional()
    }

    pub fn city_mut<'s>(
        &'s mut self,
    ) -> impl DerefMut<Target = ::unmanaged::String<A>> + 's {
        self.city.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_city(&mut self) {
        self.city.bind_mut(&mut self._common).clear();
    }

    pub fn unknown_fields(&self) -> &[u8] {
        &self._common.unknown_fields
    }
}

impl Address<::allocator_api2::alloc::Global> {
    pub fn new() -> Self {
        Self::new_in(Global)
    }
}

impl<A: Allocator + Clone + Default> Default for Address<A> {
    fn default() -> Self {
        Self::new_in(A::default())
    }
}

impl<A: Allocator + Clone> NestedMessage<A> for Address<A> {
    fn new_in(alloc: A) -> Self {
        Self::new_in(alloc)
    }
}

// ---------------------------------------------------------------------------
// Drop — releases every unmanaged field through the single allocator
// ---------------------------------------------------------------------------

impl<A: Allocator + Clone> Drop for Address<A> {
    fn drop(&mut self) {
        self.street.deallocate(&self._common);
        self.city.deallocate(&self._common);
        self._common.deallocate();
    }
}

// ---------------------------------------------------------------------------
// MessageEncode / MessageDecode
// ---------------------------------------------------------------------------

impl<A: Allocator + Clone> MessageEncode for Address<A> {
    fn encoded_len(&self) -> usize {
        let c = &self._common;
        self.street.encoded_len(c) + self.city.encoded_len(c) + c.unknown_fields.len()
    }

    fn encode_raw<B: BufMut>(&self, buf: &mut B) {
        let c = &self._common;
        self.street.encode_raw(c, buf);
        self.city.encode_raw(c, buf);
        let unknown: &[u8] = &c.unknown_fields;
        buf.put_slice(unknown);
    }
}

impl<A: Allocator + Clone> MessageDecode for Address<A> {
    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError> {
        while buf.has_remaining() {
            let (field_number, wire_type) = ::puroro_rt::decode::decode_tag(buf)?;
            match field_number {
                FIELD_STREET => {
                    // street = 1, EXPLICIT string
                    self.street
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf)?;
                }
                FIELD_CITY => {
                    // city = 2, EXPLICIT string
                    self.city
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf)?;
                }
                _ => {
                    // unknown field — preserve in _common.unknown_fields
                    ::puroro_rt::decode::skip_field_and_save(
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
