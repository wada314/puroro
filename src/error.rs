//! Decode and encode error types for the public [`Message`](crate::Message) API.
//!
//! [`DecodeError`] covers wire parsing, UTF-8 checks, recursion limits, and
//! post-decode [`Message::validate`](crate::Message::validate) failures
//! ([`DecodeError::MissingRequiredField`]). Closed-enum unknowns use
//! [`DecodeError::UnknownClosedEnum`] internally and are diverted into the
//! message unknown-field set rather than aborting `merge_from`.
//!
//! [`EncodeError`] is reserved for fallible sinks (e.g. fixed-capacity buffers);
//! encoding into a growing `Vec` / `BytesMut` is effectively infallible.

use ::core::fmt;
use ::std::error::Error;

use crate::wire_type::WireType;

/// Errors that can occur while decoding a protobuf message from the wire format.
#[derive(Debug, Clone, PartialEq)]
pub enum DecodeError {
    /// The input buffer ended before the message was complete.
    UnexpectedEof,
    /// A varint was longer than 10 bytes (exceeds u64 range).
    InvalidVarint,
    /// The tag byte contained an unrecognised wire type id.
    InvalidTag,
    /// A field was encountered with the correct field number but an unexpected wire type.
    UnexpectedWireType {
        field_number: u32,
        expected: WireType,
        actual: WireType,
    },
    /// A string field contained bytes that are not valid UTF-8.
    InvalidUtf8,
    /// A nested message's declared length exceeded the remaining buffer.
    TruncatedMessage,
    /// Decode recursion exceeded the implementation-defined limit.
    RecursionLimitExceeded,
    /// A proto2 `required` field was absent from the wire.
    ///
    /// This error is only produced by the generated `validate()` method, which
    /// callers must invoke explicitly after `merge_from` or `decode`.
    MissingRequiredField { field_number: u32 },
    /// A closed-enum field saw a wire value outside the known set.
    ///
    /// This is **not** a fatal decode failure for message parsing: the singular
    /// field `merge` path in `puroro-rt` catches it and appends `raw` to the
    /// message's unknown fields. It lives on [`DecodeError`] so the shared
    /// `decode` → `merge` `Result` path can divert without a separate
    /// control-flow type.
    ///
    /// Spec: [Enum Behavior](https://protobuf.dev/programming-guides/enum/) —
    /// closed enums store unrecognized values in the unknown field set; accessors
    /// report the field as unset and return the enum default.
    UnknownClosedEnum {
        /// Raw varint numeric value as read from the wire (for unknown-field round-trip).
        raw: u64,
    },
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::UnexpectedEof => write!(f, "unexpected end of input"),
            DecodeError::InvalidVarint => write!(f, "varint is too long (> 10 bytes)"),
            DecodeError::InvalidTag => write!(f, "tag contains an unknown wire type"),
            DecodeError::UnexpectedWireType {
                field_number,
                expected,
                actual,
            } => write!(
                f,
                "field {field_number}: expected wire type {expected:?}, got {actual:?}"
            ),
            DecodeError::InvalidUtf8 => write!(f, "string field is not valid UTF-8"),
            DecodeError::TruncatedMessage => write!(f, "message was truncated"),
            DecodeError::RecursionLimitExceeded => write!(f, "recursion limit exceeded"),
            DecodeError::MissingRequiredField { field_number } => {
                write!(f, "proto2 required field {field_number} was not present")
            }
            DecodeError::UnknownClosedEnum { raw } => {
                write!(f, "closed enum value {raw} is not in the known set")
            }
        }
    }
}

impl Error for DecodeError {}

/// Errors that can occur while encoding a protobuf message to the wire format.
///
/// In practice this is almost never triggered — encoding to a `Vec` or
/// `bytes::BytesMut` is infallible — but the type exists for completeness so
/// that callers targeting fixed-size buffers can propagate capacity errors.
#[derive(Debug, Clone, PartialEq)]
pub enum EncodeError {
    /// The output buffer did not have enough capacity.
    BufferFull,
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncodeError::BufferFull => write!(f, "output buffer is full"),
        }
    }
}

impl Error for EncodeError {}
