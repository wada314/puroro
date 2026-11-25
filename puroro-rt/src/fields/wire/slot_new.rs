//! Empty string / bytes slot constructors for SSO layouts.
//!
//! Eager `Inline` / `InlineOrHeap` always need these. Lazy `WireOrSso` /
//! `SpanOnly` impls live in [`super::lazy_len`] behind the `lazy` feature.

use ::allocator_api2::alloc::Allocator;
use ::unmanaged::DefaultIn;

use super::len::{ProtoBytes, ProtoString};
use crate::fields::shared::value_layout::{Inline, InlineOrHeap, ValueLayout};
use crate::fields::wire::sso_bytes::SsoBytes;
use crate::fields::wire::sso_string::SsoString;
use ::unmanaged::{UnmanagedString, UnmanagedVec};

/// Construct an empty string slot (eager SSO or, with `lazy`, wire/span).
pub trait StringSlotNew<A: Allocator>: ValueLayout<ProtoString, A> {
    fn new_slot(alloc: A) -> Self::Slot;
}

/// Construct an empty bytes slot (eager SSO or, with `lazy`, wire/span).
pub trait BytesSlotNew<A: Allocator>: ValueLayout<ProtoBytes, A> {
    fn new_slot(alloc: A) -> Self::Slot;
}

impl<A: Allocator> StringSlotNew<A> for Inline {
    #[inline]
    fn new_slot(alloc: A) -> Self::Slot {
        UnmanagedString::default_in(alloc)
    }
}

impl<A: Allocator, const HEAP_BIT: usize> StringSlotNew<A> for InlineOrHeap<HEAP_BIT> {
    #[inline]
    fn new_slot(alloc: A) -> Self::Slot {
        SsoString::default_in(alloc)
    }
}

impl<A: Allocator> BytesSlotNew<A> for Inline {
    #[inline]
    fn new_slot(alloc: A) -> Self::Slot {
        UnmanagedVec::default_in(alloc)
    }
}

impl<A: Allocator, const HEAP_BIT: usize> BytesSlotNew<A> for InlineOrHeap<HEAP_BIT> {
    #[inline]
    fn new_slot(alloc: A) -> Self::Slot {
        SsoBytes::default_in(alloc)
    }
}
