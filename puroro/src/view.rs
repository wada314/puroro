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

//! View types for Protocol Buffer message fields.
//!
//! This module provides types for returning flexible views of message fields
//! that support both borrowed references and owned values, with future
//! allocator support in mind.

use ::allocator_api2::boxed::Box;
use ::allocator_extras::{Allocator, Global};

/// A clone-on-write smart pointer for Protocol Buffer message views.
///
/// This is similar to `std::borrow::Cow` but designed specifically for
/// Protocol Buffer message field views with future allocator support in mind.
/// Currently uses `Box<T>` for owned values, but this will be replaced
/// with a generic allocator parameter in the future.
///
/// # Examples
///
/// ```rust
/// # use allocator_api2::boxed::Box;
/// # use allocator_extras::Global;
/// # use puroro::view::ViewCow;
/// // Borrowed reference (zero-cost)
/// let borrowed: ViewCow<'_, ()> = ViewCow::Borrowed(&());
///
/// // Owned value (heap-allocated)
/// let owned: ViewCow<'_, ()> = ViewCow::Owned(Box::new_in((), Global));
/// ```
#[derive(Debug)]
pub enum ViewCow<'a, T: ?Sized + 'a, A: Allocator = Global> {
    /// Borrowed reference to the underlying value.
    Borrowed(&'a T),
    /// Owned value stored on the heap.
    ///
    /// TODO: Replace `Box` with a generic allocator parameter when allocator support is added.
    Owned(Box<T, A>),
}

impl<'a, T: ?Sized + 'a, A: Allocator> ViewCow<'a, T, A> {
    /// Returns a reference to the underlying value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use allocator_extras::Global;
    /// # use puroro::view::ViewCow;
    /// let cow: ViewCow<'_, (), Global> = ViewCow::Borrowed(&());
    /// assert!(std::ptr::eq(cow.as_ref(), &()));
    /// ```
    pub fn as_ref(&self) -> &T {
        match self {
            ViewCow::Borrowed(r) => r,
            ViewCow::Owned(boxed) => boxed.as_ref(),
        }
    }

    /// Converts the `ViewCow` into an owned value, cloning if necessary.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use puroro::view::ViewCow;
    /// # use allocator_api2::boxed::Box;
    /// # use allocator_extras::Global;
    /// let borrowed = ViewCow::Borrowed(&42);
    /// let owned: Box<i32, Global> = borrowed.into_owned_in(Global);
    /// assert_eq!(*owned, 42);
    /// ```
    pub fn into_owned_in(self, alloc: A) -> Box<T, A>
    where
        T: Clone,
    {
        match self {
            ViewCow::Borrowed(r) => Box::new_in(r.clone(), alloc),
            ViewCow::Owned(boxed) => boxed,
        }
    }

    /// Checks if the value is borrowed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use allocator_extras::Global;
    /// # use puroro::view::ViewCow;
    /// # trait Address {}
    /// # impl Address for () {}
    /// let borrowed: ViewCow<'_, (), Global> = ViewCow::Borrowed(&());
    /// assert!(borrowed.is_borrowed());
    /// ```
    pub fn is_borrowed(&self) -> bool {
        matches!(self, ViewCow::Borrowed(_))
    }

    /// Checks if the value is owned.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use allocator_api2::boxed::Box;
    /// # use allocator_extras::Global;
    /// # use puroro::view::ViewCow;
    /// let owned: ViewCow<'_, ()> = ViewCow::Owned(Box::new_in((), Global));
    /// assert!(owned.is_owned());
    /// ```
    pub fn is_owned(&self) -> bool {
        matches!(self, ViewCow::Owned(_))
    }
}

impl<'a, T: ?Sized + 'a, A: Allocator> std::ops::Deref for ViewCow<'a, T, A> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<'a, T, A> Clone for ViewCow<'a, T, A>
where
    T: ?Sized + 'a + Clone,
    A: Allocator + Clone,
{
    fn clone(&self) -> Self {
        match self {
            ViewCow::Borrowed(r) => ViewCow::Borrowed(r),
            ViewCow::Owned(boxed) => ViewCow::Owned(boxed.clone()),
        }
    }
}

impl<'a, T> ViewCow<'a, T, Global>
where
    T: ?Sized + 'a,
{
    /// Convenience wrapper that mirrors the historical API for global allocations.
    pub fn into_owned(self) -> Box<T, Global>
    where
        T: Clone,
    {
        self.into_owned_in(Global)
    }
}
