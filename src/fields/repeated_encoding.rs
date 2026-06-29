//! Repeated scalar wire encoding policy (`Packed` vs `Expanded`).
//!
//! Decode always accepts **both** packed (LEN) and expanded (per-element) wire
//! forms regardless of schema declaration (protobuf spec). Encode follows `E`.

use ::bytes::BufMut;

use crate::encode;
use super::varint::VarintProtoType;

/// How a repeated varint-backed field is written on encode.
pub trait RepeatedVarintEncoding: Copy {
    /// Wire byte length when `values` is non-empty; `0` when empty.
    fn encoded_len<T: VarintProtoType>(field: u32, values: &[T::Value]) -> usize;

    /// Writes the field when `values` is non-empty.
    fn encode<B: BufMut, T: VarintProtoType>(field: u32, values: &[T::Value], buf: &mut B);
}

/// One LEN record containing concatenated varints (edition 2024 default for numeric).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Packed;

impl RepeatedVarintEncoding for Packed {
    fn encoded_len<T: VarintProtoType>(field: u32, values: &[T::Value]) -> usize {
        encode::encoded_len_packed_varint_field(field, values, |v| T::encode_wire(*v))
    }

    fn encode<B: BufMut, T: VarintProtoType>(field: u32, values: &[T::Value], buf: &mut B) {
        encode::encode_packed_varint_field(field, values, |v| T::encode_wire(*v), buf);
    }
}

/// One varint wire record per element.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Expanded;

impl RepeatedVarintEncoding for Expanded {
    fn encoded_len<T: VarintProtoType>(field: u32, values: &[T::Value]) -> usize {
        values
            .iter()
            .map(|v| encode::encoded_len_varint_field(field, T::encode_wire(*v)))
            .sum()
    }

    fn encode<B: BufMut, T: VarintProtoType>(field: u32, values: &[T::Value], buf: &mut B) {
        for v in values {
            encode::encode_varint_field(field, T::encode_wire(*v), buf);
        }
    }
}
