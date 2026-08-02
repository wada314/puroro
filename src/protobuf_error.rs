//! Conversion from [`protobuf_core::ProtobufError`] to puroro error types.

use ::protobuf_core::ProtobufError;

use crate::error::DecodeError;

impl From<ProtobufError> for DecodeError {
    fn from(err: ProtobufError) -> Self {
        match err {
            ProtobufError::UnexpectedEof | ProtobufError::IoError(_) => DecodeError::UnexpectedEof,
            ProtobufError::VarintTooLong | ProtobufError::VarintDowncastOutOfRange { .. } => {
                DecodeError::InvalidVarint
            }
            ProtobufError::InvalidWireType { .. } | ProtobufError::FieldNumberOutOfRange { .. } => {
                DecodeError::InvalidTag
            }
        }
    }
}
