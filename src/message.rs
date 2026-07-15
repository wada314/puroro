//! The [`Message`] trait shared by every generated protobuf message.

use ::bytes::{Buf, BufMut};

use crate::error::DecodeError;
use crate::unknown::UnknownField;

/// Implemented by every generated message type (C++ `MessageLite`-like surface).
///
/// Field accessors stay as inherent methods on the generated struct so that
/// proto field names do not collide with these helpers. If a proto field is
/// named `validate`, `unknown_fields`, `encoded_len`, etc., the inherent getter
/// wins method resolution; call the trait method via UFCS
/// (e.g. [`Message::validate`]).
///
/// Constructors (`new` / `new_in`) remain inherent methods on the generated
/// struct.
pub trait Message: Sized {
    // -- codec --------------------------------------------------------------

    /// Exact number of bytes this message occupies on the wire.
    /// Must be consistent with [`encode_raw`](Self::encode_raw).
    fn encoded_len(&self) -> usize;

    /// Writes the message body to `buf` without a framing length prefix.
    fn encode_raw<B: BufMut>(&self, buf: &mut B);

    /// Encodes into a new `Vec<u8>`.
    fn encode_to_vec(&self) -> Vec<u8> {
        let mut v = Vec::with_capacity(self.encoded_len());
        self.encode_raw(&mut v);
        v
    }

    /// Encodes into [`bytes::Bytes`].
    fn encode_to_bytes(&self) -> ::bytes::Bytes {
        ::bytes::Bytes::from(self.encode_to_vec())
    }

    /// Reads fields from `buf` and merges them into `self`.
    ///
    /// Merge semantics (identical across proto2, proto3, editions):
    /// - Singular scalar: last value seen wins.
    /// - Singular message: recursively merged.
    /// - Repeated: each occurrence appends to the list.
    /// - Unknown fields: accumulated for round-trip preservation.
    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError>;

    /// Decodes a complete message. Provided; requires `Self: Default`.
    fn decode<B: Buf>(mut buf: B) -> Result<Self, DecodeError>
    where
        Self: Default,
    {
        let mut msg = Self::default();
        msg.merge_from(&mut buf)?;
        Ok(msg)
    }

    // -- infrastructure -----------------------------------------------------

    /// Iterates preserved unknown fields (and closed-enum unknowns diverted
    /// into the unknown set).
    fn unknown_fields(&self) -> impl Iterator<Item = UnknownField<'_>> + '_;

    /// Checks `LEGACY_REQUIRED` fields.
    ///
    /// Messages with no such fields return [`Ok`].
    fn validate(&self) -> Result<(), DecodeError>;
}
