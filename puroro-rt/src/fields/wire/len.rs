//! Length-delimited scalar markers ([`ProtoString`] / [`ProtoBytes`]).
//!
//! Markers are allocator-free. Singular and repeated storage is
//! [`UnmanagedString`](::unmanaged::UnmanagedString) /
//! [`UnmanagedVec`](::unmanaged::UnmanagedVec).

/// Protobuf `string` type marker.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoString;

/// Protobuf `bytes` type marker.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProtoBytes;
