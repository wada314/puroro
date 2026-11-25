//! [`PuroroCodec`] and its encoder / decoder.

use ::core::clone::Clone;
use ::core::marker::PhantomData;
use ::puroro::{DecodeError, Message};
use ::tonic::Status;
use ::tonic::codec::{BufferSettings, Codec, DecodeBuf, Decoder, EncodeBuf, Encoder};

/// A [`Codec`] that implements `application/grpc+proto` via [`puroro`].
///
/// `T` is the outbound message type; `U` is the inbound message type. For a
/// unary RPC those are usually the response and request types respectively on
/// the server side (tonic wires encode/decode that way around the handler).
#[derive(Debug, Clone)]
pub struct PuroroCodec<T, U> {
    _pd: PhantomData<(T, U)>,
}

impl<T, U> PuroroCodec<T, U> {
    /// Creates a codec with default buffer settings.
    pub fn new() -> Self {
        Self { _pd: PhantomData }
    }
}

impl<T, U> Default for PuroroCodec<T, U> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, U> PuroroCodec<T, U>
where
    T: Message + Send + 'static,
    U: Message + Default + Send + 'static,
    U::Alloc: Clone,
{
    /// Builds an encoder with explicit buffer settings.
    pub fn raw_encoder(buffer_settings: BufferSettings) -> <Self as Codec>::Encoder {
        PuroroEncoder {
            _pd: PhantomData,
            buffer_settings,
        }
    }

    /// Builds a decoder with explicit buffer settings.
    pub fn raw_decoder(buffer_settings: BufferSettings) -> <Self as Codec>::Decoder {
        PuroroDecoder {
            _pd: PhantomData,
            buffer_settings,
        }
    }
}

impl<T, U> Codec for PuroroCodec<T, U>
where
    T: Message + Send + 'static,
    U: Message + Default + Send + 'static,
    U::Alloc: Clone,
{
    type Encode = T;
    type Decode = U;

    type Encoder = PuroroEncoder<T>;
    type Decoder = PuroroDecoder<U>;

    fn encoder(&mut self) -> Self::Encoder {
        PuroroEncoder {
            _pd: PhantomData,
            buffer_settings: BufferSettings::default(),
        }
    }

    fn decoder(&mut self) -> Self::Decoder {
        PuroroDecoder {
            _pd: PhantomData,
            buffer_settings: BufferSettings::default(),
        }
    }
}

/// A [`Encoder`] that knows how to encode `T`.
#[derive(Debug, Clone)]
pub struct PuroroEncoder<T> {
    _pd: PhantomData<T>,
    buffer_settings: BufferSettings,
}

impl<T> PuroroEncoder<T> {
    /// Creates an encoder with explicit buffer settings.
    pub fn new(buffer_settings: BufferSettings) -> Self {
        Self {
            _pd: PhantomData,
            buffer_settings,
        }
    }
}

impl<T> Default for PuroroEncoder<T> {
    fn default() -> Self {
        Self::new(BufferSettings::default())
    }
}

impl<T: Message> Encoder for PuroroEncoder<T> {
    type Item = T;
    type Error = Status;

    fn encode(&mut self, item: Self::Item, buf: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
        // puroro encode into a growing buffer is infallible.
        Message::encode(&item, buf);
        Ok(())
    }

    fn buffer_settings(&self) -> BufferSettings {
        self.buffer_settings
    }
}

/// A [`Decoder`] that knows how to decode `U`.
#[derive(Debug, Clone)]
pub struct PuroroDecoder<U> {
    _pd: PhantomData<U>,
    buffer_settings: BufferSettings,
}

impl<U> PuroroDecoder<U> {
    /// Creates a decoder with explicit buffer settings.
    pub fn new(buffer_settings: BufferSettings) -> Self {
        Self {
            _pd: PhantomData,
            buffer_settings,
        }
    }
}

impl<U> Default for PuroroDecoder<U> {
    fn default() -> Self {
        Self::new(BufferSettings::default())
    }
}

impl<U> Decoder for PuroroDecoder<U>
where
    U: Message + Default,
    U::Alloc: Clone,
{
    type Item = U;
    type Error = Status;

    fn decode(&mut self, buf: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
        Message::decode(buf)
            .map(Option::Some)
            .map_err(from_decode_error)
    }

    fn buffer_settings(&self) -> BufferSettings {
        self.buffer_settings
    }
}

fn from_decode_error(error: DecodeError) -> Status {
    // Protobuf parse failures map to INTERNAL, matching tonic-prost and
    // https://github.com/grpc/grpc/blob/master/doc/statuscodes.md
    Status::internal(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::bytes::{BufMut, Bytes, BytesMut};
    use ::core::pin::Pin;
    use ::core::task::{Context, Poll};
    use ::http_body::{Body, Frame};
    use ::puroro::Message;
    use ::puroro_sample_generated::Point;
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

    #[test]
    fn decode_error_maps_to_internal() {
        let status = from_decode_error(DecodeError::InvalidVarint);
        assert_eq!(status.code(), ::tonic::Code::Internal);
        assert!(status.message().contains("varint"));
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
}
