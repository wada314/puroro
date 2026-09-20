//! Per-ingest associated types for [`AddressImpl`](crate::address_type::AddressImpl).
//!
//! Numericals stay on the struct. String slots change with `L`.

use ::allocator_api2::alloc::Allocator;
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;
use ::puroro_rt::{
    BitStorage, Eager, Explicit, FieldDeallocate, InlineOrHeap, InteriorBitArray, Lazy,
    MessageCommon, ProtoString, SingularField, UnknownFields, WireOrSso,
};

use super::{
    BIT_CITY, BIT_CITY_LAZY_KIND, BIT_CITY_SSO, BIT_STREET, BIT_STREET_LAZY_KIND, BIT_STREET_SSO,
    FIELD_CITY, FIELD_STREET,
};

/// Field wrappers that change between eager `Address` and lazy `AddressLazy`.
///
/// Not a [`MessageScan`](::puroro_rt::decode::MessageScan) supertrait: eager
/// parents name [`AddressLazy`](crate::AddressLazy) as a child type, which must
/// stay well-formed without `A: Clone`.
pub trait AddressLayout<A: Allocator>: Sized {
    type Bits: Default + Clone + BitStorage;

    type Street: FieldDeallocate<MessageCommon<Self::Bits, A, UnknownFields<A>, Self>>;
    type City: FieldDeallocate<MessageCommon<Self::Bits, A, UnknownFields<A>, Self>>;

    fn empty_bits() -> Self::Bits;
}

impl<A: Allocator> AddressLayout<A> for Eager {
    type Bits = BitArray<[u8; 1], Lsb0>;
    type Street = SingularField<
        ProtoString,
        Explicit<{ BIT_STREET }>,
        { FIELD_STREET },
        A,
        InlineOrHeap<{ BIT_STREET_SSO }>,
    >;
    type City = SingularField<
        ProtoString,
        Explicit<{ BIT_CITY }>,
        { FIELD_CITY },
        A,
        InlineOrHeap<{ BIT_CITY_SSO }>,
    >;

    #[inline]
    fn empty_bits() -> Self::Bits {
        BitArray::ZERO
    }
}

impl<A: Allocator> AddressLayout<A> for Lazy<A> {
    type Bits = InteriorBitArray<2>;
    type Street = SingularField<
        ProtoString,
        Explicit<{ BIT_STREET }>,
        { FIELD_STREET },
        A,
        WireOrSso<{ BIT_STREET_LAZY_KIND }>,
    >;
    type City = SingularField<
        ProtoString,
        Explicit<{ BIT_CITY }>,
        { FIELD_CITY },
        A,
        WireOrSso<{ BIT_CITY_LAZY_KIND }>,
    >;

    #[inline]
    fn empty_bits() -> Self::Bits {
        InteriorBitArray::zero()
    }
}
