//! Per-ingest associated types for [`Point`](crate::Point) / [`PointLazy`](crate::PointLazy).
//!
//! Both fields are implicit int32. Only [`MessageCommon`](::puroro_rt::MessageCommon)
//! ingest (`Eager` / `Lazy`) differs.

use ::allocator_api2::alloc::Allocator;
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;
use ::puroro_rt::{BitStorage, Eager, Lazy};

/// Ingest bits for [`Point`](crate::Point) / [`PointLazy`](crate::PointLazy).
///
/// Not a [`MessageScan`](::puroro_rt::decode::MessageScan) supertrait.
/// [`Point`](crate::Point) requires `A: Clone` on the struct,
/// together with every other generated message.
pub trait Layout<A: Allocator>: Sized {
    type Bits: Default + Clone + BitStorage;

    fn empty_bits() -> Self::Bits;
}

impl<A: Allocator> Layout<A> for Eager {
    type Bits = BitArray<[u8; 1], Lsb0>;

    #[inline]
    fn empty_bits() -> Self::Bits {
        BitArray::ZERO
    }
}

impl<A: Allocator> Layout<A> for Lazy<A> {
    type Bits = BitArray<[u8; 1], Lsb0>;

    #[inline]
    fn empty_bits() -> Self::Bits {
        BitArray::ZERO
    }
}
