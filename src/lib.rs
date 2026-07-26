//! **puroro** — A performance-oriented Protocol Buffers runtime for Rust.
//!
//! This crate provides the stable public API that library users and generated
//! message code share: the [`Message`] trait (codec + shared helpers),
//! explicit-presence accessors, unknown-field views, and error types.  The
//! composable field catalog and wire helpers live in the sibling
//! **`puroro-rt`** crate, which generated code depends on (library users of
//! generated messages should not need to import `puroro-rt` directly).
//!
//! See [`DESIGN.md`](DESIGN.md) for the interface specification.

pub mod error;
pub mod map;
pub mod message;
pub mod oneof;
pub mod optional;
mod protobuf_error;
pub mod repeated;
pub mod unknown;
pub mod wire_type;

/// Allocator-aware string buffer used as `Target` of generated string `_mut` accessors.
pub use ::unmanaged::String;
pub use error::{DecodeError, EncodeError};
pub use map::{MapEntryMut, MapRef};
pub use message::{Message, RECURSION_LIMIT};
pub use oneof::{OneofView, OneofViewMut};
pub use optional::{HasDefault, Optional};
pub use repeated::{RepeatedBytesMut, RepeatedContainerMut, RepeatedStringMut};
pub use unknown::{UnknownField, UnknownPayload};
pub use wire_type::WireType;
