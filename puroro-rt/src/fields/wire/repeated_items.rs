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
use ::unmanaged::{UnmanagedString, UnmanagedVec};

use ::puroro::DecodeError;
use ::puroro::WireType;

use crate::decode;
use crate::encode;
use crate::fields::shared::DeallocateIn;

use super::len::{LenProtoType, ProtoBytes, ProtoString};
use super::proto_type::ProtoType;
use super::varint::{
    Closed, ClosedEnum, Open, OpenEnum, ProtoEnum, ProtoInt32, ProtoInt64, ProtoSint32,
    ProtoSint64, ProtoUInt32, ProtoUInt64, VarintProtoType,
};

/// Wire + storage for one element of a repeated field of marker `Self`.
pub trait RepeatedItems: ProtoType {
    /// Physical element stored in the repeated buffer.
    type Element<A: Allocator + Clone>;

    /// Tagged wire length of one expanded element.
    fn encoded_len_element<A: Allocator + Clone>(elem: &Self::Element<A>, field: u32) -> usize;

    /// Encodes one expanded (per-element tagged) occurrence.
    fn encode_element<A: Allocator + Clone, B: BufMut>(
        elem: &Self::Element<A>,
        field: u32,
        buf: &mut B,
    );

    /// Decodes one expanded occurrence after the tag has been read.
    fn decode_element<A: Allocator + Clone, B: Buf>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
    ) -> Result<Self::Element<A>, DecodeError>;

    /// Drops one element, freeing heap payload when applicable.
    ///
    /// # Safety
    ///
    /// `alloc` must own `elem`'s buffer when the element is heap-backed.
    unsafe fn deallocate_element<A: Allocator + Clone>(elem: Self::Element<A>, alloc: A);

    /// Merges one wire occurrence into `push` (append semantics).
    fn merge_occurrence<A, B, F>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
        push: F,
    ) -> Result<(), DecodeError>
    where
        A: Allocator + Clone,
        B: Buf,
        F: FnMut(Self::Element<A>);
}

/// Packable repeated numerics / enums — support packed encode and dual-form decode.
pub trait PackableRepeatedItems: RepeatedItems {
    fn encode_wire<A: Allocator + Clone>(value: Self::Element<A>) -> u64
    where
        Self::Element<A>: Copy;

    fn decode_wire<A: Allocator + Clone>(raw: u64) -> Result<Self::Element<A>, DecodeError>
    where
        Self::Element<A>: Copy;
}

/// Repeated string / bytes — elements built from a byte slice (`push_*`).
pub trait RepeatedSlicePush: RepeatedItems {
    fn element_from_slice<A: Allocator + Clone>(
        v: &[u8],
        alloc: A,
    ) -> Result<Self::Element<A>, DecodeError>;
}

// ---------------------------------------------------------------------------
// Numeric / enum varint markers
// ---------------------------------------------------------------------------

macro_rules! impl_packable_varint_repeated {
    ($marker:ty, $inner:ty) => {
        impl RepeatedItems for $marker {
            type Element<A: Allocator + Clone> = $inner;

            #[inline]
            fn encoded_len_element<A: Allocator + Clone>(elem: &$inner, field: u32) -> usize {
                encode::encoded_len_varint_field(
                    field,
                    <$marker as VarintProtoType>::encode_wire(*elem),
                )
            }

            #[inline]
            fn encode_element<A: Allocator + Clone, B: BufMut>(
                elem: &$inner,
                field: u32,
                buf: &mut B,
            ) {
                encode::encode_varint_field(
                    field,
                    <$marker as VarintProtoType>::encode_wire(*elem),
                    buf,
                );
            }

            #[inline]
            fn decode_element<A: Allocator + Clone, B: Buf>(
                wire_type: WireType,
                buf: &mut B,
                _alloc: A,
            ) -> Result<$inner, DecodeError> {
                if wire_type != WireType::Varint {
                    return Err(DecodeError::InvalidTag);
                }
                let raw = decode::decode_varint(buf)?;
                <$marker as VarintProtoType>::decode_wire(raw)
            }

            #[inline]
            unsafe fn deallocate_element<A: Allocator + Clone>(_elem: $inner, _alloc: A) {}

            fn merge_occurrence<A, B, F>(
                wire_type: WireType,
                buf: &mut B,
                _alloc: A,
                mut push: F,
            ) -> Result<(), DecodeError>
            where
                A: Allocator + Clone,
                B: Buf,
                F: FnMut($inner),
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
                            push(<$marker as VarintProtoType>::decode_wire(raw)?);
                        }
                    }
                    WireType::Varint => {
                        let raw = decode::decode_varint(buf)?;
                        push(<$marker as VarintProtoType>::decode_wire(raw)?);
                    }
                    _ => return Err(DecodeError::InvalidTag),
                }
                Ok(())
            }
        }

        impl PackableRepeatedItems for $marker {
            #[inline]
            fn encode_wire<A: Allocator + Clone>(value: $inner) -> u64 {
                <$marker as VarintProtoType>::encode_wire(value)
            }

            #[inline]
            fn decode_wire<A: Allocator + Clone>(raw: u64) -> Result<$inner, DecodeError> {
                <$marker as VarintProtoType>::decode_wire(raw)
            }
        }
    };
}

impl_packable_varint_repeated!(ProtoUInt32, u32);
impl_packable_varint_repeated!(ProtoUInt64, u64);
impl_packable_varint_repeated!(ProtoInt32, i32);
impl_packable_varint_repeated!(ProtoInt64, i64);
impl_packable_varint_repeated!(ProtoSint32, i32);
impl_packable_varint_repeated!(ProtoSint64, i64);

macro_rules! impl_packable_enum_repeated {
    ($kind:ty, $bound:ident) => {
        impl<E: $bound> RepeatedItems for ProtoEnum<E, $kind> {
            type Element<A: Allocator + Clone> = E;

            #[inline]
            fn encoded_len_element<A: Allocator + Clone>(elem: &E, field: u32) -> usize {
                encode::encoded_len_varint_field(
                    field,
                    <ProtoEnum<E, $kind> as VarintProtoType>::encode_wire(*elem),
                )
            }

            #[inline]
            fn encode_element<A: Allocator + Clone, B: BufMut>(elem: &E, field: u32, buf: &mut B) {
                encode::encode_varint_field(
                    field,
                    <ProtoEnum<E, $kind> as VarintProtoType>::encode_wire(*elem),
                    buf,
                );
            }

            #[inline]
            fn decode_element<A: Allocator + Clone, B: Buf>(
                wire_type: WireType,
                buf: &mut B,
                _alloc: A,
            ) -> Result<E, DecodeError> {
                if wire_type != WireType::Varint {
                    return Err(DecodeError::InvalidTag);
                }
                let raw = decode::decode_varint(buf)?;
                <ProtoEnum<E, $kind> as VarintProtoType>::decode_wire(raw)
            }

            #[inline]
            unsafe fn deallocate_element<A: Allocator + Clone>(_elem: E, _alloc: A) {}

            fn merge_occurrence<A, B, F>(
                wire_type: WireType,
                buf: &mut B,
                _alloc: A,
                mut push: F,
            ) -> Result<(), DecodeError>
            where
                A: Allocator + Clone,
                B: Buf,
                F: FnMut(E),
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
                            push(<ProtoEnum<E, $kind> as VarintProtoType>::decode_wire(raw)?);
                        }
                    }
                    WireType::Varint => {
                        let raw = decode::decode_varint(buf)?;
                        push(<ProtoEnum<E, $kind> as VarintProtoType>::decode_wire(raw)?);
                    }
                    _ => return Err(DecodeError::InvalidTag),
                }
                Ok(())
            }
        }

        impl<E: $bound> PackableRepeatedItems for ProtoEnum<E, $kind> {
            #[inline]
            fn encode_wire<A: Allocator + Clone>(value: E) -> u64 {
                <ProtoEnum<E, $kind> as VarintProtoType>::encode_wire(value)
            }

            #[inline]
            fn decode_wire<A: Allocator + Clone>(raw: u64) -> Result<E, DecodeError> {
                <ProtoEnum<E, $kind> as VarintProtoType>::decode_wire(raw)
            }
        }
    };
}

impl_packable_enum_repeated!(Open, OpenEnum);
impl_packable_enum_repeated!(Closed, ClosedEnum);

// ---------------------------------------------------------------------------
// LEN markers
// ---------------------------------------------------------------------------

impl RepeatedItems for ProtoString {
    type Element<A: Allocator + Clone> = UnmanagedString<A>;

    #[inline]
    fn encoded_len_element<A: Allocator + Clone>(elem: &UnmanagedString<A>, field: u32) -> usize {
        encode::encoded_len_len_field(field, <Self as LenProtoType>::as_bytes(elem).len())
    }

    #[inline]
    fn encode_element<A: Allocator + Clone, B: BufMut>(
        elem: &UnmanagedString<A>,
        field: u32,
        buf: &mut B,
    ) {
        encode::encode_len_field(field, <Self as LenProtoType>::as_bytes(elem), buf);
    }

    #[inline]
    fn decode_element<A: Allocator + Clone, B: Buf>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
    ) -> Result<UnmanagedString<A>, DecodeError> {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        <Self as LenProtoType>::decode(buf, alloc)
    }

    #[inline]
    unsafe fn deallocate_element<A: Allocator + Clone>(elem: UnmanagedString<A>, alloc: A) {
        // SAFETY: forwarded to the caller's obligation on `alloc`.
        unsafe { DeallocateIn::deallocate_in(elem, alloc) };
    }

    #[inline]
    fn merge_occurrence<A, B, F>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
        mut push: F,
    ) -> Result<(), DecodeError>
    where
        A: Allocator + Clone,
        B: Buf,
        F: FnMut(UnmanagedString<A>),
    {
        push(Self::decode_element(wire_type, buf, alloc)?);
        Ok(())
    }
}

impl RepeatedSlicePush for ProtoString {
    #[inline]
    fn element_from_slice<A: Allocator + Clone>(
        v: &[u8],
        alloc: A,
    ) -> Result<UnmanagedString<A>, DecodeError> {
        <Self as LenProtoType>::store_from_slice(v, alloc)
    }
}

impl RepeatedItems for ProtoBytes {
    type Element<A: Allocator + Clone> = UnmanagedVec<u8, A>;

    #[inline]
    fn encoded_len_element<A: Allocator + Clone>(elem: &UnmanagedVec<u8, A>, field: u32) -> usize {
        encode::encoded_len_len_field(field, <Self as LenProtoType>::as_bytes(elem).len())
    }

    #[inline]
    fn encode_element<A: Allocator + Clone, B: BufMut>(
        elem: &UnmanagedVec<u8, A>,
        field: u32,
        buf: &mut B,
    ) {
        encode::encode_len_field(field, <Self as LenProtoType>::as_bytes(elem), buf);
    }

    #[inline]
    fn decode_element<A: Allocator + Clone, B: Buf>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
    ) -> Result<UnmanagedVec<u8, A>, DecodeError> {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        <Self as LenProtoType>::decode(buf, alloc)
    }

    #[inline]
    unsafe fn deallocate_element<A: Allocator + Clone>(elem: UnmanagedVec<u8, A>, alloc: A) {
        // SAFETY: forwarded to the caller's obligation on `alloc`.
        unsafe { DeallocateIn::deallocate_in(elem, alloc) };
    }

    #[inline]
    fn merge_occurrence<A, B, F>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
        mut push: F,
    ) -> Result<(), DecodeError>
    where
        A: Allocator + Clone,
        B: Buf,
        F: FnMut(UnmanagedVec<u8, A>),
    {
        push(Self::decode_element(wire_type, buf, alloc)?);
        Ok(())
    }
}

impl RepeatedSlicePush for ProtoBytes {
    #[inline]
    fn element_from_slice<A: Allocator + Clone>(
        v: &[u8],
        alloc: A,
    ) -> Result<UnmanagedVec<u8, A>, DecodeError> {
        <Self as LenProtoType>::store_from_slice(v, alloc)
    }
}
