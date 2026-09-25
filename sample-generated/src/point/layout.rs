//! Per-ingest associated types for [`PointImpl`](crate::point_type::PointImpl).
//!
//! Both fields are implicit int32. Only [`MessageCommon`](::puroro_rt::MessageCommon)
//! ingest (`Eager` / `Lazy`) differs.

use ::allocator_api2::alloc::Allocator;
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;
use ::puroro_rt::{BitStorage, Eager, Lazy};

/// Ingest bits for [`PointImpl`](crate::point_type::PointImpl).
///
/// Not a [`MessageScan`](::puroro_rt::decode::MessageScan) supertrait.
/// [`PointImpl`](crate::point_type::PointImpl) requires `A: Clone` on the struct,
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
