//! Sample of the `oneof notification` types puroro generates for `Task`.
//!
//! A oneof that owns heap storage is represented by **four** generated types,
//! because the owned storage holds allocator-less `unmanaged` values (unsafe to
//! drop implicitly) and must not leak into the public API:
//!
//! - [`NotificationStorage`] — the owned storage enum (`pub(crate)`, deliberately
//!   *not* the canonical `Notification` name): holds `UnmanagedString`, implements
//!   [`OneofDeallocate`], and carries the group's field-number constants plus the
//!   encode / merge glue. Never public.
//! - [`NotificationCase`] — a payload-less, `Copy` discriminant of which variant
//!   is active (`NotSet` when unset).
//! - [`NotificationRef`] — a safe borrowed read view (`&str` payloads).
//! - [`NotificationMut`] — a safe borrowed mutable view (growable string guards).
//!
//! The group carries no presence bit, so the merge helpers are generic over the
//! parent's `PresenceBits` type and only reach `MessageCommon` for the allocator
//! (through the `bind` view).

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};
use ::puroro::{DecodeError, OneofDeallocate, OneofSlot, OneofSlotMut, PresenceBits, WireType};
use ::unmanaged::string::StringGuard;
use ::unmanaged::UnmanagedString;

/// Which variant of `oneof notification` is set — a payload-less discriminant.
///
/// Backs `Task::notification_case`, which returns `Option<NotificationCase>`;
/// the unset group is `None`, so this enum mirrors the real variants 1:1 (no
/// `NotSet` sentinel).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotificationCase {
    EmailAddress,
    PhoneNumber,
}

/// Borrowed read view of the active `notification` variant.
///
/// Backs `Task::notification`; the payloads borrow the message (no allocator and
/// no `unmanaged` type in sight).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotificationRef<'a> {
    EmailAddress(&'a str),
    PhoneNumber(&'a str),
}

/// Borrowed mutable view of the active `notification` variant.
///
/// Backs `Task::notification_mut`; each variant hands out a growable guard
/// (`DerefMut<Target = String<A>>`) over the active variant's storage, writing
/// edits back when the guard drops.
pub enum NotificationMut<'a, A: Allocator> {
    EmailAddress(StringGuard<'a, A>),
    PhoneNumber(StringGuard<'a, A>),
}

/// Owned storage for `oneof notification` (crate-internal).
///
/// Variants hold allocator-less [`UnmanagedString`], which cannot free itself;
/// [`OneofDeallocate`] releases the active variant through the message allocator.
/// This type is intentionally **not public** and does **not** take the canonical
/// `Notification` name: exposing an `unmanaged`-holding value by name would let a
/// caller own one and hit the panic-on-implicit-drop footgun. All public access
/// is through [`NotificationCase`] / [`NotificationRef`] / [`NotificationMut`].
pub(crate) enum NotificationStorage {
    EmailAddress(UnmanagedString),
    PhoneNumber(UnmanagedString),
}

impl NotificationStorage {
    /// `email_address` variant field number.
    pub(crate) const FIELD_EMAIL_ADDRESS: u32 = 12;
    /// `phone_number` variant field number.
    pub(crate) const FIELD_PHONE_NUMBER: u32 = 13;

    /// This variant's payload-less discriminant.
    pub(crate) fn case(&self) -> NotificationCase {
        match self {
            Self::EmailAddress(_) => NotificationCase::EmailAddress,
            Self::PhoneNumber(_) => NotificationCase::PhoneNumber,
        }
    }

    /// Safe borrowed read view of this variant.
    pub(crate) fn as_ref(&self) -> NotificationRef<'_> {
        match self {
            Self::EmailAddress(s) => NotificationRef::EmailAddress(s),
            Self::PhoneNumber(s) => NotificationRef::PhoneNumber(s),
        }
    }

    /// Safe borrowed mutable view of this variant, backed by an owned allocator
    /// clone (the guard owns it and writes edits back on drop).
    pub(crate) fn as_mut<A: Allocator>(&mut self, alloc: A) -> NotificationMut<'_, A> {
        match self {
            // SAFETY: an owned clone of the message allocator owns this string's buffer.
            Self::EmailAddress(s) => NotificationMut::EmailAddress(unsafe { s.with_alloc(alloc) }),
            // SAFETY: ditto.
            Self::PhoneNumber(s) => NotificationMut::PhoneNumber(unsafe { s.with_alloc(alloc) }),
        }
    }

    /// Encoded length of the active variant (0 when the group is unset).
    pub(crate) fn encoded_len(slot: &OneofSlot<Self>) -> usize {
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
    pub(crate) fn encode<B: BufMut>(slot: &OneofSlot<Self>, buf: &mut B) {
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

    /// Merges an `email_address` occurrence into the bound oneof slot, switching
    /// the group to that variant (last wins on the wire).
    pub(crate) fn merge_email_address<Pb: PresenceBits, A: Allocator + Clone, B: Buf>(
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
    pub(crate) fn merge_phone_number<Pb: PresenceBits, A: Allocator + Clone, B: Buf>(
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

impl OneofDeallocate for NotificationStorage {
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
