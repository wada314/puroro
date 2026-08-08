//! Pluggable varint decoders for production and microbenchmarks.
//!
//! [`super::decode_varint`] uses [`ChunkScanHot4`]; [`super::decode_tag`] uses
//! [`super::decode_varint_with`] with [`ChunkScanLikely1`]. Benches monomorphize
//! over [`VarintDecoder`] impls to compare algorithms without runtime dispatch.

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

/// Generic contiguous [`Buf::chunk`] scan, falling back to byte-at-a-time when
/// a varint spans chunk boundaries. Kept for benches / comparison.
pub struct ChunkScan;

/// Previous path: feed [`protobuf_core::IteratorExtVarint`] one `get_u8` at a time.
pub struct ByteIterator;

/// Peeks 2 wire bytes as an unaligned little-endian `u16` and decodes from that
/// word. If both bytes continue, keeps the partial value and finishes byte-wise.
pub struct BytePair;

/// Peeks 4 wire bytes as an unaligned little-endian `u32` and decodes lengths
/// 1..=4 from that word. If all four continue, finishes the remainder byte-wise.
pub struct ByteQuad;

/// [`ChunkScan`] plus an explicit 1-byte early return.
pub struct ChunkScan1;

/// 1- and 2-byte dedicated paths, then the generic chunk scan.
pub struct ChunkScan2;

/// Tag-oriented: assume the first wire byte usually terminates.
///
/// On a terminating first byte, advances and returns without building an
/// `Option` / re-scanning the slice. If it continues, keeps the partial payload
/// and finishes with [`finish_varint_bytewise`] (no re-read of byte 0).
pub struct ChunkScanLikely1;

/// Hand-unrolled paths for 1..=4 byte varints, then the generic chunk scan.
pub struct ChunkScan4;

/// When `chunk.len() >= N`, scan the first `N` bytes with a bounds-light loop
/// (see [`try_decode_varint_hot`]). Smaller `N` favors short varints; `N == 10`
/// covers the full protobuf varint width.
pub struct ChunkScanHot2;
/// Hot window of 4 bytes. See [`ChunkScanHot2`].
pub struct ChunkScanHot4;
/// Hot window of 8 bytes. See [`ChunkScanHot2`].
pub struct ChunkScanHot8;
/// Hot window of 10 bytes (full varint). See [`ChunkScanHot2`].
pub struct ChunkScanHot10;

/// Adaptive hot window: `len >= 10` → Hot10, `len >= 4` → Hot4, else generic.
///
/// Keeps the Hot10 bulk path on large remainders while covering short nested
/// scopes / trailing bytes with Hot4.
pub struct ChunkScanHotAdapt;

/// Decode short varints by locating the first cleared continuation bit in a
/// little-endian `u64` load (`trailing_zeros` on the MSB mask).
pub struct ChunkScanMsb;

struct BufVarintReader<'a, B: Buf> {
    buf: &'a mut B,
}

impl<B: Buf> Iterator for BufVarintReader<'_, B> {
    type Item = u8;

    #[inline]
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
#[inline]
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

#[inline]
fn decode_varint_byte_iterator<B: Buf>(buf: &mut B) -> Result<u64, DecodeError> {
    match (BufVarintReader { buf }).read_varint()? {
        Some(v) => Ok(v.to_uint64()),
        None => Err(DecodeError::UnexpectedEof),
    }
}

/// Fast path for workloads where most varints are a single byte (e.g. tags).
#[inline]
fn decode_varint_likely1<B: Buf>(buf: &mut B) -> Result<u64, DecodeError> {
    let chunk = buf.chunk();
    let Some(&b0) = chunk.first() else {
        return decode_varint_byte_iterator(buf);
    };
    if b0 < VARINT_CONTINUATION_BIT {
        buf.advance(1);
        return Ok(u64::from(b0));
    }
    buf.advance(1);
    finish_varint_bytewise(buf, u64::from(b0 & VARINT_PAYLOAD_MASK), 7, 1)
}

/// Continue a partially decoded varint with ordinary byte loads.
#[inline]
fn finish_varint_bytewise<B: Buf>(
    buf: &mut B,
    mut value: u64,
    mut shift: u32,
    mut bytes_consumed: usize,
) -> Result<u64, DecodeError> {
    while bytes_consumed < MAX_VARINT_BYTES {
        if !buf.has_remaining() {
            return Err(DecodeError::UnexpectedEof);
        }
        let byte = buf.get_u8();
        bytes_consumed += 1;
        value |= u64::from(byte & VARINT_PAYLOAD_MASK) << shift;
        if byte < VARINT_CONTINUATION_BIT {
            return Ok(value);
        }
        shift += 7;
    }
    Err(DecodeError::InvalidVarint)
}

/// Load two protobuf wire bytes as a little-endian `u16` without requiring
/// alignment.
///
/// Creating a `&[u16]` from an arbitrary `&[u8]` would be UB when the pointer is
/// not 2-byte aligned; `read_unaligned` is the safe form of that cast.
#[inline(always)]
fn load_wire_u16(chunk: &[u8]) -> u16 {
    debug_assert!(chunk.len() >= 2);
    // SAFETY: caller guarantees at least 2 bytes; unaligned load is intentional.
    let w = unsafe { chunk.as_ptr().cast::<u16>().read_unaligned() };
    u16::from_le(w)
}

/// Load four protobuf wire bytes as a little-endian `u32` without requiring
/// alignment.
#[inline(always)]
fn load_wire_u32(chunk: &[u8]) -> u32 {
    debug_assert!(chunk.len() >= 4);
    // SAFETY: caller guarantees at least 4 bytes; unaligned load is intentional.
    let w = unsafe { chunk.as_ptr().cast::<u32>().read_unaligned() };
    u32::from_le(w)
}

/// Pack the low 7 bits of each of the first `len` bytes of a LE wire word.
#[inline(always)]
fn pack_varint_payload_le(mut word: u32, len: usize) -> u64 {
    let mut value = 0u64;
    let mut shift = 0u32;
    for _ in 0..len {
        value |= u64::from(word & u32::from(VARINT_PAYLOAD_MASK)) << shift;
        word >>= 8;
        shift += 7;
    }
    value
}

/// Decode from a little-endian wire `u16` that covers the next 1 or 2 varint bytes.
///
/// Returns `None` when both bytes have the continuation bit set (need more input).
#[inline(always)]
fn try_decode_from_wire_u16(w: u16) -> Option<(u64, usize)> {
    // First byte finished (bit 7 clear).
    if w & 0x80 == 0 {
        return Some((u64::from(w & 0x7f), 1));
    }
    // Second byte finished (bit 15 clear):
    // (b0 & 0x7f) | ((b1 & 0x7f) << 7)  ==  (w & 0x7f) | ((w & 0x7f00) >> 1)
    if w & 0x8000 == 0 {
        let value = u64::from((w & 0x7f) | ((w & 0x7f00) >> 1));
        return Some((value, 2));
    }
    None
}

/// Decode from a little-endian wire `u32` that covers the next 1..=4 varint bytes.
///
/// Returns `None` when all four bytes have the continuation bit set.
#[inline(always)]
fn try_decode_from_wire_u32(w: u32) -> Option<(u64, usize)> {
    const CONT_MASK: u32 = 0x8080_8080;
    let terminators = !w & CONT_MASK;
    if terminators == 0 {
        return None;
    }
    let len = (terminators.trailing_zeros() / 8) as usize + 1;
    debug_assert!((1..=4).contains(&len));
    Some((pack_varint_payload_le(w, len), len))
}

/// `u16`-word peek decoder for the common 1–2 byte cases.
#[inline]
fn decode_varint_byte_pair<B: Buf>(buf: &mut B) -> Result<u64, DecodeError> {
    let chunk = buf.chunk();
    if chunk.len() >= 2 {
        let w = load_wire_u16(chunk);
        if let Some((value, len)) = try_decode_from_wire_u16(w) {
            buf.advance(len);
            return Ok(value);
        }
        // Both bytes continue — keep their payload and finish byte-wise.
        let value = pack_varint_payload_le(u32::from(w), 2);
        buf.advance(2);
        return finish_varint_bytewise(buf, value, 14, 2);
    }
    decode_varint_byte_iterator(buf)
}

/// `u32`-word peek decoder for the common 1–4 byte cases.
#[inline]
fn decode_varint_byte_quad<B: Buf>(buf: &mut B) -> Result<u64, DecodeError> {
    let chunk = buf.chunk();
    if chunk.len() >= 4 {
        let w = load_wire_u32(chunk);
        if let Some((value, len)) = try_decode_from_wire_u32(w) {
            buf.advance(len);
            return Ok(value);
        }
        // All four bytes continue — keep their payload and finish byte-wise.
        let value = pack_varint_payload_le(w, 4);
        buf.advance(4);
        return finish_varint_bytewise(buf, value, 28, 4);
    }
    // Short chunk: reuse the 2-byte word path / byte iterator.
    decode_varint_byte_pair(buf)
}

#[inline]
fn finish_from_chunk<B: Buf>(
    buf: &mut B,
    decoded: Result<Option<(u64, usize)>, DecodeError>,
) -> Result<u64, DecodeError> {
    match decoded? {
        Some((value, len)) => {
            buf.advance(len);
            Ok(value)
        }
        None => decode_varint_byte_iterator(buf),
    }
}

#[inline]
fn try_decode_varint_1(chunk: &[u8]) -> Result<Option<(u64, usize)>, DecodeError> {
    let Some(&b0) = chunk.first() else {
        return Ok(None);
    };
    if b0 < VARINT_CONTINUATION_BIT {
        return Ok(Some((u64::from(b0), 1)));
    }
    try_decode_varint_from_slice(chunk)
}

#[inline]
fn try_decode_varint_2(chunk: &[u8]) -> Result<Option<(u64, usize)>, DecodeError> {
    let Some(&b0) = chunk.first() else {
        return Ok(None);
    };
    if b0 < VARINT_CONTINUATION_BIT {
        return Ok(Some((u64::from(b0), 1)));
    }
    let Some(b1) = chunk.get(1).copied() else {
        return Ok(None);
    };
    let value = u64::from(b0 & VARINT_PAYLOAD_MASK) | (u64::from(b1 & VARINT_PAYLOAD_MASK) << 7);
    if b1 < VARINT_CONTINUATION_BIT {
        return Ok(Some((value, 2)));
    }
    try_decode_varint_from_slice(chunk)
}

#[inline]
fn try_decode_varint_4(chunk: &[u8]) -> Result<Option<(u64, usize)>, DecodeError> {
    let Some(&b0) = chunk.first() else {
        return Ok(None);
    };
    if b0 < VARINT_CONTINUATION_BIT {
        return Ok(Some((u64::from(b0), 1)));
    }

    let Some(b1) = chunk.get(1).copied() else {
        return Ok(None);
    };
    let mut value =
        u64::from(b0 & VARINT_PAYLOAD_MASK) | (u64::from(b1 & VARINT_PAYLOAD_MASK) << 7);
    if b1 < VARINT_CONTINUATION_BIT {
        return Ok(Some((value, 2)));
    }

    let Some(b2) = chunk.get(2).copied() else {
        return Ok(None);
    };
    value |= u64::from(b2 & VARINT_PAYLOAD_MASK) << 14;
    if b2 < VARINT_CONTINUATION_BIT {
        return Ok(Some((value, 3)));
    }

    let Some(b3) = chunk.get(3).copied() else {
        return Ok(None);
    };
    value |= u64::from(b3 & VARINT_PAYLOAD_MASK) << 21;
    if b3 < VARINT_CONTINUATION_BIT {
        return Ok(Some((value, 4)));
    }

    try_decode_varint_from_slice(chunk)
}

/// Bounds-light scan of the first `N` bytes when they are already contiguous.
///
/// If the varint does not finish inside that window and `N < 10`, falls back to
/// the generic slice scan. `N == 10` treats a full-continuation window as
/// [`DecodeError::InvalidVarint`].
#[inline]
fn try_decode_varint_hot<const N: usize>(
    chunk: &[u8],
) -> Result<Option<(u64, usize)>, DecodeError> {
    debug_assert!((1..=MAX_VARINT_BYTES).contains(&N));
    if chunk.len() >= N {
        let mut value = 0u64;
        let mut shift = 0u32;
        for (i, &byte) in chunk[..N].iter().enumerate() {
            value |= u64::from(byte & VARINT_PAYLOAD_MASK) << shift;
            if byte < VARINT_CONTINUATION_BIT {
                return Ok(Some((value, i + 1)));
            }
            shift += 7;
        }
        if N >= MAX_VARINT_BYTES {
            return Err(DecodeError::InvalidVarint);
        }
        return try_decode_varint_from_slice(chunk);
    }
    try_decode_varint_from_slice(chunk)
}

/// Choose a hot window from the contiguous remainder length.
#[inline]
fn try_decode_varint_hot_adapt(chunk: &[u8]) -> Result<Option<(u64, usize)>, DecodeError> {
    if chunk.len() >= 10 {
        try_decode_varint_hot::<10>(chunk)
    } else if chunk.len() >= 4 {
        try_decode_varint_hot::<4>(chunk)
    } else {
        try_decode_varint_from_slice(chunk)
    }
}

/// MSB-mask decode for varints that finish within the first 8 bytes.
///
/// Loads up to 8 little-endian bytes, finds the first cleared continuation bit
/// via `(!word & 0x80…80).trailing_zeros() / 8`, then assembles the payload.
#[inline]
fn try_decode_varint_msb(chunk: &[u8]) -> Result<Option<(u64, usize)>, DecodeError> {
    if chunk.len() < 8 {
        return try_decode_varint_from_slice(chunk);
    }

    let mut packed = [0u8; 8];
    packed.copy_from_slice(&chunk[..8]);
    let word = u64::from_le_bytes(packed);

    const CONT_MASK: u64 = 0x8080_8080_8080_8080;
    let terminators = !word & CONT_MASK;
    if terminators == 0 {
        // All 8 bytes continued; need byte 9/10 from the generic path.
        return try_decode_varint_from_slice(chunk);
    }

    let len = (terminators.trailing_zeros() / 8) as usize + 1;
    debug_assert!((1..=8).contains(&len));

    let mut value = 0u64;
    let mut shift = 0u32;
    for &byte in &chunk[..len] {
        value |= u64::from(byte & VARINT_PAYLOAD_MASK) << shift;
        shift += 7;
    }
    Ok(Some((value, len)))
}

impl VarintDecoder for ChunkScan {
    const NAME: &'static str = "ChunkScan";

    #[inline]
    fn decode_varint<B: Buf>(buf: &mut B) -> Result<u64, DecodeError> {
        finish_from_chunk(buf, try_decode_varint_from_slice(buf.chunk()))
    }
}

impl VarintDecoder for ByteIterator {
    const NAME: &'static str = "ByteIterator";

    #[inline]
    fn decode_varint<B: Buf>(buf: &mut B) -> Result<u64, DecodeError> {
        decode_varint_byte_iterator(buf)
    }
}

impl VarintDecoder for BytePair {
    const NAME: &'static str = "BytePair";

    #[inline]
    fn decode_varint<B: Buf>(buf: &mut B) -> Result<u64, DecodeError> {
        decode_varint_byte_pair(buf)
    }
}

impl VarintDecoder for ByteQuad {
    const NAME: &'static str = "ByteQuad";

    #[inline]
    fn decode_varint<B: Buf>(buf: &mut B) -> Result<u64, DecodeError> {
        decode_varint_byte_quad(buf)
    }
}

impl VarintDecoder for ChunkScan1 {
    const NAME: &'static str = "ChunkScan1";

    #[inline]
    fn decode_varint<B: Buf>(buf: &mut B) -> Result<u64, DecodeError> {
        finish_from_chunk(buf, try_decode_varint_1(buf.chunk()))
    }
}

impl VarintDecoder for ChunkScanLikely1 {
    const NAME: &'static str = "ChunkScanLikely1";

    #[inline]
    fn decode_varint<B: Buf>(buf: &mut B) -> Result<u64, DecodeError> {
        decode_varint_likely1(buf)
    }
}

impl VarintDecoder for ChunkScan2 {
    const NAME: &'static str = "ChunkScan2";

    #[inline]
    fn decode_varint<B: Buf>(buf: &mut B) -> Result<u64, DecodeError> {
        finish_from_chunk(buf, try_decode_varint_2(buf.chunk()))
    }
}

impl VarintDecoder for ChunkScan4 {
    const NAME: &'static str = "ChunkScan4";

    #[inline]
    fn decode_varint<B: Buf>(buf: &mut B) -> Result<u64, DecodeError> {
        finish_from_chunk(buf, try_decode_varint_4(buf.chunk()))
    }
}

impl VarintDecoder for ChunkScanHot2 {
    const NAME: &'static str = "ChunkScanHot2";

    #[inline]
    fn decode_varint<B: Buf>(buf: &mut B) -> Result<u64, DecodeError> {
        finish_from_chunk(buf, try_decode_varint_hot::<2>(buf.chunk()))
    }
}

impl VarintDecoder for ChunkScanHot4 {
    const NAME: &'static str = "ChunkScanHot4";

    #[inline]
    fn decode_varint<B: Buf>(buf: &mut B) -> Result<u64, DecodeError> {
        finish_from_chunk(buf, try_decode_varint_hot::<4>(buf.chunk()))
    }
}

impl VarintDecoder for ChunkScanHot8 {
    const NAME: &'static str = "ChunkScanHot8";

    #[inline]
    fn decode_varint<B: Buf>(buf: &mut B) -> Result<u64, DecodeError> {
        finish_from_chunk(buf, try_decode_varint_hot::<8>(buf.chunk()))
    }
}

impl VarintDecoder for ChunkScanHot10 {
    const NAME: &'static str = "ChunkScanHot10";

    #[inline]
    fn decode_varint<B: Buf>(buf: &mut B) -> Result<u64, DecodeError> {
        finish_from_chunk(buf, try_decode_varint_hot::<10>(buf.chunk()))
    }
}

impl VarintDecoder for ChunkScanHotAdapt {
    const NAME: &'static str = "ChunkScanHotAdapt";

    #[inline]
    fn decode_varint<B: Buf>(buf: &mut B) -> Result<u64, DecodeError> {
        finish_from_chunk(buf, try_decode_varint_hot_adapt(buf.chunk()))
    }
}

impl VarintDecoder for ChunkScanMsb {
    const NAME: &'static str = "ChunkScanMsb";

    #[inline]
    fn decode_varint<B: Buf>(buf: &mut B) -> Result<u64, DecodeError> {
        finish_from_chunk(buf, try_decode_varint_msb(buf.chunk()))
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
        {
            type $D = $crate::decode::BytePair;
            $body
        }
        {
            type $D = $crate::decode::ByteQuad;
            $body
        }
        {
            type $D = $crate::decode::ChunkScan1;
            $body
        }
        {
            type $D = $crate::decode::ChunkScanLikely1;
            $body
        }
        {
            type $D = $crate::decode::ChunkScan2;
            $body
        }
        {
            type $D = $crate::decode::ChunkScan4;
            $body
        }
        {
            type $D = $crate::decode::ChunkScanHot2;
            $body
        }
        {
            type $D = $crate::decode::ChunkScanHot4;
            $body
        }
        {
            type $D = $crate::decode::ChunkScanHot8;
            $body
        }
        {
            type $D = $crate::decode::ChunkScanHot10;
            $body
        }
        {
            type $D = $crate::decode::ChunkScanHotAdapt;
            $body
        }
        {
            type $D = $crate::decode::ChunkScanMsb;
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
