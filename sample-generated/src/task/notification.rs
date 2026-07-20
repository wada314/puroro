//! Sample of the `oneof notification` types puroro generates for `Task`.
//!
//! A oneof that owns heap storage is represented by several generated items,
//! because the owned storage holds allocator-less `unmanaged` values (unsafe to
//! drop implicitly) and must not leak into the public API:
//!
//! - [`Notification`] — canonical **shape** enum; variant payloads are type
//!   parameters. [`NotificationStorage`] / [`NotificationRef`] /
//!   [`NotificationMut`] are aliases of this shape.
//! - [`NotificationStorage`] — `pub(crate)` alias with field-wrapper payloads;
//!   implements [`OneofGroup`], [`OneofDeallocate`], encode glue. Never public.
//! - [`NotificationCase`] — a payload-less, `Copy` discriminant of which variant
//!   is active.
//! - [`NotificationRef`] / [`NotificationMut`] — safe projected aliases of the
//!   *active* variant (`as_ref` / `as_mut` on [`OneofView`] /
//!   [`OneofViewMut`](::puroro_rt::OneofViewMut)). Payloads come from
//!   [`ProtoType`](::puroro_rt::ProtoType) on each variant's type marker
//!   (no `StringGuard` / `BitRef` hard-coding in generated aliases).
//!
//! Group bound views come from `puroro-rt` ([`OneofView`] /
//! [`OneofViewMut`](::puroro_rt::OneofViewMut)), not per-oneof generated structs.
//!
//! This group is deliberately **heterogeneous** to show every field kind:
//!
//! | variant | proto | field wrapper | `Ref` payload | `Mut` payload |
//! |---|---|---|---|---|
//! | `email_address` / `phone_number` | `string` | [`SingularField`] (+ `ProtoDefault`) | `&str` | string guard |
//! | `webhook_id` | `int32` `[default = -1]` | [`SingularField`] + [`WebhookIdDefault`] | `i32` (by value) | `&mut i32` |
//! | `postal` | `Address` message | [`SingularField`] + [`ProtoMessage`] | `&Address<A>` | `&mut Address<A>` |
//! | `urgent` | `bool` | [`SingularField`] + [`ProtoBool`] | `bool` | `ProtoType::Mut` (named bit handle) |
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
//! number (one match arm per variant) and calls
//! `slot.bind_mut(common).variant_mut::<V>().bind_mut(common).merge(...)`
//! — uniformly for every variant kind.
//!
//! Per-variant dispatch uses [`EnumVariant`] on zero-sized marker types in
//! [`variant`]; see that module for the type-parameter wiring.
//!
//! Note: this enum cannot carry unused lifetime/allocator parameters via
//! `PhantomData` (unlike a struct). Integer-only oneofs therefore omit `'a` / `A`
//! from the shape and from `Ref`/`Mut` aliases when no variant payload needs them.

use ::allocator_api2::alloc::Allocator;
use ::bytes::BufMut;
use ::puroro_rt::{
    BitPacked, EnumVariant, FieldDeallocate, Inline, MessageCommon, Oneof, OneofDeallocate,
    OneofEncodable, OneofGroup, PresenceBits, ProtoBool, ProtoInt32, ProtoMessage, ProtoString,
    ProtoType, SingularField,
};

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

/// Canonical shape for `oneof notification`.
///
/// Note: this enum cannot carry unused lifetime/allocator parameters via
/// `PhantomData` (unlike a struct). Integer-only oneofs therefore omit `'a` / `A`
/// from the shape and from `Ref`/`Mut` aliases when no variant payload needs them.
#[derive(Clone, Copy, PartialEq)]
pub enum Notification<Ea, Pn, Wh, Po, Ur> {
    EmailAddress(Ea),
    PhoneNumber(Pn),
    WebhookId(Wh),
    Postal(Po),
    Urgent(Ur),
}

/// Which variant of `oneof notification` is set — a payload-less discriminant.
///
/// Returned by [`OneofView::case`](::puroro::OneofView::case) as
/// `Option<NotificationCase>`; the unset group is `None`, so this enum mirrors
/// the real variants 1:1 (no `NotSet` sentinel).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotificationCase {
    EmailAddress,
    PhoneNumber,
    WebhookId,
    Postal,
    Urgent,
}

type EmailAddressField<A> = SingularField<ProtoString, Oneof, { super::FIELD_EMAIL_ADDRESS }, A>;
type PhoneNumberField<A> = SingularField<ProtoString, Oneof, { super::FIELD_PHONE_NUMBER }, A>;
type WebhookIdField<A> =
    SingularField<ProtoInt32, Oneof, { super::FIELD_WEBHOOK_ID }, A, Inline, WebhookIdDefault>;
type PostalField<A> = SingularField<ProtoMessage<Address<A>>, Oneof, { super::FIELD_POSTAL }, A>;
type UrgentField<A> = SingularField<
    ProtoBool,
    Oneof,
    { super::FIELD_URGENT },
    A,
    BitPacked<{ super::BIT_URGENT_VALUE }>,
>;

/// Owned storage for `oneof notification` (crate-internal).
pub(crate) type NotificationStorage<A> = Notification<
    EmailAddressField<A>,
    PhoneNumberField<A>,
    WebhookIdField<A>,
    PostalField<A>,
    UrgentField<A>,
>;

/// Borrowed read view of the active `notification` variant.
pub type NotificationRef<'a, A> = Notification<
    <ProtoString as ProtoType>::Ref<'a, A>,
    <ProtoString as ProtoType>::Ref<'a, A>,
    <ProtoInt32 as ProtoType>::Ref<'a, A>,
    <ProtoMessage<Address<A>> as ProtoType>::Ref<'a, A>,
    <ProtoBool as ProtoType>::Ref<'a, A>,
>;

/// Borrowed mutable projection of the active `notification` variant.
///
/// Payload types come from [`ProtoType::Mut`] on each variant's type marker.
/// Public `_mut` accessors may still return `impl Trait` (e.g. bool) where
/// ergonomics prefer it; enum variants need the named associated type.
pub type NotificationMut<'a, A> = Notification<
    <ProtoString as ProtoType>::Mut<'a, A>,
    <ProtoString as ProtoType>::Mut<'a, A>,
    <ProtoInt32 as ProtoType>::Mut<'a, A>,
    <ProtoMessage<Address<A>> as ProtoType>::Mut<'a, A>,
    <ProtoBool as ProtoType>::Mut<'a, A>,
>;

impl<A: Allocator + Clone> OneofGroup for NotificationStorage<A> {
    type Case = NotificationCase;
    type Ref<'a>
        = NotificationRef<'a, A>
    where
        A: 'a;
    type Mut<'a>
        = NotificationMut<'a, A>
    where
        A: 'a;
    type Presence = TaskPresence;
    type Alloc = A;

    fn case(storage: &Self) -> Self::Case {
        match storage {
            Self::EmailAddress(_) => NotificationCase::EmailAddress,
            Self::PhoneNumber(_) => NotificationCase::PhoneNumber,
            Self::WebhookId(_) => NotificationCase::WebhookId,
            Self::Postal(_) => NotificationCase::Postal,
            Self::Urgent(_) => NotificationCase::Urgent,
        }
    }

    fn to_ref<'a>(
        storage: &'a Self,
        common: &'a MessageCommon<Self::Presence, Self::Alloc>,
    ) -> Self::Ref<'a> {
        match storage {
            Self::EmailAddress(f) => Notification::EmailAddress(f.value(common)),
            Self::PhoneNumber(f) => Notification::PhoneNumber(f.value(common)),
            Self::WebhookId(f) => Notification::WebhookId(f.value(common)),
            Self::Postal(f) => Notification::Postal(f.value(common)),
            Self::Urgent(f) => Notification::Urgent(f.value(common)),
        }
    }

    fn to_mut<'a>(
        storage: &'a mut Self,
        common: &'a mut MessageCommon<Self::Presence, Self::Alloc>,
    ) -> Self::Mut<'a> {
        match storage {
            Self::EmailAddress(f) => Notification::EmailAddress(f.value_mut(common)),
            Self::PhoneNumber(f) => Notification::PhoneNumber(f.value_mut(common)),
            Self::WebhookId(f) => Notification::WebhookId(f.value_mut(common)),
            Self::Postal(f) => Notification::Postal(f.value_mut(common)),
            Self::Urgent(_) => {
                Notification::Urgent(common.presence.bit_ref_mut(super::BIT_URGENT_VALUE))
            }
        }
    }

    fn clone_storage_in(
        storage: &Self,
        common: &MessageCommon<Self::Presence, Self::Alloc>,
        alloc: Self::Alloc,
    ) -> Self {
        match storage {
            Self::EmailAddress(f) => Self::EmailAddress(f.clone_in(common, alloc)),
            Self::PhoneNumber(f) => Self::PhoneNumber(f.clone_in(common, alloc)),
            Self::WebhookId(f) => Self::WebhookId(f.clone_in(common, alloc)),
            Self::Postal(f) => Self::Postal(f.clone_in(common, alloc)),
            Self::Urgent(f) => Self::Urgent(f.clone_in(common, alloc)),
        }
    }
}

impl<A: Allocator + Clone> EnumVariant<EmailAddress> for NotificationStorage<A> {
    type Value = EmailAddressField<A>;
    type Alloc = A;

    fn new_value(alloc: A) -> Self::Value {
        SingularField::new_in(alloc)
    }

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
    type Value = PhoneNumberField<A>;
    type Alloc = A;

    fn new_value(alloc: A) -> Self::Value {
        SingularField::new_in(alloc)
    }

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
    type Value = WebhookIdField<A>;
    type Alloc = A;

    fn new_value(alloc: A) -> Self::Value {
        SingularField::new_in(alloc)
    }

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
    type Value = PostalField<A>;
    type Alloc = A;

    fn new_value(alloc: A) -> Self::Value {
        SingularField::with_message_in(alloc)
    }

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
    type Value = UrgentField<A>;
    type Alloc = A;

    fn new_value(alloc: A) -> Self::Value {
        SingularField::new_in(alloc)
    }

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
