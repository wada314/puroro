//! Optional [`tonic::codec::Codec`] adapter for [`Message`] types.
//!
//! Enabled with the `tonic` Cargo feature. This module does not implement
//! HTTP/2, gRPC framing, or service/stub codegen — only the byte codec used by
//! those layers.
//!
//! # Using gRPC with generated stubs
//!
//! `protoc-gen-puroro` emits tonic client/server modules when the `.proto` has
//! `service` / `rpc`. Those stubs wire this codec via [`Default`] (eager
//! messages, [`allocator_api2::alloc::Global`]). Enable the generated crate's
//! `tonic` feature, implement the service trait, then serve or connect with
//! `tonic`.
//!
//! Full walkthrough (generate, depend, implement, call):
//! [puroro README — gRPC stubs (tonic)](https://github.com/wada314/puroro/blob/master/README.md#grpc-stubs-tonic).
//!
//! # Custom allocators
//!
//! Decode uses [`Message::new_in`] with a caller-supplied allocator (or
//! [`Default`] for the usual `Global` path). Generated stubs construct the
//! codec via [`Default`]. Pass [`PuroroCodec::with_alloc`] /
//! [`PuroroDecoder::with_alloc`] when wiring tonic by hand with a custom
//! allocator; stub APIs do not take a per-request allocator yet.
//!
//! [`tonic::codec::Codec`]: https://docs.rs/tonic/latest/tonic/codec/trait.Codec.html
//! [`allocator_api2::alloc::Global`]: https://docs.rs/allocator-api2/latest/allocator_api2/alloc/struct.Global.html

use ::core::clone::Clone;
use ::core::default::Default;
use ::core::marker::PhantomData;
use ::tonic::Status;
use ::tonic::codec::{BufferSettings, Codec, DecodeBuf, Decoder, EncodeBuf, Encoder};

use crate::error::DecodeError;
use crate::message::Message;

/// A [`tonic::codec::Codec`] that implements `application/grpc+proto` via this crate.
///
/// `T` is the outbound message type; `U` is the inbound message type. For a
/// unary RPC those are usually the response and request types respectively on
/// the server side (tonic wires encode/decode that way around the handler).
///
/// Generated `protoc-gen-puroro` stubs use this type through [`Default`]. For
/// the generate → feature → trait → `Server` / `Client` path, see the
/// [gRPC stubs walkthrough](https://github.com/wada314/puroro/blob/master/README.md#grpc-stubs-tonic).
///
/// The decode allocator is cloned for each inbound message. [`Default`]
/// requires `U::Alloc: Default` (typically [`allocator_api2::alloc::Global`]).
///
/// [`tonic::codec::Codec`]: https://docs.rs/tonic/latest/tonic/codec/trait.Codec.html
/// [`allocator_api2::alloc::Global`]: https://docs.rs/allocator-api2/latest/allocator_api2/alloc/struct.Global.html
#[derive(Debug, Clone)]
pub struct PuroroCodec<T, U: Message> {
    decode_alloc: U::Alloc,
    _pd: PhantomData<T>,
}

impl<T, U> PuroroCodec<T, U>
where
    U: Message,
    U::Alloc: Default,
{
    /// Creates a codec that decodes with `U::Alloc::default()`.
    pub fn new() -> Self {
        Self::with_alloc(U::Alloc::default())
    }
}

impl<T, U: Message> PuroroCodec<T, U> {
    /// Creates a codec that decodes inbound messages with `decode_alloc`.
    pub fn with_alloc(decode_alloc: U::Alloc) -> Self {
        Self {
            decode_alloc,
            _pd: PhantomData,
        }
    }
}

impl<T, U> Default for PuroroCodec<T, U>
where
    U: Message,
    U::Alloc: Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, U> PuroroCodec<T, U>
where
    T: Message + Send + 'static,
    U: Message + Send + 'static,
    U::Alloc: Clone + Send,
{
    /// Builds an encoder with explicit buffer settings.
    pub fn raw_encoder(buffer_settings: BufferSettings) -> <Self as Codec>::Encoder {
        PuroroEncoder {
            _pd: PhantomData,
            buffer_settings,
        }
    }

    /// Builds a decoder with explicit buffer settings and a clone of this
    /// codec's decode allocator.
    pub fn raw_decoder(&self, buffer_settings: BufferSettings) -> <Self as Codec>::Decoder {
        PuroroDecoder {
            alloc: self.decode_alloc.clone(),
            buffer_settings,
        }
    }
}

impl<T, U> Codec for PuroroCodec<T, U>
where
    T: Message + Send + 'static,
    U: Message + Send + 'static,
    U::Alloc: Clone + Send + 'static,
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
            alloc: self.decode_alloc.clone(),
            buffer_settings: BufferSettings::default(),
        }
    }
}

/// An [`tonic::codec::Encoder`] that knows how to encode `T`.
///
/// [`tonic::codec::Encoder`]: https://docs.rs/tonic/latest/tonic/codec/trait.Encoder.html
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

/// A [`tonic::codec::Decoder`] that knows how to decode `U`.
///
/// [`tonic::codec::Decoder`]: https://docs.rs/tonic/latest/tonic/codec/trait.Decoder.html
#[derive(Debug, Clone)]
pub struct PuroroDecoder<U: Message> {
    alloc: U::Alloc,
    buffer_settings: BufferSettings,
}

impl<U> PuroroDecoder<U>
where
    U: Message,
    U::Alloc: Default,
{
    /// Creates a decoder with `U::Alloc::default()` and the given buffer settings.
    pub fn new(buffer_settings: BufferSettings) -> Self {
        Self::with_alloc(U::Alloc::default(), buffer_settings)
    }
}

impl<U: Message> PuroroDecoder<U> {
    /// Creates a decoder that builds each message with `alloc`.
    pub fn with_alloc(alloc: U::Alloc, buffer_settings: BufferSettings) -> Self {
        Self {
            alloc,
            buffer_settings,
        }
    }
}

impl<U> Default for PuroroDecoder<U>
where
    U: Message,
    U::Alloc: Default,
{
    fn default() -> Self {
        Self::new(BufferSettings::default())
    }
}

impl<U> Decoder for PuroroDecoder<U>
where
    U: Message,
    U::Alloc: Clone,
{
    type Item = U;
    type Error = Status;

    fn decode(&mut self, buf: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
        let mut msg = U::new_in(self.alloc.clone());
        msg.merge_from(buf).map_err(from_decode_error)?;
        Ok(Some(msg))
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
