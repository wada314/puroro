//! View types for Protocol Buffer message fields.
//!
//! This module provides types for returning flexible views of message fields
//! that support both borrowed references and owned values, with future
//! allocator support in mind.

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
/// # use puroro::view::ViewCow;
/// # trait Address {}
/// # impl Address for () {}
/// // Borrowed reference (zero-cost)
/// let borrowed: ViewCow<'_, dyn Address> = ViewCow::Borrowed(&());
///
/// // Owned value (heap-allocated)
/// let owned: ViewCow<'_, dyn Address> = ViewCow::Owned(Box::new(()));
/// ```
#[derive(Debug, Clone)]
pub enum ViewCow<'a, T: ?Sized + 'a> {
    /// Borrowed reference to the underlying value.
    Borrowed(&'a T),
    /// Owned value stored on the heap.
    ///
    /// TODO: Replace `Box` with a generic allocator parameter when allocator support is added.
    Owned(Box<T>),
}

impl<'a, T: ?Sized + 'a> ViewCow<'a, T> {
    /// Returns a reference to the underlying value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use puroro::view::ViewCow;
    /// # trait Address {}
    /// # impl Address for () {}
    /// let cow = ViewCow::Borrowed(&());
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
    /// let borrowed = ViewCow::Borrowed(&42);
    /// let owned: Box<i32> = borrowed.into_owned();
    /// assert_eq!(*owned, 42);
    /// ```
    pub fn into_owned(self) -> Box<T>
    where
        T: Clone,
    {
        match self {
            ViewCow::Borrowed(r) => Box::new(r.clone()),
            ViewCow::Owned(boxed) => boxed,
        }
    }

    /// Checks if the value is borrowed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use puroro::view::ViewCow;
    /// # trait Address {}
    /// # impl Address for () {}
    /// let borrowed = ViewCow::Borrowed(&());
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
    /// # use puroro::view::ViewCow;
    /// # trait Address {}
    /// # impl Address for () {}
    /// let owned = ViewCow::Owned(Box::new(()));
    /// assert!(owned.is_owned());
    /// ```
    pub fn is_owned(&self) -> bool {
        matches!(self, ViewCow::Owned(_))
    }
}

impl<'a, T: ?Sized + 'a> std::ops::Deref for ViewCow<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}
