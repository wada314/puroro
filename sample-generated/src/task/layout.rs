//! Per-ingest associated types for [`TaskImpl`](crate::task_type::TaskImpl).
//!
//! [`Eager`](::puroro_rt::Eager) and [`Lazy`](::puroro_rt::Lazy) already select
//! [`MessageCommon`](::puroro_rt::MessageCommon) ingest. This trait adds the
//! field wrappers that actually differ (string layout, span lists, nested
//! `*Lazy`, oneof storage). Numerical catalog slots stay on `TaskImpl`.

use ::allocator_api2::alloc::Allocator;
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;
use ::puroro_rt::decode::MessageScan;
use ::puroro_rt::{
    BitStorage, Eager, Expanded, Explicit, FieldDeallocate, InlineOrHeap, InteriorBitArray, Lazy,
    LegacyRequired, MapField, MapReady, MapSpans, MessageCommon, OneofDeallocate, ProtoBytes,
    ProtoInt32, ProtoMessage, ProtoString, RepeatedField, RepeatedReady, RepeatedSpans,
    SingularField, UnknownFields, WireOrSso,
};

use crate::Address;
use crate::AddressLazy;
use crate::Point;
use crate::point::PointLazy;

use super::notification::NotificationStorage;
use super::notification_lazy::NotificationLazyStorage;
use super::{
    BIT_ORIGIN, BIT_OWNER_ID, BIT_OWNER_ID_LAZY_KIND, BIT_OWNER_ID_SSO, BIT_PAYLOAD,
    BIT_PAYLOAD_LAZY_KIND, BIT_PAYLOAD_SSO, BIT_TITLE, BIT_TITLE_LAZY_KIND, BIT_TITLE_SSO,
    FIELD_ATTRIBUTES, FIELD_LABELS, FIELD_ORIGIN, FIELD_OWNER_ID, FIELD_PAYLOAD, FIELD_TITLE,
    FIELD_WATCHERS,
};

/// Field wrappers that change between eager `Task` and lazy `TaskLazy`.
pub trait TaskLayout<A: Allocator>: MessageScan<A> + Sized {
    /// Presence / SSO / lazy-kind bits packed into [`MessageCommon`].
    type Bits: Default + Clone + BitStorage;

    type Title: FieldDeallocate<MessageCommon<Self::Bits, A, UnknownFields<A>, Self>>;
    type OwnerId: FieldDeallocate<MessageCommon<Self::Bits, A, UnknownFields<A>, Self>>;
    type Payload: FieldDeallocate<MessageCommon<Self::Bits, A, UnknownFields<A>, Self>>;
    type Labels: FieldDeallocate<MessageCommon<Self::Bits, A, UnknownFields<A>, Self>>;
    type Attributes: FieldDeallocate<MessageCommon<Self::Bits, A, UnknownFields<A>, Self>>;
    type Origin: FieldDeallocate<MessageCommon<Self::Bits, A, UnknownFields<A>, Self>>;
    type Watchers: FieldDeallocate<MessageCommon<Self::Bits, A, UnknownFields<A>, Self>>;
    type Notification: OneofDeallocate<MessageCommon<Self::Bits, A, UnknownFields<A>, Self>>;

    fn empty_bits() -> Self::Bits;
}

impl<A: Allocator> TaskLayout<A> for Eager {
    type Bits = BitArray<[u8; 2], Lsb0>;
    type Title = SingularField<
        ProtoString,
        Explicit<{ BIT_TITLE }>,
        { FIELD_TITLE },
        A,
        InlineOrHeap<{ BIT_TITLE_SSO }>,
    >;
    type OwnerId = SingularField<
        ProtoString,
        LegacyRequired<{ BIT_OWNER_ID }>,
        { FIELD_OWNER_ID },
        A,
        InlineOrHeap<{ BIT_OWNER_ID_SSO }>,
    >;
    type Payload = SingularField<
        ProtoBytes,
        Explicit<{ BIT_PAYLOAD }>,
        { FIELD_PAYLOAD },
        A,
        InlineOrHeap<{ BIT_PAYLOAD_SSO }>,
    >;
    type Labels = RepeatedField<ProtoString, Expanded, { FIELD_LABELS }, A, RepeatedReady>;
    type Attributes = MapField<ProtoString, ProtoInt32, { FIELD_ATTRIBUTES }, A, MapReady>;
    type Origin =
        SingularField<ProtoMessage<Point<A>>, Explicit<{ BIT_ORIGIN }>, { FIELD_ORIGIN }, A>;
    type Watchers = RepeatedField<ProtoMessage<Address<A>>, Expanded, { FIELD_WATCHERS }, A>;
    type Notification = NotificationStorage<A>;

    #[inline]
    fn empty_bits() -> Self::Bits {
        BitArray::ZERO
    }
}

impl<A: Allocator + Clone> TaskLayout<A> for Lazy<A> {
    type Bits = InteriorBitArray<4>;
    type Title = SingularField<
        ProtoString,
        Explicit<{ BIT_TITLE }>,
        { FIELD_TITLE },
        A,
        WireOrSso<{ BIT_TITLE_LAZY_KIND }>,
    >;
    type OwnerId = SingularField<
        ProtoString,
        LegacyRequired<{ BIT_OWNER_ID }>,
        { FIELD_OWNER_ID },
        A,
        WireOrSso<{ BIT_OWNER_ID_LAZY_KIND }>,
    >;
    type Payload = SingularField<
        ProtoBytes,
        Explicit<{ BIT_PAYLOAD }>,
        { FIELD_PAYLOAD },
        A,
        WireOrSso<{ BIT_PAYLOAD_LAZY_KIND }>,
    >;
    type Labels = RepeatedField<ProtoString, Expanded, { FIELD_LABELS }, A, RepeatedSpans>;
    type Attributes = MapField<ProtoString, ProtoInt32, { FIELD_ATTRIBUTES }, A, MapSpans>;
    type Origin =
        SingularField<ProtoMessage<PointLazy<A>>, Explicit<{ BIT_ORIGIN }>, { FIELD_ORIGIN }, A>;
    type Watchers = RepeatedField<ProtoMessage<AddressLazy<A>>, Expanded, { FIELD_WATCHERS }, A>;
    type Notification = NotificationLazyStorage<A>;

    #[inline]
    fn empty_bits() -> Self::Bits {
        InteriorBitArray::zero()
    }
}
