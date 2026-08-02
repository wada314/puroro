//! Wire encode / decode for one protobuf map entry (`key = 1`, `value = 2`).

use ::allocator_api2::alloc::Allocator;
use ::bytes::BufMut;
use ::puroro::{DecodeBuf, DecodeError, WireType};

use crate::decode;
use crate::encode;
use crate::fields::wire::encode_type::{encode_field, encoded_len_field};
use crate::fields::wire::map_element::MapKey;
use crate::fields::wire::repeated_element::{RepeatedElement, RepeatedElementMerge};
use crate::message_encode::EncodeCtx;

const KEY_FIELD: u32 = 1;
const VALUE_FIELD: u32 = 2;

type DecodedEntry<K, V, A> = (
    <K as RepeatedElement>::Element<A>,
    <V as RepeatedElement>::Element<A>,
);

/// Tagged wire length of one map-entry message body (fields 1 and 2 only).
#[inline]
pub(super) fn entry_payload_len<K, V, A>(
    key: &K::Element<A>,
    value: &V::Element<A>,
    ctx: &mut EncodeCtx,
) -> usize
where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator + Clone,
{
    encoded_len_field::<K, A>(K::wire_view(key), KEY_FIELD, ctx)
        + encoded_len_field::<V, A>(V::wire_view(value), VALUE_FIELD, ctx)
}

/// Encodes one map field occurrence: `tag(FIELD, Len) + len + entry body`.
#[inline]
pub(super) fn encode_map_entry<K, V, A, B>(
    field: u32,
    key: &K::Element<A>,
    value: &V::Element<A>,
    ctx: &mut EncodeCtx,
    buf: &mut B,
) where
    K: MapKey,
    V: RepeatedElement,
    A: Allocator + Clone,
    B: BufMut,
{
    let payload_len = entry_payload_len::<K, V, A>(key, value, ctx);
    encode::encode_tag(field, WireType::Len, buf);
    encode::encode_varint(payload_len as u64, buf);
    encode_field::<K, A, B>(K::wire_view(key), KEY_FIELD, ctx, buf);
    encode_field::<V, A, B>(V::wire_view(value), VALUE_FIELD, ctx, buf);
}

fn discard_partial_entry<K, V, A>(
    key: Option<K::Element<A>>,
    value: Option<V::Element<A>>,
    alloc: A,
) where
    K: MapKey + RepeatedElementMerge<A>,
    V: RepeatedElementMerge<A>,
    A: Allocator + Clone,
{
    if let Some(k) = key {
        // SAFETY: message allocator owns decoded key payloads.
        unsafe { K::deallocate_element(k, alloc.clone()) };
    }
    if let Some(v) = value {
        // SAFETY: message allocator owns decoded value payloads.
        unsafe { V::deallocate_element(v, alloc) };
    }
}

/// Decodes one map-entry LEN payload into `(key, value)` (defaults if omitted).
pub(super) fn decode_map_entry<K, V, A, B>(
    buf: &mut B,
    alloc: A,
    depth: usize,
) -> Result<DecodedEntry<K, V, A>, DecodeError>
where
    K: MapKey + RepeatedElementMerge<A>,
    V: RepeatedElementMerge<A>,
    A: Allocator + Clone,
    B: DecodeBuf,
{
    let mut key: Option<K::Element<A>> = None;
    let mut value: Option<V::Element<A>> = None;

    while buf.has_remaining() {
        let (field_number, wire_type) = match decode::decode_tag(buf) {
            Ok(t) => t,
            Err(e) => {
                discard_partial_entry::<K, V, A>(key, value, alloc);
                return Err(e);
            }
        };
        match field_number {
            KEY_FIELD => match K::decode_element(wire_type, buf, alloc.clone(), depth) {
                Ok(next) => {
                    if let Some(old) = key.replace(next) {
                        // SAFETY: message allocator owns replaced key payloads.
                        unsafe { K::deallocate_element(old, alloc.clone()) };
                    }
                }
                Err(e) => {
                    discard_partial_entry::<K, V, A>(key, value, alloc);
                    return Err(e);
                }
            },
            VALUE_FIELD => match V::decode_element(wire_type, buf, alloc.clone(), depth) {
                Ok(next) => {
                    if let Some(old) = value.replace(next) {
                        // SAFETY: message allocator owns replaced value payloads.
                        unsafe { V::deallocate_element(old, alloc.clone()) };
                    }
                }
                Err(e) => {
                    discard_partial_entry::<K, V, A>(key, value, alloc);
                    return Err(e);
                }
            },
            _ => {
                if let Err(e) = decode::skip_field(wire_type, buf) {
                    discard_partial_entry::<K, V, A>(key, value, alloc);
                    return Err(e);
                }
            }
        }
    }

    let key = key.unwrap_or_else(|| K::default_element(alloc.clone()));
    let value = value.unwrap_or_else(|| V::default_element(alloc));
    Ok((key, value))
}
