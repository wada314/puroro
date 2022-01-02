// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Yet another Rust implementation of [Google Protocol Buffer](https://developers.google.com/protocol-buffers).
//!
//! __Warning! The interface is still experimental and it will change very frequently!!__
//!
//! # How to compile your .pb files to .rs files
//!
//! Please check [the `readme` of this repository](https://github.com/wada314/puroro#readme)
//!
//! # Simple example
//!
//! For an input .proto file like this:
//! ```protobuf
//! syntax = "proto3";
//! package library;
//! message Book {
//!     string title = 1;
//!     uint32 num_pages = 2;
//! }
//! ```
//!
//! A struct like this is output:
//! ```rust
//! #[derive(Clone, Default, PartialEq)]
//! pub struct MyMessage { /* ... */ }
//! impl MyMessage {
//!     pub fn new() {
//!         // ...
//! #       todo!()
//!     }
//!
//!     pub fn title(&self) -> &str {
//!         // ...
//! #       todo!()
//!     }
//!     pub fn title_opt(&self) -> Option<&str> {
//!         // ...
//! #       todo!()
//!     }
//!     pub fn has_title(&self) -> bool {
//!         // ...
//! #       todo!()
//!     }
//!     pub fn title_mut(&mut self) -> &mut String {
//!         // ...
//! #       todo!()
//!     }
//!     pub fn clear_title(&mut self) {
//!         // ...
//! #       todo!()
//!     }
//!
//!     pub fn num_pages(&self) -> u32 {
//!         // ...
//! #       todo!()
//!     }
//!     pub fn num_pages_opt(&self) -> Option<u32> {
//!         // ...
//! #       todo!()
//!     }
//!     pub fn has_num_pages(&self) -> bool {
//!         // ...
//! #       todo!()
//!     }
//!     pub fn num_pages_mut(&mut self) -> &mut u32 {
//!         // ...
//! #       todo!()
//!     }
//!     pub fn clear_num_pages(&mut self) {
//!         // ...
//! #       todo!()
//!     }
//! }
//! ```
//!
//! Let's assume the generated code is in `puroro-doc-samples` crate,
//! then you can use the generated protobuf like this:
//!
//! ```rust
//! use ::puroro_doc_samples::library::Book;
//!
//! let mut k_and_r = Book::default();
//! *k_and_r.title_mut() = "The C Programming Language".to_string();
//! // The number of pages is unknown so we do not set it.
//!
//! assert_eq!("The C Programming Language", k_and_r.title());
//! assert!(!k_and_r.has_num_pages());
//! ```
//!
//! # Deserializing
//!
//! ```rust
//! use puroro::Message; // For from_bytes() method
//! use std::io::Read; // For bytes() method
//! use puroro_doc_samples::library::Book;
//!
//! let input1 = vec![0x10, 0x82, 0x01]; // encoded `num_pages: 130`
//! let input2 = vec![0x0a, 0x02, 0x59, 0x6f]; // encoded `title: "Yo"`
//!
//! // You can use `from_bytes()` method to deserialize a message
//! // from an input buffer.
//! let mut msg = Book::from_bytes(input1.bytes()).unwrap();
//! assert_eq!(130, msg.num_pages());
//!
//! // Or, you can use `merge_from_bytes(&mut self)` method to deserialize
//! // and merge from an input buffer to an existing message instance.
//! msg.merge_from_bytes(input2.bytes()).unwrap();
//! assert_eq!("Yo", msg.title());
//! assert_eq!(130, msg.num_pages());
//! ```
//!
//! And serialize it to `std::io::Write`:
//! ```ignore
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
//! # Trait impls
//! ([Detailed doc](internal::impls::traits))
//!
//! The trait `MyMessageTrait` is not only implemented for the struct `MyMessage`,
//! but for few other items:
//!
//! ```rust
//! # trait MyMessageTrait {}
//! impl MyMessageTrait for () { /* ... */ }
//! impl<'a, T: MyMessageTrait> MyMessageTrait for &'a T { /* ... */ }
//! impl<'a, T: MyMessageTrait> MyMessageTrait for &'a mut T { /* ... */ }
//! impl<T: MyMessageTrait> MyMessageTrait for Box<T> { /* ... */ }
//! impl<T: MyMessageTrait> MyMessageTrait for Option<T> { /* ... */ }
//! impl<T: MyMessageTrait, U: MyMessageTrait> MyMessageTrait for (T, U) { /* ... */ }
//! impl<T: MyMessageTrait, U: MyMessageTrait> MyMessageTrait for puroro::Either<T, U> { /* ... */ }
//! ```
//!
//! Notably, `(T, U)` works as a merged message.
//!
//! # Builder struct
//! ([Detailed doc](internal::impls::builder))
//!
//! puroro also generates a message builder.
//! A message generated by this builder has an advantage against
//! the normal message struct in memory size: It only consumes
//! the memory for the explicitly set fields.
//!  
//! ```rust
//! # use ::std::ops::Deref;
//! # trait MyMessageTrait {}
//! pub struct MyMessageBuilder<T>(T);
//! impl MyMessageBuilder<()> {
//!     pub fn new() -> Self {
//! #       todo!()
//!         /* ... */
//!     }
//! }
//! impl<T: MyMessageTrait> MyMessageBuilder<T> {
//!     pub fn append_my_number<U>(self, value: U)
//!     -> MyMessageBuilder<(T, MyMessageSingleField1<U>)>
//!     where
//!         U: Into<i32>,
//!     {
//! #       todo!()
//!         /* ... */
//!     }
//!     pub fn append_my_name<U, V>(self, value: U)
//!     -> MyMessageBuilder<(T, MyMessageSingleField2<U, V>)>
//!     where
//!         for<'a> &'a U: IntoIterator<Item=&'a V>,
//!         V: AsRef<str>,
//!     {
//! #       todo!()
//!         /* ... */
//!     }
//!     pub fn append_my_child<U: MyMessageTrait>(self, value: U)
//!     -> MyMessageBuilder<(T, MyMessageSingleField3<U>)> {
//! #       todo!()
//!         /* ... */
//!     }
//! }
//! # pub struct MyMessageSingleField1<T>(T);
//! # pub struct MyMessageSingleField2<T, U>(T, U);
//! # pub struct MyMessageSingleField3<T>(T);
//! ```
//!
//! Usage:
//!
//! ```ignored
//! let my_message = MyMessageBuilder::new()
//!     .append_my_number(10)
//!     .append_my_name(vec!["foo", "bar"])
//!     .build();
//! assert_eq!(10, my_message.my_number());
//! ```
//!
#![allow(incomplete_features)]
#![feature(backtrace)]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]
mod common_traits;
mod error;
pub mod internal;
pub mod repeated_field;
pub mod tags;

pub use self::common_traits::*;
pub use self::error::{ErrorKind, PuroroError};
pub use self::repeated_field::{AsRefRepeatedField, CloneThenIntoRepeatedField, RepeatedField};
pub type Result<T> = ::std::result::Result<T, PuroroError>;

// Re-exports
pub use ::bitvec;
#[cfg(feature = "puroro-bumpalo")]
pub use ::bumpalo;
pub use ::either::Either;

use ::std::ops::{Deref, DerefMut};

// Bumpalo wrapper
pub struct BumpaloOwned<T> {
    // The field order matters, `Drop` drops the field in decl order.
    t: T,
    bump: Box<crate::bumpalo::Bump>,
}
impl<T> BumpaloOwned<T> {
    pub fn bump(this: &BumpaloOwned<T>) -> &crate::bumpalo::Bump {
        &this.bump
    }
    pub fn inner(this: &BumpaloOwned<T>) -> &T {
        &this.t
    }
    pub fn inner_mut(this: &mut BumpaloOwned<T>) -> &mut T {
        &mut this.t
    }
}
impl<T> BumpaloOwned<T>
where
    T: crate::internal::BumpDefault<'static>,
{
    pub fn new() -> Self {
        let bump = Box::new(crate::bumpalo::Bump::new());
        let t = crate::internal::BumpDefault::default_in(unsafe {
            ::std::mem::transmute(bump.as_ref())
        });
        Self { t, bump }
    }
}
impl<T> Default for BumpaloOwned<T>
where
    T: crate::internal::BumpDefault<'static>,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> Deref for BumpaloOwned<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.t
    }
}
impl<T> DerefMut for BumpaloOwned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.t
    }
}

impl<M, T> Message<M> for BumpaloOwned<T> where T: Message<M> {}
