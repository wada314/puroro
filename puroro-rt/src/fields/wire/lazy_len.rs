//! Lazy singular LEN store / get policies shared by [`WireOrSso`] and [`SpanOnly`].
//!
//! Ingest writes a [`WireSpan`](crate::decode::WireSpan) through [`LazyLenStore`].
//! Getters read through [`LazyStringGet`] / [`LazyBytesGet`] (promote vs peek).

use ::allocator_api2::alloc::Allocator;
use ::puroro::DecodeError;
use ::unmanaged::DefaultIn;

use super::len::{
    BytesCodec, BytesLikeLenCodec, LenScalar, ProtoBytes, ProtoString, ProtoStringUnchecked,
    UncheckedStringCodec,
};
use super::singular_type::SingularType;
use super::span_only::SpanOnly;
use super::wire_or_sso::{WireOrSso, WireOrSsoSlot};
use crate::decode::WireSpan;
use crate::fields::shared::slot_init::SlotInitMut;
use crate::fields::shared::value_layout::{Inline, InlineOrHeap, ValueLayout, ValueLayoutClone};
use crate::fields::shared::value_slot::ValueSlot;
use crate::fields::shared::{MessageBindingMut, MessageCommonAlloc, MessageCommonSharedBits};
use crate::fields::wire::sso_bytes::SsoBytes;
use crate::fields::wire::sso_string::SsoString;
use ::unmanaged::{UnmanagedString, UnmanagedVec};

/// Write a LEN payload span into a lazy singular slot during ingest.
pub trait LazyLenStore<T: SingularType, A: Allocator>: ValueLayout<T, A> {
    fn store_len_span<VS, I, Cx>(slot: &mut VS, init: I, common: &mut Cx, span: WireSpan)
    where
        VS: ValueSlot<Self::Slot, A>,
        I: SlotInitMut,
        Cx: MessageBindingMut<A>,
        A: Clone;
}

/// Read a present lazy string slot (UTF-8).
pub trait LazyStringGet<A: Allocator>: ValueLayout<ProtoString, A> {
    fn get_str<'msg, Cx>(
        slot: &'msg Self::Slot,
        common: &'msg Cx,
        wire: &'msg [u8],
        alloc: A,
    ) -> Result<&'msg str, DecodeError>
    where
        Cx: MessageCommonSharedBits + MessageCommonAlloc<Alloc = A>,
        A: Clone;
}

/// Read a present lazy bytes / unchecked-string slot.
pub trait LazyBytesGet<C: BytesLikeLenCodec, A: Allocator>: ValueLayout<LenScalar<C>, A> {
    fn get_bytes<'msg, Cx>(
        slot: &'msg Self::Slot,
        common: &'msg Cx,
        wire: &'msg [u8],
        alloc: A,
    ) -> Result<&'msg [u8], DecodeError>
    where
        Cx: MessageCommonSharedBits + MessageCommonAlloc<Alloc = A>,
        A: Clone;
}

/// Construct an empty string slot (eager SSO or lazy wire/span).
pub trait StringSlotNew<A: Allocator>: ValueLayout<ProtoString, A> {
    fn new_slot(alloc: A) -> Self::Slot;
}

/// Construct an empty bytes slot (eager SSO or lazy wire/span).
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

impl<A: Allocator, const KIND: usize> StringSlotNew<A> for WireOrSso<KIND> {
    #[inline]
    fn new_slot(_alloc: A) -> Self::Slot {
        WireOrSsoSlot::empty()
    }
}

impl<A: Allocator> StringSlotNew<A> for SpanOnly {
    #[inline]
    fn new_slot(_alloc: A) -> Self::Slot {
        WireSpan::default()
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

impl<A: Allocator, const KIND: usize> BytesSlotNew<A> for WireOrSso<KIND> {
    #[inline]
    fn new_slot(_alloc: A) -> Self::Slot {
        WireOrSsoSlot::empty()
    }
}

impl<A: Allocator> BytesSlotNew<A> for SpanOnly {
    #[inline]
    fn new_slot(_alloc: A) -> Self::Slot {
        WireSpan::default()
    }
}

/// Bundle of layout + store + get for a lazy UTF-8 string slot.
///
/// `Slot: DefaultIn` is required so proto3 [`Implicit`](crate::Implicit)
/// fields can call [`ValueSlotNew`](crate::fields::shared::value_slot::ValueSlotNew)
/// through an abstract [`LazyProfile`](crate::decode::LazyProfile)::`StringLen`.
pub trait LazyStringLen<A: Allocator + Clone>:
    ValueLayout<ProtoString, A, Slot: DefaultIn<A>>
    + ValueLayoutClone<ProtoString, A>
    + LazyLenStore<ProtoString, A>
    + LazyStringGet<A>
    + StringSlotNew<A>
{
}

impl<A: Allocator + Clone, const KIND: usize> LazyStringLen<A> for WireOrSso<KIND> {}
impl<A: Allocator + Clone> LazyStringLen<A> for SpanOnly {}

/// Bundle of layout + store + get for a lazy bytes / unchecked-string slot.
///
/// Same `Slot: DefaultIn` rationale as [`LazyStringLen`] (proto3 Implicit).
/// `ProtoBytes` and `ProtoStringUnchecked` share one physical slot type so
/// oneof install can convert `BytesSlotNew::new_slot()` without knowing `P`.
pub trait LazyBytesLen<A: Allocator + Clone>:
    ValueLayout<ProtoBytes, A, Slot: DefaultIn<A>>
    + ValueLayoutClone<ProtoBytes, A>
    + ValueLayout<ProtoStringUnchecked, A, Slot = <Self as ValueLayout<ProtoBytes, A>>::Slot>
    + ValueLayoutClone<ProtoStringUnchecked, A>
    + LazyLenStore<ProtoBytes, A>
    + LazyLenStore<ProtoStringUnchecked, A>
    + LazyBytesGet<BytesCodec, A>
    + LazyBytesGet<UncheckedStringCodec, A>
    + BytesSlotNew<A>
{
}

impl<A: Allocator + Clone, const KIND: usize> LazyBytesLen<A> for WireOrSso<KIND> {}
impl<A: Allocator + Clone> LazyBytesLen<A> for SpanOnly {}
