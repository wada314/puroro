//! Message-body merge/decode surface used by the field catalog and generated code.
//!
//! This is intentionally **not** part of the user-facing [`puroro::Message`](::puroro::Message)
//! trait. Library users call [`Message::merge_from`](::puroro::Message::merge_from);
//! generated impls forward that to [`merge_message`].

use ::bytes::Buf;
use ::protobuf_core::FieldNumber;
use ::puroro::{DecodeBuf, DecodeError, ScopedBuf};

/// How far a [`MessageMerge`] loop should read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeBound {
    /// Read until the buffer scope is exhausted.
    Message,
    /// Read until matching end-group is consumed (inclusive).
    Group(FieldNumber),
}

/// Wire merge for a message body (depth-aware nested decode).
///
/// Implemented by generated messages and hand-written test / bench messages.
/// Nested-message catalog code ([`ProtoMessage`](crate::ProtoMessage)) bounds on
/// this trait for recursive merge, not on [`puroro::Message`](::puroro::Message).
///
/// User-facing [`Message::merge_from`](::puroro::Message::merge_from) is a thin
/// wrapper around [`merge_message`].
pub trait MessageMerge {
    /// Reads fields from `buf` and merges them into `self`, threading decode
    /// nesting `depth`, until the current buffer scope is exhausted.
    ///
    /// `buf` must be a [`DecodeBuf`] (normally a [`ScopedBuf`] created by
    /// [`merge_message`]). Nested LEN frames call
    /// [`DecodeBuf::push_limit`](DecodeBuf::push_limit) /
    /// [`DecodeBuf::pop_limit`](DecodeBuf::pop_limit) on the same buffer so the
    /// concrete type does not change across recursion depths.
    ///
    /// Generated / catalog code must pass `depth + 1` into nested
    /// `merge_from_with_depth` / [`merge_group_body`](Self::merge_group_body)
    /// calls. Returns [`DecodeError::RecursionLimitExceeded`] when
    /// `depth >= `[`puroro::RECURSION_LIMIT`](::puroro::RECURSION_LIMIT).
    #[inline]
    fn merge_from_with_depth<B: DecodeBuf>(
        &mut self,
        buf: &mut B,
        depth: usize,
    ) -> Result<(), DecodeError> {
        self.merge_from_bounded(buf, depth, MergeBound::Message)
    }

    /// Merges a classic / Editions group body after the opening `SGroup` tag
    /// was consumed: same `buf`, stop when the matching `EGroup` is read.
    #[inline]
    fn merge_group_body<B: DecodeBuf>(
        &mut self,
        buf: &mut B,
        depth: usize,
        group: FieldNumber,
    ) -> Result<(), DecodeError> {
        self.merge_from_bounded(buf, depth, MergeBound::Group(group))
    }

    /// Shared merge loop for [`MergeBound::Message`] and [`MergeBound::Group`].
    fn merge_from_bounded<B: DecodeBuf>(
        &mut self,
        buf: &mut B,
        depth: usize,
        bound: MergeBound,
    ) -> Result<(), DecodeError>;
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
