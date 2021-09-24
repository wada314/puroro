//! Yet another Rust implementation of [Google Protocol Buffer](https://developers.google.com/protocol-buffers).
//!
//! # Generated structs
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
//! pub struct MyMessage {
//!     pub my_number: i32,
//!     pub my_name: Vec<String>,
//!     pub my_child: Option<Box<MyMessage>>,
//! }
//! ```
//!
//! You can deserialize a struct from `Iterator<std::io::Result<u8>>`
//! (which is a return type of `std::io::Read::bytes()` method):
//! ```rust
//! # #[derive(Default)]
//! # pub struct MyMessage {
//! #     pub my_number: i32,
//! # }
//! # use ::puroro::*;
//! # impl Message<MyMessage> for MyMessage {}
//! # impl internal::DeserializableMessageFromBytesIterator for MyMessage {
//! #     fn deser<I>(&mut self, iter: I) -> Result<()>
//! #     where
//! #         I: Iterator<Item = ::std::io::Result<u8>>
//! #     {
//! #         internal::de::from_iter::deser_from_iter(self, iter)
//! #     }
//! # }
//! # impl internal::de::DeserFieldsFromBytesIter for MyMessage {
//! #     fn deser_field<I>(
//! #         &mut self,
//! #         field_number: i32,
//! #         data: types::FieldData<&mut internal::de::from_iter::ScopedIter<I>>,
//! #     ) -> Result<()>
//! #     where
//! #         I: Iterator<Item = ::std::io::Result<u8>>,
//! #     {
//! #         use internal::impls::simple::de::DeserFieldFromBytesIter;
//! #         match field_number {
//! #             1 => DeserFieldFromBytesIter::<
//! #                 tags::Unlabeled, tags::Int32
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
//! # use ::puroro::{internal, Result, tags};
//! # impl Message<MyMessage> for MyMessage {}
//! # impl ::puroro::internal::SerializableMessageToIoWrite for MyMessage {
//! #     fn ser<W>(&self, out: &mut W) -> Result<()> where W: std::io::Write {
//! #         internal::impls::simple::se::SerFieldToIoWrite::<tags::Unlabeled, tags::Int32>::ser_field(
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
//! # Generated traits
//!
//! puroro generates not only the `struct MyMessage { ... }` but few other structs and traits.
//! The most important one is `trait MyMessageTrait`.
//! For the same input as above:
//! ```protobuf
//! syntax = "proto3";
//! message MyMessage {
//!     int32 my_number = 1;
//!     repeated string my_name = 2;
//!     MyMessage my_child = 3;
//! }
//! ```
//!
//! Trait like this is generated
//! (Omitting some bounds for explanation. Please check the TBD page for detail):
//!
//! ```ignore
//! // A readonly trait for message `MyMessage`
//! pub trait MyMessageTrait {
//!     fn my_number(&self) -> i32;
//!     type Field2StringType<'this>: Deref<Target=str>;
//!     type Field2RepeatedType<'this>: IntoIterator<Item=Self::Field2StringType<'this>>;
//!     fn my_name(&self) -> Self::Field2RepeatedType<'_>;
//!     type Field3MessageType<'this>: MyMessageTrait;
//!     fn my_child(&self) -> Option<Self::Field3MessageType<'_>>;
//! }
//! ```
//!
//! This may look like little complicated but actually it's not so much.
//! Just remember that there are the methods which have same name with the
//! `struct MyMessage`'s fields, and those returns a readonly value
//! (sorry, mutable trait interface is still TBD!).
//!
//! This itself is not very interesting, it's just a limited interface of the `struct MyMessage`.
//! But it's not only `struct MyMessage` which is implementing this trait:
//!
//! ```ignore
//! impl MyMessageTrait for () { /* ... */ }
//! impl<'a, T: MyMessageTrait> MyMessageTrait for &'a T { /* ... */ }
//! impl<'a, T: MyMessageTrait> MyMessageTrait for &'a mut T { /* ... */ }
//! impl<T: MyMessageTrait> MyMessageTrait for Box<T> { /* ... */ }
//! impl<T: MyMessageTrait> MyMessageTrait for Option<T> { /* ... */ }
//! impl<T: MyMessageTrait, U: MyMessageTrait> MyMessageTrait for (T, U) { /* ... */ }
//! impl<T: MyMessageTrait, U: MyMessageTrait> MyMessageTrait for puroro::Either<T, U> { /* ... */ }
//! ```
//!
//! Because protocol buffer well defines the default behavior, we can implement
//! `MyMessageTrait` even for `()` (it returns default values for the fields getter methods).
//! And for `(T, U)`, it behaves like a merged message of `T` and `U`!
//!
//! # Builder and SingleField structs
//!
//! ```ignore
//! pub struct MyMessageBuilder<T>(T);
//! impl MyMessageBuilder<()> {
//!     pub fn new() -> Self { ... }
//! }
//! impl<T: MyMessageTrait> MyMessageBuilder<T> {
//!     pub fn append_my_number(self, value: i32) -> MyMessageBuilder<(T, MyMessageSingleField1)>;
//!     pub fn append_my_name<U: Deref<Target=str>>(self, value: Vec<U>) -> MyMessageBuilder<(T, MyMessageSingleField2<U>)>;
//!     pub fn append_my_child<U: MyMessageTrait>(self, value: U) -> MyMessageBuilder<(T, MyMessageSingleField3<U>)>;
//! }
//!
//! pub struct MyMessageSingleField1 {
//!     pub my_number: i32,
//! }
//! pub struct MyMessageSingleField2<T: Deref<Target=str>> {
//!     pub my_name: Vec<T>,
//! }
//! pub struct MyMessageSingleField3<T: MyMessageTrait> {
//!     pub my_child: T,
//! }
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
