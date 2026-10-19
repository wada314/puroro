//! Unknown field view types for the stable public API.
//!
//! Generated messages expose unknown fields as an iterator of [`UnknownField`],
//! not as a raw wire blob. Internal storage may still be a contiguous partial
//! protobuf stream; that representation is an implementation detail.

use crate::WireType;

/// One unknown field occurrence as seen on the wire.
///
/// The same field number may appear more than once (e.g. repeated unknowns or
/// multiple closed-enum unknowns diverted into the unknown set).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnknownField<'a> {
    number: u32,
    payload: UnknownPayload<'a>,
}

impl<'a> UnknownField<'a> {
    /// Creates a view of one unknown field.
    ///
    /// Intended for runtime helpers that parse preserved wire bytes; application
    /// code normally obtains instances from message `unknown_fields()` iterators.
    #[inline]
    pub fn new(number: u32, payload: UnknownPayload<'a>) -> Self {
        Self { number, payload }
    }

    /// Field number from the wire tag.
    #[inline]
    pub fn number(&self) -> u32 {
        self.number
    }

    /// Decoded payload for this field.
    #[inline]
    pub fn payload(&self) -> &UnknownPayload<'a> {
        &self.payload
    }

    /// Wire type implied by [`payload`](Self::payload).
    #[inline]
    pub fn wire_type(&self) -> WireType {
        self.payload.wire_type()
    }
}

/// Payload of an unknown field, discriminated by wire type.
///
/// Deprecated group wire types (`SGroup` / `EGroup`) are never preserved and
/// therefore have no variant here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnknownPayload<'a> {
    /// Wire type VARINT.
    Varint(u64),
    /// Wire type I64 (fixed64 / sfixed64 / double bit pattern).
    Fixed64(u64),
    /// Wire type LEN — payload bytes only (length prefix not included).
    Bytes(&'a [u8]),
    /// Wire type I32 (fixed32 / sfixed32 / float bit pattern).
    Fixed32(u32),
}

impl UnknownPayload<'_> {
    /// Wire type corresponding to this payload variant.
    #[inline]
    pub fn wire_type(&self) -> WireType {
        match self {
            Self::Varint(_) => WireType::Varint,
            Self::Fixed64(_) => WireType::Int64,
            Self::Bytes(_) => WireType::Len,
            Self::Fixed32(_) => WireType::Int32,
        }
    }
}
