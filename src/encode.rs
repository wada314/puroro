//! The [`MessageEncode`] trait for generated protobuf messages.

use ::bytes::BufMut;

/// Implemented by every generated message type.
pub trait MessageEncode {
    fn encoded_len(&self) -> usize;

    fn encode_raw<B: BufMut>(&self, buf: &mut B);

    fn encode_to_vec(&self) -> Vec<u8> {
        let mut v = Vec::with_capacity(self.encoded_len());
        self.encode_raw(&mut v);
        v
    }

    fn encode_to_bytes(&self) -> ::bytes::Bytes {
        ::bytes::Bytes::from(self.encode_to_vec())
    }
}
