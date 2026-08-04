//! Varint family markers (`ProtoInt32`, enums, [`ProtoBool`], …).
//!
//! Numeric / enum / bool wire codecs live on [`NumericalType`](super::numerical::NumericalType).
//! Singular [`ProtoBool`] storage is bit-packed via [`BitPacked`](crate::BitPacked).

use ::core::convert::TryFrom;
use ::core::marker::PhantomData;

use crate::fields::shared::value_slot::AddressableSlot;

// ---------------------------------------------------------------------------
// Enum markers
// ---------------------------------------------------------------------------

/// Open-enum kind marker for [`ProtoEnum`] (`enum_type = OPEN`).
pub struct Open;

/// Closed-enum kind marker for [`ProtoEnum`] (`enum_type = CLOSED`).
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

/// Allocator-free enum type marker (`Open` / `Closed`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoEnum<E, K>(PhantomData<(E, K)>);

impl<E: ProtoEnumStorage> AddressableSlot for E {}

// Enum storage types are `Copy` + `Default`; `CloneIn` / `DefaultIn` /
// `DeallocateIn` come from `unmanaged` blankets. [`ProtoEmpty`] lives in
// `shared`.

// ---------------------------------------------------------------------------
// Numeric markers (wire via NumericalType)
// ---------------------------------------------------------------------------

/// Protobuf `uint32`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoUInt32;

/// Protobuf `uint64`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoUInt64;

/// Protobuf `int32`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoInt32;

/// Protobuf `int64`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoInt64;

/// Protobuf `sint32` — varint with ZigZag encoding.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoSInt32;

/// Protobuf `sint64` — varint with ZigZag encoding.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoSInt64;

/// Protobuf `bool` type marker — varint 0 or 1.
///
/// Singular / oneof: slot is `()`; logical value via [`BitPacked`](crate::BitPacked).
/// Repeated: plain `bool` elements in the vec (no MessageCommon bit index).
/// Wire codec: [`NumericalType`](super::numerical::NumericalType).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoBool;
