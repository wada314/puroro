//! Repeated-element semantics for protobuf type markers.
//!
//! Singular fields store [`ProtoType::Slot`](super::proto_type::ProtoType::Slot).
//! Repeated fields store [`RepeatedElement::Element`] — often the inner payload
//! (`i32`, `UnmanagedString`, …), and for nested-message repeated fields the
//! message type `M` itself (not [`UnmanagedBox`](::unmanaged::UnmanagedBox)).
//!
//! Singular [`ProtoBool`](super::varint::ProtoBool) uses bit-packed storage;
//! repeated uses plain `bool` elements via this trait (no MessageCommon bit).

use ::allocator_api2::alloc::Allocator;
use ::bytes::{Buf, BufMut};
use ::unmanaged::{UnmanagedString, UnmanagedVec};

use ::puroro::{DecodeError, Message, WireType};

use crate::decode;
use crate::encode;
use ::unmanaged::DeallocateIn;

use super::fixed::{
    Fixed32ProtoType, Fixed64ProtoType, ProtoDouble, ProtoFixed32, ProtoFixed64, ProtoFloat,
    ProtoSFixed32, ProtoSFixed64,
};
use super::len::{ProtoBytes, ProtoString};
use super::proto_message::ProtoMessage;
use super::proto_type::ProtoType;
use super::varint::{
    Closed, ClosedEnum, Open, OpenEnum, ProtoBool, ProtoEnum, ProtoInt32, ProtoInt64, ProtoSint32,
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
        depth: usize,
        push: F,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
        F: FnMut(Self::Element<A>);
}

/// Packable repeated numerics / enums — support packed encode and dual-form decode.
///
/// Packed payload layout is type-specific (concatenated varints, or fixed-width
/// LE bytes). Decode always accepts both packed (`Len`) and expanded forms.
pub trait PackableRepeatedElement: RepeatedElement {
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
                _depth: usize,
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
            fn packed_payload_len<A: Allocator + Clone>(values: &[$inner]) -> usize {
                values
                    .iter()
                    .map(|v| {
                        encode::encoded_len_varint(<$marker as VarintProtoType>::encode_wire(*v))
                    })
                    .sum()
            }

            #[inline]
            fn encode_packed_payload<A: Allocator + Clone, B: BufMut>(
                values: &[$inner],
                buf: &mut B,
            ) {
                for v in values {
                    encode::encode_varint(<$marker as VarintProtoType>::encode_wire(*v), buf);
                }
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
impl_packable_varint_repeated!(ProtoBool, bool);

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
                _depth: usize,
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
            fn packed_payload_len<A: Allocator + Clone>(values: &[E]) -> usize {
                values
                    .iter()
                    .map(|v| {
                        encode::encoded_len_varint(
                            <ProtoEnum<E, $kind> as VarintProtoType>::encode_wire(*v),
                        )
                    })
                    .sum()
            }

            #[inline]
            fn encode_packed_payload<A: Allocator + Clone, B: BufMut>(values: &[E], buf: &mut B) {
                for v in values {
                    encode::encode_varint(
                        <ProtoEnum<E, $kind> as VarintProtoType>::encode_wire(*v),
                        buf,
                    );
                }
            }
        }

        impl<E: $bound> RepeatedVecMut for ProtoEnum<E, $kind> {}
    };
}

impl_packable_enum_repeated!(Open, OpenEnum);
impl_packable_enum_repeated!(Closed, ClosedEnum);

// ---------------------------------------------------------------------------
// Fixed-width markers
// ---------------------------------------------------------------------------

macro_rules! impl_packable_fixed32_repeated {
    ($marker:ty, $inner:ty) => {
        impl RepeatedElement for $marker {
            type Element<A: Allocator + Clone> = $inner;

            #[inline]
            fn encoded_len_element<A: Allocator + Clone>(_elem: &$inner, field: u32) -> usize {
                encode::encoded_len_fixed32_field(field)
            }

            #[inline]
            fn encode_element<A: Allocator + Clone, B: BufMut>(
                elem: &$inner,
                field: u32,
                buf: &mut B,
            ) {
                encode::encode_fixed32_field(field, elem.to_le_bytes(), buf);
            }

            #[inline]
            unsafe fn deallocate_element<A: Allocator + Clone>(_elem: $inner, _alloc: A) {}
        }

        impl<A: Allocator + Clone> RepeatedElementMerge<A> for $marker {
            fn merge_occurrence<B, F>(
                wire_type: WireType,
                buf: &mut B,
                _alloc: A,
                _depth: usize,
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
                        if len % ::protobuf_core::FIXED32_BYTES != 0 {
                            return Err(DecodeError::TruncatedMessage);
                        }
                        let mut sub = buf.take(len);
                        while sub.has_remaining() {
                            push(<$marker as Fixed32ProtoType>::decode_wire(&mut sub)?);
                        }
                    }
                    WireType::Int32 => {
                        push(<$marker as Fixed32ProtoType>::decode_wire(buf)?);
                    }
                    _ => return Err(DecodeError::InvalidTag),
                }
                Ok(())
            }
        }

        impl PackableRepeatedElement for $marker {
            #[inline]
            fn packed_payload_len<A: Allocator + Clone>(values: &[$inner]) -> usize {
                values.len() * ::protobuf_core::FIXED32_BYTES
            }

            #[inline]
            fn encode_packed_payload<A: Allocator + Clone, B: BufMut>(
                values: &[$inner],
                buf: &mut B,
            ) {
                for v in values {
                    <$marker as Fixed32ProtoType>::encode_wire(*v, buf);
                }
            }
        }

        impl RepeatedVecMut for $marker {}
    };
}

macro_rules! impl_packable_fixed64_repeated {
    ($marker:ty, $inner:ty) => {
        impl RepeatedElement for $marker {
            type Element<A: Allocator + Clone> = $inner;

            #[inline]
            fn encoded_len_element<A: Allocator + Clone>(_elem: &$inner, field: u32) -> usize {
                encode::encoded_len_fixed64_field(field)
            }

            #[inline]
            fn encode_element<A: Allocator + Clone, B: BufMut>(
                elem: &$inner,
                field: u32,
                buf: &mut B,
            ) {
                encode::encode_fixed64_field(field, elem.to_le_bytes(), buf);
            }

            #[inline]
            unsafe fn deallocate_element<A: Allocator + Clone>(_elem: $inner, _alloc: A) {}
        }

        impl<A: Allocator + Clone> RepeatedElementMerge<A> for $marker {
            fn merge_occurrence<B, F>(
                wire_type: WireType,
                buf: &mut B,
                _alloc: A,
                _depth: usize,
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
                        if len % ::protobuf_core::FIXED64_BYTES != 0 {
                            return Err(DecodeError::TruncatedMessage);
                        }
                        let mut sub = buf.take(len);
                        while sub.has_remaining() {
                            push(<$marker as Fixed64ProtoType>::decode_wire(&mut sub)?);
                        }
                    }
                    WireType::Int64 => {
                        push(<$marker as Fixed64ProtoType>::decode_wire(buf)?);
                    }
                    _ => return Err(DecodeError::InvalidTag),
                }
                Ok(())
            }
        }

        impl PackableRepeatedElement for $marker {
            #[inline]
            fn packed_payload_len<A: Allocator + Clone>(values: &[$inner]) -> usize {
                values.len() * ::protobuf_core::FIXED64_BYTES
            }

            #[inline]
            fn encode_packed_payload<A: Allocator + Clone, B: BufMut>(
                values: &[$inner],
                buf: &mut B,
            ) {
                for v in values {
                    <$marker as Fixed64ProtoType>::encode_wire(*v, buf);
                }
            }
        }

        impl RepeatedVecMut for $marker {}
    };
}

impl_packable_fixed32_repeated!(ProtoFixed32, u32);
impl_packable_fixed32_repeated!(ProtoSFixed32, i32);
impl_packable_fixed32_repeated!(ProtoFloat, f32);
impl_packable_fixed64_repeated!(ProtoFixed64, u64);
impl_packable_fixed64_repeated!(ProtoSFixed64, i64);
impl_packable_fixed64_repeated!(ProtoDouble, f64);

// ---------------------------------------------------------------------------
// LEN markers
// ---------------------------------------------------------------------------

impl RepeatedElement for ProtoString {
    type Element<A: Allocator + Clone> = UnmanagedString<A>;

    #[inline]
    fn encoded_len_element<A: Allocator + Clone>(elem: &UnmanagedString<A>, field: u32) -> usize {
        encode::encoded_len_len_field(field, elem.as_bytes().len())
    }

    #[inline]
    fn encode_element<A: Allocator + Clone, B: BufMut>(
        elem: &UnmanagedString<A>,
        field: u32,
        buf: &mut B,
    ) {
        encode::encode_len_field(field, elem.as_bytes(), buf);
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
        _depth: usize,
        mut push: F,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
        F: FnMut(UnmanagedString<A>),
    {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        push(decode::decode_string_in(buf, alloc)?);
        Ok(())
    }
}

impl RepeatedSlicePush for ProtoString {
    #[inline]
    fn element_from_slice<A: Allocator + Clone>(
        v: &[u8],
        alloc: A,
    ) -> Result<UnmanagedString<A>, DecodeError> {
        let s = ::core::str::from_utf8(v).map_err(|_| DecodeError::InvalidUtf8)?;
        Ok(decode::str_to_unmanaged_in(s, alloc))
    }
}

impl RepeatedElement for ProtoBytes {
    type Element<A: Allocator + Clone> = UnmanagedVec<u8, A>;

    #[inline]
    fn encoded_len_element<A: Allocator + Clone>(elem: &UnmanagedVec<u8, A>, field: u32) -> usize {
        encode::encoded_len_len_field(field, elem.len())
    }

    #[inline]
    fn encode_element<A: Allocator + Clone, B: BufMut>(
        elem: &UnmanagedVec<u8, A>,
        field: u32,
        buf: &mut B,
    ) {
        encode::encode_len_field(field, elem, buf);
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
        _depth: usize,
        mut push: F,
    ) -> Result<(), DecodeError>
    where
        B: Buf,
        F: FnMut(UnmanagedVec<u8, A>),
    {
        if wire_type != WireType::Len {
            return Err(DecodeError::InvalidTag);
        }
        push(decode::decode_bytes_in(buf, alloc)?);
        Ok(())
    }
}

impl RepeatedSlicePush for ProtoBytes {
    #[inline]
    fn element_from_slice<A: Allocator + Clone>(
        v: &[u8],
        alloc: A,
    ) -> Result<UnmanagedVec<u8, A>, DecodeError> {
        Ok(decode::bytes_to_unmanaged_in(v, alloc))
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
        // Inline repeated elements are not behind `UnmanagedBox`; free via
        // `Drop` / `_common.alloc` (same body as `unmanaged::DeallocateIn` on
        // generated messages). The `alloc` parameter is unused here.
        drop(elem);
    }
}

impl<A, M> RepeatedElementMerge<A> for ProtoMessage<M>
where
    A: Allocator + Clone,
    M: Message<Alloc = A> + ::unmanaged::DeallocateIn<A>,
{
    #[inline]
    fn merge_occurrence<B, F>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
        depth: usize,
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
        // Concrete `&[u8]` avoids infinite `Take<…>` monomorphization for
        // recursive message types (same rationale as singular `ProtoMessage`).
        let payload = buf.copy_to_bytes(len);
        let mut sub: &[u8] = payload.as_ref();
        let mut msg = M::new_in(alloc);
        msg.merge_from_with_depth(&mut sub, depth + 1)?;
        push(msg);
        Ok(())
    }
}

impl<M: Message> RepeatedVecMut for ProtoMessage<M> {}
