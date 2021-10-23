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

//! A very basic implementation of protobuf message in generated Rust code.
//!
//! # Basic
//!
//! puroro generates a plain struct which has public fields.
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
//! Here's the list of the field types correspondence between protobuf and Rust code:
//!
//! | base protobuf type | `required` / `optional` | (unlabeled) | `repeated` |
//! |--------------------|-------------------------|-------------|------------|
//! | `int32`            | `Option<i32>`           | `i32`       | `Vec<i32>` |
//! | (Any numeric types)| `Option<T>`             | `T`         | `Vec<T>`   |
//! | `bytes`            | `Option<Vec<u8>>`       | `Vec<u8>`   | `Vec<Vec<u8>>`|
//! | `string`           | `Option<String>`        | `String`    | `Vec<String>`|
//! | `SomeMessage`      | `Option<Box<SomeMessage>>`|`Option<Box<SomeMessage>>`|`Vec<SomeMessage>`|
//!
//! ## Enums
//!
//! ### proto2
//!
//! ```protobuf
//! syntax = "proto2";
//! enum MyEnum {
//!     ZERO = 0;
//!     TWENTY_ONE = 21;
//! }
//! ```
//!
//! Generates an enum like this:
//!
//! ```ignore
//! enum MyEnum {
//!     Zero,
//!     TwentyOne,
//! }
//! impl puroro::Enum2 for MyEnum { /* ... */ }
//! ```
//!
//! Please note that the integral values (`0`, `21`) are not directly
//! assigned to the Rust enum values.
//! Though the trait [`Enum2`](crate::Enum2) has supertraits
//! [`TryFrom<i32, Error=i32>`](`std::convert::TryFrom`) and [`Into<i32>`].
//! So you can convert the enum value from / to i32 as follows:
//!
//! ```rust
//! # #[derive(Debug, PartialEq)]
//! # enum MyEnum {
//! #     Zero,
//! #     TwentyOne,
//! # }
//! # impl std::convert::TryFrom<i32> for MyEnum {
//! #     type Error = i32;
//! #     fn try_from(val: i32) -> Result<Self, Self::Error> {
//! #         match val.clone() {
//! #             0 => Ok(MyEnum::Zero),
//! #             21 => Ok(MyEnum::TwentyOne),
//! #             _ => Err(val),
//! #         }
//! #     }
//! # }
//! # impl Into<i32> for MyEnum {
//! #     fn into(self) -> i32 {
//! #         match self {
//! #             MyEnum::Zero => 0,
//! #             MyEnum::TwentyOne => 21,
//! #         }
//! #     }
//! # }
//! # use std::convert::TryFrom;
//! assert_eq!(MyEnum::TwentyOne, MyEnum::try_from(21).unwrap());
//! assert_eq!(21, MyEnum::TwentyOne.into());
//! ```
//!
//! [`Enum2`](crate::Enum2) also has supertrait [`Default`], where
//! the default value is the first value in the enum definition
//! ([official document](https://developers.google.com/protocol-buffers/docs/proto#optional)).
//!
//! ### proto3
//!
//! ```protobuf
//! syntax = "proto3";
//! enum MyEnum {
//!     ZERO = 0;
//!     TWENTY_ONE = 21;
//! }
//! ```
//!
//! Generates an enum like this:
//!
//! ```ignore
//! enum MyEnum {
//!     Zero,
//!     TwentyOne,
//!     _Unknown(i32),
//! }
//! impl puroro::Enum3 for MyEnum { /* ... */ }
//! ```
//!
//! Unlike proto2 enum, proto3 enum contains `_Unknown(i32)` field so it
//! implements [`From<i32>`] instead of proto2's [`TryFrom<i32>`](std::convert::TryFrom).
//! [proto3 spec](https://developers.google.com/protocol-buffers/docs/proto3#enum)
//! forces the first value in the definition is 0, so [`Default`] implementation
//! always returns that value corresponding to 0.
//!
//! # Nested message / enum
//!
//! ```protobuf
//! syntax = "proto3";
//! package my_package;
//! message MyMessage {
//!     message MySubMessage {
//!     }
//!     MySubMessage my_child = 1;
//! }
//! ```
//!
//! If a message or an enum is nested into Message, then that message / enum
//! are nested into a submodule.
//! The outer message name is converted into ***lower_snake_case*** and be used
//! as the submodule name.
//!
//! ```rust
//! pub mod my_package {
//!     pub struct MyMessage {
//!         my_child: Option<Box<my_message::MySubMessage>>,
//!     }
//!     pub mod my_message {
//!         pub struct MySubMessage {}
//!     }
//! }
//! ```
//!
//! If you have a same name proto subpackage with the outer message,
//! then the module generated from subpackage shadowes the module
//! generated from the outer message. If you want to access the
//! nested items' module, then please add `_puroro_nested` into the
//! path like this:
//!
//! ```rust
//! # pub mod my_package {
//! #     pub mod _puroro_nested {
//! #         pub mod my_message {
//! #             #[derive(Default)]
//! #             pub struct MySubMessage {}
//! #         }
//! #     }
//! # }
//! let nested_child = my_package::_puroro_nested::my_message::MySubMessage::default();
//! ```
//!
//! ## oneofs
//!
//! From a proto like this:
//! ```protobuf
//! syntax = "proto3";
//! message MyMessage {
//!     oneof my_oneofs {
//!         int32 item1 = 1;
//!         float item2 = 2;
//!     }
//! }
//! ```
//!
//! This struct and enum are generated:
//!
//! ```rust
//! pub struct MyMessage {
//!     pub my_oneofs: Option<my_message::MyOneofs>,
//! }
//! pub mod my_message {
//!     pub enum MyOneofs {
//!         Item1(i32),
//!         Item2(f32),
//!     }
//! }
//! ```
//!
//! # Default values
//!
//! ## proto2
//!
//! In proto2, you can set a field's default value:
//! ```protobuf
//! syntax = "proto2";
//! message MyMessage {
//!     optional int32 my_number = 1 [default = 42];
//! }
//! ```
//!
//! For the generated struct, the value is initialized by the default value:
//!
//! ```rust
//! # pub struct MyMessage {
//! #     pub my_number: i32,
//! # }
//! # impl Default for MyMessage {
//! #     fn default() -> Self {
//! #         Self { my_number: 42 }
//! #     }
//! # }
//! let mut msg = MyMessage::default();
//! assert_eq!(42, msg.my_number);
//! ```
//!
//! ## proto3
//!
//! Because proto3 does not allow to set the default value explicitly,
//! the field's default values are always the same as default `std::default::Default` impl.
//!

pub mod de;
pub mod se;

use ::std::borrow::Borrow;
use ::std::marker::PhantomData;

pub struct BorrowedIter<B: ?Sized, I>(I, PhantomData<B>);
impl<B: ?Sized, I> BorrowedIter<B, I> {
    pub fn new(iter: I) -> Self {
        Self(iter, PhantomData)
    }
}
impl<'a, B, I, T> Iterator for BorrowedIter<B, I>
where
    I: Iterator<Item = &'a T>,
    T: 'a + Borrow<B>,
    B: 'a + ?Sized,
{
    type Item = &'a B;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|owned| owned.borrow())
    }
}

pub trait VecOrOptionOrBare<T> {
    fn push(&mut self, val: T);
    fn get_or_insert_with<F>(&mut self, f: F) -> &mut T
    where
        F: FnOnce() -> T;
    type Iter<'a>: Iterator<Item = &'a T>
    where
        T: 'a;
    fn iter(&self) -> Self::Iter<'_>;
}
impl<T> VecOrOptionOrBare<T> for Option<T> {
    fn push(&mut self, val: T) {
        *self = Some(val);
    }
    fn get_or_insert_with<F>(&mut self, f: F) -> &mut T
    where
        F: FnOnce() -> T,
    {
        <Option<T>>::get_or_insert_with(self, f)
    }
    type Iter<'a>
    where
        T: 'a,
    = ::std::option::Iter<'a, T>;
    fn iter(&self) -> Self::Iter<'_> {
        Option::iter(self)
    }
}
impl<T> VecOrOptionOrBare<T> for Vec<T> {
    fn push(&mut self, val: T) {
        self.push(val);
    }
    fn get_or_insert_with<F>(&mut self, f: F) -> &mut T
    where
        F: FnOnce() -> T,
    {
        <Vec<T>>::push(self, (f)());
        self.last_mut().unwrap()
    }
    type Iter<'a>
    where
        T: 'a,
    = ::std::slice::Iter<'a, T>;
    fn iter(&self) -> <Self as VecOrOptionOrBare<T>>::Iter<'_> {
        <[T]>::iter(self)
    }
}
impl<T> VecOrOptionOrBare<T> for T {
    fn push(&mut self, val: T) {
        *self = val;
    }
    fn get_or_insert_with<F>(&mut self, _: F) -> &mut T
    where
        F: FnOnce() -> T,
    {
        self
    }
    type Iter<'a>
    where
        T: 'a,
    = ::std::iter::Once<&'a T>;
    fn iter(&self) -> Self::Iter<'_> {
        ::std::iter::once(self)
    }
}
