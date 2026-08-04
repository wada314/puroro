//! Fixed-width markers (`ProtoFixed32`, `ProtoFloat`, `ProtoDouble`, …).
//!
//! Wire codecs live on [`NumericalType`](super::numerical::NumericalType).
//!
//! Float [`ProtoEmpty`] uses Rust `== 0.0` (`-0.0` is empty; `NaN` is non-empty).

use crate::fields::shared::ProtoEmpty;
use crate::fields::shared::value_slot::AddressableSlot;

/// Protobuf `fixed32`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoFixed32;

/// Protobuf `sfixed32`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoSFixed32;

/// Protobuf `float`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoFloat;

/// Protobuf `fixed64`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoFixed64;

/// Protobuf `sfixed64`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoSFixed64;

/// Protobuf `double`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoDouble;

// ---------------------------------------------------------------------------
// Slot infrastructure for f32 / f64
// ---------------------------------------------------------------------------

impl AddressableSlot for f32 {}
impl AddressableSlot for f64 {}

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
