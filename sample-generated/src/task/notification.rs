//! Sample of the `oneof notification` types puroro generates for `Task`.
//!
//! A oneof that owns heap storage is represented by several generated types,
//! because the owned storage holds allocator-less `unmanaged` values (unsafe to
//! drop implicitly) and must not leak into the public API:
//!
//! - [`NotificationStorage`] — the owned storage enum (`pub(crate)`, deliberately
//!   *not* the canonical `Notification` name): each variant owns the same **field
//!   wrapper** a singular field of that kind uses, implements [`OneofDeallocate`],
//!   and carries the encode glue. Never public.
//! - [`NotificationCase`] — a payload-less, `Copy` discriminant of which variant
//!   is active.
//! - [`NotificationView`] / [`NotificationViewMut`] — bound views of the group
//!   (slot + [`MessageCommon`]), returned by `Task::notification` /
//!   `notification_mut` even when unset.
//! - [`NotificationRef`] / [`NotificationMut`] — safe projected enums of the
//!   *active* variant (`as_ref` / `as_mut` on the bound views).
//!
//! This group is deliberately **heterogeneous** to show every field kind:
//!
//! | variant | proto | field wrapper | `Ref` payload | `Mut` payload |
//! |---|---|---|---|---|
//! | `email_address` / `phone_number` | `string` | [`SingularLenField`] (+ `ProtoDefault`) | `&str` | string guard |
//! | `webhook_id` | `int32` `[default = -1]` | [`SingularVarintField`] + [`WebhookIdDefault`] | `i32` (by value) | `&mut i32` |
//! | `postal` | `Address` message | [`NestedMessageField`] | `&Address<A>` | `&mut Address<A>` |
//! | `urgent` | `bool` | [`BoolField`] | `bool` | bitvec `BitRef<'_, Mut, …>` |
//!
//! Per-variant **immutable** getters return [`Optional`](::puroro::Optional) whose
//! `D` is the field wrapper's default marker: when the case is unset or another
//! variant, `get()` yields that proto default (custom or type-zero) without
//! selecting the variant — same contract as official const getters.
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
//!
//! Per-variant dispatch uses [`EnumVariant`] on zero-sized marker types in
//! [`variant`]; see that module for the type-parameter wiring.

use ::allocator_api2::alloc::Allocator;
use ::bytes::BufMut;
use ::puroro_rt::{
    Bindable, BindableMut, BoolField, BoolFieldMut, EnumVariant, FieldDeallocate, MessageCommon,
    NestedMessageField, NestedMessageFieldMut, Oneof, OneofDeallocate, OneofEncodable, OneofSlot,
    PresenceBits, ProtoInt32, ProtoString, SingularLenField, SingularLenFieldMut,
    SingularVarintField, SingularVarintFieldMut,
};
use ::unmanaged::string::StringGuard;

use crate::address::Address;

use super::TaskPresence;
use super::defaults::WebhookIdDefault;

/// Zero-sized markers for [`EnumVariant`] dispatch on [`NotificationStorage`].
pub(crate) mod variant {
    pub struct EmailAddress;
    pub struct PhoneNumber;
    pub struct WebhookId;
    pub struct Postal;
    pub struct Urgent;
}

use variant::{EmailAddress, PhoneNumber, Postal, Urgent, WebhookId};

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
    Urgent,
}

/// Borrowed read view of the active `notification` variant.
pub enum NotificationRef<'a, A: Allocator + Clone> {
    EmailAddress(&'a str),
    PhoneNumber(&'a str),
    WebhookId(i32),
    Postal(&'a Address<A>),
    Urgent(bool),
}

impl<A: Allocator + Clone> Clone for NotificationRef<'_, A> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<A: Allocator + Clone> Copy for NotificationRef<'_, A> {}

/// Borrowed mutable projection of the active `notification` variant.
pub enum NotificationMut<'a, A: Allocator + Clone> {
    EmailAddress(StringGuard<'a, A>),
    PhoneNumber(StringGuard<'a, A>),
    WebhookId(&'a mut i32),
    Postal(&'a mut Address<A>),
    /// Named only because enum variants cannot hold `impl Trait`; public
    /// `_mut` accessors still return `impl DerefMut<Target = bool>`.
    Urgent(::bitvec::ptr::BitRef<'a, ::bitvec::ptr::Mut, u8, ::bitvec::order::Lsb0>),
}

/// Shared bound view of the `notification` oneof group (slot + message common).
///
/// Returned by [`Task::notification`](super::Task::notification) even when the
/// group is unset. Project the active variant with [`as_ref`](Self::as_ref).
pub struct NotificationView<'a, A: Allocator + Clone> {
    slot: &'a OneofSlot<NotificationStorage<A>>,
    common: &'a MessageCommon<TaskPresence, A>,
}

impl<'a, A: Allocator + Clone> NotificationView<'a, A> {
    #[inline]
    pub(crate) fn new(
        slot: &'a OneofSlot<NotificationStorage<A>>,
        common: &'a MessageCommon<TaskPresence, A>,
    ) -> Self {
        Self { slot, common }
    }

    /// Which variant is set (`None` when the group is unset).
    #[inline]
    pub fn case(&self) -> Option<NotificationCase> {
        self.slot.as_ref().map(|s| s.case())
    }

    /// Projected read view of the active variant, if any.
    #[inline]
    pub fn as_ref(&self) -> Option<NotificationRef<'a, A>> {
        self.slot.as_ref().map(|s| s.to_ref(self.common))
    }
}

/// Mutable bound view of the `notification` oneof group (slot + message common).
///
/// Returned by [`Task::notification_mut`](super::Task::notification_mut) even
/// when the group is unset. Shared getters go through [`as_view`](Self::as_view);
/// mutation uses [`as_mut`](Self::as_mut) / [`clear`](Self::clear).
pub struct NotificationViewMut<'a, A: Allocator + Clone> {
    slot: &'a mut OneofSlot<NotificationStorage<A>>,
    common: &'a mut MessageCommon<TaskPresence, A>,
}

impl<'a, A: Allocator + Clone> NotificationViewMut<'a, A> {
    #[inline]
    pub(crate) fn new(
        slot: &'a mut OneofSlot<NotificationStorage<A>>,
        common: &'a mut MessageCommon<TaskPresence, A>,
    ) -> Self {
        Self { slot, common }
    }

    /// Reborrow as a shared bound view (for `case` / `as_ref` while mutating).
    #[inline]
    pub fn as_view(&self) -> NotificationView<'_, A> {
        NotificationView::new(self.slot, self.common)
    }

    /// Projected mutable view of the *currently active* variant (no switch).
    ///
    /// Consumes this bound view. Returns `None` when the group is unset.
    #[inline]
    pub fn as_mut(self) -> Option<NotificationMut<'a, A>> {
        self.slot.as_mut().map(|s| s.to_mut(self.common))
    }

    /// Clears whichever variant is active (freeing it through the message allocator).
    #[inline]
    pub fn clear(self) {
        self.slot.bind_mut(self.common).clear();
    }
}

/// Owned storage for `oneof notification` (crate-internal).
///
/// Parametrised only by the allocator `A`. Each variant's field wrapper uses the
/// parent message's `FIELD_*` constant directly as its `FIELD` type argument,
/// since the field numbers are fixed for this generated oneof.
pub(crate) enum NotificationStorage<A: Allocator + Clone> {
    EmailAddress(SingularLenField<ProtoString, Oneof, { super::FIELD_EMAIL_ADDRESS }>),
    PhoneNumber(SingularLenField<ProtoString, Oneof, { super::FIELD_PHONE_NUMBER }>),
    WebhookId(
        SingularVarintField<ProtoInt32, Oneof, { super::FIELD_WEBHOOK_ID }, WebhookIdDefault>,
    ),
    Postal(NestedMessageField<Address<A>, Oneof, { super::FIELD_POSTAL }, A>),
    Urgent(BoolField<Oneof, { super::BIT_URGENT_VALUE }, { super::FIELD_URGENT }>),
}

impl<A: Allocator + Clone> NotificationStorage<A> {
    pub(crate) fn case(&self) -> NotificationCase {
        match self {
            Self::EmailAddress(_) => NotificationCase::EmailAddress,
            Self::PhoneNumber(_) => NotificationCase::PhoneNumber,
            Self::WebhookId(_) => NotificationCase::WebhookId,
            Self::Postal(_) => NotificationCase::Postal,
            Self::Urgent(_) => NotificationCase::Urgent,
        }
    }

    pub(crate) fn to_ref<'a, Pb: PresenceBits>(
        &'a self,
        common: &'a MessageCommon<Pb, A>,
    ) -> NotificationRef<'a, A> {
        match self {
            Self::EmailAddress(f) => NotificationRef::EmailAddress(f.value()),
            Self::PhoneNumber(f) => NotificationRef::PhoneNumber(f.value()),
            Self::WebhookId(f) => NotificationRef::WebhookId(f.value()),
            Self::Postal(f) => NotificationRef::Postal(f.value()),
            Self::Urgent(f) => NotificationRef::Urgent(f.bind(common).value()),
        }
    }

    pub(crate) fn to_mut<'a>(
        &'a mut self,
        common: &'a mut MessageCommon<TaskPresence, A>,
    ) -> NotificationMut<'a, A> {
        match self {
            Self::EmailAddress(f) => {
                let alloc = common.alloc.clone();
                NotificationMut::EmailAddress(f.value_mut(alloc))
            }
            Self::PhoneNumber(f) => {
                let alloc = common.alloc.clone();
                NotificationMut::PhoneNumber(f.value_mut(alloc))
            }
            Self::WebhookId(f) => {
                let alloc = common.alloc.clone();
                NotificationMut::WebhookId(f.value_mut(alloc))
            }
            Self::Postal(f) => NotificationMut::Postal(f.value_mut()),
            // Enum variants cannot store RPITIT; use the concrete BitRef.
            Self::Urgent(_) => {
                NotificationMut::Urgent(common.presence.bit_ref_mut(super::BIT_URGENT_VALUE))
            }
        }
    }

    pub(crate) fn bind_email_address_mut<'f, 'c, Pb: PresenceBits>(
        slot: &'f mut OneofSlot<Self>,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> SingularLenFieldMut<
        'f,
        'c,
        ProtoString,
        Oneof,
        { super::FIELD_EMAIL_ADDRESS },
        ::puroro_rt::ProtoDefault,
        Pb,
        A,
    > {
        let field = slot
            .bind_mut(common)
            .variant_mut::<EmailAddress>(|alloc| SingularLenField::new_in(alloc));
        field.bind_mut(common)
    }

    pub(crate) fn bind_phone_number_mut<'f, 'c, Pb: PresenceBits>(
        slot: &'f mut OneofSlot<Self>,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> SingularLenFieldMut<
        'f,
        'c,
        ProtoString,
        Oneof,
        { super::FIELD_PHONE_NUMBER },
        ::puroro_rt::ProtoDefault,
        Pb,
        A,
    > {
        let field = slot
            .bind_mut(common)
            .variant_mut::<PhoneNumber>(|alloc| SingularLenField::new_in(alloc));
        field.bind_mut(common)
    }

    pub(crate) fn bind_webhook_id_mut<'f, 'c, Pb: PresenceBits>(
        slot: &'f mut OneofSlot<Self>,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> SingularVarintFieldMut<
        'f,
        'c,
        ProtoInt32,
        Oneof,
        { super::FIELD_WEBHOOK_ID },
        WebhookIdDefault,
        Pb,
        A,
    > {
        let field = slot
            .bind_mut(common)
            .variant_mut::<WebhookId>(|alloc| SingularVarintField::new_in(alloc));
        field.bind_mut(common)
    }

    pub(crate) fn bind_postal_mut<'f, 'c, Pb: PresenceBits>(
        slot: &'f mut OneofSlot<Self>,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> NestedMessageFieldMut<'f, 'c, Address<A>, Oneof, { super::FIELD_POSTAL }, A, Pb> {
        let field = slot
            .bind_mut(common)
            .variant_mut::<Postal>(|alloc| NestedMessageField::with_message_in(alloc));
        field.bind_mut(common)
    }

    pub(crate) fn bind_urgent_mut<'f, 'c, Pb: PresenceBits>(
        slot: &'f mut OneofSlot<Self>,
        common: &'c mut MessageCommon<Pb, A>,
    ) -> BoolFieldMut<
        'f,
        'c,
        Oneof,
        { super::BIT_URGENT_VALUE },
        { super::FIELD_URGENT },
        ::puroro_rt::ProtoDefault,
        Pb,
        A,
    > {
        let field = slot
            .bind_mut(common)
            .variant_mut::<Urgent>(|alloc| BoolField::new_in(alloc));
        field.bind_mut(common)
    }
}

impl<A: Allocator + Clone> EnumVariant<EmailAddress> for NotificationStorage<A> {
    type Value = SingularLenField<ProtoString, Oneof, { super::FIELD_EMAIL_ADDRESS }>;

    fn variant_ref(&self) -> Option<&Self::Value> {
        match self {
            Self::EmailAddress(f) => Some(f),
            _ => None,
        }
    }

    fn variant_mut(&mut self) -> Option<&mut Self::Value> {
        match self {
            Self::EmailAddress(f) => Some(f),
            _ => None,
        }
    }

    fn from_variant(value: Self::Value) -> Self {
        Self::EmailAddress(value)
    }
}

impl<A: Allocator + Clone> EnumVariant<PhoneNumber> for NotificationStorage<A> {
    type Value = SingularLenField<ProtoString, Oneof, { super::FIELD_PHONE_NUMBER }>;

    fn variant_ref(&self) -> Option<&Self::Value> {
        match self {
            Self::PhoneNumber(f) => Some(f),
            _ => None,
        }
    }

    fn variant_mut(&mut self) -> Option<&mut Self::Value> {
        match self {
            Self::PhoneNumber(f) => Some(f),
            _ => None,
        }
    }

    fn from_variant(value: Self::Value) -> Self {
        Self::PhoneNumber(value)
    }
}

impl<A: Allocator + Clone> EnumVariant<WebhookId> for NotificationStorage<A> {
    type Value =
        SingularVarintField<ProtoInt32, Oneof, { super::FIELD_WEBHOOK_ID }, WebhookIdDefault>;

    fn variant_ref(&self) -> Option<&Self::Value> {
        match self {
            Self::WebhookId(f) => Some(f),
            _ => None,
        }
    }

    fn variant_mut(&mut self) -> Option<&mut Self::Value> {
        match self {
            Self::WebhookId(f) => Some(f),
            _ => None,
        }
    }

    fn from_variant(value: Self::Value) -> Self {
        Self::WebhookId(value)
    }
}

impl<A: Allocator + Clone> EnumVariant<Postal> for NotificationStorage<A> {
    type Value = NestedMessageField<Address<A>, Oneof, { super::FIELD_POSTAL }, A>;

    fn variant_ref(&self) -> Option<&Self::Value> {
        match self {
            Self::Postal(f) => Some(f),
            _ => None,
        }
    }

    fn variant_mut(&mut self) -> Option<&mut Self::Value> {
        match self {
            Self::Postal(f) => Some(f),
            _ => None,
        }
    }

    fn from_variant(value: Self::Value) -> Self {
        Self::Postal(value)
    }
}

impl<A: Allocator + Clone> EnumVariant<Urgent> for NotificationStorage<A> {
    type Value = BoolField<Oneof, { super::BIT_URGENT_VALUE }, { super::FIELD_URGENT }>;

    fn variant_ref(&self) -> Option<&Self::Value> {
        match self {
            Self::Urgent(f) => Some(f),
            _ => None,
        }
    }

    fn variant_mut(&mut self) -> Option<&mut Self::Value> {
        match self {
            Self::Urgent(f) => Some(f),
            _ => None,
        }
    }

    fn from_variant(value: Self::Value) -> Self {
        Self::Urgent(value)
    }
}

impl<A: Allocator + Clone> OneofEncodable<A> for NotificationStorage<A> {
    fn encoded_len<Pb: PresenceBits>(&self, common: &MessageCommon<Pb, A>) -> usize {
        match self {
            Self::EmailAddress(f) => f.encoded_len(common),
            Self::PhoneNumber(f) => f.encoded_len(common),
            Self::WebhookId(f) => f.encoded_len(common),
            Self::Postal(f) => f.encoded_len(common),
            Self::Urgent(f) => f.encoded_len(common),
        }
    }

    fn encode_raw<Pb: PresenceBits, B: BufMut>(&self, common: &MessageCommon<Pb, A>, buf: &mut B) {
        match self {
            Self::EmailAddress(f) => f.encode_raw(common, buf),
            Self::PhoneNumber(f) => f.encode_raw(common, buf),
            Self::WebhookId(f) => f.encode_raw(common, buf),
            Self::Postal(f) => f.encode_raw(common, buf),
            Self::Urgent(f) => f.encode_raw(common, buf),
        }
    }
}

impl<A: Allocator + Clone, Pb: PresenceBits> OneofDeallocate<Pb, A> for NotificationStorage<A> {
    /// # Safety
    ///
    /// `common.alloc` must be the allocator that owns the variant's buffer.
    unsafe fn deallocate(self, common: &MessageCommon<Pb, A>) {
        match self {
            Self::EmailAddress(mut f) => f.deallocate(common),
            Self::PhoneNumber(mut f) => f.deallocate(common),
            Self::WebhookId(mut f) => f.deallocate(common),
            Self::Postal(mut f) => f.deallocate(common),
            Self::Urgent(mut f) => f.deallocate(common),
        }
    }
}
