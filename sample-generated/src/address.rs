//! Generated `Address` message (`example.Address`).

use ::allocator_api2::alloc::{Allocator, Global};
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;
use ::bytes::{Buf, BufMut};

use ::puroro::{
    DecodeError, ExplicitString, MessageCommon, MessageDecode, MessageEncode, NestedMessage,
    PresenceBits,
};

// ---------------------------------------------------------------------------
// Presence bitfield (2 EXPLICIT string fields)
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
// Field constants
// ---------------------------------------------------------------------------

const FIELD_STREET: u32 = 1;
const FIELD_CITY: u32 = 2;

const BIT_STREET: usize = 0;
const BIT_CITY: usize = 1;

// ---------------------------------------------------------------------------
// Message struct
// ---------------------------------------------------------------------------

/// `message Address { string street = 1; string city = 2; }`
pub struct Address<A: Allocator = Global> {
    _common: MessageCommon<AddressPresence, A>,
    street: ExplicitString<A>,
    city: ExplicitString<A>,
}

impl<A: Allocator + Clone> Address<A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            _common: MessageCommon::new_in(AddressPresence::ZERO, alloc.clone()),
            street: ExplicitString::new_in(alloc.clone()),
            city: ExplicitString::new_in(alloc),
        }
    }

    // -- street (EXPLICIT string, field 1) ----------------------------------

    pub fn street<'a>(
        &'a self,
    ) -> ::puroro::Optional<&'a str, impl ::puroro::HasDefault<&'a str>> {
        struct StreetDefault;
        impl<'a> ::puroro::HasDefault<&'a str> for StreetDefault {
            const DEFAULT: &'a str = "";
        }
        self.street.optional::<_, _, BIT_STREET>(&self._common, StreetDefault)
    }

    pub fn has_street(&self) -> bool {
        self.street.has::<_, BIT_STREET>(&self._common)
    }

    pub fn set_street(&mut self, v: &str) {
        self.street.set_str::<_, BIT_STREET>(&mut self._common, v);
    }

    pub fn clear_street(&mut self) {
        self.street.clear::<_, BIT_STREET>(&mut self._common);
    }

    // -- city (EXPLICIT string, field 2) ------------------------------------

    pub fn city<'a>(&'a self) -> ::puroro::Optional<&'a str, impl ::puroro::HasDefault<&'a str>> {
        struct CityDefault;
        impl<'a> ::puroro::HasDefault<&'a str> for CityDefault {
            const DEFAULT: &'a str = "";
        }
        self.city.optional::<_, _, BIT_CITY>(&self._common, CityDefault)
    }

    pub fn has_city(&self) -> bool {
        self.city.has::<_, BIT_CITY>(&self._common)
    }

    pub fn set_city(&mut self, v: &str) {
        self.city.set_str::<_, BIT_CITY>(&mut self._common, v);
    }

    pub fn clear_city(&mut self) {
        self.city.clear::<_, BIT_CITY>(&mut self._common);
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

impl<A: Allocator + Clone> MessageEncode for Address<A> {
    fn encoded_len(&self) -> usize {
        let c = &self._common;
        self.street.encoded_len::<_, FIELD_STREET, BIT_STREET>(c)
            + self.city.encoded_len::<_, FIELD_CITY, BIT_CITY>(c)
            + c.unknown_fields.len()
    }

    fn encode_raw<B: BufMut>(&self, buf: &mut B) {
        let c = &self._common;
        self.street
            .encode_raw::<_, _, FIELD_STREET, BIT_STREET>(c, buf);
        self.city
            .encode_raw::<_, _, FIELD_CITY, BIT_CITY>(c, buf);
        buf.put_slice(&c.unknown_fields);
    }
}

impl<A: Allocator + Clone> MessageDecode for Address<A> {
    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError> {
        while buf.has_remaining() {
            let (field_number, wire_type) = ::puroro::decode::decode_tag(buf)?;
            match field_number {
                FIELD_STREET => {
                    self.street
                        .merge::<_, _, BIT_STREET>(&mut self._common, wire_type, buf)?;
                }
                FIELD_CITY => {
                    self.city
                        .merge::<_, _, BIT_CITY>(&mut self._common, wire_type, buf)?;
                }
                _ => ::puroro::decode::skip_field_and_save(
                    field_number,
                    wire_type,
                    buf,
                    &mut self._common.unknown_fields,
                )?,
            }
        }
        Ok(())
    }
}
