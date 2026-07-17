//! Repeated-element semantics for protobuf type markers.
//!
//! Singular fields store [`ProtoType::Slot`](super::proto_type::ProtoType::Slot).
//! Repeated fields store [`RepeatedItems::Element`] — often the inner payload
//! (`i32`, `UnmanagedString`, …), and for a future nested-message repeated
//! field the message type `M` itself (not [`UnmanagedBox`](::unmanaged::UnmanagedBox)).
//!
//! [`ProtoBool`](super::varint::ProtoBool) (singular bit-packed via
//! [`BitPacked`](crate::fields::shared::value_layout::BitPacked)) does **not**
//! implement this trait; a future `repeated bool` will use plain `bool` elements.

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};
use ::core::ops::Deref;

use ::puroro::DecodeError;
use ::puroro::WireType;

use crate::decode;
use crate::encode;
use crate::fields::shared::value_slot::AddressableSlot;
use crate::fields::shared::{DeallocateIn, ProtoEmpty};

use super::len::{LenProtoType, ProtoBytes, ProtoString};
use super::proto_type::ProtoType;
use super::varint::VarintProtoType;

/// Wire + storage for one element of a repeated field of marker `Self`.
pub trait RepeatedItems: ProtoType {
    /// Physical element stored in the repeated buffer.
    type Element;

    /// Tagged wire length of one expanded element.
    fn encoded_len_element(elem: &Self::Element, field: u32) -> usize;

    /// Encodes one expanded (per-element tagged) occurrence.
    fn encode_element<B: BufMut>(elem: &Self::Element, field: u32, buf: &mut B);

    /// Decodes one expanded occurrence after the tag has been read.
    fn decode_element<B: Buf>(
        wire_type: WireType,
        buf: &mut B,
        alloc: Self::Alloc,
    ) -> Result<Self::Element, DecodeError>;

    /// Drops one element, freeing heap payload when applicable.
    ///
    /// # Safety
    ///
    /// `alloc` must own `elem`'s buffer when the element is heap-backed.
    unsafe fn deallocate_element(elem: Self::Element, alloc: Self::Alloc);

    /// Merges one wire occurrence into `push` (append semantics).
    ///
    /// Packable types accept both packed LEN and expanded VARINT; others decode
    /// a single expanded element.
    fn merge_occurrence<B, F>(
        wire_type: WireType,
        buf: &mut B,
        alloc: Self::Alloc,
        push: F,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
        F: FnMut(Self::Element);
}

/// Packable repeated numerics / enums — support packed encode and dual-form decode.
pub trait PackableRepeatedItems: RepeatedItems
where
    Self::Element: Copy,
{
    fn encode_wire(value: Self::Element) -> u64;
    fn decode_wire(raw: u64) -> Result<Self::Element, DecodeError>;
}

/// Repeated string / bytes — elements built from a byte slice (`push_*`).
pub trait RepeatedSlicePush: RepeatedItems {
    fn element_from_slice(v: &[u8], alloc: Self::Alloc) -> Result<Self::Element, DecodeError>;
}

// ---------------------------------------------------------------------------
// Addressable varint wrappers + enums (Element = wire Value)
// ---------------------------------------------------------------------------

impl<T> RepeatedItems for T
where
    T: VarintProtoType
        + AddressableSlot
        + From<T::Value>
        + ProtoEmpty
        + Deref<Target = T::Value>
        + ProtoType,
    T::Value: Copy,
{
    type Element = T::Value;

    #[inline]
    fn encoded_len_element(elem: &Self::Element, field: u32) -> usize {
        encode::encoded_len_varint_field(field, T::encode_wire(*elem))
    }

    #[inline]
    fn encode_element<B: BufMut>(elem: &Self::Element, field: u32, buf: &mut B) {
        encode::encode_varint_field(field, T::encode_wire(*elem), buf);
    }

    #[inline]
    fn decode_element<B: Buf>(
        wire_type: WireType,
        buf: &mut B,
        _alloc: Self::Alloc,
    ) -> Result<Self::Element, DecodeError> {
        if wire_type != WireType::Varint {
            return Err(DecodeError::InvalidTag);
        }
        let raw = decode::decode_varint(buf)?;
        T::decode_wire(raw)
    }

    #[inline]
    unsafe fn deallocate_element(_elem: Self::Element, _alloc: Self::Alloc) {}

    fn merge_occurrence<B, F>(
        wire_type: WireType,
        buf: &mut B,
        _alloc: Self::Alloc,
        mut push: F,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
        F: FnMut(Self::Element),
    {
        match wire_type {
            WireType::Len => {
                let len = decode::decode_varint(buf)? as usize;
                if buf.remaining() < len {
                    return Err(DecodeError::TruncatedMessage);
                }
                let mut sub = buf.take(len);
                while sub.has_remaining() {
                    let raw = decode::decode_varint(&mut sub)?;
                    push(T::decode_wire(raw)?);
                }
            }
            WireType::Varint => {
                let raw = decode::decode_varint(buf)?;
                push(T::decode_wire(raw)?);
            }
            _ => return Err(DecodeError::InvalidTag),
        }
        Ok(())
    }
}

impl<T> PackableRepeatedItems for T
where
    T: RepeatedItems<Element = <T as VarintProtoType>::Value>
        + VarintProtoType
        + AddressableSlot
        + From<T::Value>
        + ProtoEmpty
        + Deref<Target = T::Value>
        + ProtoType,
    T::Value: Copy,
{
    #[inline]
    fn encode_wire(value: Self::Element) -> u64 {
        T::encode_wire(value)
    }

    #[inline]
    fn decode_wire(raw: u64) -> Result<Self::Element, DecodeError> {
        T::decode_wire(raw)
    }
}

// ---------------------------------------------------------------------------
// LEN markers
// ---------------------------------------------------------------------------

impl<A: Allocator + Clone> RepeatedItems for ProtoString<A> {
    type Element = <Self as LenProtoType>::Storage;

    #[inline]
    fn encoded_len_element(elem: &Self::Element, field: u32) -> usize {
        encode::encoded_len_len_field(field, <Self as LenProtoType>::as_bytes(elem).len())
    }

    #[inline]
    fn encode_element<B: BufMut>(elem: &Self::Element, field: u32, buf: &mut B) {
        encode::encode_len_field(field, <Self as LenProtoType>::as_bytes(elem), buf);
    }

    #[inline]
    fn decode_element<B: Buf>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
    ) -> Result<Self::Element, DecodeError> {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        <Self as LenProtoType>::decode(buf, alloc)
    }

    #[inline]
    unsafe fn deallocate_element(elem: Self::Element, alloc: A) {
        // SAFETY: forwarded to the caller's obligation on `alloc`.
        unsafe { DeallocateIn::deallocate_in(elem, alloc) };
    }

    #[inline]
    fn merge_occurrence<B, F>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
        mut push: F,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
        F: FnMut(Self::Element),
    {
        push(Self::decode_element(wire_type, buf, alloc)?);
        Ok(())
    }
}

impl<A: Allocator + Clone> RepeatedSlicePush for ProtoString<A> {
    #[inline]
    fn element_from_slice(v: &[u8], alloc: A) -> Result<Self::Element, DecodeError> {
        <Self as LenProtoType>::store_from_slice(v, alloc)
    }
}

impl<A: Allocator + Clone> RepeatedItems for ProtoBytes<A> {
    type Element = <Self as LenProtoType>::Storage;

    #[inline]
    fn encoded_len_element(elem: &Self::Element, field: u32) -> usize {
        encode::encoded_len_len_field(field, <Self as LenProtoType>::as_bytes(elem).len())
    }

    #[inline]
    fn encode_element<B: BufMut>(elem: &Self::Element, field: u32, buf: &mut B) {
        encode::encode_len_field(field, <Self as LenProtoType>::as_bytes(elem), buf);
    }

    #[inline]
    fn decode_element<B: Buf>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
    ) -> Result<Self::Element, DecodeError> {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        <Self as LenProtoType>::decode(buf, alloc)
    }

    #[inline]
    unsafe fn deallocate_element(elem: Self::Element, alloc: A) {
        // SAFETY: forwarded to the caller's obligation on `alloc`.
        unsafe { DeallocateIn::deallocate_in(elem, alloc) };
    }

    #[inline]
    fn merge_occurrence<B, F>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
        mut push: F,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
        F: FnMut(Self::Element),
    {
        push(Self::decode_element(wire_type, buf, alloc)?);
        Ok(())
    }
}

impl<A: Allocator + Clone> RepeatedSlicePush for ProtoBytes<A> {
    #[inline]
    fn element_from_slice(v: &[u8], alloc: A) -> Result<Self::Element, DecodeError> {
        <Self as LenProtoType>::store_from_slice(v, alloc)
    }
}
