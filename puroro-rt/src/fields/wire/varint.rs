//! Varint-family markers and enum kind traits.
//!
//! Scalar markers (`ProtoInt32`, [`ProtoBool`], …) and [`ProtoEnum`] live in
//! [`numerical`](super::numerical). Singular [`ProtoBool`] storage is
//! bit-packed via [`BitPacked`](crate::BitPacked).

use ::core::convert::TryFrom;

use crate::fields::shared::value_slot::AddressableSlot;

// ---------------------------------------------------------------------------
// Enum kind markers
// ---------------------------------------------------------------------------

/// Open-enum kind marker for [`ProtoEnum`](super::numerical::ProtoEnum)
/// (`enum_type = OPEN`).
pub struct Open;

/// Closed-enum kind marker for [`ProtoEnum`](super::numerical::ProtoEnum)
/// (`enum_type = CLOSED`).
pub struct Closed;

/// Generated enum newtype storage: wire `i32` plus a known default.
pub trait ProtoEnumStorage: Copy + PartialEq + Default + 'static {
    /// Numeric value written on the wire.
    fn to_wire(self) -> i32;
}

/// Open enum: unknown wire values are retained via [`From`]`<i32>`.
pub trait OpenEnum: ProtoEnumStorage + From<i32> {}

/// Closed enum: unknown wire values fail [`TryFrom`]`<i32>` and become unknowns.
pub trait ClosedEnum: ProtoEnumStorage + TryFrom<i32, Error = i32> {}

impl<E: ProtoEnumStorage> AddressableSlot for E {}

// Enum storage types are `Copy` + `Default`; `CloneIn` / `DefaultIn` /
// `DeallocateIn` come from `unmanaged` blankets.

// Re-export varint scalar / enum markers defined with [`Numerical`](super::numerical::Numerical).
pub use super::numerical::{
    ProtoBool, ProtoEnum, ProtoInt32, ProtoInt64, ProtoSInt32, ProtoSInt64, ProtoUInt32,
    ProtoUInt64,
};
