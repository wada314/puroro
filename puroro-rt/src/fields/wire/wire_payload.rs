//! Marker-level wire payload + tagged-field framing.
//!
//! [`WirePayload`] encodes the bare value bytes (no tag, and for LEN no length
//! prefix). [`encode_field`] / [`encoded_len_field`] wrap that payload with a
//! tag (and LEN length when needed). Presence omit stays on
//! [`FieldEncode`](crate::fields::shared::FieldEncode).

use ::allocator_api2::alloc::Allocator;
use ::bytes::BufMut;

use ::puroro::{Message, WireType};

use crate::encode;

use super::len::{ProtoBytes, ProtoString};
use super::proto_message::ProtoMessage;
use super::varint::ProtoBool;

/// Bare wire payload for a type marker (`Varint` / fixed / `Len` body).
///
/// Markers remain the implementors so the same Rust value type can use
/// different codecs (e.g. `i32` as int32 / sint32 / sfixed32).
pub trait WirePayload {
    /// View passed to payload / tagged encode (matches singular [`ProtoType::Ref`](super::proto_type::ProtoType::Ref)).
    ///
    /// Always `Copy` (by-value scalars or shared references) so LEN framing can
    /// measure then write the same view.
    type View<'a, A: Allocator + Clone>: Copy
    where
        Self: 'a,
        A: 'a;

    /// Fixed per marker (`Varint` / `Int32` / `Int64` / `Len`).
    const WIRE_TYPE: WireType;

    /// Byte length of the bare payload (no tag / LEN length prefix).
    fn payload_len<'a, A: Allocator + Clone>(value: Self::View<'a, A>) -> usize
    where
        Self: 'a;

    /// Writes the bare payload bytes (no tag / LEN length prefix).
    fn encode_payload<'a, A, B>(value: Self::View<'a, A>, buf: &mut B)
    where
        Self: 'a,
        A: Allocator + Clone,
        B: BufMut;
}

/// Tagged occurrence length: tag (+ LEN length prefix) + payload. No omit.
#[inline]
pub fn encoded_len_field<'a, T, A>(value: T::View<'a, A>, field: u32) -> usize
where
    T: WirePayload + 'a,
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

/// Tagged occurrence: tag (+ LEN length prefix) + payload. No omit.
#[inline]
pub fn encode_field<'a, T, A, B>(value: T::View<'a, A>, field: u32, buf: &mut B)
where
    T: WirePayload + 'a,
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

impl WirePayload for ProtoBool {
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

impl WirePayload for ProtoString {
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

impl WirePayload for ProtoBytes {
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

impl<M: Message> WirePayload for ProtoMessage<M> {
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
