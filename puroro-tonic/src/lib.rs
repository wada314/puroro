//! [`tonic`] codec for [`puroro`] messages.
//!
//! Bridges [`puroro::Message`] to [`tonic::codec::Codec`]. Transport, framing,
//! and service codegen stay outside this crate.
//!
//! # Example
//!
//! ```rust,ignore
//! use puroro_tonic::PuroroCodec;
//!
//! let codec = PuroroCodec::<Request, Response>::default();
//! ```

#![warn(missing_docs, missing_debug_implementations, unreachable_pub)]

mod codec;

pub use codec::{PuroroCodec, PuroroDecoder, PuroroEncoder};
