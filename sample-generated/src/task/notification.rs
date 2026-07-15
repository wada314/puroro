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
//!   [`SingularAccess`](::puroro_rt::SingularAccess) on each variant's field
//!   wrapper (no `StringGuard` / `BitRef` hard-coding in generated aliases).
//!
//! Group bound views come from `puroro-rt` ([`OneofView`] /
//! [`OneofViewMut`](::puroro_rt::OneofViewMut)), not per-oneof generated structs.
//!
//! This group is deliberately **heterogeneous** to show every field kind:
//!
//! | variant | proto | field wrapper | `Ref` payload | `Mut` payload |
//! |---|---|---|---|---|
//! | `email_address` / `phone_number` | `string` | [`SingularLenField`] (+ `ProtoDefault`) | `&str` | string guard |
//! | `webhook_id` | `int32` `[default = -1]` | [`SingularVarintField`] + [`WebhookIdDefault`] | `i32` (by value) | `&mut i32` |
//! | `postal` | `Address` message | [`NestedMessageField`] | `&Address<A>` | `&mut Address<A>` |
//! | `urgent` | `bool` | [`SingularVarintField`] + [`ProtoBool`] | `bool` | `SingularAccess::Mut` (named bit handle) |
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
//! `slot.bind_mut(common).variant_mut::<V>().merge(...)` — uniformly for every
//! variant kind.
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
    EnumVariant, FieldDeallocate, MessageCommon, NestedMessageField, Oneof, OneofDeallocate,
    OneofEncodable, OneofGroup, PresenceBits, ProtoBool, ProtoInt32, ProtoString, SingularAccess,
    SingularLenField, SingularVarintField,
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
pub enum Notification<Ea, Pn, Wh, Po, Ur> {
    EmailAddress(Ea),
    PhoneNumber(Pn),
    WebhookId(Wh),
    Postal(Po),
    Urgent(Ur),
}

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

type EmailAddressField<A> =
    SingularLenField<ProtoString<A>, Oneof, { super::FIELD_EMAIL_ADDRESS }>;
type PhoneNumberField<A> =
    SingularLenField<ProtoString<A>, Oneof, { super::FIELD_PHONE_NUMBER }>;
type WebhookIdField<A> = SingularVarintField<
    ProtoInt32<A>,
    Oneof,
    { super::FIELD_WEBHOOK_ID },
    WebhookIdDefault,
>;
type PostalField<A> = NestedMessageField<Address<A>, Oneof, { super::FIELD_POSTAL }, A>;
type UrgentField<A> = SingularVarintField<
    ProtoBool<A, { super::BIT_URGENT_VALUE }>,
    Oneof,
    { super::FIELD_URGENT },
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
    <EmailAddressField<A> as SingularAccess>::Ref<'a>,
    <PhoneNumberField<A> as SingularAccess>::Ref<'a>,
    <WebhookIdField<A> as SingularAccess>::Ref<'a>,
    <PostalField<A> as SingularAccess>::Ref<'a>,
    <UrgentField<A> as SingularAccess>::Ref<'a>,
>;

/// Borrowed mutable projection of the active `notification` variant.
///
/// Payload types come from [`SingularAccess::Mut`] on each field wrapper. Public
/// `_mut` accessors may still return `impl Trait` (e.g. bool) where ergonomics
/// prefer it; enum variants need the named associated type.
pub type NotificationMut<'a, A> = Notification<
    <EmailAddressField<A> as SingularAccess>::Mut<'a>,
    <PhoneNumberField<A> as SingularAccess>::Mut<'a>,
    <WebhookIdField<A> as SingularAccess>::Mut<'a>,
    <PostalField<A> as SingularAccess>::Mut<'a>,
    <UrgentField<A> as SingularAccess>::Mut<'a>,
>;

// `A` must appear structurally (not only inside an associated-type projection)
// for these impls — see rustc E0207. `PostalField`'s `Ref` is `&Address<A>`.
impl<'a, A: Allocator + Clone> Clone
    for Notification<
        <EmailAddressField<A> as SingularAccess>::Ref<'a>,
        <PhoneNumberField<A> as SingularAccess>::Ref<'a>,
        <WebhookIdField<A> as SingularAccess>::Ref<'a>,
        &'a Address<A>,
        <UrgentField<A> as SingularAccess>::Ref<'a>,
    >
{
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, A: Allocator + Clone> Copy
    for Notification<
        <EmailAddressField<A> as SingularAccess>::Ref<'a>,
        <PhoneNumberField<A> as SingularAccess>::Ref<'a>,
        <WebhookIdField<A> as SingularAccess>::Ref<'a>,
        &'a Address<A>,
        <UrgentField<A> as SingularAccess>::Ref<'a>,
    >
{
}

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
    type Storage = Self;
    type Presence = TaskPresence;
    type Alloc = A;

    fn case(storage: &Self::Storage) -> Self::Case {
        match storage {
            Self::EmailAddress(_) => NotificationCase::EmailAddress,
            Self::PhoneNumber(_) => NotificationCase::PhoneNumber,
            Self::WebhookId(_) => NotificationCase::WebhookId,
            Self::Postal(_) => NotificationCase::Postal,
            Self::Urgent(_) => NotificationCase::Urgent,
        }
    }

    fn to_ref<'a>(
        storage: &'a Self::Storage,
        common: &'a MessageCommon<Self::Presence, Self::Alloc>,
    ) -> Self::Ref<'a> {
        match storage {
            Self::EmailAddress(f) => Notification::EmailAddress(f.value(common)),
            Self::PhoneNumber(f) => Notification::PhoneNumber(f.value(common)),
            Self::WebhookId(f) => Notification::WebhookId(f.value(common)),
            Self::Postal(f) => Notification::Postal(f.value()),
            Self::Urgent(f) => Notification::Urgent(f.value(common)),
        }
    }

    fn to_mut<'a>(
        storage: &'a mut Self::Storage,
        common: &'a mut MessageCommon<Self::Presence, Self::Alloc>,
    ) -> Self::Mut<'a> {
        match storage {
            Self::EmailAddress(f) => Notification::EmailAddress(f.value_mut(common)),
            Self::PhoneNumber(f) => Notification::PhoneNumber(f.value_mut(common)),
            Self::WebhookId(f) => Notification::WebhookId(f.value_mut(common)),
            Self::Postal(f) => Notification::Postal(f.value_mut()),
            Self::Urgent(_) => {
                Notification::Urgent(common.presence.bit_ref_mut(super::BIT_URGENT_VALUE))
            }
        }
    }
}

impl<A: Allocator + Clone> EnumVariant<EmailAddress> for NotificationStorage<A> {
    type Value = EmailAddressField<A>;
    type Alloc = A;

    fn new_value(alloc: A) -> Self::Value {
        SingularLenField::new_in(alloc)
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
        SingularLenField::new_in(alloc)
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
        SingularVarintField::new_in(alloc)
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
        NestedMessageField::with_message_in(alloc)
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
        SingularVarintField::new_in(alloc)
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
