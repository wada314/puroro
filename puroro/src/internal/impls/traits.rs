//! # Generated trait
//! For every message in input .pb files, puroro generates a Trait named
//! `<MyMessageName>Trait`.
//!
//!
//! For an input protobuf:
//! ```protobuf
//! syntax = "proto3";
//! message MyMessage {
//!     int32 my_number = 1;
//!     repeated string my_name = 2;
//!     MyMessage my_child = 3;
//! }
//! ```
//!
//! Trait like this is generated:
//!
//! ```rust
//! // A readonly trait for message `MyMessage`
//! # #![feature(generic_associated_types)]
//! # use ::std::fmt::Debug;
//! # use ::std::ops::Deref;
//! pub trait MyMessageTrait {
//!     fn my_number(&self) -> i32;
//!     type Field2StringType<'this>: Deref<Target=str> + Clone + PartialEq + Debug where Self: 'this;
//!     type Field2RepeatedType<'this>: IntoIterator<Item=Self::Field2StringType<'this>> where Self: 'this;
//!     fn my_name(&self) -> Self::Field2RepeatedType<'_>;
//!     type Field3MessageType<'this>: MyMessageTrait + Clone + PartialEq + Debug where Self: 'this;
//!     fn my_child(&self) -> Option<Self::Field3MessageType<'_>>;
//! }
//! ```
//!
//! Each message field will have a single getter method which has same name
//! (lower_snake_case_nized if it's not) with the original protobuf fields.
//! If the field type is oneof string, bytes or a message, then an associated type
//! `Field[number][String|Bytes|Message]Type` is generated.
//! This associated type implements:
//! - [`Deref<Target=[u8]>`](std::ops::Deref) for Bytes
//! - [`Deref<Target=str>`](std::ops::Deref) for String
//! - ***`SomeMessageTrait`*** for a Message.
//!
//! And if the field is a repeated type, another associated type
//! `Field[number]RepeatedType` is generated.
//! This type implements [`puroro::RepeatedField`](crate::RepeatedField), which is the same
//! as the [`IntoIterator`].
//!
//! The list of the generated methods' return types:
//!
//! | base protobuf type | `required` / `optional` / `oneof` field | (unlabeled) | `repeated` |
//! |--------------------|-------------------------|-------------|------------|
//! | `int32`            | `Option<i32>`           | `i32`       | `impl IntoIterator<Item=i32>`|
//! | (Any numeric types)| `Option<T>`             | `T`         | `impl IntoIterator<Item=T>`|
//! | `bytes`            | `Option<impl Deref<Target=[u8]>>`|`impl Deref<Target=[u8]>`|`impl IntoIterator<Item=impl Deref<Target=[u8]>>`|
//! | `string`           | `Option<impl Deref<Target=str>>`|`impl Deref<Target=str>`|`impl IntoIterator<Item=impl Deref<Target=str>>`|
//! | `SomeMessage`      | `Option<impl SomeMessage>`|`Option<impl SomeMessage>`|`impl IntoIterator<Item=impl SomeMessage>`|
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
//! This trait and enum are generated:
//!
//! ```rust
//! pub trait MyMessageTrait {
//!     fn my_oneofs(&self) -> Option<my_message::MyOneofs>;
//! }
//! pub mod my_message {
//!     pub enum MyOneofs {
//!         Item1(i32),
//!         Item2(f32),
//!     }
//! }
//! ```
//!
//! # trait impls
//!
//! The generated trait is implemented for the generated message structs and
//! the following types:
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
//! ### `&'a T`, `&'a mut T`, `Box<T>`
//! Behaves as same as `T`.
//!
//! ### `()`
//! Always returns default values in every methods.
//!
//! ### `Option<T>`
//! If the value is `Some`, then behaves as same as `T`.
//! If the value is `None`, then behaves as same as `()`.
//!
//! ### `puroro::Either<T, U>`
//! Behaves as either `T` or `U`.
//!
//! ### `(T, U)`
//! Behaves as a merged message of `T` and `U`.
//! - Non-repeated, non-message field: Prioritize `U`'s value.
//! - Non-repeated, message field: Merges `T`'s and `U`'s values.
//! - Repeated field: Concatenates `T` and `U`'s repaeted values.
//!
