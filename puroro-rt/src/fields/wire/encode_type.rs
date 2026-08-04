//! Proto-type marker encode facet + tagged-field framing.
//!
//! [`EncodeType`] is implemented by protobuf **types** (e.g. `int32` /
//! `ProtoInt32`, `string` / `ProtoString` — not wire shapes like `Varint` / `Len`
//! alone). [`payload_len`](EncodeType::payload_len) /
//! [`encode_payload`](EncodeType::encode_payload) write the **complete untagged
//! wire body** for [`WIRE_TYPE`](EncodeType::WIRE_TYPE) (for `Len` types such as
//! string / bytes / message: length varint + content).
//!
//! [`encode_field`] / [`encoded_len_field`] add only the tag. Presence omit stays
//! on [`FieldEncode`](crate::fields::shared::FieldEncode).

use ::allocator_api2::alloc::Allocator;
use ::bytes::BufMut;
use ::puroro::{Message, WireType};

use crate::encode;
use crate::message_encode::{EncodeCtx, MessageEncode};

use super::len::{ProtoBytes, ProtoString};
use super::numerical::NumericalType;
use super::proto_message::ProtoMessage;
use super::wire_payload::WirePayload;

/// Encode facet of a protobuf **type** marker (`ProtoInt32`, `ProtoString`, …).
///
/// Distinct from a **wire type** (`WireType::Varint`, `WireType::Len`, …): the
/// same Rust value (e.g. `i32`) can be `int32`, `sint32`, or `sfixed32`, each
/// with its own marker and codec.
///
/// Markers remain the implementors so that typed view → wire-body mapping stays
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

    /// Byte length of the complete untagged wire body (no tag; for `Len`,
    /// includes the length varint).
    fn payload_len<'a, A: Allocator + Clone>(
        value: Self::View<'a, A>,
        ctx: &mut EncodeCtx,
    ) -> usize
    where
        Self: 'a;

    /// Writes the complete untagged wire body (no tag; for `Len`, includes the
    /// length varint).
    fn encode_payload<'a, A, B>(value: Self::View<'a, A>, ctx: &mut EncodeCtx, buf: &mut B)
    where
        Self: 'a,
        A: Allocator + Clone,
        B: BufMut;
}

/// Tagged occurrence length: tag + untagged wire body. No omit.
#[inline]
pub fn encoded_len_field<'a, T, A>(value: T::View<'a, A>, field: u32, ctx: &mut EncodeCtx) -> usize
where
    T: EncodeType + 'a,
    A: Allocator + Clone + 'a,
{
    match T::WIRE_TYPE {
        WireType::Varint | WireType::Int32 | WireType::Int64 | WireType::Len => {
            encode::encoded_len_tag(field, T::WIRE_TYPE) + T::payload_len::<A>(value, ctx)
        }
        WireType::SGroup | WireType::EGroup => {
            unreachable!("generated markers never use group wire types")
        }
    }
}

/// Tagged occurrence: tag + untagged wire body. No omit.
#[inline]
pub fn encode_field<'a, T, A, B>(
    value: T::View<'a, A>,
    field: u32,
    ctx: &mut EncodeCtx,
    buf: &mut B,
) where
    T: EncodeType + 'a,
    A: Allocator + Clone + 'a,
    B: BufMut,
{
    match T::WIRE_TYPE {
        WireType::Varint | WireType::Int32 | WireType::Int64 | WireType::Len => {
            encode::encode_tag(field, T::WIRE_TYPE, buf);
            T::encode_payload::<A, B>(value, ctx, buf);
        }
        WireType::SGroup | WireType::EGroup => {
            unreachable!("generated markers never use group wire types")
        }
    }
}

impl<T: NumericalType> EncodeType for T {
    type View<'a, A: Allocator + Clone>
        = T::NativeType
    where
        Self: 'a,
        A: 'a;

    const WIRE_TYPE: WireType = <T::WireBody as WirePayload>::WIRE_TYPE;

    #[inline]
    fn payload_len<'a, A: Allocator + Clone>(value: T::NativeType, _ctx: &mut EncodeCtx) -> usize
    where
        Self: 'a,
    {
        T::to_wire_body(value).encoded_len()
    }

    #[inline]
    fn encode_payload<'a, A, B>(value: T::NativeType, _ctx: &mut EncodeCtx, buf: &mut B)
    where
        Self: 'a,
        A: Allocator + Clone,
        B: BufMut,
    {
        T::to_wire_body(value).encode(buf);
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
    fn payload_len<'a, A: Allocator + Clone>(value: &'a str, _ctx: &mut EncodeCtx) -> usize
    where
        Self: 'a,
    {
        let n = value.len();
        encode::encoded_len_varint(n as u64) + n
    }

    #[inline]
    fn encode_payload<'a, A, B>(value: &'a str, _ctx: &mut EncodeCtx, buf: &mut B)
    where
        Self: 'a,
        A: Allocator + Clone,
        B: BufMut,
    {
        encode::encode_varint(value.len() as u64, buf);
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
    fn payload_len<'a, A: Allocator + Clone>(value: &'a [u8], _ctx: &mut EncodeCtx) -> usize
    where
        Self: 'a,
    {
        let n = value.len();
        encode::encoded_len_varint(n as u64) + n
    }

    #[inline]
    fn encode_payload<'a, A, B>(value: &'a [u8], _ctx: &mut EncodeCtx, buf: &mut B)
    where
        Self: 'a,
        A: Allocator + Clone,
        B: BufMut,
    {
        encode::encode_varint(value.len() as u64, buf);
        buf.put_slice(value);
    }
}

impl<M: Message + MessageEncode> EncodeType for ProtoMessage<M> {
    type View<'a, A: Allocator + Clone>
        = &'a M
    where
        Self: 'a,
        A: 'a;

    const WIRE_TYPE: WireType = WireType::Len;

    #[inline]
    fn payload_len<'a, A: Allocator + Clone>(value: &'a M, ctx: &mut EncodeCtx) -> usize
    where
        Self: 'a,
    {
        let n = ctx.body_len_for(value);
        encode::encoded_len_varint(n as u64) + n
    }

    #[inline]
    fn encode_payload<'a, A, B>(value: &'a M, ctx: &mut EncodeCtx, buf: &mut B)
    where
        Self: 'a,
        A: Allocator + Clone,
        B: BufMut,
    {
        let n = ctx.body_len_for(value);
        encode::encode_varint(n as u64, buf);
        value.encode_raw(ctx, buf);
    }
}
