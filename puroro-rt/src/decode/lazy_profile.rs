//! Manifest for a lazy message's island buffer and singular LEN slot layout.
//!
//! [`Owned`] keeps a [`Bytes`](bytes::Bytes) island and caches string / bytes
//! in [`WireOrSso`](crate::WireOrSso). [`Borrowed`] stores `&[u8]` and uses
//! [`SpanOnly`](crate::SpanOnly) (span into the island, no Inline / Heap /
//! Failed). Returned string / bytes views are tied to the `&self` borrow
//! (`&'msg str`).

use ::allocator_api2::alloc::Allocator;
use ::bytes::Bytes;
use ::core::marker::PhantomData;

use super::wire_buf::WireBuf;
use crate::fields::wire::lazy_len::{LazyBytesLen, LazyStringLen};
use crate::fields::wire::span_only::SpanOnly;
use crate::fields::wire::wire_or_sso::WireOrSso;

/// Island buffer plus singular string / bytes slot layout for a lazy message.
pub trait LazyProfile: Clone {
    /// Contiguous wire handle stored on [`super::LazyScan`].
    type Buf: WireBuf;

    /// Singular `string` (`utf8_validation=VERIFY`) slot layout.
    ///
    /// `KIND` is the first of two kind bits for [`WireOrSso`]; [`SpanOnly`]
    /// ignores it (Phase B keeps bit allocations for now).
    type StringLen<A: Allocator + Clone, const KIND: usize>: LazyStringLen<A>;

    /// Singular `bytes` / `utf8_validation=NONE` string slot layout.
    type BytesLen<A: Allocator + Clone, const KIND: usize>: LazyBytesLen<A>;
}

/// Owned island (`Bytes`) with UTF-8 / bytes promote on get (`WireOrSso`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Owned;

impl LazyProfile for Owned {
    type Buf = Bytes;
    type StringLen<A: Allocator + Clone, const KIND: usize> = WireOrSso<KIND>;
    type BytesLen<A: Allocator + Clone, const KIND: usize> = WireOrSso<KIND>;
}

/// Borrowed island (`&[u8]`) with span-only slots (no promote cache).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Borrowed<'buf> {
    _buf: PhantomData<&'buf [u8]>,
}

impl<'buf> LazyProfile for Borrowed<'buf> {
    type Buf = &'buf [u8];
    type StringLen<A: Allocator + Clone, const KIND: usize> = SpanOnly;
    type BytesLen<A: Allocator + Clone, const KIND: usize> = SpanOnly;
}
