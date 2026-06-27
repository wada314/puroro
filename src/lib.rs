//! **puroro** — A performance-oriented Protocol Buffers runtime for Rust.
//!
//! This crate provides the runtime traits and wire-format helpers that
//! protobuf-generated code depends on. It is intentionally kept small: the
//! code generator is a separate tool that emits Rust source using the types
//! and functions defined here.
//!
//! # Crate layout
//!
//! | Module | Contents |
//! |--------|----------|
//! | [`encode`] | [`MessageEncode`] trait + per-field encode helpers |
//! | [`decode`] | [`MessageDecode`] trait + per-field decode helpers |
//! | [`error`] | [`DecodeError`] and [`EncodeError`] types |
//! | [`wire_type`] | [`WireType`] enum (VARINT, I64, LEN, …) |
//! | [`sample`] | Hand-written example of what generated code looks like |
//!
//! # Quick example (using generated code)
//!
//! ```rust
//! use puroro::{MessageEncode as _, MessageDecode as _};
//! use puroro::sample::example::Person;
//!
//! let mut p = Person::new();
//! p.set_name("Alice");
//! p.set_age(30);
//! p.push_email("alice@example.com");
//!
//! let bytes = p.encode_to_vec();
//!
//! // `Person` is shorthand for `Person<Global>`. An explicit type is needed
//! // because the allocator cannot be inferred from the decode call alone.
//! let p2: Person = Person::decode(bytes::Bytes::from(bytes)).unwrap();
//! assert_eq!(p2.name(), "Alice");
//! assert_eq!(p2.age(), 30);
//! assert_eq!(p2.emails().len(), 1);
//! ```

pub mod decode;
pub mod encode;
pub mod error;
pub mod sample;
pub mod wire_type;

// Flat re-exports for convenience in generated code.
pub use decode::MessageDecode;
pub use encode::MessageEncode;
pub use error::{DecodeError, EncodeError};
pub use wire_type::WireType;
