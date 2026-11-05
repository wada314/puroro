//! Message-body merge/decode surface used by the field catalog and generated code.
//!
//! This is intentionally **not** part of the user-facing [`puroro::Message`]
//! trait. Library users call [`Message::merge_from`](::puroro::Message::merge_from);
//! generated impls forward that to [`merge_message`].

use ::bytes::Buf;
use ::puroro::{DecodeBuf, DecodeError, ScopedBuf};

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
    /// `buf` must be a [`DecodeBuf`] (normally a [`ScopedBuf`] created by
    /// [`merge_message`]). Nested LEN frames call
    /// [`DecodeBuf::push_limit`] /
    /// [`DecodeBuf::pop_limit`] on the same buffer so the
    /// concrete type does not change across recursion depths.
    ///
    /// Generated / catalog code must pass `depth + 1` into nested
    /// merge calls. Returns [`DecodeError::RecursionLimitExceeded`] when
    /// `depth >= `[`puroro::RECURSION_LIMIT`].
    ///
    /// When the remainder is one chunk, generated impls forward to
    /// [`Self::merge_from_slice`] and advance `buf` by that length.
    fn merge_from_with_depth<B: DecodeBuf>(
        &mut self,
        buf: &mut B,
        depth: usize,
    ) -> Result<(), DecodeError>;

    /// Merge one contiguous message body.
    ///
    /// Generated messages walk a [`crate::decode::SliceCursor`] and, for a
    /// length-delimited child, pass that child's bytes to this method.
    /// Hand-written impls keep the default, which runs
    /// [`Self::merge_from_with_depth`] on a cursor over `slice`.
    #[inline]
    fn merge_from_slice(&mut self, slice: &[u8], depth: usize) -> Result<(), DecodeError> {
        let mut cursor = crate::decode::SliceCursor::new(slice);
        self.merge_from_with_depth(&mut cursor, depth)
    }
}

/// Merges `buf` into `message` at depth `0` (wraps in [`ScopedBuf`]).
#[inline]
pub fn merge_message<M: MessageMerge, B: Buf>(
    message: &mut M,
    buf: &mut B,
) -> Result<(), DecodeError> {
    let mut scoped = ScopedBuf::new(buf);
    message.merge_from_with_depth(&mut scoped, 0)
}
