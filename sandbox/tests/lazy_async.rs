//! Integration tests for async lazy parser implementation (PersonLazyAsyncImpl, AddressLazyAsyncImpl).
//!
//! Uses `futures::executor::block_on` as a lightweight runtime (no tokio). Tests mirror the
//! synchronous lazy_parser tests and verify that parsed values are cached so that subsequent
//! getter invocations return the same values without re-reading the stream.

use sandbox::generated::address::{AddressAsync, AddressLazyAsyncImpl};
use sandbox::generated::person::PersonLazyAsyncImpl;

use ::bytes::Bytes;
use ::futures::executor::block_on;
use ::puroro::protobuf_core::{Field, FieldNumber, FieldValue, WriteExtProtobuf};
use ::std::pin::Pin;

fn build_varint_field(field_number: u32, value: i32) -> Vec<u8> {
    let field = Field::<Vec<u8>>::new(
        FieldNumber::try_from(field_number).unwrap(),
        FieldValue::from_int32(value),
    );
    let mut bytes = Vec::new();
    bytes.write_protobuf_field(&field).unwrap();
    bytes
}

fn build_length_delimited_field(field_number: u32, value_bytes: &[u8]) -> Vec<u8> {
    let field = Field::<Vec<u8>>::new(
        FieldNumber::try_from(field_number).unwrap(),
        FieldValue::Len(value_bytes.to_vec()),
    );
    let mut bytes = Vec::new();
    bytes.write_protobuf_field(&field).unwrap();
    bytes
}

fn build_string_field(field_number: u32, value: &str) -> Vec<u8> {
    build_length_delimited_field(field_number, value.as_bytes())
}

/// Encoded Address message bytes (fields 1=street, 2=city, 3=zip_code). Not wrapped in a Person field.
fn build_address_message(street: &str, city: &str, zip_code: i32) -> Vec<u8> {
    let mut bytes = Vec::new();
    if !street.is_empty() {
        bytes.extend_from_slice(&build_string_field(1, street));
    }
    if !city.is_empty() {
        bytes.extend_from_slice(&build_string_field(2, city));
    }
    if zip_code != 0 {
        bytes.extend_from_slice(&build_varint_field(3, zip_code));
    }
    bytes
}

/// Person.address field (field 6) containing an Address message.
fn build_address_field(street: &str, city: &str, zip_code: i32) -> Vec<u8> {
    let address_bytes = build_address_message(street, city, zip_code);
    build_length_delimited_field(6, &address_bytes)
}

/// AsyncRead that yields data in configurable chunk sizes (for testing partial reads).
struct ChunkedReader {
    data: Vec<u8>,
    pos: usize,
    chunks: Vec<usize>,
    chunk_idx: usize,
}

impl ChunkedReader {
    fn new(data: Vec<u8>, chunks: Vec<usize>) -> Self {
        Self {
            data,
            pos: 0,
            chunks,
            chunk_idx: 0,
        }
    }

    /// Reader that yields the entire buffer in one read (for simple tests).
    fn whole(data: Vec<u8>) -> Self {
        let n = data.len();
        Self::new(data, if n == 0 { vec![] } else { vec![n] })
    }
}

impl ::futures_io::AsyncRead for ChunkedReader {
    fn poll_read(
        mut self: Pin<&mut Self>,
        _cx: &mut ::std::task::Context<'_>,
        out: &mut [u8],
    ) -> ::std::task::Poll<Result<usize, ::std::io::Error>> {
        let chunk_limit = self
            .chunks
            .get(self.chunk_idx)
            .copied()
            .unwrap_or(usize::MAX);
        self.chunk_idx = self.chunk_idx.saturating_add(1);

        let remaining_len = self.data.len().saturating_sub(self.pos);
        if remaining_len == 0 {
            return ::std::task::Poll::Ready(Ok(0));
        }

        let n = remaining_len.min(out.len()).min(chunk_limit.max(1));
        out[..n].copy_from_slice(&self.data[self.pos..self.pos + n]);
        self.pos += n;
        ::std::task::Poll::Ready(Ok(n))
    }
}

// =============================================================================
// Person: existing test (chunked read)
// =============================================================================

#[test]
fn test_person_lazy_async_random_split() {
    // Build a person message:
    // - age = 30
    // - scores = [10, 20, 30]
    // - addresses = [Address{street="Main St", city="NY", zip_code=10001}]
    let mut encoded = Vec::new();
    encoded.extend_from_slice(&build_varint_field(2, 30));
    encoded.extend_from_slice(&build_varint_field(10, 10));
    encoded.extend_from_slice(&build_varint_field(10, 20));
    encoded.extend_from_slice(&build_varint_field(10, 30));

    let addr_bytes = build_address_message("Main St", "NY", 10001);
    encoded.extend_from_slice(&build_length_delimited_field(9, &addr_bytes));

    // Feed the bytes using awkward, small chunks (random split simulation).
    let reader = ChunkedReader::new(encoded, vec![1, 2, 1, 3, 1, 1, 4, 2, 1, 5, 1, 2, 1]);
    let person = PersonLazyAsyncImpl::new(reader, None);

    let age = block_on(person.age()).unwrap();
    assert_eq!(age, 30);

    let scores = person.scores();
    let s0 = block_on(scores.get_async(0)).unwrap().unwrap();
    let s2 = block_on(scores.get_async(2)).unwrap().unwrap();
    assert_eq!(s0, 10);
    assert_eq!(s2, 30);

    let scores_len = block_on(scores.len_async()).unwrap();
    assert_eq!(scores_len, 3);

    let addresses = person.addresses();
    let a0 = block_on(addresses.get_async(0)).unwrap().unwrap();
    let city = block_on(a0.city()).unwrap();
    let zip = block_on(a0.zip_code()).unwrap();
    assert_eq!(city, "NY");
    assert_eq!(zip, 10001);
}

// =============================================================================
// Person: age field (mirror lazy_parser)
// =============================================================================

#[test]
fn test_person_lazy_async_age_only() {
    let encoded = build_varint_field(2, 30);
    let reader = ChunkedReader::whole(encoded);
    let person = PersonLazyAsyncImpl::new(reader, None);
    let age = block_on(person.age()).unwrap();
    assert_eq!(age, 30);
}

#[test]
fn test_person_lazy_async_age_default() {
    let encoded: Vec<u8> = vec![];
    let reader = ChunkedReader::whole(encoded);
    let person = PersonLazyAsyncImpl::new(reader, None);
    let age = block_on(person.age()).unwrap();
    assert_eq!(age, 0);
}

#[test]
fn test_person_lazy_async_age_multiple_occurrences() {
    let mut encoded = build_varint_field(2, 25);
    encoded.extend_from_slice(&build_varint_field(2, 30));
    let reader = ChunkedReader::whole(encoded);
    let person = PersonLazyAsyncImpl::new(reader, None);
    let age = block_on(person.age()).unwrap();
    assert_eq!(age, 30);
}

// =============================================================================
// Person: scores (repeated) — mirror lazy_parser
// =============================================================================

#[test]
fn test_person_lazy_async_scores_single() {
    let encoded = build_varint_field(10, 85);
    let reader = ChunkedReader::whole(encoded);
    let person = PersonLazyAsyncImpl::new(reader, None);
    let scores = person.scores();
    let len = block_on(scores.len_async()).unwrap();
    assert_eq!(len, 1);
    let v = block_on(scores.get_async(0)).unwrap().unwrap();
    assert_eq!(v, 85);
}

#[test]
fn test_person_lazy_async_scores_multiple() {
    let mut encoded = Vec::new();
    encoded.extend_from_slice(&build_varint_field(10, 10));
    encoded.extend_from_slice(&build_varint_field(10, 20));
    encoded.extend_from_slice(&build_varint_field(10, 30));
    let reader = ChunkedReader::whole(encoded);
    let person = PersonLazyAsyncImpl::new(reader, None);
    let scores = person.scores();
    let len = block_on(scores.len_async()).unwrap();
    assert_eq!(len, 3);
    assert_eq!(block_on(scores.get_async(0)).unwrap().unwrap(), 10);
    assert_eq!(block_on(scores.get_async(1)).unwrap().unwrap(), 20);
    assert_eq!(block_on(scores.get_async(2)).unwrap().unwrap(), 30);
}

#[test]
fn test_person_lazy_async_scores_empty() {
    let encoded: Vec<u8> = vec![];
    let reader = ChunkedReader::whole(encoded);
    let person = PersonLazyAsyncImpl::new(reader, None);
    let scores = person.scores();
    let len = block_on(scores.len_async()).unwrap();
    assert_eq!(len, 0);
    assert!(block_on(scores.get_async(0)).unwrap().is_none());
}

#[test]
fn test_person_lazy_async_scores_and_age() {
    let mut encoded = Vec::new();
    encoded.extend_from_slice(&build_varint_field(2, 30));
    encoded.extend_from_slice(&build_varint_field(10, 85));
    encoded.extend_from_slice(&build_varint_field(10, 90));
    let reader = ChunkedReader::whole(encoded);
    let person = PersonLazyAsyncImpl::new(reader, None);
    assert_eq!(block_on(person.age()).unwrap(), 30);
    let scores = person.scores();
    let len = block_on(scores.len_async()).unwrap();
    assert_eq!(len, 2);
    assert_eq!(block_on(scores.get_async(0)).unwrap().unwrap(), 85);
    assert_eq!(block_on(scores.get_async(1)).unwrap().unwrap(), 90);
}

// =============================================================================
// Person: address (scalar message) — mirror lazy_parser
// =============================================================================

#[test]
fn test_person_lazy_async_address_simple() {
    let encoded = build_address_field("Main St", "New York", 10001);
    let reader = ChunkedReader::whole(encoded);
    let person = PersonLazyAsyncImpl::new(reader, None);
    let addr_opt = block_on(person.address()).unwrap();
    assert!(addr_opt.is_some());
    let addr = addr_opt.unwrap();
    assert_eq!(block_on(addr.street()).unwrap(), "Main St");
    assert_eq!(block_on(addr.city()).unwrap(), "New York");
    assert_eq!(block_on(addr.zip_code()).unwrap(), 10001);
}

#[test]
fn test_person_lazy_async_address_empty() {
    let encoded: Vec<u8> = vec![];
    let reader = ChunkedReader::whole(encoded);
    let person = PersonLazyAsyncImpl::new(reader, None);
    let addr_opt = block_on(person.address()).unwrap();
    assert!(addr_opt.is_none());
}

#[test]
fn test_person_lazy_async_address_multiple_slices() {
    let first_slice = build_address_message("Main St", "", 0);
    let first_field = build_length_delimited_field(6, &first_slice);
    let second_slice = build_address_message("", "New York", 10001);
    let second_field = build_length_delimited_field(6, &second_slice);
    let mut encoded = first_field;
    encoded.extend_from_slice(&second_field);
    let reader = ChunkedReader::whole(encoded);
    let person = PersonLazyAsyncImpl::new(reader, None);
    let addr_opt = block_on(person.address()).unwrap();
    assert!(addr_opt.is_some());
    let addr = addr_opt.unwrap();
    assert_eq!(block_on(addr.street()).unwrap(), "Main St");
    assert_eq!(block_on(addr.city()).unwrap(), "New York");
    assert_eq!(block_on(addr.zip_code()).unwrap(), 10001);
}

#[test]
fn test_person_lazy_async_address_and_age() {
    let mut encoded = Vec::new();
    encoded.extend_from_slice(&build_varint_field(2, 30));
    encoded.extend_from_slice(&build_address_field("Oak Ave", "Boston", 02115));
    let reader = ChunkedReader::whole(encoded);
    let person = PersonLazyAsyncImpl::new(reader, None);
    assert_eq!(block_on(person.age()).unwrap(), 30);
    let addr_opt = block_on(person.address()).unwrap();
    assert!(addr_opt.is_some());
    let addr = addr_opt.unwrap();
    assert_eq!(block_on(addr.street()).unwrap(), "Oak Ave");
    assert_eq!(block_on(addr.city()).unwrap(), "Boston");
    assert_eq!(block_on(addr.zip_code()).unwrap(), 02115);
}

// =============================================================================
// Person: caching — second getter invocations use cached values
// =============================================================================

#[test]
fn test_person_lazy_async_caching() {
    let mut encoded = Vec::new();
    encoded.extend_from_slice(&build_varint_field(2, 42));
    encoded.extend_from_slice(&build_address_field("Cached St", "Cached City", 12345));
    let reader = ChunkedReader::whole(encoded);
    let person = PersonLazyAsyncImpl::new(reader, None);

    let age1 = block_on(person.age()).unwrap();
    assert_eq!(age1, 42);

    let addr_opt = block_on(person.address()).unwrap();
    assert!(addr_opt.is_some());
    let addr = addr_opt.unwrap();
    assert_eq!(block_on(addr.street()).unwrap(), "Cached St");

    // Second invocation of age() must return the same value from cache (stream already consumed).
    let age2 = block_on(person.age()).unwrap();
    assert_eq!(age2, 42);
    assert_eq!(age1, age2);
}

// =============================================================================
// Address: from_bytes (in-memory lazy async)
// =============================================================================

#[test]
fn test_address_lazy_async_from_bytes() {
    let bytes = build_address_message("123 Main St", "NYC", 10001);
    let addr = AddressLazyAsyncImpl::from_bytes(Bytes::from(bytes));
    assert_eq!(block_on(addr.street()).unwrap(), "123 Main St");
    assert_eq!(block_on(addr.city()).unwrap(), "NYC");
    assert_eq!(block_on(addr.zip_code()).unwrap(), 10001);
}

#[test]
fn test_address_lazy_async_from_bytes_empty() {
    let bytes: Vec<u8> = vec![];
    let addr = AddressLazyAsyncImpl::from_bytes(Bytes::from(bytes));
    assert_eq!(block_on(addr.street()).unwrap(), "");
    assert_eq!(block_on(addr.city()).unwrap(), "");
    assert_eq!(block_on(addr.zip_code()).unwrap(), 0);
}

#[test]
fn test_address_lazy_async_from_reader() {
    let encoded = build_address_message("Reader St", "Reader City", 99999);
    let len = encoded.len();
    let reader = ChunkedReader::whole(encoded);
    let addr = AddressLazyAsyncImpl::new(reader, Some(len));
    assert_eq!(block_on(addr.street()).unwrap(), "Reader St");
    assert_eq!(block_on(addr.city()).unwrap(), "Reader City");
    assert_eq!(block_on(addr.zip_code()).unwrap(), 99999);
}

