//! Shared leftover + `_wire` ingest used by handwritten lazy messages.

use ::allocator_api2::alloc::Allocator;

#[cfg(feature = "lazy")]
use super::record::WireSpan;

/// Ingest layout stored in [`MessageCommon`](crate::MessageCommon)::`lazy`.
///
/// Implemented by [`Eager`] (ZST) and, with the `lazy` feature, [`Lazy`].
pub trait MessageScan<A: Allocator>: Sized {
    fn new_scan(alloc: A) -> Self;
    fn clone_scan(&self, alloc: A) -> Self;

    /// Remember one lazy unknown record. Eager keeps unknowns in the store.
    #[cfg(feature = "lazy")]
    fn record_policy_span(&mut self, span: WireSpan) {
        let _ = span;
    }
}

/// Eager ingest layout: no leftover, island buffer, or unparsed regions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Eager;

impl<A: Allocator> MessageScan<A> for Eager {
    #[inline]
    fn new_scan(_alloc: A) -> Self {
        Self
    }

    #[inline]
    fn clone_scan(&self, _alloc: A) -> Self {
        Self
    }
}

#[cfg(feature = "lazy")]
#[path = "lazy_scan_lazy.rs"]
mod lazy_impl;

#[cfg(feature = "lazy")]
pub use self::lazy_impl::{Lazy, LazyMessage, LazyScan};
