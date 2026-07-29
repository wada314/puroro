//! Proto-type marker encode facet + tagged-field framing.
//!
//! [`EncodeType`] is implemented by protobuf **types** (e.g. `int32` /
//! `ProtoInt32`, `string` / `ProtoString`, not wire shapes like Varint / Len
//! alone). It writes the **untagged** payload body (for Len types such as
//! string / bytes / message: raw contents only — no length prefix).
//!
//! [`encode_field`] / [`encoded_len_field`] add the tag and, for Len, the
//! length varint. Presence omit stays on
//! [`FieldEncode`](crate::fields::shared::FieldEncode).

use ::allocator_api2::alloc::Allocator;
use ::bytes::BufMut;

use ::puroro::{Message, WireType};

use crate::encode;

use super::len::{ProtoBytes, ProtoString};
use super::proto_message::ProtoMessage;
use super::varint::ProtoBool;

/// Encode facet of a protobuf **type** marker (`ProtoInt32`, `ProtoString`, …).
///
/// Distinct from a **wire type** (`WireType::Varint`, `WireType::Len`, …): the
/// same Rust value (e.g. `i32`) can be `int32`, `sint32`, or `sfixed32`, each
/// with its own marker and codec.
///
/// Markers remain the implementors so that typed view → payload mapping stays
/// with the proto type.
pub trait EncodeType {
    /// View passed to payload / tagged encode (also the singular getter view).
    ///
    /// Always `Copy` (by-value scalars such as `i32`, or shared references such
    /// as `&str`) so Len framing can measure then write the same view.
    type View<'a, A: Allocator + Clone>: Copy
    where
        Self: 'a,
        A: 'a;

    /// Wire type used on the tag (`Varint`, `Int32` / fixed32-family, `Int64` /
    /// fixed64-family, or `Len`).
    const WIRE_TYPE: WireType;

    /// Byte length of the untagged payload (no tag; for Len, no length prefix).
    fn payload_len<'a, A: Allocator + Clone>(value: Self::View<'a, A>) -> usize
    where
        Self: 'a;

    /// Writes the untagged payload bytes (no tag; for Len, no length prefix).
    fn encode_payload<'a, A, B>(value: Self::View<'a, A>, buf: &mut B)
    where
        Self: 'a,
        A: Allocator + Clone,
        B: BufMut;
}

/// Tagged occurrence length: tag (+ Len length prefix) + payload. No omit.
#[inline]
pub fn encoded_len_field<'a, T, A>(value: T::View<'a, A>, field: u32) -> usize
where
    T: EncodeType + 'a,
    A: Allocator + Clone + 'a,
{
    match T::WIRE_TYPE {
        WireType::Varint | WireType::Int32 | WireType::Int64 => {
            encode::encoded_len_tag(field, T::WIRE_TYPE) + T::payload_len::<A>(value)
        }
        WireType::Len => encode::encoded_len_len_field(field, T::payload_len::<A>(value)),
        WireType::SGroup | WireType::EGroup => {
            unreachable!("generated markers never use group wire types")
        }
    }
}

/// Tagged occurrence: tag (+ Len length prefix) + payload. No omit.
#[inline]
pub fn encode_field<'a, T, A, B>(value: T::View<'a, A>, field: u32, buf: &mut B)
where
    T: EncodeType + 'a,
    A: Allocator + Clone + 'a,
    B: BufMut,
{
    match T::WIRE_TYPE {
        WireType::Varint | WireType::Int32 | WireType::Int64 => {
            encode::encode_tag(field, T::WIRE_TYPE, buf);
            T::encode_payload::<A, B>(value, buf);
        }
        WireType::Len => {
            let payload_len = T::payload_len::<A>(value);
            encode::encode_tag(field, WireType::Len, buf);
            encode::encode_varint(payload_len as u64, buf);
            T::encode_payload::<A, B>(value, buf);
        }
        WireType::SGroup | WireType::EGroup => {
            unreachable!("generated markers never use group wire types")
        }
    }
}

impl EncodeType for ProtoBool {
    type View<'a, A: Allocator + Clone>
        = bool
    where
        Self: 'a,
        A: 'a;

    const WIRE_TYPE: WireType = WireType::Varint;

    #[inline]
    fn payload_len<'a, A: Allocator + Clone>(value: bool) -> usize
    where
        Self: 'a,
    {
        encode::encoded_len_varint(Self::encode_wire(value))
    }

    #[inline]
    fn encode_payload<'a, A, B>(value: bool, buf: &mut B)
    where
        Self: 'a,
        A: Allocator + Clone,
        B: BufMut,
    {
        encode::encode_varint(Self::encode_wire(value), buf);
    }
}

impl EncodeType for ProtoString {
    type View<'a, A: Allocator + Clone>
        = &'a str
    where
        Self: 'a,
        A: 'a;

    const WIRE_TYPE: WireType = WireType::Len;

    #[inline]
    fn payload_len<'a, A: Allocator + Clone>(value: &'a str) -> usize
    where
        Self: 'a,
    {
        value.len()
    }

    #[inline]
    fn encode_payload<'a, A, B>(value: &'a str, buf: &mut B)
    where
        Self: 'a,
        A: Allocator + Clone,
        B: BufMut,
    {
        buf.put_slice(value.as_bytes());
    }
}

impl EncodeType for ProtoBytes {
    type View<'a, A: Allocator + Clone>
        = &'a [u8]
    where
        Self: 'a,
        A: 'a;

    const WIRE_TYPE: WireType = WireType::Len;

    #[inline]
    fn payload_len<'a, A: Allocator + Clone>(value: &'a [u8]) -> usize
    where
        Self: 'a,
    {
        value.len()
    }

    #[inline]
    fn encode_payload<'a, A, B>(value: &'a [u8], buf: &mut B)
    where
        Self: 'a,
        A: Allocator + Clone,
        B: BufMut,
    {
        buf.put_slice(value);
    }
}

impl<M: Message> EncodeType for ProtoMessage<M> {
    type View<'a, A: Allocator + Clone>
        = &'a M
    where
        Self: 'a,
        A: 'a;

    const WIRE_TYPE: WireType = WireType::Len;

    #[inline]
    fn payload_len<'a, A: Allocator + Clone>(value: &'a M) -> usize
    where
        Self: 'a,
    {
        value.encoded_len()
    }

    #[inline]
    fn encode_payload<'a, A, B>(value: &'a M, buf: &mut B)
    where
        Self: 'a,
        A: Allocator + Clone,
        B: BufMut,
    {
        value.encode_raw(buf);
    }
}
