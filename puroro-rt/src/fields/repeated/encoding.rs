//! Repeated scalar wire encoding policy (`Packed` vs `Expanded`).
//!
//! Decode always accepts **both** packed (LEN) and expanded (per-element) wire
//! forms for packable fields, regardless of schema declaration (protobuf spec).
//! Encode follows `E`. Non-packable types (string / bytes / future message) only
//! implement [`Expanded`].

use ::allocator_api2::alloc::Allocator;
use ::bytes::BufMut;

use crate::encode;
use crate::fields::wire::repeated_element::{PackableRepeatedElement, RepeatedElement};
use crate::fields::wire::wire_payload::{encode_field, encoded_len_field};

/// How a repeated field is written on encode.
pub trait RepeatedEncoding<T: RepeatedElement, A: Allocator + Clone>: Copy {
    /// Wire byte length when `values` is non-empty; `0` when empty.
    fn encoded_len(field: u32, values: &[T::Element<A>]) -> usize;

    /// Writes the field when `values` is non-empty.
    fn encode<B: BufMut>(field: u32, values: &[T::Element<A>], buf: &mut B);
}

/// One tagged record per element (expanded varint, string, bytes, …).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Expanded;

impl<T: RepeatedElement, A: Allocator + Clone> RepeatedEncoding<T, A> for Expanded {
    fn encoded_len(field: u32, values: &[T::Element<A>]) -> usize {
        values
            .iter()
            .map(|v| encoded_len_field::<T, A>(T::wire_view(v), field))
            .sum()
    }

    fn encode<B: BufMut>(field: u32, values: &[T::Element<A>], buf: &mut B) {
        for v in values {
            encode_field::<T, A, B>(T::wire_view(v), field, buf);
        }
    }
}

/// One LEN record containing concatenated packed elements (edition 2024 default for numeric).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Packed;

impl<T: PackableRepeatedElement, A: Allocator + Clone> RepeatedEncoding<T, A> for Packed
where
    T::Element<A>: Copy,
{
    fn encoded_len(field: u32, values: &[T::Element<A>]) -> usize {
        if values.is_empty() {
            return 0;
        }
        encode::encoded_len_len_field(field, T::packed_payload_len(values))
    }

    fn encode<B: BufMut>(field: u32, values: &[T::Element<A>], buf: &mut B) {
        if values.is_empty() {
            return;
        }
        let payload_len = T::packed_payload_len(values);
        encode::encode_tag(field, ::puroro::WireType::Len, buf);
        encode::encode_varint(payload_len as u64, buf);
        T::encode_packed_payload(values, buf);
    }
}
