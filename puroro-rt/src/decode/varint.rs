//! Pluggable varint decoders for production and microbenchmarks.
//!
//! Production code uses [`decode_varint`], which always calls [`ChunkScan`].
//! Benches can monomorphize over [`VarintDecoder`] impls to compare algorithms
//! without runtime dispatch.

use ::bytes::Buf;
use ::protobuf_core::{
    IteratorExtVarint, MAX_VARINT_BYTES, VARINT_CONTINUATION_BIT, VARINT_PAYLOAD_MASK,
};
use ::puroro::DecodeError;

/// A varint decode algorithm over [`Buf`].
///
/// Implementations are zero-sized types so benches can switch algorithms at
/// compile time via monomorphization.
pub trait VarintDecoder {
    /// Stable name used in Criterion benchmark IDs.
    const NAME: &'static str;

    /// Decodes one base-128 varint from `buf`, advancing past the consumed bytes.
    fn decode_varint<B: Buf>(buf: &mut B) -> Result<u64, DecodeError>;
}

/// Current production algorithm: scan a contiguous [`Buf::chunk`], falling back
/// to byte-at-a-time reading when a varint spans chunk boundaries.
pub struct ChunkScan;

/// Previous path: feed [`protobuf_core::IteratorExtVarint`] one `get_u8` at a time.
pub struct ByteIterator;

struct BufVarintReader<'a, B: Buf> {
    buf: &'a mut B,
}

impl<B: Buf> Iterator for BufVarintReader<'_, B> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.has_remaining() {
            Some(self.buf.get_u8())
        } else {
            None
        }
    }
}

/// Try to decode a full varint from a contiguous slice.
///
/// - `Ok(Some((value, len)))` — complete varint of `len` bytes (1..=10)
/// - `Ok(None)` — not enough bytes in `chunk` (caller should use another source)
/// - `Err(InvalidVarint)` — 10 continuation bytes (too long)
fn try_decode_varint_from_slice(chunk: &[u8]) -> Result<Option<(u64, usize)>, DecodeError> {
    let mut value = 0u64;
    let mut shift = 0u32;

    for (i, &byte) in chunk.iter().take(MAX_VARINT_BYTES).enumerate() {
        value |= u64::from(byte & VARINT_PAYLOAD_MASK) << shift;
        if byte & VARINT_CONTINUATION_BIT == 0 {
            return Ok(Some((value, i + 1)));
        }
        shift += 7;
    }

    if chunk.len() >= MAX_VARINT_BYTES {
        Err(DecodeError::InvalidVarint)
    } else {
        Ok(None)
    }
}

fn decode_varint_byte_iterator<B: Buf>(buf: &mut B) -> Result<u64, DecodeError> {
    match (BufVarintReader { buf }).read_varint()? {
        Some(v) => Ok(v.to_uint64()),
        None => Err(DecodeError::UnexpectedEof),
    }
}

impl VarintDecoder for ChunkScan {
    const NAME: &'static str = "ChunkScan";

    fn decode_varint<B: Buf>(buf: &mut B) -> Result<u64, DecodeError> {
        match try_decode_varint_from_slice(buf.chunk())? {
            Some((value, len)) => {
                buf.advance(len);
                Ok(value)
            }
            None => decode_varint_byte_iterator(buf),
        }
    }
}

impl VarintDecoder for ByteIterator {
    const NAME: &'static str = "ByteIterator";

    fn decode_varint<B: Buf>(buf: &mut B) -> Result<u64, DecodeError> {
        decode_varint_byte_iterator(buf)
    }
}

/// Invokes `$body` once per known [`VarintDecoder`] impl, binding `$D` to the type.
///
/// Used by benches so adding a decoder only requires updating this list.
#[macro_export]
macro_rules! for_each_varint_decoder {
    ($D:ident => $body:block) => {{
        {
            type $D = $crate::decode::ChunkScan;
            $body
        }
        {
            type $D = $crate::decode::ByteIterator;
            $body
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::bytes::Buf;
    use ::protobuf_core::Varint;

    /// `Buf` that exposes `first` and `rest` as separate chunks.
    struct SplitBuf<'a> {
        first: &'a [u8],
        rest: &'a [u8],
    }

    impl Buf for SplitBuf<'_> {
        fn remaining(&self) -> usize {
            self.first.len() + self.rest.len()
        }

        fn chunk(&self) -> &[u8] {
            if self.first.is_empty() {
                self.rest
            } else {
                self.first
            }
        }

        fn advance(&mut self, cnt: usize) {
            assert!(cnt <= self.remaining());
            if cnt <= self.first.len() {
                self.first = &self.first[cnt..];
            } else {
                let into_rest = cnt - self.first.len();
                self.first = &[];
                self.rest = &self.rest[into_rest..];
            }
        }
    }

    fn encode_u64(value: u64) -> Vec<u8> {
        let (bytes, count) = Varint::from_uint64(value).encode();
        bytes[..count].to_vec()
    }

    fn assert_decode_lengths<D: VarintDecoder>() {
        for value in [0u64, 1, 127, 150, 1 << 28, u64::from(u32::MAX), u64::MAX] {
            let wire = encode_u64(value);
            let mut cursor = wire.as_slice();
            assert_eq!(
                D::decode_varint(&mut cursor).unwrap(),
                value,
                "{} failed for {value}",
                D::NAME
            );
            assert!(!cursor.has_remaining(), "{}", D::NAME);
        }
    }

    fn assert_decode_too_long<D: VarintDecoder>() {
        let wire = [0x80u8; 10];
        let mut cursor = wire.as_slice();
        assert!(
            matches!(
                D::decode_varint(&mut cursor),
                Err(DecodeError::InvalidVarint)
            ),
            "{}",
            D::NAME
        );
    }

    fn assert_decode_empty<D: VarintDecoder>() {
        let mut cursor = [].as_slice();
        assert!(
            matches!(
                D::decode_varint(&mut cursor),
                Err(DecodeError::UnexpectedEof)
            ),
            "{}",
            D::NAME
        );
    }

    fn assert_decode_split_across_chunks<D: VarintDecoder>() {
        // 150 == [0x96, 0x01], split after the first byte.
        let mut buf = SplitBuf {
            first: &[0x96],
            rest: &[0x01],
        };
        assert_eq!(D::decode_varint(&mut buf).unwrap(), 150, "{}", D::NAME);
        assert_eq!(buf.remaining(), 0, "{}", D::NAME);
    }

    fn assert_decode_split_then_more_data<D: VarintDecoder>() {
        let mut buf = SplitBuf {
            first: &[0x96],
            rest: &[0x01, 0x07],
        };
        assert_eq!(D::decode_varint(&mut buf).unwrap(), 150, "{}", D::NAME);
        assert_eq!(D::decode_varint(&mut buf).unwrap(), 7, "{}", D::NAME);
    }

    #[test]
    fn all_decoders_lengths() {
        crate::for_each_varint_decoder!(D => {
            assert_decode_lengths::<D>();
        });
    }

    #[test]
    fn all_decoders_too_long() {
        crate::for_each_varint_decoder!(D => {
            assert_decode_too_long::<D>();
        });
    }

    #[test]
    fn all_decoders_empty() {
        crate::for_each_varint_decoder!(D => {
            assert_decode_empty::<D>();
        });
    }

    #[test]
    fn all_decoders_split_across_chunks() {
        crate::for_each_varint_decoder!(D => {
            assert_decode_split_across_chunks::<D>();
        });
    }

    #[test]
    fn all_decoders_split_then_more_data() {
        crate::for_each_varint_decoder!(D => {
            assert_decode_split_then_more_data::<D>();
        });
    }
}
