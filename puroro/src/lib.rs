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
//!     pub fn new() -> Self {
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
//! let mut book = Book::default();
//! *book.title_mut() = "The C Programming Language".to_string();
//! // The number of pages is unknown so we do not set it.
//!
//! assert_eq!("The C Programming Language", book.title());
//! assert!(!book.has_num_pages());
//! ```
//!
//! # Deserializing
//!
//! You can use [`Message::from_bytes()`] method or [`Message::merge_from_bytes()`]
//! method to deserialize a message from [`std::io::Read`] bytes stream.
//!
//! ```rust
//! use puroro::Message; // For from_bytes(), merge_from_bytes() methods
//! use std::io::Read; // For bytes() method
//! use puroro_doc_samples::library::Book;
//!
//! let input1 = vec![0x10, 0x82, 0x01]; // encoded `num_pages: 130`
//! let input2 = vec![0x0a, 0x02, 0x59, 0x6f]; // encoded `title: "Yo"`
//!
//! // You can use `from_bytes()` method to deserialize a message
//! // from an input buffer.
//! let mut book = Book::from_bytes(input1.bytes()).unwrap();
//! assert_eq!(130, book.num_pages());
//!
//! // Or, you can use `merge_from_bytes(&mut self)` method to deserialize
//! // and merge from an input buffer to an existing message instance.
//! book.merge_from_bytes(input2.bytes()).unwrap();
//! assert_eq!("Yo", book.title());
//! assert_eq!(130, book.num_pages());
//! ```
//!
//! # Serializing
//!
//! You can serialize the message into [`std::io::Write`] using [`Message::ser()`] method.
//!
//! ```rust
//! use puroro::Message; // For ser() method
//! use puroro_doc_samples::library::Book;
//!
//! let mut output = vec![];
//! let mut book = Book::new();
//!
//! *book.title_mut() = "Yo".to_string();
//! book.ser(&mut output).unwrap();
//!
//! assert_eq!(vec![0x0a, 0x02, 0x59, 0x6f], output);
//! ```
//!
//! # Trait and builder
//!
//! The input proto message generates not only a struct, but also a trait.
//! The trait name is *MessageName*Trait (e.g. `BookTrait` for `message Book`).
//! The trait only has immutable methods.
//!
//! ```rust
//! pub trait BookTrait {
//!     fn title(&self) -> &str;
//!     fn has_title(&self) -> bool;
//!     fn title_opt(&self) -> Option<&str>;
//!     fn num_pages(&self) -> u32;
//!     fn has_num_pages(&self) -> bool;
//!     fn num_pages_opt(&self) -> Option<u32>;
//! }
//! ```
//!
//! Of course, the `struct Book` we described at above implements this trait.
//! Please note that for some kind of fields the `struct Book`'s interface and
//! `trait BookTrait`'s interface are slightly different
//! (e.g. `string` fields, or `repeated` fields).
//!
//! The trait is not only implemented for the `struct Book`,
//! but also for some other types:
//!
//! ```rust
//! # trait BookTrait {}
//! // Returns default value for all fields
//! impl BookTrait for () {}
//!
//! // If the value is present then behaves like the internal value.
//! // If the value is not present then returns default value for all fields.
//! impl<T: BookTrait> BookTrait for Option<T> {}
//!
//! // Behaves as as the either of the types present.
//! impl<T: BookTrait, U: BookTrait> BookTrait for puroro::Either<T, U> {}
//!
//! // Behaves like a merged message. Behaves like `U` is merged into `T`.
//! impl<T: BookTrait, U: BookTrait> BookTrait for (T, U) {}
//! ```
//!
//! # Builders
//!
//! puroro also generates a builder type for each message.
//! The name of builder is *MessageName*Builder (e.g. `BookBuilder` for `message Book`).
//!
//! ```rust
//! # use puroro_doc_samples::library::{
//! #     BookTrait, BookSingleField1, BookSingleField2
//! # };
//! pub struct BookBuilder<T>(T);
//!
//! impl BookBuilder<()> {
//!     pub fn new() -> Self {
//!         // ...
//! #       todo!()
//!     }
//! }
//!
//! impl<T> BookBuilder<T>
//! where
//!     T: BookTrait,
//! {
//!     pub fn build(self) -> T {
//!         // ...
//! #       todo!()
//!     }
//!
//!     pub fn append_title<ScalarType>(
//!         self,
//!         value: ScalarType,
//!     ) -> BookBuilder<(T, BookSingleField1<ScalarType>)>
//!     where
//!         ScalarType: ::std::convert::AsRef<str>
//!             + ::std::clone::Clone
//!             + ::std::cmp::PartialEq
//!             + ::std::fmt::Debug,
//!     {
//!         // ...
//! #       todo!()
//!     }
//!
//!     pub fn append_num_pages<ScalarType>(
//!         self,
//!         value: ScalarType,
//!     ) -> BookBuilder<(T, BookSingleField2<ScalarType>)>
//!     where
//!         ScalarType: ::std::convert::Into<u32>
//!             + ::std::clone::Clone
//!             + ::std::cmp::PartialEq
//!             + ::std::fmt::Debug,
//!     {
//!         // ...
//! #       todo!()
//!     }
//! }
//! ```
//!
//! This might look like little complicated, but the usage is very simple:
//!
//! ```rust
//! use puroro_doc_samples::library::{BookBuilder, BookTrait};
//!
//! let book = BookBuilder::new()
//!     .append_title("The C Programming Language")
//!     .build();
//!
//! assert_eq!("The C Programming Language", book.title());
//! assert!(!book.has_num_pages());
//! ```
//!
//! You need to call `new()` method of builder first, then call `append_***()` methods
//! as many as you like, then terminate with `build()` method.
//! Then you can get a generated type which is implementing `BookTrait` trait,
//! and of course it can be serialized.
//!
//! There are some benefits of using this builder instead of the normal `struct Book`:
//! * The builder generated type has lesser memory footprint. It only consumes the memory
//! for explicitly appended fields.
//! * The field type is more flexible. Note that you don't need to call `to_string()` method
//! when setting the string field. Actually, in the example above the internal field type
//! is not `String` but `&str`, which does not allocate any heap memory.
//!
//! Instead, the builder has some downsides compared to the normal struct.
//! * You can only `append` the field. Particurally, you cannot clear field nor
//! edit previously added repeated field values.
//! * You always need to manually write a code to use the builder. No deserialization support.
//!
//! # Using [`bumpalo`](https://github.com/fitzgen/bumpalo) allocator
//!
//! puroro has an experimental implementation using [`bumpalo`](https://github.com/fitzgen/bumpalo)
//! allocator instead of the default global allocator.
//!
//! ```rust
//! pub struct BookBumpalo<'bump> {
//! #   phantom: std::marker::PhantomData<&'bump ()>,
//!     // ...
//! }
//! impl<'bump> BookBumpalo<'bump> {
//!     pub fn new_in(bump: &'bump ::puroro::bumpalo::Bump) -> Self {
//! #       todo!()
//!         // ...
//!     }
//!
//!     pub fn title(&self) -> &'this str {
//! #       todo!()
//!         // ...
//!     }
//!     pub fn title_opt(&self) -> Option<&str> {
//! #       todo!()
//!         // ...
//!     }
//!     pub fn has_title(&self) -> bool {
//! #       todo!()
//!         // ...
//!     }
//!     pub fn title_mut<'this>(
//!         &'this mut self,
//!     ) -> impl 'this + DerefMut<Target = ::puroro::bumpalo::collections::String<'bump>>
//!     {
//! #       todo!()
//!         // ...
//!     }
//!     pub fn clear_title(&mut self) {
//! #       todo!()
//!         // ...
//!     }
//!
//!     pub fn num_pages(&self) -> u32 {
//! #       todo!()
//!         // ...
//!     }
//!     pub fn num_pages_opt(&self) -> Option<u32> {
//! #       todo!()
//!         // ...
//!     }
//!     pub fn has_num_pages(&self) -> bool {
//! #       todo!()
//!         // ...
//!     }
//!     pub fn num_pages_mut(&mut self) -> &mut u32 {
//! #       todo!()
//!         // ...
//!     }
//!     pub fn clear_num_pages(&mut self) {
//! #       todo!()
//!         // ...
//!     }
//! }
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
