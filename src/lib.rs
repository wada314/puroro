//! **puroro** — A performance-oriented Protocol Buffers runtime for Rust.
//!
//! This crate provides the runtime traits and wire-format helpers that
//! protobuf-generated code depends on.  The code generator is a separate tool
//! that emits Rust source using the types and functions defined here.
//!
//! # Crate layout
//!
//! | Module | Contents |
//! |--------|----------|
//! | [`encode`] | [`MessageEncode`] trait + per-field encode helpers |
//! | [`decode`] | [`MessageDecode`] trait + per-field decode helpers |
//! | [`error`] | [`DecodeError`] and [`EncodeError`] types |
//! | [`wire_type`] | [`WireType`] enum (VARINT, I64, LEN, …) |
//!
//! See [`DESIGN.md`](https://github.com/wada314/puroro/blob/cursor/protobuf-interface-design-ea49/DESIGN.md)
//! for the interface specification and
//! [`IMPLEMENTATION.md`](https://github.com/wada314/puroro/blob/cursor/protobuf-interface-design-ea49/IMPLEMENTATION.md)
//! for the generated-code implementation reference.

pub mod decode;
pub mod encode;
pub mod error;
pub mod wire_type;

// Flat re-exports for convenience in generated code.
pub use decode::MessageDecode;
pub use encode::MessageEncode;
pub use error::{DecodeError, EncodeError};
pub use wire_type::WireType;
