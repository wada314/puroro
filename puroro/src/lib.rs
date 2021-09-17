//! Yet another Rust implementation of [Google Protocol Buffer](https://developers.google.com/protocol-buffers).
//!
//! # Quick sample
//!
//! For an input .proto file like this:
//! ```protobuf
//! syntax = "proto3";
//! message MyMessage {
//!     int32 my_number = 1;
//!     repeated string my_name = 2;
//!     MyMessage my_child = 3;
//! }
//! ```
//!
//! A struct like this is output:
//! ```rust
//! # use ::std::borrow::Cow;
//! pub struct MyMessage {
//!     pub my_number: i32,
//!     pub my_name: Vec<Cow<'static, str>>,
//!     pub my_child: Option<Box<MyMessage>>,
//! }
//! ```
//!
//! You can deserialize a struct from `Iterator<std::io::Result<u8>>`:
//! ```no_run
//! use ::puroro::Message; // For from_bytes() method
//! use ::std::io::Read; // For bytes() method
//! # use ::std::borrow::Cow;
//! # pub struct MyMessage {
//! #     pub my_number: i32,
//! #     pub my_name: Vec<Cow<'static, str>>,
//! #     pub my_child: Option<Box<MyMessage>>,
//! # }
//! let input = vec![0x08, 0x0A];
//! let msg = MyMessage::from_bytes(input.bytes());
//! assert_eq!(10, msg.my_number);
//! ```
//!
#![cfg_attr(feature = "puroro-nightly", feature(backtrace))]
#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

pub mod bool;
mod common_traits;
pub mod desc;
mod error;
pub mod fixed_bits;
pub mod internal;
pub mod tags;
pub mod types;
pub mod variant;

pub use self::common_traits::*;
pub use self::error::{ErrorKind, PuroroError};
pub type Result<T> = ::std::result::Result<T, PuroroError>;

// Re-exports
pub use ::either::Either;
pub use ::once_cell;
