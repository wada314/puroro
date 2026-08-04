//! Fixed-width markers (`ProtoFixed32`, `ProtoFloat`, `ProtoDouble`, …).
//!
//! Wire codecs live on [`NumericalType`](super::numerical::NumericalType).

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
