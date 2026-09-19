//! Lazy `oneof notification` storage: same shape as eager, different field types.
//!
//! String variants use [`WireOrSso`]; `postal` is [`AddressLazy`]. Numerical
//! variants keep the eager catalog wrappers and apply during the parent scan.

use ::allocator_api2::alloc::Allocator;
use ::puroro_rt::{
    BitPacked, FieldCloneIn, FieldDeallocate, Inline, InteriorBitArray, Lazy, MessageCommon,
    MessageCommonBits, Oneof, OneofDeallocate, OneofGroup, ProtoBool, ProtoInt32, ProtoMessage,
    ProtoString, SingularField, UnknownFields, UnknownStore, WireOrSso,
};

use crate::AddressLazy;

use super::Notification;
use super::NotificationCase;
use super::defaults::WebhookIdDefault;

type EmailAddressField<A> = SingularField<
    ProtoString,
    Oneof,
    { super::FIELD_EMAIL_ADDRESS },
    A,
    WireOrSso<{ super::BIT_EMAIL_ADDRESS_LAZY_KIND }>,
>;
type PhoneNumberField<A> = SingularField<
    ProtoString,
    Oneof,
    { super::FIELD_PHONE_NUMBER },
    A,
    WireOrSso<{ super::BIT_PHONE_NUMBER_LAZY_KIND }>,
>;
type WebhookIdField<A> =
    SingularField<ProtoInt32, Oneof, { super::FIELD_WEBHOOK_ID }, A, Inline, WebhookIdDefault>;
type PostalField<A> =
    SingularField<ProtoMessage<AddressLazy<A>>, Oneof, { super::FIELD_POSTAL }, A>;
type UrgentField<A> = SingularField<
    ProtoBool,
    Oneof,
    { super::FIELD_URGENT },
    A,
    BitPacked<{ super::BIT_URGENT_VALUE }>,
>;

/// Owned lazy storage for `oneof notification` (crate-internal).
pub(crate) type NotificationLazyStorage<A> = Notification<
    EmailAddressField<A>,
    PhoneNumberField<A>,
    WebhookIdField<A>,
    PostalField<A>,
    UrgentField<A>,
>;

impl<A: Allocator> OneofGroup for NotificationLazyStorage<A> {
    type Case = NotificationCase;
    type Ref<'a>
        = NotificationCase
    where
        A: 'a;
    type Mut<'a>
        = ()
    where
        A: 'a;
    type Bits = InteriorBitArray<4>;
    type Alloc = A;
    type Unknown = UnknownFields<A>;
    type Layout = Lazy<A>;

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
        _common: &'a MessageCommon<Self::Bits, Self::Alloc, Self::Unknown, Self::Layout>,
    ) -> Self::Ref<'a> {
        Self::case(storage)
    }

    fn to_mut<'a>(
        _storage: &'a mut Self,
        _common: &'a mut MessageCommon<Self::Bits, Self::Alloc, Self::Unknown, Self::Layout>,
    ) -> Self::Mut<'a>
    where
        A: Clone,
    {
    }

    fn clone_storage_in(
        storage: &Self,
        common: &MessageCommon<Self::Bits, Self::Alloc, Self::Unknown, Self::Layout>,
        alloc: Self::Alloc,
    ) -> Self
    where
        A: Clone,
    {
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

    fn after_deallocate(
        common: &mut MessageCommon<Self::Bits, Self::Alloc, Self::Unknown, Self::Layout>,
    ) where
        A: Clone,
    {
        common.set_bit(super::BIT_EMAIL_ADDRESS_LAZY_KIND, false);
        common.set_bit(super::BIT_EMAIL_ADDRESS_LAZY_KIND + 1, false);
        common.set_bit(super::BIT_PHONE_NUMBER_LAZY_KIND, false);
        common.set_bit(super::BIT_PHONE_NUMBER_LAZY_KIND + 1, false);
        common.set_bit(super::BIT_URGENT_VALUE, false);
    }
}

impl<A: Allocator, P, U: UnknownStore<A>, L> OneofDeallocate<MessageCommon<P, A, U, L>>
    for NotificationLazyStorage<A>
where
    MessageCommon<P, A, U, L>: MessageCommonBits,
{
    unsafe fn deallocate(self, common: &MessageCommon<P, A, U, L>) {
        match self {
            Self::EmailAddress(mut f) => f.deallocate(common),
            Self::PhoneNumber(mut f) => f.deallocate(common),
            Self::WebhookId(mut f) => f.deallocate(common),
            Self::Postal(mut f) => f.deallocate(common),
            Self::Urgent(mut f) => f.deallocate(common),
        }
    }
}
