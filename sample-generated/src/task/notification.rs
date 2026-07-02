//! Sample of the `oneof notification` enum puroro generates for `Task`.
//!
//! The whole oneof group — its variant field numbers plus its encode / merge
//! glue — lives here, next to the enum, rather than as free functions on the
//! parent message. The group carries no presence bit, so the merge helpers are
//! generic over the parent's `PresenceBits` type and only borrow `MessageCommon`
//! for the allocator (through the `bind` view).

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};
use ::puroro::{
    DecodeError, OneofDeallocate, OneofSlot, OneofSlotMut, PresenceBits, WireType,
};
use ::unmanaged::UnmanagedString;

/// `oneof notification { string email_address = 12; string phone_number = 13; }`
///
/// Variants hold allocator-less [`UnmanagedString`]; the parent `Task` releases
/// the active variant via the [`OneofDeallocate`] impl (from its `Drop` and
/// before overwriting the oneof).
pub enum Notification {
    EmailAddress(UnmanagedString),
    PhoneNumber(UnmanagedString),
}

impl Notification {
    /// `email_address` variant field number.
    pub const FIELD_EMAIL_ADDRESS: u32 = 12;
    /// `phone_number` variant field number.
    pub const FIELD_PHONE_NUMBER: u32 = 13;

    /// Encoded length of the active variant (0 when the group is unset).
    pub fn encoded_len(slot: &OneofSlot<Self>) -> usize {
        match slot.get() {
            Some(Self::EmailAddress(s)) => {
                ::puroro::encode::encoded_len_len_field(Self::FIELD_EMAIL_ADDRESS, s.len())
            }
            Some(Self::PhoneNumber(s)) => {
                ::puroro::encode::encoded_len_len_field(Self::FIELD_PHONE_NUMBER, s.len())
            }
            None => 0,
        }
    }

    /// Encodes the active variant (nothing when the group is unset).
    pub fn encode<B: BufMut>(slot: &OneofSlot<Self>, buf: &mut B) {
        match slot.get() {
            Some(Self::EmailAddress(s)) => {
                ::puroro::encode::encode_len_field(Self::FIELD_EMAIL_ADDRESS, s.as_bytes(), buf);
            }
            Some(Self::PhoneNumber(s)) => {
                ::puroro::encode::encode_len_field(Self::FIELD_PHONE_NUMBER, s.as_bytes(), buf);
            }
            None => {}
        }
    }

    /// Merges an `email_address` occurrence into the bound oneof slot,
    /// switching the group to that variant (last wins on the wire).
    pub fn merge_email_address<Pb: PresenceBits, A: Allocator + Clone, B: Buf>(
        view: OneofSlotMut<'_, '_, Self, Pb, A>,
        wire_type: WireType,
        buf: &mut B,
    ) -> Result<(), DecodeError> {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        view.try_set_with(|alloc| {
            Ok(Self::EmailAddress(::puroro::decode::decode_string_in(
                buf, alloc,
            )?))
        })
    }

    /// Merges a `phone_number` occurrence into the bound oneof slot, switching
    /// the group to that variant (last wins on the wire).
    pub fn merge_phone_number<Pb: PresenceBits, A: Allocator + Clone, B: Buf>(
        view: OneofSlotMut<'_, '_, Self, Pb, A>,
        wire_type: WireType,
        buf: &mut B,
    ) -> Result<(), DecodeError> {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        view.try_set_with(|alloc| {
            Ok(Self::PhoneNumber(::puroro::decode::decode_string_in(
                buf, alloc,
            )?))
        })
    }
}

impl OneofDeallocate for Notification {
    /// Drops the active variant's string and frees it through `alloc`.
    ///
    /// # Safety
    ///
    /// `alloc` must be the allocator that owns the variant's buffer.
    unsafe fn deallocate<A: Allocator>(self, alloc: A) {
        match self {
            Self::EmailAddress(s) | Self::PhoneNumber(s) => unsafe { s.deallocate(alloc) },
        }
    }
}

impl ::core::fmt::Debug for Notification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Self::EmailAddress(s) => f.debug_tuple("EmailAddress").field(&&**s).finish(),
            Self::PhoneNumber(s) => f.debug_tuple("PhoneNumber").field(&&**s).finish(),
        }
    }
}

impl PartialEq for Notification {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::EmailAddress(a), Self::EmailAddress(b)) => **a == **b,
            (Self::PhoneNumber(a), Self::PhoneNumber(b)) => **a == **b,
            _ => false,
        }
    }
}

impl Eq for Notification {}
