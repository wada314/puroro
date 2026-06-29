//! Conversion from [`protobuf_core::ProtobufError`] to puroro error types.

use ::protobuf_core::ProtobufError;

use crate::error::{DecodeError, EncodeError};

impl From<ProtobufError> for DecodeError {
    fn from(err: ProtobufError) -> Self {
        match err {
            ProtobufError::UnexpectedEof => DecodeError::UnexpectedEof,
            ProtobufError::VarintTooLong => DecodeError::InvalidVarint,
            ProtobufError::InvalidWireType { .. }
            | ProtobufError::MalformedTag { .. } => DecodeError::InvalidTag,
            ProtobufError::VarintDowncastOutOfRange { .. } => DecodeError::InvalidVarint,
            ProtobufError::FieldTypeDowncastError { .. } => DecodeError::InvalidTag,
            ProtobufError::FieldNumberOutOfRange { .. } => DecodeError::InvalidTag,
            ProtobufError::IoError(_) => DecodeError::UnexpectedEof,
        }
    }
}

impl From<ProtobufError> for EncodeError {
    fn from(err: ProtobufError) -> Self {
        match err {
            ProtobufError::IoError(_) => EncodeError::BufferFull,
            _ => EncodeError::BufferFull,
        }
    }
}
