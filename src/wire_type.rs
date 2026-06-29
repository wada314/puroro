//! Wire types for protobuf fields.
//!
//! Re-exported from [`protobuf_core`] — puroro does not define its own copy.

pub use ::protobuf_core::WireType;

/// Decode a raw 3-bit value from a tag into a [`WireType`].
#[inline]
pub fn from_raw(raw: u8) -> Result<WireType, crate::error::DecodeError> {
    WireType::try_from(raw).map_err(Into::into)
}
