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
//! Variant proto field numbers are supplied as separate const generic parameters
//! on [`NotificationStorage`] (Rust has no variable-length const-generic list).
//! The parent message module defines named `FIELD_*` constants and passes them
//! when instantiating the storage type, so `merge_from` match arms stay readable.
//!
//! **Merge has no bespoke `merge_*` helpers on this enum.** Because each variant
//! *is* a field wrapper, the parent's `merge_from` selects the variant through
//! `OneofSlot::bind(...).variant_mut(...)` (which frees any other variant) and
//! then merges into it with the field's **own** bind idiom —
//! `field.bind(common).merge(...)` for every variant kind.

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};
use ::puroro::{
    DecodeError, MessageCommon, NestedMessageField, Oneof, OneofDeallocate, OneofEncodable, OneofGroup,
    OneofSlot, PresenceBits, ProtoInt32, ProtoString, SingularLenField, SingularVarintField,
    WireType,
};
use ::unmanaged::string::StringGuard;

use crate::address::Address;

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
pub enum NotificationMut<'a, A: Allocator + Clone> {
    EmailAddress(StringGuard<'a, A>),
    PhoneNumber(StringGuard<'a, A>),
    WebhookId(&'a mut i32),
    Postal(&'a mut Address<A>),
}

/// Owned storage for `oneof notification` (crate-internal).
///
/// Parametrised by the allocator `A` and one const generic per variant proto
/// field number. Each variant's field wrapper reuses the corresponding parameter
/// as its `FIELD` type argument.
pub(crate) enum NotificationStorage<
    A: Allocator + Clone,
    const FIELD_EMAIL_ADDRESS: u32,
    const FIELD_PHONE_NUMBER: u32,
    const FIELD_WEBHOOK_ID: u32,
    const FIELD_POSTAL: u32,
> {
    EmailAddress(SingularLenField<ProtoString, Oneof, { FIELD_EMAIL_ADDRESS }, A>),
    PhoneNumber(SingularLenField<ProtoString, Oneof, { FIELD_PHONE_NUMBER }, A>),
    WebhookId(SingularVarintField<ProtoInt32, Oneof, { FIELD_WEBHOOK_ID }>),
    Postal(NestedMessageField<Address<A>, Oneof, { FIELD_POSTAL }, A>),
}

impl<
        A: Allocator + Clone,
        const FIELD_EMAIL_ADDRESS: u32,
        const FIELD_PHONE_NUMBER: u32,
        const FIELD_WEBHOOK_ID: u32,
        const FIELD_POSTAL: u32,
    > NotificationStorage<A, FIELD_EMAIL_ADDRESS, FIELD_PHONE_NUMBER, FIELD_WEBHOOK_ID, FIELD_POSTAL>
{
    pub(crate) fn case(&self) -> NotificationCase {
        match self {
            Self::EmailAddress(_) => NotificationCase::EmailAddress,
            Self::PhoneNumber(_) => NotificationCase::PhoneNumber,
            Self::WebhookId(_) => NotificationCase::WebhookId,
            Self::Postal(_) => NotificationCase::Postal,
        }
    }

    pub(crate) fn to_ref(&self) -> NotificationRef<'_, A> {
        match self {
            Self::EmailAddress(f) => NotificationRef::EmailAddress(f.value()),
            Self::PhoneNumber(f) => NotificationRef::PhoneNumber(f.value()),
            Self::WebhookId(f) => NotificationRef::WebhookId(f.value()),
            Self::Postal(f) => NotificationRef::Postal(f.value()),
        }
    }

    pub(crate) fn to_mut(&mut self, alloc: A) -> NotificationMut<'_, A> {
        match self {
            Self::EmailAddress(f) => NotificationMut::EmailAddress(f.value_mut(alloc)),
            Self::PhoneNumber(f) => NotificationMut::PhoneNumber(f.value_mut(alloc)),
            Self::WebhookId(f) => NotificationMut::WebhookId(f.value_mut()),
            Self::Postal(f) => NotificationMut::Postal(f.value_mut()),
        }
    }

    pub(crate) fn bind_email_address_mut<'f, Pb: PresenceBits>(
        slot: &'f mut OneofSlot<Self>,
        common: &mut MessageCommon<Pb, A>,
    ) -> &'f mut SingularLenField<ProtoString, Oneof, { FIELD_EMAIL_ADDRESS }, A> {
        let variant = slot.bind(common).variant_mut(
            |n| matches!(n, Self::EmailAddress(_)),
            |alloc| Self::EmailAddress(SingularLenField::new_in(alloc)),
        );
        let Self::EmailAddress(f) = variant else {
            unreachable!()
        };
        f
    }

    pub(crate) fn bind_phone_number_mut<'f, Pb: PresenceBits>(
        slot: &'f mut OneofSlot<Self>,
        common: &mut MessageCommon<Pb, A>,
    ) -> &'f mut SingularLenField<ProtoString, Oneof, { FIELD_PHONE_NUMBER }, A> {
        let variant = slot.bind(common).variant_mut(
            |n| matches!(n, Self::PhoneNumber(_)),
            |alloc| Self::PhoneNumber(SingularLenField::new_in(alloc)),
        );
        let Self::PhoneNumber(f) = variant else {
            unreachable!()
        };
        f
    }

    pub(crate) fn bind_webhook_id_mut<'f, Pb: PresenceBits>(
        slot: &'f mut OneofSlot<Self>,
        common: &mut MessageCommon<Pb, A>,
    ) -> &'f mut SingularVarintField<ProtoInt32, Oneof, { FIELD_WEBHOOK_ID }> {
        let variant = slot.bind(common).variant_mut(
            |n| matches!(n, Self::WebhookId(_)),
            |_alloc| Self::WebhookId(SingularVarintField::new()),
        );
        let Self::WebhookId(f) = variant else {
            unreachable!()
        };
        f
    }

    pub(crate) fn bind_postal_mut<'f, Pb: PresenceBits>(
        slot: &'f mut OneofSlot<Self>,
        common: &mut MessageCommon<Pb, A>,
    ) -> &'f mut NestedMessageField<Address<A>, Oneof, { FIELD_POSTAL }, A> {
        let variant = slot.bind(common).variant_mut(
            |n| matches!(n, Self::Postal(_)),
            |alloc| Self::Postal(NestedMessageField::with_message_in(alloc)),
        );
        let Self::Postal(f) = variant else {
            unreachable!()
        };
        f
    }
}

impl<
        A: Allocator + Clone,
        const FIELD_EMAIL_ADDRESS: u32,
        const FIELD_PHONE_NUMBER: u32,
        const FIELD_WEBHOOK_ID: u32,
        const FIELD_POSTAL: u32,
    > OneofEncodable
    for NotificationStorage<A, FIELD_EMAIL_ADDRESS, FIELD_PHONE_NUMBER, FIELD_WEBHOOK_ID, FIELD_POSTAL>
{
    fn encoded_len_wire(&self) -> usize {
        match self {
            Self::EmailAddress(f) => f.encoded_len_wire(),
            Self::PhoneNumber(f) => f.encoded_len_wire(),
            Self::WebhookId(f) => f.encoded_len_wire(),
            Self::Postal(f) => f.encoded_len_wire(),
        }
    }

    fn encode_raw_wire<B: BufMut>(&self, buf: &mut B) {
        match self {
            Self::EmailAddress(f) => f.encode_raw_wire(buf),
            Self::PhoneNumber(f) => f.encode_raw_wire(buf),
            Self::WebhookId(f) => f.encode_raw_wire(buf),
            Self::Postal(f) => f.encode_raw_wire(buf),
        }
    }
}

impl<
        A: Allocator + Clone,
        const FIELD_EMAIL_ADDRESS: u32,
        const FIELD_PHONE_NUMBER: u32,
        const FIELD_WEBHOOK_ID: u32,
        const FIELD_POSTAL: u32,
    > OneofGroup<A>
    for NotificationStorage<A, FIELD_EMAIL_ADDRESS, FIELD_PHONE_NUMBER, FIELD_WEBHOOK_ID, FIELD_POSTAL>
{
    fn merge_wire<Pb, B>(
        slot: &mut OneofSlot<Self>,
        common: &mut MessageCommon<Pb, A>,
        field_number: u32,
        wire_type: WireType,
        buf: &mut B,
    ) -> Result<(), DecodeError>
    where
        Pb: PresenceBits,
        B: Buf,
    {
        if field_number == FIELD_EMAIL_ADDRESS {
            Self::bind_email_address_mut(slot, common)
                .bind(common)
                .merge(wire_type, buf)
        } else if field_number == FIELD_PHONE_NUMBER {
            Self::bind_phone_number_mut(slot, common)
                .bind(common)
                .merge(wire_type, buf)
        } else if field_number == FIELD_WEBHOOK_ID {
            Self::bind_webhook_id_mut(slot, common)
                .bind(common)
                .merge(wire_type, buf)
        } else if field_number == FIELD_POSTAL {
            Self::bind_postal_mut(slot, common)
                .bind(common)
                .merge(wire_type, buf)
        } else {
            Err(DecodeError::InvalidTag)
        }
    }
}

impl<
        A: Allocator + Clone,
        const FIELD_EMAIL_ADDRESS: u32,
        const FIELD_PHONE_NUMBER: u32,
        const FIELD_WEBHOOK_ID: u32,
        const FIELD_POSTAL: u32,
    > OneofDeallocate<A>
    for NotificationStorage<A, FIELD_EMAIL_ADDRESS, FIELD_PHONE_NUMBER, FIELD_WEBHOOK_ID, FIELD_POSTAL>
{
    /// # Safety
    ///
    /// `alloc` must be the allocator that owns the variant's buffer.
    unsafe fn deallocate(self, alloc: A) {
        match self {
            Self::EmailAddress(mut f) => f.deallocate(alloc),
            Self::PhoneNumber(mut f) => f.deallocate(alloc),
            Self::WebhookId(_) => {}
            Self::Postal(f) => f.deallocate(alloc),
        }
    }
}
