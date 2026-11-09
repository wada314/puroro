//! Message-body merge/decode surface used by the field catalog and generated code.
//!
//! This is intentionally **not** part of the user-facing [`puroro::Message`]
//! trait. Library users call [`Message::merge_from`](::puroro::Message::merge_from);
//! generated impls forward that to [`merge_message`].

use ::bytes::Buf;
use ::puroro::{DecodeBuf, DecodeError, ScopedBuf};

use crate::decode::{SliceCursor, contiguous_remaining};

/// Wire merge for a message body (depth-aware nested decode).
///
/// Implemented by generated messages and hand-written test / bench messages.
/// Nested-message catalog code ([`ProtoMessage`](crate::ProtoMessage)) bounds on
/// this trait for recursive merge, not on [`puroro::Message`].
///
/// User-facing [`Message::merge_from`](::puroro::Message::merge_from) is a thin
/// wrapper around [`merge_message`].
pub trait MessageMerge {
    /// Reads fields from `buf` and merges them into `self`, threading decode
    /// nesting `depth`.
    ///
    /// Generated messages run their field `match` here. [`merge_message`] calls
    /// [`Self::merge_from_slice`] when the input is one chunk, and calls this
    /// method on a [`ScopedBuf`] when the remainder spans chunks. A nested
    /// length-delimited body that fits in one chunk is passed to
    /// [`Self::merge_from_slice`] as a subslice.
    ///
    /// Returns [`DecodeError::RecursionLimitExceeded`] when
    /// `depth >= `[`puroro::RECURSION_LIMIT`].
    fn merge_from_with_depth<B: DecodeBuf>(
        &mut self,
        buf: &mut B,
        depth: usize,
    ) -> Result<(), DecodeError>;

    /// Merge one contiguous message body.
    ///
    /// The default puts `slice` in a [`crate::decode::SliceCursor`] and calls
    /// [`Self::merge_from_with_depth`].
    #[inline]
    fn merge_from_slice(&mut self, slice: &[u8], depth: usize) -> Result<(), DecodeError> {
        let mut cursor = SliceCursor::new(slice);
        self.merge_from_with_depth(&mut cursor, depth)
    }
}

/// Merges `buf` into `message` at depth `0`.
///
/// One contiguous chunk uses [`MessageMerge::merge_from_slice`]. A remainder
/// that spans chunks is wrapped in [`ScopedBuf`].
#[inline]
pub fn merge_message<M: MessageMerge, B: Buf>(
    message: &mut M,
    buf: &mut B,
) -> Result<(), DecodeError> {
    if let Some(slice) = contiguous_remaining(buf) {
        let len = slice.len();
        message.merge_from_slice(slice, 0)?;
        buf.advance(len);
        return Ok(());
    }
    let mut scoped = ScopedBuf::new(buf);
    message.merge_from_with_depth(&mut scoped, 0)
}
