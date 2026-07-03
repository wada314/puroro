//! Sample of the `oneof notification` types puroro generates for `Task`.
//!
//! A oneof that owns heap storage is represented by **four** generated types,
//! because the owned storage holds allocator-less `unmanaged` values (unsafe to
//! drop implicitly) and must not leak into the public API:
//!
//! - [`NotificationStorage`] — the owned storage enum (`pub(crate)`, deliberately
//!   *not* the canonical `Notification` name): each variant owns the same **field
//!   wrapper** a singular field of that kind uses ([`SingularLenField`] here),
//!   implements [`OneofDeallocate`], and carries the encode / merge glue. Never
//!   public.
//! - [`NotificationCase`] — a payload-less, `Copy` discriminant of which variant
//!   is active.
//! - [`NotificationRef`] — a safe borrowed read view (`&str` payloads).
//! - [`NotificationMut`] — a safe borrowed mutable view (growable string guards).
//!
//! The group's field-number constants live at module scope (rather than as
//! associated `const`s) so they stay usable as `match` patterns even though
//! [`NotificationStorage`] is generic over the allocator `A`.
//!
//! The group carries no presence bit, so the merge helpers are generic over the
//! parent's `PresenceBits` type and only reach `MessageCommon` for the allocator
//! (through the `bind` view).

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};
use ::puroro::{
    DecodeError, Implicit, OneofDeallocate, OneofSlot, OneofSlotMut, PresenceBits, ProtoString,
    SingularLenField, WireType,
};
use ::unmanaged::string::StringGuard;

/// `email_address` variant field number.
pub(crate) const FIELD_EMAIL_ADDRESS: u32 = 12;
/// `phone_number` variant field number.
pub(crate) const FIELD_PHONE_NUMBER: u32 = 13;

/// The field wrapper each `string` variant owns: a singular LEN field minus
/// presence.
///
/// The presence policy (`Implicit`) is inert here — the enclosing [`OneofSlot`]
/// tracks which variant is set, and [`NotificationStorage`] frames encode / merge
/// itself, so the wrapper's presence-aware methods are never called. Reusing the
/// singular field wrapper keeps oneof members and ordinary fields uniform (same
/// storage, `value` / `value_mut` / `deallocate`).
type StringVariant<A> = SingularLenField<ProtoString, Implicit, A>;

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
/// Variants own a [`StringVariant`] field wrapper (allocator-less storage that
/// cannot free itself); [`OneofDeallocate`] releases the active variant through
/// the message allocator. This type is intentionally **not public** and does
/// **not** take the canonical `Notification` name: exposing an `unmanaged`-holding
/// value by name would let a caller own one and hit the panic-on-implicit-drop
/// footgun. All public access is through [`NotificationCase`] / [`NotificationRef`]
/// / [`NotificationMut`].
pub(crate) enum NotificationStorage<A: Allocator> {
    EmailAddress(StringVariant<A>),
    PhoneNumber(StringVariant<A>),
}

impl<A: Allocator> NotificationStorage<A> {
    /// This variant's payload-less discriminant.
    pub(crate) fn case(&self) -> NotificationCase {
        match self {
            Self::EmailAddress(_) => NotificationCase::EmailAddress,
            Self::PhoneNumber(_) => NotificationCase::PhoneNumber,
        }
    }

    /// Safe borrowed read view of this variant.
    pub(crate) fn to_ref(&self) -> NotificationRef<'_> {
        match self {
            Self::EmailAddress(f) => NotificationRef::EmailAddress(f.value()),
            Self::PhoneNumber(f) => NotificationRef::PhoneNumber(f.value()),
        }
    }

    /// Safe borrowed mutable view of this variant, backed by an owned allocator
    /// clone (the guard owns it and writes edits back on drop).
    pub(crate) fn to_mut(&mut self, alloc: A) -> NotificationMut<'_, A> {
        match self {
            Self::EmailAddress(f) => NotificationMut::EmailAddress(f.value_mut(alloc)),
            Self::PhoneNumber(f) => NotificationMut::PhoneNumber(f.value_mut(alloc)),
        }
    }

    /// Encoded length of the active variant (0 when the group is unset).
    pub(crate) fn encoded_len(slot: &OneofSlot<Self>) -> usize {
        match slot.get() {
            Some(Self::EmailAddress(f)) => {
                ::puroro::encode::encoded_len_len_field(FIELD_EMAIL_ADDRESS, f.value().len())
            }
            Some(Self::PhoneNumber(f)) => {
                ::puroro::encode::encoded_len_len_field(FIELD_PHONE_NUMBER, f.value().len())
            }
            None => 0,
        }
    }

    /// Encodes the active variant (nothing when the group is unset).
    pub(crate) fn encode<B: BufMut>(slot: &OneofSlot<Self>, buf: &mut B) {
        match slot.get() {
            Some(Self::EmailAddress(f)) => {
                ::puroro::encode::encode_len_field(FIELD_EMAIL_ADDRESS, f.value().as_bytes(), buf);
            }
            Some(Self::PhoneNumber(f)) => {
                ::puroro::encode::encode_len_field(FIELD_PHONE_NUMBER, f.value().as_bytes(), buf);
            }
            None => {}
        }
    }

    /// Merges an `email_address` occurrence into the bound oneof slot, switching
    /// the group to that variant (last wins on the wire).
    pub(crate) fn merge_email_address<Pb: PresenceBits, B: Buf>(
        view: OneofSlotMut<'_, '_, Self, Pb, A>,
        wire_type: WireType,
        buf: &mut B,
    ) -> Result<(), DecodeError>
    where
        A: Clone,
    {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        view.try_set_with(|alloc| {
            Ok(Self::EmailAddress(SingularLenField::from_storage(
                ::puroro::decode::decode_string_in(buf, alloc)?,
            )))
        })
    }

    /// Merges a `phone_number` occurrence into the bound oneof slot, switching
    /// the group to that variant (last wins on the wire).
    pub(crate) fn merge_phone_number<Pb: PresenceBits, B: Buf>(
        view: OneofSlotMut<'_, '_, Self, Pb, A>,
        wire_type: WireType,
        buf: &mut B,
    ) -> Result<(), DecodeError>
    where
        A: Clone,
    {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        view.try_set_with(|alloc| {
            Ok(Self::PhoneNumber(SingularLenField::from_storage(
                ::puroro::decode::decode_string_in(buf, alloc)?,
            )))
        })
    }
}

impl<A: Allocator> OneofDeallocate<A> for NotificationStorage<A> {
    /// Drops the active variant's field wrapper and frees it through `alloc`.
    ///
    /// # Safety
    ///
    /// `alloc` must be the allocator that owns the variant's buffer.
    unsafe fn deallocate(self, alloc: A) {
        match self {
            Self::EmailAddress(mut f) | Self::PhoneNumber(mut f) => f.deallocate(alloc),
        }
    }
}
