//! Per-ingest **kinds** for [`AddressImpl`](crate::address_type::AddressImpl).
//!
//! Numericals stay on the struct. This trait only names the string value
//! slots that change (`InlineOrHeap` vs `WireOrSso`).
//!
//! Not a [`MessageScan`](::puroro_rt::decode::MessageScan) supertrait: eager
//! parents name [`AddressLazy`](crate::AddressLazy) as a child type, which must
//! stay well-formed without `A: Clone`.

use ::allocator_api2::alloc::Allocator;
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;
use ::puroro_rt::{
    BitStorage, Eager, InlineOrHeap, InteriorBitArray, Lazy, ProtoString, ValueLayout, WireOrSso,
};

use super::{BIT_CITY_LAZY_KIND, BIT_CITY_SSO, BIT_STREET_LAZY_KIND, BIT_STREET_SSO};

/// Catalog kinds that change between eager `Address` and lazy `AddressLazy`.
pub trait AddressLayout<A: Allocator>: Sized {
    type Bits: Default + Clone + BitStorage;

    /// Singular string slot (`InlineOrHeap<{SSO}>` or `WireOrSso<{KIND}>`).
    type StreetLen: ValueLayout<ProtoString, A>;
    type CityLen: ValueLayout<ProtoString, A>;

    fn empty_bits() -> Self::Bits;
}

impl<A: Allocator> AddressLayout<A> for Eager {
    type Bits = BitArray<[u8; 1], Lsb0>;
    type StreetLen = InlineOrHeap<{ BIT_STREET_SSO }>;
    type CityLen = InlineOrHeap<{ BIT_CITY_SSO }>;

    #[inline]
    fn empty_bits() -> Self::Bits {
        BitArray::ZERO
    }
}

impl<A: Allocator> AddressLayout<A> for Lazy<A> {
    type Bits = InteriorBitArray<2>;
    type StreetLen = WireOrSso<{ BIT_STREET_LAZY_KIND }>;
    type CityLen = WireOrSso<{ BIT_CITY_LAZY_KIND }>;

    #[inline]
    fn empty_bits() -> Self::Bits {
        InteriorBitArray::zero()
    }
}
