//! Integration tests for `puroro`'s optional `tonic` feature.

use ::bytes::{BufMut, Bytes, BytesMut};
use ::core::pin::Pin;
use ::core::task::{Context, Poll};
use ::http_body::{Body, Frame};
use ::puroro::Message;
use ::puroro::tonic::PuroroDecoder;
use ::puroro_sample_generated::Point;
use ::tonic::Status;
use ::tonic::codec::{HEADER_SIZE, Streaming};

#[test]
fn round_trip_point_payload() {
    let mut point = Point::new();
    *point.x_mut() = 7;
    *point.y_mut() = -3;

    let wire = Message::encode_to_vec(&point);
    let decoded: Point = Point::decode(wire.as_slice()).expect("decode");
    assert_eq!(decoded.x(), 7);
    assert_eq!(decoded.y(), -3);
}

#[tokio::test]
async fn streaming_decode_one_point() {
    let mut point = Point::new();
    *point.x_mut() = 11;
    *point.y_mut() = 22;
    let payload = Message::encode_to_vec(&point);

    let mut buf = BytesMut::new();
    buf.reserve(payload.len() + HEADER_SIZE);
    buf.put_u8(0);
    buf.put_u32(payload.len() as u32);
    buf.put_slice(&payload);

    let body = MockBody::new(&buf);
    let decoder = PuroroDecoder::<Point>::default();
    let mut stream = Streaming::new_request(decoder, body, None, None);

    let got = stream
        .message()
        .await
        .expect("status")
        .expect("one message");
    assert_eq!(got.x(), 11);
    assert_eq!(got.y(), 22);
    assert!(stream.message().await.expect("eof status").is_none());
}

#[tokio::test]
async fn streaming_decode_bad_payload_is_internal() {
    // compression flag 0, length 2, truncated/invalid protobuf body
    let framed = [0u8, 0, 0, 0, 2, 0xff, 0xff];
    let body = MockBody::new(&framed);
    let decoder = PuroroDecoder::<Point>::default();
    let mut stream = Streaming::new_request(decoder, body, None, None);

    let err = stream.message().await.expect_err("decode should fail");
    assert_eq!(err.code(), ::tonic::Code::Internal);
}

#[derive(Debug)]
struct MockBody {
    data: Bytes,
}

impl MockBody {
    fn new(b: &[u8]) -> Self {
        Self {
            data: Bytes::copy_from_slice(b),
        }
    }
}

impl Body for MockBody {
    type Data = Bytes;
    type Error = Status;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        if self.data.is_empty() {
            return Poll::Ready(None);
        }
        let len = self.data.len();
        let data = self.data.split_to(len);
        Poll::Ready(Some(Ok(Frame::data(data))))
    }
}
