//! Sample of the `oneof notification` types puroro generates for `Task`.
//!
//! A oneof that owns heap storage is represented by **four** generated types,
//! because the owned storage holds allocator-less `unmanaged` values (unsafe to
//! drop implicitly) and must not leak into the public API:
//!
//! - [`NotificationStorage`] — the owned storage enum (`pub(crate)`, deliberately
//!   *not* the canonical `Notification` name): each variant owns the same **field
//!   wrapper** a singular field of that kind uses, implements [`OneofDeallocate`],
//!   and carries the encode glue. Never public.
//! - [`NotificationCase`] — a payload-less, `Copy` discriminant of which variant
//!   is active.
//! - [`NotificationRef`] — a safe borrowed read view.
//! - [`NotificationMut`] — a safe borrowed mutable view.
//!
//! This group is deliberately **heterogeneous** to show every field kind:
//!
//! | variant | proto | field wrapper | `Ref` payload | `Mut` payload |
//! |---|---|---|---|---|
//! | `email_address` / `phone_number` | `string` | [`SingularLenField`] | `&str` | string guard |
//! | `webhook_id` | `int32` | [`SingularVarintField`] | `i32` (by value) | `&mut i32` |
//! | `postal` | `Address` message | [`NestedMessageField`] | `&Address<A>` | `&mut Address<A>` |
//!
//! The scalar variant owns no heap, so its `OneofDeallocate` arm is a no-op; the
//! LEN and message variants free their storage through the message allocator.
//!
//! The group's field-number constants live at module scope (rather than as
//! associated `const`s) so they stay usable as `match` patterns even though
//! [`NotificationStorage`] is generic over the allocator `A`.
//!
//! **Merge has no bespoke `merge_*` helpers on this enum.** Because each variant
//! *is* a field wrapper, the parent's `merge_from` selects the variant through
//! `OneofSlot::bind(...).variant_mut(...)` (which frees any other variant) and
//! then merges into it with the field's **own** bind idiom —
//! `field.bind_oneof(common).merge(...)` for LEN / varint variants, or
//! `field.merge(common, ...)` for the message variant (mirroring how ordinary
//! message fields merge). A oneof carries no presence bit; `bind_oneof` exists
//! for exactly that case.

use ::allocator_api2::alloc::Allocator;
use ::bytes::BufMut;
use ::puroro::{
    Implicit, NestedMessageField, OneofDeallocate, OneofSlot, ProtoInt32, ProtoString,
    SingularLenField, SingularVarintField, VarintProtoType,
};
use ::unmanaged::string::StringGuard;

use crate::address::Address;

/// `email_address` variant field number.
pub(crate) const FIELD_EMAIL_ADDRESS: u32 = 12;
/// `phone_number` variant field number.
pub(crate) const FIELD_PHONE_NUMBER: u32 = 13;
/// `webhook_id` variant field number.
pub(crate) const FIELD_WEBHOOK_ID: u32 = 14;
/// `postal` variant field number.
pub(crate) const FIELD_POSTAL: u32 = 15;

/// Which variant of `oneof notification` is set — a payload-less discriminant.
///
/// Backs `Task::notification_case`, which returns `Option<NotificationCase>`;
/// the unset group is `None`, so this enum mirrors the real variants 1:1 (no
/// `NotSet` sentinel).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotificationCase {
    EmailAddress,
    PhoneNumber,
    WebhookId,
    Postal,
}

/// Borrowed read view of the active `notification` variant.
///
/// Backs `Task::notification`; every payload borrows the message (no allocator
/// and no `unmanaged` type in sight). All payloads are `Copy` (scalars, `&str`,
/// and a `&Address<A>` reference), so the view is `Copy` — but the derive is
/// hand-written to avoid a spurious `A: Copy` bound from `#[derive(Copy)]`, and
/// `Debug` / `PartialEq` are omitted because the message payload `Address<A>`
/// implements neither.
pub enum NotificationRef<'a, A: Allocator + Clone> {
    EmailAddress(&'a str),
    PhoneNumber(&'a str),
    WebhookId(i32),
    Postal(&'a Address<A>),
}

impl<A: Allocator + Clone> Clone for NotificationRef<'_, A> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<A: Allocator + Clone> Copy for NotificationRef<'_, A> {}

/// Borrowed mutable view of the active `notification` variant.
///
/// Backs `Task::notification_mut`; string variants hand out a growable guard
/// (`DerefMut<Target = String<A>>`), the scalar variant a `&mut i32`, and the
/// message variant a `&mut Address<A>`. Edits are written back live (the string
/// guard on drop; the `&mut` targets in place).
pub enum NotificationMut<'a, A: Allocator + Clone> {
    EmailAddress(StringGuard<'a, A>),
    PhoneNumber(StringGuard<'a, A>),
    WebhookId(&'a mut i32),
    Postal(&'a mut Address<A>),
}

/// Owned storage for `oneof notification` (crate-internal).
///
/// Each variant owns the field wrapper an ordinary singular field of that kind
/// would use — allocator-less storage that cannot free itself; [`OneofDeallocate`]
/// releases the active variant through the message allocator (a no-op for the
/// scalar variant). This type is intentionally **not public** and does **not**
/// take the canonical `Notification` name: exposing an `unmanaged`-holding value
/// by name would let a caller own one and hit the panic-on-implicit-drop footgun.
/// All public access is through [`NotificationCase`] / [`NotificationRef`] /
/// [`NotificationMut`].
///
/// The LEN/varint wrappers carry a presence policy (`Implicit`) that is inert
/// here: the enclosing [`OneofSlot`] tracks which variant is set, and this enum
/// frames encode / merge itself, so the wrappers' presence-aware methods are
/// never called.
pub(crate) enum NotificationStorage<A: Allocator + Clone> {
    EmailAddress(SingularLenField<ProtoString, Implicit, A>),
    PhoneNumber(SingularLenField<ProtoString, Implicit, A>),
    WebhookId(SingularVarintField<ProtoInt32, Implicit>),
    Postal(NestedMessageField<Address<A>, A>),
}

impl<A: Allocator + Clone> NotificationStorage<A> {
    /// This variant's payload-less discriminant.
    pub(crate) fn case(&self) -> NotificationCase {
        match self {
            Self::EmailAddress(_) => NotificationCase::EmailAddress,
            Self::PhoneNumber(_) => NotificationCase::PhoneNumber,
            Self::WebhookId(_) => NotificationCase::WebhookId,
            Self::Postal(_) => NotificationCase::Postal,
        }
    }

    /// Safe borrowed read view of this variant.
    pub(crate) fn to_ref(&self) -> NotificationRef<'_, A> {
        match self {
            Self::EmailAddress(f) => NotificationRef::EmailAddress(f.value()),
            Self::PhoneNumber(f) => NotificationRef::PhoneNumber(f.value()),
            Self::WebhookId(f) => NotificationRef::WebhookId(f.value()),
            // The variant invariant guarantees the child is present.
            Self::Postal(f) => NotificationRef::Postal(f.get().unwrap()),
        }
    }

    /// Safe borrowed mutable view of this variant. `alloc` (an owned message
    /// allocator clone) backs the string guards; the other variants ignore it.
    pub(crate) fn to_mut(&mut self, alloc: A) -> NotificationMut<'_, A> {
        match self {
            Self::EmailAddress(f) => NotificationMut::EmailAddress(f.value_mut(alloc)),
            Self::PhoneNumber(f) => NotificationMut::PhoneNumber(f.value_mut(alloc)),
            Self::WebhookId(f) => NotificationMut::WebhookId(f.value_mut()),
            // The variant invariant guarantees the child is present.
            Self::Postal(f) => NotificationMut::Postal(f.get_present_mut().unwrap()),
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
            Some(Self::WebhookId(f)) => ::puroro::encode::encoded_len_varint_field(
                FIELD_WEBHOOK_ID,
                ProtoInt32::encode_wire(f.value()),
            ),
            Some(Self::Postal(f)) => f.encoded_len(FIELD_POSTAL),
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
            Some(Self::WebhookId(f)) => {
                ::puroro::encode::encode_varint_field(
                    FIELD_WEBHOOK_ID,
                    ProtoInt32::encode_wire(f.value()),
                    buf,
                );
            }
            Some(Self::Postal(f)) => f.encode_raw(FIELD_POSTAL, buf),
            None => {}
        }
    }
}

impl<A: Allocator + Clone> OneofDeallocate<A> for NotificationStorage<A> {
    /// Frees the active variant through `alloc`: LEN and message variants release
    /// their heap storage; the scalar variant has nothing to free.
    ///
    /// # Safety
    ///
    /// `alloc` must be the allocator that owns the variant's buffer.
    unsafe fn deallocate(self, alloc: A) {
        match self {
            Self::EmailAddress(mut f) | Self::PhoneNumber(mut f) => f.deallocate(alloc),
            Self::WebhookId(_) => {}
            Self::Postal(mut f) => f.deallocate(alloc),
        }
    }
}
