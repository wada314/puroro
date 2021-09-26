//! A very basic implementation of protobuf message in generated Rust code.
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
