//! Repeated-element semantics for protobuf **type** markers (e.g. `ProtoInt32`,
//! `ProtoString`, [`ProtoMessage`](super::proto_message::ProtoMessage)).
//!
//! Singular fields store [`ValueLayout::Slot`](crate::fields::shared::value_layout::ValueLayout).
//! Repeated / map fields store [`RepeatedElement::Element`] — often the inner
//! payload (`i32`, `UnmanagedString`, …), and for nested-message repeated fields
//! the message type `M` itself (not [`UnmanagedBox`](::unmanaged::UnmanagedBox)).
//!
//! [`RefView`](RepeatedElement::RefView) is the shared borrow projection of one
//! element (`i32`, `str`, …) used by map getters and (future) element-wise
//! repeated getters. Distinct from [`EncodeType::View`](super::encode_type::EncodeType::View)
//! (encode / singular getter; numerics are by-value).
//!
//! Singular [`ProtoBool`](super::numerical::ProtoBool) uses bit-packed storage;
//! repeated uses plain `bool` elements via [`NumericalType`](super::numerical::NumericalType)
//! blankets (no MessageCommon bit).
//!
//! Tagged encode uses [`wire_view`](Self::wire_view) +
//! [`encode_field`](super::encode_type::encode_field) (not a separate
//! `encode_element` entry point).

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};
use ::core::ops::{Deref, DerefMut};

use ::puroro::{DecodeBuf, DecodeError, Message, WireType};

use crate::decode;
use crate::message_encode::MessageEncode;
use crate::message_merge::MessageMerge;
use ::unmanaged::DeallocateIn;

use ::protobuf_core::{FIXED32_BYTES, FIXED64_BYTES};

use super::encode_type::EncodeType;
use super::len::{LenCodec, LenScalar};
use super::numerical::{Numerical, NumericalType};
use super::proto_message::ProtoMessage;
use super::wire_payload::{CopyWirePayload, WirePayload};

/// Wire + storage for one element of a repeated / map field of marker `Self`.
///
/// Independent of [`SingularType`](super::singular_type::SingularType) (singular
/// `Slot` / `Mut`). Dual-use markers implement both traits and share only
/// [`EncodeType`].
///
/// - [`Element`](Self::Element): physical storage / `as_slice` element
/// - [`RefView`](Self::RefView): shared borrow projection (`&RefView`) for map
///   getters and element-wise reads
///
/// Decode / merge live on [`RepeatedElementMerge`] so nested messages can
/// constrain `M::Alloc = A`. Tagged encode goes through [`EncodeType`] via
/// [`wire_view`](Self::wire_view).
pub trait RepeatedElement: EncodeType {
    /// Physical element stored in the repeated / map buffer.
    type Element<A: Allocator + Clone>;

    /// User-facing shared view of one element (`i32`, `str`, `M`, …).
    ///
    /// Map [`MapRef`](::puroro::MapRef) / [`MapMut`](::puroro::MapMut) use
    /// `&RefView`. Not the same as [`EncodeType::View`] (numerics are by-value
    /// there).
    type RefView: ?Sized;

    /// Borrows `elem` as [`RefView`](Self::RefView).
    fn as_ref_view<A: Allocator + Clone>(elem: &Self::Element<A>) -> &Self::RefView;

    /// Borrow / copy an element as a [`EncodeType::View`] for tagged encode.
    fn wire_view<'a, A: Allocator + Clone>(
        elem: &'a Self::Element<A>,
    ) -> <Self as EncodeType>::View<'a, A>
    where
        Self: 'a;

    /// Drops one element, freeing heap payload when applicable.
    ///
    /// # Safety
    ///
    /// `alloc` must own `elem`'s buffer when the element is heap-backed.
    unsafe fn deallocate_element<A: Allocator + Clone>(elem: Self::Element<A>, alloc: A);
}

/// Decode / merge for a repeated element marker under allocator `A`.
///
/// Separated from [`RepeatedElement`] so `ProtoMessage<M>` can require
/// `M: Message<Alloc = A>` (inline `Element = M`).
pub trait RepeatedElementMerge<A: Allocator + Clone>: RepeatedElement {
    /// Protobuf empty / type-zero element (missing map-entry key or value).
    fn default_element(alloc: A) -> Self::Element<A>;

    /// Decodes one **singular** field occurrence into an element.
    ///
    /// Unlike [`merge_occurrence`], packed `Len` is rejected — map-entry key /
    /// value fields are singular on the wire.
    fn decode_element<B: DecodeBuf>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
        depth: usize,
    ) -> Result<Self::Element<A>, DecodeError>;

    /// Merges one wire occurrence into `push` (append semantics).
    fn merge_occurrence<B, F>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
        depth: usize,
        push: F,
    ) -> Result<(), DecodeError>
    where
        B: DecodeBuf,
        F: FnMut(Self::Element<A>);
}

/// Packable repeated numerics / enums — support packed encode and dual-form decode.
///
/// Packed payload layout is type-specific (concatenated varints, or fixed-width
/// LE bytes). Decode always accepts both packed (`Len`) and expanded forms.
pub(crate) trait PackableRepeatedElement: RepeatedElement {
    /// Byte length of the packed payload (excluding tag and length prefix).
    fn packed_payload_len<A: Allocator + Clone>(values: &[Self::Element<A>]) -> usize
    where
        Self::Element<A>: Copy;

    /// Writes the packed payload bytes (no tag / length prefix).
    fn encode_packed_payload<A: Allocator + Clone, B: BufMut>(
        values: &[Self::Element<A>],
        buf: &mut B,
    ) where
        Self::Element<A>: Copy;
}

/// Elements that may be mutated through a growable `Vec` (`values_mut`).
///
/// Implemented for copy scalars / enums and nested messages. Not implemented
/// for string / bytes (those use [`RepeatedElementMut`] via
/// [`RepeatedContainerMut`](::puroro::RepeatedContainerMut)).
pub trait RepeatedVecMut: RepeatedElement {}

/// How to obtain a mutable element handle for
/// [`RepeatedContainerMut`](::puroro::RepeatedContainerMut).
///
/// Usually matches singular [`ValueLayout::Mut`](crate::fields::shared::value_layout::ValueLayout::Mut), except [`ProtoBool`] (singular
/// is bit-packed; repeated stores plain `bool`).
pub trait RepeatedElementMut: RepeatedElement {
    /// Target of [`ElementMut`](Self::ElementMut) (`i32`, [`String`](::unmanaged::String), …).
    type MutTarget<A: Allocator + Clone>: ?Sized;

    /// Mutable handle for one repeated element.
    type ElementMut<'a, A: Allocator + Clone>: DerefMut<Target = Self::MutTarget<A>>
    where
        Self: 'a,
        A: 'a;

    /// # Safety
    ///
    /// `alloc` must own `elem`'s heap storage when the element is heap-backed
    /// (string / bytes). Ignored for copy scalars and inline messages.
    unsafe fn element_mut<'a, A: Allocator + Clone + 'a>(
        elem: &'a mut Self::Element<A>,
        alloc: A,
    ) -> Self::ElementMut<'a, A>
    where
        Self: 'a;
}

// ---------------------------------------------------------------------------
// Numerical markers (varint / fixed / enum) — one blanket for all families
// ---------------------------------------------------------------------------

impl<C: NumericalType> RepeatedElement for Numerical<C> {
    type Element<A: Allocator + Clone> = C::NativeType;
    type RefView = C::NativeType;

    #[inline]
    fn as_ref_view<A: Allocator + Clone>(elem: &C::NativeType) -> &C::NativeType {
        elem
    }

    #[inline]
    fn wire_view<'a, A: Allocator + Clone>(elem: &'a C::NativeType) -> C::NativeType
    where
        Self: 'a,
    {
        *elem
    }

    #[inline]
    unsafe fn deallocate_element<A: Allocator + Clone>(_elem: C::NativeType, _alloc: A) {}
}

impl<A: Allocator + Clone, C: NumericalType> RepeatedElementMerge<A> for Numerical<C> {
    #[inline]
    fn default_element(_alloc: A) -> C::NativeType {
        C::NativeType::default()
    }

    #[inline]
    fn decode_element<B: DecodeBuf>(
        wire_type: WireType,
        buf: &mut B,
        _alloc: A,
        _depth: usize,
    ) -> Result<C::NativeType, DecodeError> {
        C::from_wire_body(C::WireBody::decode(wire_type, buf)?)
    }

    fn merge_occurrence<B, F>(
        wire_type: WireType,
        buf: &mut B,
        _alloc: A,
        _depth: usize,
        mut push: F,
    ) -> Result<(), DecodeError>
    where
        B: DecodeBuf,
        F: FnMut(C::NativeType),
    {
        match <C::WireBody as WirePayload>::WIRE_TYPE {
            WireType::Varint => match wire_type {
                WireType::Len => {
                    let len = decode::decode_varint(buf)? as usize;
                    if buf.remaining() < len {
                        return Err(DecodeError::TruncatedMessage);
                    }
                    let mut sub = buf.take(len);
                    while sub.has_remaining() {
                        push(C::from_wire_body(C::WireBody::decode(
                            WireType::Varint,
                            &mut sub,
                        )?)?);
                    }
                    Ok(())
                }
                WireType::Varint => {
                    push(C::from_wire_body(C::WireBody::decode(
                        WireType::Varint,
                        buf,
                    )?)?);
                    Ok(())
                }
                _ => Err(DecodeError::InvalidTag),
            },
            WireType::Int32 => match wire_type {
                WireType::Len => {
                    let len = decode::decode_varint(buf)? as usize;
                    if buf.remaining() < len {
                        return Err(DecodeError::TruncatedMessage);
                    }
                    if !len.is_multiple_of(FIXED32_BYTES) {
                        return Err(DecodeError::TruncatedMessage);
                    }
                    let mut sub = buf.take(len);
                    while sub.has_remaining() {
                        push(C::from_wire_body(C::WireBody::decode(
                            WireType::Int32,
                            &mut sub,
                        )?)?);
                    }
                    Ok(())
                }
                WireType::Int32 => {
                    push(C::from_wire_body(C::WireBody::decode(
                        WireType::Int32,
                        buf,
                    )?)?);
                    Ok(())
                }
                _ => Err(DecodeError::InvalidTag),
            },
            WireType::Int64 => match wire_type {
                WireType::Len => {
                    let len = decode::decode_varint(buf)? as usize;
                    if buf.remaining() < len {
                        return Err(DecodeError::TruncatedMessage);
                    }
                    if !len.is_multiple_of(FIXED64_BYTES) {
                        return Err(DecodeError::TruncatedMessage);
                    }
                    let mut sub = buf.take(len);
                    while sub.has_remaining() {
                        push(C::from_wire_body(C::WireBody::decode(
                            WireType::Int64,
                            &mut sub,
                        )?)?);
                    }
                    Ok(())
                }
                WireType::Int64 => {
                    push(C::from_wire_body(C::WireBody::decode(
                        WireType::Int64,
                        buf,
                    )?)?);
                    Ok(())
                }
                _ => Err(DecodeError::InvalidTag),
            },
            WireType::Len | WireType::SGroup | WireType::EGroup => {
                unreachable!("numerical WireBody is never Len or group")
            }
        }
    }
}

impl<C: NumericalType> PackableRepeatedElement for Numerical<C> {
    #[inline]
    fn packed_payload_len<A: Allocator + Clone>(values: &[C::NativeType]) -> usize
    where
        C::NativeType: Copy,
    {
        // Packed numerics never need nested-length memoization.
        values
            .iter()
            .map(|v| C::to_wire_body(*v).encoded_len())
            .sum()
    }

    #[inline]
    fn encode_packed_payload<A: Allocator + Clone, B: BufMut>(values: &[C::NativeType], buf: &mut B)
    where
        C::NativeType: Copy,
    {
        for v in values {
            C::to_wire_body(*v).encode(buf);
        }
    }
}

impl<C: NumericalType> RepeatedVecMut for Numerical<C> {}

impl<C: NumericalType> RepeatedElementMut for Numerical<C> {
    type MutTarget<A: Allocator + Clone> = C::NativeType;

    type ElementMut<'a, A: Allocator + Clone>
        = &'a mut C::NativeType
    where
        Self: 'a,
        A: 'a;

    #[inline]
    unsafe fn element_mut<'a, A: Allocator + Clone + 'a>(
        elem: &'a mut C::NativeType,
        _alloc: A,
    ) -> &'a mut C::NativeType
    where
        Self: 'a,
    {
        elem
    }
}

// ---------------------------------------------------------------------------
// LEN scalars (`LenScalar<C>`)
// ---------------------------------------------------------------------------

impl<C: LenCodec> RepeatedElement for LenScalar<C> {
    type Element<A: Allocator + Clone> = C::Slot<A>;
    type RefView = C::RefView;

    #[inline]
    fn as_ref_view<A: Allocator + Clone>(elem: &C::Slot<A>) -> &C::RefView {
        Deref::deref(elem)
    }

    #[inline]
    fn wire_view<'a, A: Allocator + Clone>(elem: &'a C::Slot<A>) -> &'a C::RefView
    where
        Self: 'a,
    {
        Deref::deref(elem)
    }

    #[inline]
    unsafe fn deallocate_element<A: Allocator + Clone>(elem: C::Slot<A>, alloc: A) {
        // SAFETY: forwarded to the caller's obligation on `alloc`.
        unsafe { DeallocateIn::deallocate_in(elem, alloc) };
    }
}

impl<A: Allocator + Clone, C: LenCodec> RepeatedElementMerge<A> for LenScalar<C> {
    #[inline]
    fn default_element(alloc: A) -> C::Slot<A> {
        C::new_slot(alloc)
    }

    #[inline]
    fn decode_element<B: DecodeBuf>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
        _depth: usize,
    ) -> Result<C::Slot<A>, DecodeError> {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        C::decode_in(buf, alloc)
    }

    #[inline]
    fn merge_occurrence<B, F>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
        _depth: usize,
        mut push: F,
    ) -> Result<(), DecodeError>
    where
        B: DecodeBuf,
        F: FnMut(C::Slot<A>),
    {
        push(Self::decode_element(wire_type, buf, alloc, _depth)?);
        Ok(())
    }
}

impl<C: LenCodec> RepeatedElementMut for LenScalar<C> {
    type MutTarget<A: Allocator + Clone> = C::MutTarget<A>;

    type ElementMut<'a, A: Allocator + Clone>
        = C::Mut<'a, A>
    where
        Self: 'a,
        A: 'a;

    #[inline]
    unsafe fn element_mut<'a, A: Allocator + Clone + 'a>(
        elem: &'a mut C::Slot<A>,
        alloc: A,
    ) -> Self::ElementMut<'a, A>
    where
        Self: 'a,
    {
        // SAFETY: caller guarantees `alloc` owns `elem`'s buffer.
        unsafe { C::slot_with_alloc(elem, alloc) }
    }
}

// ---------------------------------------------------------------------------
// Nested message
// ---------------------------------------------------------------------------

impl<M: Message + MessageEncode> RepeatedElement for ProtoMessage<M> {
    /// Inline message value (not [`UnmanagedBox`](::unmanaged::UnmanagedBox)).
    ///
    /// Use sites must pair the same allocator: e.g.
    /// `RepeatedField<ProtoMessage<Address<A>>, Expanded, FIELD, A>`.
    type Element<A: Allocator + Clone> = M;
    type RefView = M;

    #[inline]
    fn as_ref_view<A: Allocator + Clone>(elem: &M) -> &M {
        elem
    }

    #[inline]
    fn wire_view<'a, A: Allocator + Clone>(elem: &'a M) -> &'a M
    where
        Self: 'a,
    {
        elem
    }

    #[inline]
    unsafe fn deallocate_element<A: Allocator + Clone>(elem: M, _alloc: A) {
        // Inline repeated elements are not behind `UnmanagedBox`; free via
        // `Drop` / `_common.alloc` (same body as `unmanaged::DeallocateIn` on
        // generated messages). The `alloc` parameter is unused here.
        drop(elem);
    }
}

impl<A, M> RepeatedElementMerge<A> for ProtoMessage<M>
where
    A: Allocator + Clone,
    M: Message<Alloc = A> + MessageEncode + MessageMerge + ::unmanaged::DeallocateIn<A>,
{
    #[inline]
    fn default_element(alloc: A) -> M {
        M::new_in(alloc)
    }

    #[inline]
    fn decode_element<B: DecodeBuf>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
        depth: usize,
    ) -> Result<M, DecodeError> {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        let len = decode::decode_varint(buf)? as usize;
        let mut guard = buf.push_limit_guard(len)?;
        let mut msg = M::new_in(alloc);
        MessageMerge::merge_from_with_depth(&mut msg, &mut *guard, depth + 1)?;
        Ok(msg)
    }

    #[inline]
    fn merge_occurrence<B, F>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
        depth: usize,
        mut push: F,
    ) -> Result<(), DecodeError>
    where
        B: DecodeBuf,
        F: FnMut(M),
    {
        push(Self::decode_element(wire_type, buf, alloc, depth)?);
        Ok(())
    }
}

impl<M: Message + MessageEncode> RepeatedVecMut for ProtoMessage<M> {}

impl<M: Message + MessageEncode> RepeatedElementMut for ProtoMessage<M> {
    type MutTarget<A: Allocator + Clone> = M;

    type ElementMut<'a, A: Allocator + Clone>
        = &'a mut M
    where
        Self: 'a,
        A: 'a;

    #[inline]
    unsafe fn element_mut<'a, A: Allocator + Clone + 'a>(elem: &'a mut M, _alloc: A) -> &'a mut M
    where
        Self: 'a,
    {
        elem
    }
}
