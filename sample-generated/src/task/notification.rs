//! Sample of the `oneof notification` types puroro generates for `Task`.
//!
//! A oneof that owns heap storage is represented by several generated items,
//! because the owned storage holds allocator-less `unmanaged` values (unsafe to
//! drop implicitly) and must not leak into the public API:
//!
//! - [`Notification`] — canonical **shape** enum; variant payloads are type
//!   parameters. [`NotificationStorage`] is a `pub(crate)` alias of this shape.
//! - [`NotificationStorage`] — field-wrapper payloads; implements [`OneofGroup`],
//!   [`OneofDeallocate`], encode glue. Never public.
//! - [`NotificationCase`] — a payload-less, `Copy` discriminant of which variant
//!   is active.
//!
//! Shared projections (`OneofGroup::Ref` / `Mut`) are written inline on
//! [`OneofGroup`] — Ref uses concrete user-facing types; Mut uses
//! [`SingularType::Mut`](::puroro_rt::SingularType). There are no public Ref/Mut aliases.
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
//! | `urgent` | `bool` | [`SingularField`] + [`ProtoBool`] | `bool` | `SingularType::Mut` (named bit handle) |
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
//! `slot.bind_mut(common).variant_mut::<FIELD_…>().bind_mut(common).merge(...)`
//! — uniformly for every variant kind.
//!
//! Per-variant dispatch uses [`OneofVariant`] keyed by each variant's proto
//! field number (`super::FIELD_*`).
//!
//! Note: this enum cannot carry unused lifetime/allocator parameters via
//! `PhantomData` (unlike a struct). Integer-only oneofs therefore omit `'a` / `A`
//! from the shape when no variant payload needs them.

use ::allocator_api2::alloc::Allocator;
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;
use ::bytes::BufMut;
use ::puroro_rt::{
    BitPacked, FieldCloneIn, FieldDeallocate, FieldEncode, Inline, MessageCommon,
    MessageCommonAlloc, MessageCommonBits, Oneof, OneofDeallocate, OneofEncodable, OneofGroup,
    OneofVariant, ProtoBool, ProtoInt32, ProtoMessage, ProtoString, SingularField, SingularType,
};

use crate::address::Address;

use super::defaults::WebhookIdDefault;

/// Canonical shape for `oneof notification`.
///
/// Note: this enum cannot carry unused lifetime/allocator parameters via
/// `PhantomData` (unlike a struct). Integer-only oneofs therefore omit `'a` / `A`
/// from the shape when no variant payload needs them.
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

impl<A: Allocator + Clone> OneofGroup for NotificationStorage<A> {
    type Case = NotificationCase;
    type Ref<'a>
        = Notification<&'a str, &'a str, i32, &'a Address<A>, bool>
    where
        A: 'a;
    type Mut<'a>
        = Notification<
        <ProtoString as SingularType>::Mut<'a, A>,
        <ProtoString as SingularType>::Mut<'a, A>,
        <ProtoInt32 as SingularType>::Mut<'a, A>,
        <ProtoMessage<Address<A>> as SingularType>::Mut<'a, A>,
        <ProtoBool as SingularType>::Mut<'a, A>,
    >
    where
        A: 'a;
    type Presence = BitArray<[u8; 2], Lsb0>;
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
            Self::Urgent(_) => Notification::Urgent(common.bit_mut(super::BIT_URGENT_VALUE)),
        }
    }

    fn clone_storage_in(
        storage: &Self,
        common: &MessageCommon<Self::Presence, Self::Alloc>,
        alloc: Self::Alloc,
    ) -> Self {
        match storage {
            Self::EmailAddress(f) => {
                Self::EmailAddress(FieldCloneIn::clone_field(f, common, alloc))
            }
            Self::PhoneNumber(f) => Self::PhoneNumber(FieldCloneIn::clone_field(f, common, alloc)),
            Self::WebhookId(f) => Self::WebhookId(FieldCloneIn::clone_field(f, common, alloc)),
            Self::Postal(f) => Self::Postal(FieldCloneIn::clone_field(f, common, alloc)),
            Self::Urgent(f) => Self::Urgent(FieldCloneIn::clone_field(f, common, alloc)),
        }
    }
}

impl<A: Allocator + Clone> OneofVariant<{ super::FIELD_EMAIL_ADDRESS }> for NotificationStorage<A> {
    type Value = EmailAddressField<A>;

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

impl<A: Allocator + Clone> OneofVariant<{ super::FIELD_PHONE_NUMBER }> for NotificationStorage<A> {
    type Value = PhoneNumberField<A>;

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

impl<A: Allocator + Clone> OneofVariant<{ super::FIELD_WEBHOOK_ID }> for NotificationStorage<A> {
    type Value = WebhookIdField<A>;

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

impl<A: Allocator + Clone> OneofVariant<{ super::FIELD_POSTAL }> for NotificationStorage<A> {
    type Value = PostalField<A>;

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

impl<A: Allocator + Clone> OneofVariant<{ super::FIELD_URGENT }> for NotificationStorage<A> {
    type Value = UrgentField<A>;

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
    fn encoded_len<P>(&self, common: &MessageCommon<P, A>) -> usize
    where
        MessageCommon<P, A>: MessageCommonBits + MessageCommonAlloc<Alloc = A>,
    {
        match self {
            Self::EmailAddress(f) => FieldEncode::encoded_len(f, common),
            Self::PhoneNumber(f) => FieldEncode::encoded_len(f, common),
            Self::WebhookId(f) => FieldEncode::encoded_len(f, common),
            Self::Postal(f) => FieldEncode::encoded_len(f, common),
            Self::Urgent(f) => FieldEncode::encoded_len(f, common),
        }
    }

    fn encode_raw<P, B: BufMut>(&self, common: &MessageCommon<P, A>, buf: &mut B)
    where
        MessageCommon<P, A>: MessageCommonBits + MessageCommonAlloc<Alloc = A>,
    {
        match self {
            Self::EmailAddress(f) => FieldEncode::encode_raw(f, common, buf),
            Self::PhoneNumber(f) => FieldEncode::encode_raw(f, common, buf),
            Self::WebhookId(f) => FieldEncode::encode_raw(f, common, buf),
            Self::Postal(f) => FieldEncode::encode_raw(f, common, buf),
            Self::Urgent(f) => FieldEncode::encode_raw(f, common, buf),
        }
    }
}

impl<A: Allocator + Clone, P> OneofDeallocate<MessageCommon<P, A>> for NotificationStorage<A> {
    /// # Safety
    ///
    /// `common.alloc` must be the allocator that owns the variant's buffer.
    unsafe fn deallocate(self, common: &MessageCommon<P, A>) {
        match self {
            Self::EmailAddress(mut f) => f.deallocate(common),
            Self::PhoneNumber(mut f) => f.deallocate(common),
            Self::WebhookId(mut f) => f.deallocate(common),
            Self::Postal(mut f) => f.deallocate(common),
            Self::Urgent(mut f) => f.deallocate(common),
        }
    }
}
