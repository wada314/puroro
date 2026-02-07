use sandbox::generated::person::PersonLazyAsyncImpl;

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
    let city_bytes = block_on(a0.city_bytes()).unwrap();
    let zip = block_on(a0.zip_code()).unwrap();
    assert_eq!(::std::str::from_utf8(city_bytes.as_ref()).unwrap(), "NY");
    assert_eq!(zip, 10001);
}

