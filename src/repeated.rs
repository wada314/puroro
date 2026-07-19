//! User-facing mutator API for repeated fields.

use ::allocator_api2::alloc::Allocator;
use ::core::ops::DerefMut;

use crate::String;

/// Minimal mutator API for a repeated field container.
///
/// Generated repeated `_mut` accessors may return `impl RepeatedContainerMut`
/// when the element mutator target is a Rust scalar / `Vec` (known without
/// naming catalog types). For `repeated string`, see [`RepeatedStringMut`].
///
/// No `IndexMut`: out-of-range access goes through [`get_mut`](Self::get_mut).
/// [`push`](Self::push) appends a type-default empty element and returns a
/// mutable handle to it (C++ `add_foo()` style).
pub trait RepeatedContainerMut {
    /// Mutable handle for one element (`&mut i32`, …).
    type Mut<'a>: DerefMut
    where
        Self: 'a;

    fn len(&self) -> usize;

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Appends a default / empty element and returns a mutator for it.
    fn push(&mut self) -> Self::Mut<'_>;

    /// Mutable handle for the element at `index`, or `None` if out of range.
    fn get_mut(&mut self, index: usize) -> Option<Self::Mut<'_>>;

    /// Removes all elements (heap payloads are released).
    fn clear(&mut self);

    /// Removes the last element (and releases it). Returns whether one existed.
    fn pop(&mut self) -> bool;
}

/// Mutator API for a `repeated string` field.
///
/// [`Mut`](Self::Mut) is constrained to [`String`] so callers get `push_str`
/// without naming runtime catalog types.
pub trait RepeatedStringMut<A: Allocator> {
    /// Mutable string handle (`DerefMut<Target = String<A>>`).
    type Mut<'a>: DerefMut<Target = String<A>>
    where
        Self: 'a;

    fn len(&self) -> usize;

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn push(&mut self) -> Self::Mut<'_>;

    fn get_mut(&mut self, index: usize) -> Option<Self::Mut<'_>>;

    fn clear(&mut self);

    fn pop(&mut self) -> bool;
}
