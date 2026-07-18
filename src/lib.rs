//! **puroro** — A performance-oriented Protocol Buffers runtime for Rust.
//!
//! This crate provides the stable public API that library users and generated
//! message code share: the [`Message`] trait (codec + shared helpers),
//! explicit-presence accessors, unknown-field views, and error types.  The
//! composable field catalog and wire helpers live in the sibling
//! **`puroro-rt`** crate, which generated code depends on transitively.
//!
//! See [`DESIGN.md`](DESIGN.md) for the interface specification.

pub mod error;
pub mod message;
pub mod optional;
mod protobuf_error;
pub mod unknown;
pub mod wire_type;

pub use error::{DecodeError, EncodeError};
pub use message::{Message, RECURSION_LIMIT};
pub use optional::{HasDefault, Optional};
pub use unknown::{UnknownField, UnknownPayload};
pub use wire_type::WireType;
