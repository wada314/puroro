//! Fixed-width markers (`ProtoFixed32`, `ProtoFloat`, `ProtoDouble`, …).
//!
//! Wire codecs live on [`NumericalType`](super::numerical::NumericalType).
//!
//! Float [`ProtoEmpty`] uses Rust `== 0.0` (`-0.0` is empty; `NaN` is non-empty).

use ::allocator_api2::alloc::Allocator;

use crate::fields::shared::value_slot::AddressableSlot;
use crate::fields::shared::{DefaultIn, ProtoEmpty};

macro_rules! proto_fixed_marker {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
        pub struct $name;
    };
}

proto_fixed_marker! {
    /// Protobuf `fixed32`.
    ProtoFixed32
}
proto_fixed_marker! {
    /// Protobuf `sfixed32`.
    ProtoSFixed32
}
proto_fixed_marker! {
    /// Protobuf `float`.
    ProtoFloat
}
proto_fixed_marker! {
    /// Protobuf `fixed64`.
    ProtoFixed64
}
proto_fixed_marker! {
    /// Protobuf `sfixed64`.
    ProtoSFixed64
}
proto_fixed_marker! {
    /// Protobuf `double`.
    ProtoDouble
}

// ---------------------------------------------------------------------------
// Slot infrastructure for f32 / f64
// ---------------------------------------------------------------------------

impl AddressableSlot for f32 {}
impl AddressableSlot for f64 {}

impl<A: Allocator + Clone> DefaultIn<A> for f32 {
    #[inline]
    fn default_in(_alloc: A) -> Self {
        0.0
    }
}

impl<A: Allocator + Clone> DefaultIn<A> for f64 {
    #[inline]
    fn default_in(_alloc: A) -> Self {
        0.0
    }
}

/// Float empty check uses Rust `== 0.0` (`-0.0` is empty; `NaN` is non-empty).
impl ProtoEmpty for f32 {
    #[inline]
    fn is_proto_empty(&self) -> bool {
        *self == 0.0
    }
}

/// Float empty check uses Rust `== 0.0` (`-0.0` is empty; `NaN` is non-empty).
impl ProtoEmpty for f64 {
    #[inline]
    fn is_proto_empty(&self) -> bool {
        *self == 0.0
    }
}
