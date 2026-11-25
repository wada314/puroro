//! Optional [`::tonic::codec::Codec`] adapter for [`Message`] types.
//!
//! Enabled with the `tonic` Cargo feature. This module does not implement
//! HTTP/2, gRPC framing, or service/stub codegen — only the byte codec.

use ::core::clone::Clone;
use ::core::marker::PhantomData;
use ::tonic::Status;
use ::tonic::codec::{BufferSettings, Codec, DecodeBuf, Decoder, EncodeBuf, Encoder};

use crate::error::DecodeError;
use crate::message::Message;

/// A [`Codec`] that implements `application/grpc+proto` via this crate.
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

    #[test]
    fn decode_error_maps_to_internal() {
        let status = from_decode_error(DecodeError::InvalidVarint);
        assert_eq!(status.code(), ::tonic::Code::Internal);
        assert!(status.message().contains("varint"));
    }
}
