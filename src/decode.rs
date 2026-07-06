//! The [`MessageDecode`] trait for generated protobuf messages.

use ::bytes::Buf;

use crate::error::DecodeError;

/// Implemented by every generated message type.
pub trait MessageDecode: Sized {
    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError>;

    fn decode<B: Buf>(mut buf: B) -> Result<Self, DecodeError>
    where
        Self: Default,
    {
        let mut msg = Self::default();
        msg.merge_from(&mut buf)?;
        Ok(msg)
    }
}
