//! Repeated scalar wire encoding policy (`Packed` vs `Expanded`).
//!
//! Decode always accepts **both** packed (LEN) and expanded (per-element) wire
//! forms for packable fields, regardless of schema declaration (protobuf spec).
//! Encode follows `E`. Non-packable types (string / bytes / future message) only
//! implement [`Expanded`].

use ::bytes::BufMut;

use crate::encode;
use crate::fields::wire::repeated_items::{PackableRepeatedItems, RepeatedItems};

/// How a repeated field is written on encode.
pub trait RepeatedEncoding<T: RepeatedItems>: Copy {
    /// Wire byte length when `values` is non-empty; `0` when empty.
    fn encoded_len(field: u32, values: &[T::Element]) -> usize;

    /// Writes the field when `values` is non-empty.
    fn encode<B: BufMut>(field: u32, values: &[T::Element], buf: &mut B);
}

/// One tagged record per element (expanded varint, string, bytes, …).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Expanded;

impl<T: RepeatedItems> RepeatedEncoding<T> for Expanded {
    fn encoded_len(field: u32, values: &[T::Element]) -> usize {
        values
            .iter()
            .map(|v| T::encoded_len_element(v, field))
            .sum()
    }

    fn encode<B: BufMut>(field: u32, values: &[T::Element], buf: &mut B) {
        for v in values {
            T::encode_element(v, field, buf);
        }
    }
}

/// One LEN record containing concatenated varints (edition 2024 default for numeric).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Packed;

impl<T: PackableRepeatedItems> RepeatedEncoding<T> for Packed
where
    T::Element: Copy,
{
    fn encoded_len(field: u32, values: &[T::Element]) -> usize {
        encode::encoded_len_packed_varint_field(field, values, |v| T::encode_wire(*v))
    }

    fn encode<B: BufMut>(field: u32, values: &[T::Element], buf: &mut B) {
        encode::encode_packed_varint_field(field, values, |v| T::encode_wire(*v), buf);
    }
}
