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
//! ```rust
//! # #[derive(Default)]
//! # pub struct MyMessage {
//! #     pub my_number: i32,
//! # }
//! # impl ::puroro::Message<MyMessage> for MyMessage {}
//! # impl ::puroro::DeserializableMessageFromBytesIterator for MyMessage {
//! #     fn deser<I>(&mut self, iter: I) -> ::puroro::Result<()>
//! #     where
//! #         I: Iterator<Item = ::std::io::Result<u8>>
//! #     {
//! #         ::puroro::internal::de::from_iter::deser_from_iter(self, iter)
//! #     }
//! # }
//! # impl ::puroro::internal::de::DeserFieldsFromBytesIter for MyMessage {
//! #     fn deser_field<I>(
//! #         &mut self,
//! #         field_number: i32,
//! #         data: ::puroro::types::FieldData<&mut ::puroro::internal::de::from_iter::ScopedIter<I>>,
//! #     ) -> ::puroro::Result<()>
//! #     where
//! #         I: ::std::iter::Iterator<Item = ::std::io::Result<u8>>,
//! #     {
//! #         use ::puroro::internal::impls::simple::de::DeserFieldFromBytesIter;
//! #         match field_number {
//! #             1 => DeserFieldFromBytesIter::<
//! #                 ::puroro::tags::Unlabeled, ::puroro::tags::Int32
//! #             >::deser_field(&mut self.my_number, data),
//! #             _ => panic!(),
//! #         }
//! #     }
//! # }
//! use puroro::Message; // For from_bytes() method
//! use std::io::Read; // For bytes() method
//! let input = vec![0x08, 0x0a];
//! let msg = MyMessage::from_bytes(input.bytes()).unwrap();
//! assert_eq!(10, msg.my_number);
//! ```
//!
//! And serialize it to `std::io::Write`:
//! ```rust
//! # #[derive(Default)]
//! # pub struct MyMessage {
//! #     pub my_number: i32,
//! # }
//! # impl ::puroro::Message<MyMessage> for MyMessage {}
//! # impl ::puroro::SerializableMessageToIoWrite for MyMessage {
//! #     fn ser<W>(&self, out: &mut W) -> ::puroro::Result<()> where W: std::io::Write {
//! #         ::puroro::internal::impls::simple::se::SerFieldToIoWrite::<::puroro::tags::Unlabeled, ::puroro::tags::Int32>::ser_field(
//! #             &self.my_number, 1, out
//! #         )
//! #     }
//! # }
//! use puroro::Message; // For ser() method
//! let mut output = vec![];
//! let mut msg = MyMessage::default();
//! msg.my_number = 10;
//! msg.ser(&mut output).unwrap();
//! assert_eq!(vec![0x08, 0x0a], output);
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
