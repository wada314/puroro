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
//! Variant proto field numbers are fixed for this generated oneof, so
//! [`NotificationStorage`] is *not* parametrised by them: each variant's field
//! wrapper references the parent message module's named `FIELD_*` constant
//! (via `super::FIELD_*`) directly, keeping a single source of truth.
//!
//! **Merge has no bespoke `merge_*` helpers on this enum.** Because each variant
//! *is* a field wrapper, the parent message's `merge_from` dispatches on field
//! number (one match arm per variant) and calls `bind_<variant>_mut` (which
//! frees any other variant and returns the field's bound mutation view), then
//! `merge(...)` on it — uniformly for every variant kind.

use ::allocator_api2::alloc::Allocator;
use ::bytes::BufMut;
use ::puroro_rt::{
    BindableMut, MessageCommon, NestedMessageField, NestedMessageFieldMut, Oneof, OneofDeallocate,
    OneofEncodable, OneofSlot, PresenceBits, ProtoInt32, ProtoString, SingularLenField,
    SingularLenFieldMut, SingularVarintField, SingularVarintFieldMut,
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
/// Parametrised only by the allocator `A`. Each variant's field wrapper uses the
/// parent message's `FIELD_*` constant directly as its `FIELD` type argument,
/// since the field numbers are fixed for this generated oneof.
pub(crate) enum NotificationStorage<A: Allocator + Clone> {
    EmailAddress(SingularLenField<ProtoString, Oneof, { super::FIELD_EMAIL_ADDRESS }, A>),
    PhoneNumber(SingularLenField<ProtoString, Oneof, { super::FIELD_PHONE_NUMBER }, A>),
    WebhookId(SingularVarintField<ProtoInt32, Oneof, { super::FIELD_WEBHOOK_ID }>),
    Postal(NestedMessageField<Address<A>, Oneof, { super::FIELD_POSTAL }, A>),
}

impl<A: Allocator + Clone> NotificationStorage<A> {
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

    pub(crate) fn bind_email_address_mut<'f, 'c, Pb: PresenceBits>(
        slot: &'f mut OneofSlot<Self>,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> SingularLenFieldMut<'f, 'c, ProtoString, Oneof, { super::FIELD_EMAIL_ADDRESS }, A, ::puroro_rt::ProtoDefault, Pb>
    {
        let field = slot.bind_mut(common).variant_mut(
            |e| matches!(e, Self::EmailAddress(_)),
            |e| match e {
                Self::EmailAddress(f) => Some(f),
                _ => None,
            },
            |alloc| Self::EmailAddress(SingularLenField::new_in(alloc)),
        );
        field.bind_mut(common)
    }

    pub(crate) fn bind_phone_number_mut<'f, 'c, Pb: PresenceBits>(
        slot: &'f mut OneofSlot<Self>,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> SingularLenFieldMut<'f, 'c, ProtoString, Oneof, { super::FIELD_PHONE_NUMBER }, A, ::puroro_rt::ProtoDefault, Pb>
    {
        let field = slot.bind_mut(common).variant_mut(
            |e| matches!(e, Self::PhoneNumber(_)),
            |e| match e {
                Self::PhoneNumber(f) => Some(f),
                _ => None,
            },
            |alloc| Self::PhoneNumber(SingularLenField::new_in(alloc)),
        );
        field.bind_mut(common)
    }

    pub(crate) fn bind_webhook_id_mut<'f, 'c, Pb: PresenceBits>(
        slot: &'f mut OneofSlot<Self>,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> SingularVarintFieldMut<'f, 'c, ProtoInt32, Oneof, { super::FIELD_WEBHOOK_ID }, ::puroro_rt::ProtoDefault, Pb, A>
    {
        let field = slot.bind_mut(common).variant_mut(
            |e| matches!(e, Self::WebhookId(_)),
            |e| match e {
                Self::WebhookId(f) => Some(f),
                _ => None,
            },
            |_alloc| Self::WebhookId(SingularVarintField::new_in(_alloc)),
        );
        field.bind_mut(common)
    }

    pub(crate) fn bind_postal_mut<'f, 'c, Pb: PresenceBits>(
        slot: &'f mut OneofSlot<Self>,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> NestedMessageFieldMut<'f, 'c, Address<A>, Oneof, { super::FIELD_POSTAL }, A, Pb> {
        let field = slot.bind_mut(common).variant_mut(
            |e| matches!(e, Self::Postal(_)),
            |e| match e {
                Self::Postal(f) => Some(f),
                _ => None,
            },
            |alloc| Self::Postal(NestedMessageField::with_message_in(alloc)),
        );
        field.bind_mut(common)
    }

    pub fn email_address(
        &self,
    ) -> Option<&SingularLenField<ProtoString, Oneof, { super::FIELD_EMAIL_ADDRESS }, A>> {
        match self {
            Self::EmailAddress(f) => Some(f),
            _ => None,
        }
    }

    pub fn phone_number(
        &self,
    ) -> Option<&SingularLenField<ProtoString, Oneof, { super::FIELD_PHONE_NUMBER }, A>> {
        match self {
            Self::PhoneNumber(f) => Some(f),
            _ => None,
        }
    }

    pub fn webhook_id(
        &self,
    ) -> Option<&SingularVarintField<ProtoInt32, Oneof, { super::FIELD_WEBHOOK_ID }>> {
        match self {
            Self::WebhookId(f) => Some(f),
            _ => None,
        }
    }

    pub fn postal(
        &self,
    ) -> Option<&NestedMessageField<Address<A>, Oneof, { super::FIELD_POSTAL }, A>> {
        match self {
            Self::Postal(f) => Some(f),
            _ => None,
        }
    }
}

impl<A: Allocator + Clone> OneofEncodable<A> for NotificationStorage<A> {
    fn encoded_len<Pb: PresenceBits>(&self, common: &MessageCommon<Pb, A>) -> usize {
        match self {
            Self::EmailAddress(f) => f.encoded_len(common),
            Self::PhoneNumber(f) => f.encoded_len(common),
            Self::WebhookId(f) => f.encoded_len(common),
            Self::Postal(f) => f.encoded_len(common),
        }
    }

    fn encode_raw<Pb: PresenceBits, B: BufMut>(&self, common: &MessageCommon<Pb, A>, buf: &mut B) {
        match self {
            Self::EmailAddress(f) => f.encode_raw(common, buf),
            Self::PhoneNumber(f) => f.encode_raw(common, buf),
            Self::WebhookId(f) => f.encode_raw(common, buf),
            Self::Postal(f) => f.encode_raw(common, buf),
        }
    }
}

impl<A: Allocator + Clone> OneofDeallocate<A> for NotificationStorage<A> {
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
