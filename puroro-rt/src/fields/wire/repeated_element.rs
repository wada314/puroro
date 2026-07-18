//! Repeated-element semantics for protobuf type markers.
//!
//! Singular fields store [`ProtoType::Slot`](super::proto_type::ProtoType::Slot).
//! Repeated fields store [`RepeatedElement::Element`] — often the inner payload
//! (`i32`, `UnmanagedString`, …), and for nested-message repeated fields the
//! message type `M` itself (not [`UnmanagedBox`](::unmanaged::UnmanagedBox)).
//!
//! [`ProtoBool`](super::varint::ProtoBool) (singular bit-packed via
//! [`BitPacked`](crate::fields::shared::value_layout::BitPacked)) does **not**
//! implement this trait; a future `repeated bool` will use plain `bool` elements.

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};
use ::unmanaged::{UnmanagedString, UnmanagedVec};

use ::puroro::{DecodeError, Message, WireType};

use crate::decode;
use crate::encode;
use crate::fields::shared::DeallocateIn;

use super::len::{LenProtoType, ProtoBytes, ProtoString};
use super::proto_message::ProtoMessage;
use super::proto_type::ProtoType;
use super::varint::{
    Closed, ClosedEnum, Open, OpenEnum, ProtoEnum, ProtoInt32, ProtoInt64, ProtoSint32,
    ProtoSint64, ProtoUInt32, ProtoUInt64, VarintProtoType,
};

/// Wire + storage for one element of a repeated field of marker `Self`.
///
/// Encode / length / deallocate live here. Decode / merge live on
/// [`RepeatedElementMerge`] so nested messages can constrain `M::Alloc = A`.
pub trait RepeatedElement: ProtoType {
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
    /// Merges one wire occurrence into `push` (append semantics).
    fn merge_occurrence<B, F>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
        push: F,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
        F: FnMut(Self::Element<A>);
}

/// Packable repeated numerics / enums — support packed encode and dual-form decode.
pub trait PackableRepeatedElement: RepeatedElement {
    fn encode_wire<A: Allocator + Clone>(value: Self::Element<A>) -> u64
    where
        Self::Element<A>: Copy;

    fn decode_wire<A: Allocator + Clone>(raw: u64) -> Result<Self::Element<A>, DecodeError>
    where
        Self::Element<A>: Copy;
}

/// Elements that may be mutated through a growable `Vec` (`values_mut`).
///
/// Implemented for copy scalars / enums and nested messages. Not implemented
/// for string / bytes (those use [`RepeatedSlicePush::element_from_slice`]).
pub trait RepeatedVecMut: RepeatedElement {}

/// Repeated string / bytes — elements built from a byte slice (`push_*`).
pub trait RepeatedSlicePush: RepeatedElement {
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
        impl RepeatedElement for $marker {
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
            unsafe fn deallocate_element<A: Allocator + Clone>(_elem: $inner, _alloc: A) {}
        }

        impl<A: Allocator + Clone> RepeatedElementMerge<A> for $marker {
            fn merge_occurrence<B, F>(
                wire_type: WireType,
                buf: &mut B,
                _alloc: A,
                mut push: F,
            ) -> Result<(), DecodeError>
            where
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

        impl PackableRepeatedElement for $marker {
            #[inline]
            fn encode_wire<A: Allocator + Clone>(value: $inner) -> u64 {
                <$marker as VarintProtoType>::encode_wire(value)
            }

            #[inline]
            fn decode_wire<A: Allocator + Clone>(raw: u64) -> Result<$inner, DecodeError> {
                <$marker as VarintProtoType>::decode_wire(raw)
            }
        }

        impl RepeatedVecMut for $marker {}
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
        impl<E: $bound> RepeatedElement for ProtoEnum<E, $kind> {
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
            unsafe fn deallocate_element<A: Allocator + Clone>(_elem: E, _alloc: A) {}
        }

        impl<A: Allocator + Clone, E: $bound> RepeatedElementMerge<A> for ProtoEnum<E, $kind> {
            fn merge_occurrence<B, F>(
                wire_type: WireType,
                buf: &mut B,
                _alloc: A,
                mut push: F,
            ) -> Result<(), DecodeError>
            where
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

        impl<E: $bound> PackableRepeatedElement for ProtoEnum<E, $kind> {
            #[inline]
            fn encode_wire<A: Allocator + Clone>(value: E) -> u64 {
                <ProtoEnum<E, $kind> as VarintProtoType>::encode_wire(value)
            }

            #[inline]
            fn decode_wire<A: Allocator + Clone>(raw: u64) -> Result<E, DecodeError> {
                <ProtoEnum<E, $kind> as VarintProtoType>::decode_wire(raw)
            }
        }

        impl<E: $bound> RepeatedVecMut for ProtoEnum<E, $kind> {}
    };
}

impl_packable_enum_repeated!(Open, OpenEnum);
impl_packable_enum_repeated!(Closed, ClosedEnum);

// ---------------------------------------------------------------------------
// LEN markers
// ---------------------------------------------------------------------------

impl RepeatedElement for ProtoString {
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
    unsafe fn deallocate_element<A: Allocator + Clone>(elem: UnmanagedString<A>, alloc: A) {
        // SAFETY: forwarded to the caller's obligation on `alloc`.
        unsafe { DeallocateIn::deallocate_in(elem, alloc) };
    }
}

impl<A: Allocator + Clone> RepeatedElementMerge<A> for ProtoString {
    #[inline]
    fn merge_occurrence<B, F>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
        mut push: F,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
        F: FnMut(UnmanagedString<A>),
    {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        push(<Self as LenProtoType>::decode(buf, alloc)?);
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

impl RepeatedElement for ProtoBytes {
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
    unsafe fn deallocate_element<A: Allocator + Clone>(elem: UnmanagedVec<u8, A>, alloc: A) {
        // SAFETY: forwarded to the caller's obligation on `alloc`.
        unsafe { DeallocateIn::deallocate_in(elem, alloc) };
    }
}

impl<A: Allocator + Clone> RepeatedElementMerge<A> for ProtoBytes {
    #[inline]
    fn merge_occurrence<B, F>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
        mut push: F,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
        F: FnMut(UnmanagedVec<u8, A>),
    {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        push(<Self as LenProtoType>::decode(buf, alloc)?);
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

// ---------------------------------------------------------------------------
// Nested message
// ---------------------------------------------------------------------------

impl<M: Message> RepeatedElement for ProtoMessage<M> {
    /// Inline message value (not [`UnmanagedBox`](::unmanaged::UnmanagedBox)).
    ///
    /// Use sites must pair the same allocator: e.g.
    /// `RepeatedField<ProtoMessage<Address<A>>, Expanded, FIELD, A>`.
    type Element<A: Allocator + Clone> = M;

    #[inline]
    fn encoded_len_element<A: Allocator + Clone>(elem: &M, field: u32) -> usize {
        <Self as ProtoType>::encoded_len::<A>(elem, field)
    }

    #[inline]
    fn encode_element<A: Allocator + Clone, B: BufMut>(elem: &M, field: u32, buf: &mut B) {
        <Self as ProtoType>::encode::<A, B>(elem, field, buf);
    }

    #[inline]
    unsafe fn deallocate_element<A: Allocator + Clone>(elem: M, _alloc: A) {
        // Generated messages free their own heap via `Drop` / `_common.alloc`.
        drop(elem);
    }
}

impl<A, M> RepeatedElementMerge<A> for ProtoMessage<M>
where
    A: Allocator + Clone,
    M: Message<Alloc = A>,
{
    #[inline]
    fn merge_occurrence<B, F>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
        mut push: F,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
        F: FnMut(M),
    {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        let len = decode::decode_varint(buf)? as usize;
        if buf.remaining() < len {
            return Err(DecodeError::TruncatedMessage);
        }
        let mut sub = buf.take(len);
        let mut msg = M::new_in(alloc);
        msg.merge_from(&mut sub)?;
        push(msg);
        Ok(())
    }
}

impl<M: Message> RepeatedVecMut for ProtoMessage<M> {}
