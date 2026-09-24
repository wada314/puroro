//! Per-ingest **kinds** for [`TaskImpl`](crate::task_type::TaskImpl).
//!
//! [`Eager`](::puroro_rt::Eager) / [`Lazy`](::puroro_rt::Lazy) already select
//! [`MessageCommon`](::puroro_rt::MessageCommon) ingest. This trait only names
//! the catalog layouts that actually change:
//!
//! - singular string / bytes value slot (`InlineOrHeap` vs `WireOrSso`)
//! - repeated / map storage (`*Ready` vs `*Spans`)
//! - nested message type (`*Impl<A, Eager>` vs `*Impl<A, Lazy<A>>`)
//! - oneof string slots and `postal` child
//!
//! Field wrappers themselves live on [`TaskImpl`](crate::task_type::TaskImpl).

use ::allocator_api2::alloc::Allocator;
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;
use ::puroro_rt::decode::MessageScan;
use ::puroro_rt::{
    BitStorage, DeallocateBound, DeallocateIn, Eager, Expanded, InlineOrHeap, InteriorBitArray,
    Lazy, MapLayout, MapReady, MapSpans, MessageEncode, ProtoBytes, ProtoInt32, ProtoString,
    RepeatedLayout, RepeatedReady, RepeatedSpans, ValueLayout, WireOrSso,
};

use crate::AddressImpl;
use crate::PointImpl;

use super::{
    BIT_EMAIL_ADDRESS_LAZY_KIND, BIT_EMAIL_ADDRESS_SSO, BIT_OWNER_ID_LAZY_KIND, BIT_OWNER_ID_SSO,
    BIT_PAYLOAD_LAZY_KIND, BIT_PAYLOAD_SSO, BIT_PHONE_NUMBER_LAZY_KIND, BIT_PHONE_NUMBER_SSO,
    BIT_TITLE_LAZY_KIND, BIT_TITLE_SSO, FIELD_ATTRIBUTES, FIELD_LABELS,
};

/// Catalog kinds that change between eager `Task` and lazy `TaskLazy`.
pub trait TaskLayout<A: Allocator>: MessageScan<A> + Sized {
    /// Presence / SSO / lazy-kind bits packed into [`MessageCommon`].
    type Bits: Default + Clone + BitStorage;

    /// Singular string / bytes slot (`InlineOrHeap<{SSO}>` or `WireOrSso<{KIND}>`).
    type TitleLen: ValueLayout<ProtoString, A>;
    type OwnerIdLen: ValueLayout<ProtoString, A>;
    type PayloadLen: ValueLayout<ProtoBytes, A>;

    /// Repeated LEN storage (`RepeatedReady` or `RepeatedSpans`).
    type Repeated: RepeatedLayout<ProtoString, Expanded, { FIELD_LABELS }, A>;
    /// Map storage (`MapReady` or `MapSpans`).
    type Map: MapLayout<ProtoString, ProtoInt32, { FIELD_ATTRIBUTES }, A>;

    /// Nested `origin` message (`PointImpl<A, Eager>` or `PointImpl<A, Lazy<A>>`).
    type OriginMsg: MessageEncode + DeallocateBound<A>;
    /// Nested `watchers` element (`AddressImpl<A, Eager>` or `AddressImpl<A, Lazy<A>>`).
    type WatchersMsg: MessageEncode + DeallocateBound<A> + DeallocateIn<A>;

    /// `notification` string slots (`InlineOrHeap<{SSO}>` or `WireOrSso<{KIND}>`).
    type EmailLen: ValueLayout<ProtoString, A>;
    type PhoneLen: ValueLayout<ProtoString, A>;
    /// `notification.postal` (`AddressImpl<A, Eager>` or `AddressImpl<A, Lazy<A>>`).
    type PostalMsg: MessageEncode + DeallocateBound<A> + DeallocateIn<A>;

    fn empty_bits() -> Self::Bits;
}

impl<A: Allocator> TaskLayout<A> for Eager {
    type Bits = BitArray<[u8; 2], Lsb0>;
    type TitleLen = InlineOrHeap<{ BIT_TITLE_SSO }>;
    type OwnerIdLen = InlineOrHeap<{ BIT_OWNER_ID_SSO }>;
    type PayloadLen = InlineOrHeap<{ BIT_PAYLOAD_SSO }>;
    type Repeated = RepeatedReady;
    type Map = MapReady;
    type OriginMsg = PointImpl<A, Eager>;
    type WatchersMsg = AddressImpl<A, Eager>;
    type EmailLen = InlineOrHeap<{ BIT_EMAIL_ADDRESS_SSO }>;
    type PhoneLen = InlineOrHeap<{ BIT_PHONE_NUMBER_SSO }>;
    type PostalMsg = AddressImpl<A, Eager>;

    #[inline]
    fn empty_bits() -> Self::Bits {
        BitArray::ZERO
    }
}

impl<A: Allocator + Clone> TaskLayout<A> for Lazy<A> {
    type Bits = InteriorBitArray<4>;
    type TitleLen = WireOrSso<{ BIT_TITLE_LAZY_KIND }>;
    type OwnerIdLen = WireOrSso<{ BIT_OWNER_ID_LAZY_KIND }>;
    type PayloadLen = WireOrSso<{ BIT_PAYLOAD_LAZY_KIND }>;
    type Repeated = RepeatedSpans;
    type Map = MapSpans;
    type OriginMsg = PointImpl<A, Lazy<A>>;
    type WatchersMsg = AddressImpl<A, Lazy<A>>;
    type EmailLen = WireOrSso<{ BIT_EMAIL_ADDRESS_LAZY_KIND }>;
    type PhoneLen = WireOrSso<{ BIT_PHONE_NUMBER_LAZY_KIND }>;
    type PostalMsg = AddressImpl<A, Lazy<A>>;

    #[inline]
    fn empty_bits() -> Self::Bits {
        InteriorBitArray::zero()
    }
}
