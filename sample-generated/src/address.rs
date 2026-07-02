//! @generated from example.proto — do not edit
//! Message `example.Address`

use ::allocator_api2::alloc::{Allocator, Global};
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;
use ::bytes::{Buf, BufMut};

use ::puroro::{
    DecodeError, Explicit, MessageCommon, MessageDecode, MessageEncode, NestedMessage,
    PresenceBits, ProtoString, SingularLenField,
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
}

// ---------------------------------------------------------------------------
// Message struct
// ---------------------------------------------------------------------------

pub struct Address<A: Allocator + Clone = Global> {
    _common: MessageCommon<AddressPresence, A>,
    street: SingularLenField<ProtoString, Explicit, A>, // proto: string street = 1;
    city: SingularLenField<ProtoString, Explicit, A>,   // proto: string city = 2;
}

// ---------------------------------------------------------------------------
// Field constants (associated with `Address`)
// ---------------------------------------------------------------------------

impl<A: Allocator + Clone> Address<A> {
    pub const FIELD_STREET: u32 = 1; // street
    pub const FIELD_CITY: u32 = 2; // city

    pub const BIT_STREET: usize = 0; // street (EXPLICIT)
    pub const BIT_CITY: usize = 1; // city (EXPLICIT)
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

    pub fn street<'a>(
        &'a self,
    ) -> ::puroro::Optional<&'a str, impl ::puroro::HasDefault<&'a str>> {
        struct StreetDefault;
        impl<'a> ::puroro::HasDefault<&'a str> for StreetDefault {
            const DEFAULT: &'a str = "";
        }
        self.street.optional(&self._common, Self::BIT_STREET, StreetDefault)
    }

    pub fn has_street(&self) -> bool {
        self.street.has(&self._common, Self::BIT_STREET)
    }

    pub fn street_mut<'s>(
        &'s mut self,
    ) -> impl ::core::ops::DerefMut<Target = ::unmanaged::String<A>> + 's {
        self._common.set_presence(Self::BIT_STREET, true);
        self.street.value_mut(self._common.alloc.clone())
    }

    pub fn clear_street(&mut self) {
        self.street.clear(&mut self._common, Self::BIT_STREET);
    }

    // -- city (EXPLICIT string, proto field 2) ------------------------------

    pub fn city<'a>(&'a self) -> ::puroro::Optional<&'a str, impl ::puroro::HasDefault<&'a str>> {
        struct CityDefault;
        impl<'a> ::puroro::HasDefault<&'a str> for CityDefault {
            const DEFAULT: &'a str = "";
        }
        self.city.optional(&self._common, Self::BIT_CITY, CityDefault)
    }

    pub fn has_city(&self) -> bool {
        self.city.has(&self._common, Self::BIT_CITY)
    }

    pub fn city_mut<'s>(
        &'s mut self,
    ) -> impl ::core::ops::DerefMut<Target = ::unmanaged::String<A>> + 's {
        self._common.set_presence(Self::BIT_CITY, true);
        self.city.value_mut(self._common.alloc.clone())
    }

    pub fn clear_city(&mut self) {
        self.city.clear(&mut self._common, Self::BIT_CITY);
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
        self.street.deallocate(self._common.alloc.clone());
        self.city.deallocate(self._common.alloc.clone());
        self._common.deallocate();
    }
}

// ---------------------------------------------------------------------------
// MessageEncode / MessageDecode
// ---------------------------------------------------------------------------

impl<A: Allocator + Clone> MessageEncode for Address<A> {
    fn encoded_len(&self) -> usize {
        let c = &self._common;
        self.street.encoded_len(c, Self::FIELD_STREET, Self::BIT_STREET)
            + self.city.encoded_len(c, Self::FIELD_CITY, Self::BIT_CITY)
            + c.unknown_fields.len()
    }

    fn encode_raw<B: BufMut>(&self, buf: &mut B) {
        let c = &self._common;
        self.street
            .encode_raw(c, Self::FIELD_STREET, Self::BIT_STREET, buf);
        self.city
            .encode_raw(c, Self::FIELD_CITY, Self::BIT_CITY, buf);
        let unknown: &[u8] = &c.unknown_fields;
        buf.put_slice(unknown);
    }
}

impl<A: Allocator + Clone> MessageDecode for Address<A> {
    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError> {
        while buf.has_remaining() {
            let (field_number, wire_type) = ::puroro::decode::decode_tag(buf)?;
            match field_number {
                Self::FIELD_STREET => {
                    // street = 1, EXPLICIT string
                    self.street
                        .merge(&mut self._common, Self::BIT_STREET, wire_type, buf)?;
                }
                Self::FIELD_CITY => {
                    // city = 2, EXPLICIT string
                    self.city
                        .merge(&mut self._common, Self::BIT_CITY, wire_type, buf)?;
                }
                _ => {
                    // unknown field — preserve in _common.unknown_fields
                    ::puroro::decode::skip_field_and_save(
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
