//! **puroro** — an experimental Protocol Buffers runtime for Rust.
//!
//! This is not an official Google product. Google publishes
//! [`google-protobuf`](https://crates.io/crates/google-protobuf)
//! ([Rust reference](https://protobuf.dev/reference/rust/)).
//! Do not use this crate in most cases. The crates.io / GitHub landing page
//! is the repository [`README`](https://github.com/wada314/puroro/blob/master/README.md).
//!
//! This crate is the public API shared by library users and generated
//! message code:
//!
//! - [`Message`] — encode / decode / merge / validate / unknown fields
//! - [`Optional`] / [`HasDefault`] — explicit-presence singular accessors
//! - [`StringMut`] / [`BytesMut`] — singular `string` / `bytes` mutators
//! - [`MapRef`] / [`MapMut`], [`RepeatedContainerMut`] / [`RepeatedStringMut`] /
//!   [`RepeatedBytesMut`] — map and repeated mutators
//! - [`OneofView`] / [`OneofViewMut`] — oneof group views
//! - [`DecodeError`], [`UnknownField`], [`WireType`]
//!
//! Generated messages expose proto fields as **inherent methods** on the concrete
//! type (`title()`, `title_mut()`, `clear_title()`, …). Explicit-presence getters
//! return [`Optional`]; eager messages do not generate `has_*`. Codec helpers
//! stay on [`Message`] so proto names do not collide; call them via UFCS when
//! needed (`Message::validate(&msg)`).
//!
//! The composable field catalog and wire helpers live in the sibling
//! **`puroro-rt`** crate. Generated code depends on it; application code that
//! only uses generated messages should not need to import `puroro-rt` directly.
//!
//! See [`DESIGN.md`](DESIGN.md) for the interface specification.
//!
//! # Example
//!
//! ```ignore
//! use ::puroro::{Message, StringMut};
//!
//! let mut task = Task::decode(bytes.as_slice())?;
//! task.title_mut().push_str("hello");
//! assert_eq!(task.title().get(), "hello");
//! let out = task.encode_to_vec();
//! # Ok::<(), puroro::DecodeError>(())
//! ```

mod bytes_mut;
pub mod error;
pub mod map;
pub mod message;
pub mod oneof;
pub mod optional;
mod protobuf_error;
pub mod repeated;
pub mod scoped_buf;
mod string_mut;
pub mod unknown;
pub mod wire_type;

/// Allocator-aware string buffer used as element mutator target for `repeated string`.
pub use ::unmanaged::String;
pub use bytes_mut::BytesMut;
pub use error::DecodeError;
pub use map::{MapMut, MapRef};
pub use message::{Message, RECURSION_LIMIT};
pub use oneof::{OneofView, OneofViewMut};
pub use optional::{HasDefault, Optional};
pub use repeated::{RepeatedBytesMut, RepeatedContainerMut, RepeatedStringMut};
pub use scoped_buf::{DecodeBuf, LimitGuard, ScopedBuf};
pub use string_mut::StringMut;
pub use unknown::{UnknownField, UnknownPayload};
pub use wire_type::WireType;
