// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// This file is modified from the original version by puroro prject:
// https://github.com/wada314/puroro

//! A pointer type for heap allocation.
//!
//! [`Box<T>`], casually referred to as a 'box', provides the simplest form of
//! heap allocation in Rust. Boxes provide ownership for this allocation, and
//! drop their contents when they go out of scope. Boxes also ensure that they
//! never allocate more than `isize::MAX` bytes.
//!
//! # Examples
//!
//! Move a value from the stack to the heap by creating a [`Box`]:
//!
//! ```
//! let val: u8 = 5;
//! let boxed: Box<u8> = Box::new(val);
//! ```
//!
//! Move a value from a [`Box`] back to the stack by [dereferencing]:
//!
//! ```
//! let boxed: Box<u8> = Box::new(5);
//! let val: u8 = *boxed;
//! ```
//!
//! Creating a recursive data structure:
//!
//! ```
//! #[derive(Debug)]
//! enum List<T> {
//!     Cons(T, Box<List<T>>),
//!     Nil,
//! }
//!
//! let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
//! println!("{:?}", list);
//! ```
//!
//! This will print `Cons(1, Cons(2, Nil))`.
//!
//! Recursive structures must be boxed, because if the definition of `Cons`
//! looked like this:
//!
//! ```compile_fail,E0072
//! # enum List<T> {
//! Cons(T, List<T>),
//! # }
//! ```
//!
//! It wouldn't work. This is because the size of a `List` depends on how many
//! elements are in the list, and so we don't know how much memory to allocate
//! for a `Cons`. By introducing a [`Box<T>`], which has a defined size, we know how
//! big `Cons` needs to be.
//!
//! # Memory layout
//!
//! For non-zero-sized values, a [`Box`] will use the [`Global`] allocator for
//! its allocation. It is valid to convert both ways between a [`Box`] and a
//! raw pointer allocated with the [`Global`] allocator, given that the
//! [`Layout`] used with the allocator is correct for the type. More precisely,
//! a `value: *mut T` that has been allocated with the [`Global`] allocator
//! with `Layout::for_value(&*value)` may be converted into a box using
//! [`Box::<T>::from_raw(value)`]. Conversely, the memory backing a `value: *mut
//! T` obtained from [`Box::<T>::into_raw`] may be deallocated using the
//! [`Global`] allocator with [`Layout::for_value(&*value)`].
//!
//! For zero-sized values, the `Box` pointer still has to be [valid] for reads
//! and writes and sufficiently aligned. In particular, casting any aligned
//! non-zero integer literal to a raw pointer produces a valid pointer, but a
//! pointer pointing into previously allocated memory that since got freed is
//! not valid. The recommended way to build a Box to a ZST if `Box::new` cannot
//! be used is to use [`ptr::NonNull::dangling`].
//!
//! So long as `T: Sized`, a `Box<T>` is guaranteed to be represented
//! as a single pointer and is also ABI-compatible with C pointers
//! (i.e. the C type `T*`). This means that if you have extern "C"
//! Rust functions that will be called from C, you can define those
//! Rust functions using `Box<T>` types, and use `T*` as corresponding
//! type on the C side. As an example, consider this C header which
//! declares functions that create and destroy some kind of `Foo`
//! value:
//!
//! ```c
//! /* C header */
//!
//! /* Returns ownership to the caller */
//! struct Foo* foo_new(void);
//!
//! /* Takes ownership from the caller; no-op when invoked with null */
//! void foo_delete(struct Foo*);
//! ```
//!
//! These two functions might be implemented in Rust as follows. Here, the
//! `struct Foo*` type from C is translated to `Box<Foo>`, which captures
//! the ownership constraints. Note also that the nullable argument to
//! `foo_delete` is represented in Rust as `Option<Box<Foo>>`, since `Box<Foo>`
//! cannot be null.
//!
//! ```
//! #[repr(C)]
//! pub struct Foo;
//!
//! #[no_mangle]
//! pub extern "C" fn foo_new() -> Box<Foo> {
//!     Box::new(Foo)
//! }
//!
//! #[no_mangle]
//! pub extern "C" fn foo_delete(_: Option<Box<Foo>>) {}
//! ```
//!
//! Even though `Box<T>` has the same representation and C ABI as a C pointer,
//! this does not mean that you can convert an arbitrary `T*` into a `Box<T>`
//! and expect things to work. `Box<T>` values will always be fully aligned,
//! non-null pointers. Moreover, the destructor for `Box<T>` will attempt to
//! free the value with the global allocator. In general, the best practice
//! is to only use `Box<T>` for pointers that originated from the global
//! allocator.
//!
//! **Important.** At least at present, you should avoid using
//! `Box<T>` types for functions that are defined in C but invoked
//! from Rust. In those cases, you should directly mirror the C types
//! as closely as possible. Using types like `Box<T>` where the C
//! definition is just using `T*` can lead to undefined behavior, as
//! described in [rust-lang/unsafe-code-guidelines#198][ucg#198].
//!
//! [ucg#198]: https://github.com/rust-lang/unsafe-code-guidelines/issues/198
//! [dereferencing]: core::ops::Deref
//! [`Box::<T>::from_raw(value)`]: Box::from_raw
//! [`Global`]: crate::alloc::Global
//! [`Layout`]: crate::alloc::Layout
//! [`Layout::for_value(&*value)`]: crate::alloc::Layout::for_value
//! [valid]: ptr#safety

use core::any::Any;
use core::borrow;
use core::cmp::Ordering;
use core::convert::{From, TryFrom};
use core::fmt;
use core::hash::{Hash, Hasher};
use core::iter::{FusedIterator, Iterator};
use core::mem;
use core::ops::{Deref, DerefMut};
use core::pin::Pin;
use core::ptr::NonNull;

use bumpalo::Bump;

/// A pointer type for heap allocation.
///
/// See the [module-level documentation](../../std/boxed/index.html) for more.

// The declaration of the `Box` struct must be kept in sync with the
// `alloc::alloc::box_free` function or ICEs will happen. See the comment
// on `box_free` for more details.
pub struct Box<T: ?Sized>(NonNull<T>);

impl<T> Box<T> {
    /// Allocates memory on the heap and then places `x` into it.
    ///
    /// This doesn't actually allocate if `T` is zero-sized.
    ///
    /// # Examples
    ///
    /// ```
    /// let five = Box::new(5);
    /// ```
    #[cfg(not(no_global_oom_handling))]
    #[inline(always)]
    #[must_use]
    pub fn new_in(x: T, bump: &Bump) -> Self {
        Box(NonNull::new(bump.alloc(x)))
    }

    /// Constructs a new `Pin<Box<T>>`. If `T` does not implement `Unpin`, then
    /// `x` will be pinned in memory and unable to be moved.
    #[cfg(not(no_global_oom_handling))]
    #[must_use]
    #[inline(always)]
    pub fn pin_in(x: T, bump: &Bump) -> Pin<Box<T>> {
        Self::new_in(x, bump).into()
    }
}

impl<T: ?Sized> Box<T> {
    /// Constructs a box from a raw pointer.
    ///
    /// After calling this function, the raw pointer is owned by the
    /// resulting `Box`. Specifically, the `Box` destructor will call
    /// the destructor of `T` and free the allocated memory. For this
    /// to be safe, the memory must have been allocated in accordance
    /// with the [memory layout] used by `Box` .
    ///
    /// # Safety
    ///
    /// This function is unsafe because improper use may lead to
    /// memory problems. For example, a double-free may occur if the
    /// function is called twice on the same raw pointer.
    ///
    /// The safety conditions are described in the [memory layout] section.
    ///
    /// # Examples
    ///
    /// Recreate a `Box` which was previously converted to a raw pointer
    /// using [`Box::into_raw`]:
    /// ```
    /// let x = Box::new(5);
    /// let ptr = Box::into_raw(x);
    /// let x = unsafe { Box::from_raw(ptr) };
    /// ```
    /// Manually create a `Box` from scratch by using the global allocator:
    /// ```
    /// use std::alloc::{alloc, Layout};
    ///
    /// unsafe {
    ///     let ptr = alloc(Layout::new::<i32>()) as *mut i32;
    ///     // In general .write is required to avoid attempting to destruct
    ///     // the (uninitialized) previous contents of `ptr`, though for this
    ///     // simple example `*ptr = 5` would have worked as well.
    ///     ptr.write(5);
    ///     let x = Box::from_raw(ptr);
    /// }
    /// ```
    ///
    /// [memory layout]: self#memory-layout
    /// [`Layout`]: crate::Layout
    #[inline]
    pub unsafe fn from_raw(raw: *mut T) -> Self {
        unsafe { Self(NonNull::new_unchecked(raw)) }
    }
}

impl<T: ?Sized> Box<T> {
    /// Consumes the `Box`, returning a wrapped raw pointer.
    ///
    /// The pointer will be properly aligned and non-null.
    ///
    /// After calling this function, the caller is responsible for the
    /// memory previously managed by the `Box`. In particular, the
    /// caller should properly destroy `T` and release the memory, taking
    /// into account the [memory layout] used by `Box`. The easiest way to
    /// do this is to convert the raw pointer back into a `Box` with the
    /// [`Box::from_raw`] function, allowing the `Box` destructor to perform
    /// the cleanup.
    ///
    /// Note: this is an associated function, which means that you have
    /// to call it as `Box::into_raw(b)` instead of `b.into_raw()`. This
    /// is so that there is no conflict with a method on the inner type.
    ///
    /// # Examples
    /// Converting the raw pointer back into a `Box` with [`Box::from_raw`]
    /// for automatic cleanup:
    /// ```
    /// let x = Box::new(String::from("Hello"));
    /// let ptr = Box::into_raw(x);
    /// let x = unsafe { Box::from_raw(ptr) };
    /// ```
    /// Manual cleanup by explicitly running the destructor and deallocating
    /// the memory:
    /// ```
    /// use std::alloc::{dealloc, Layout};
    /// use std::ptr;
    ///
    /// let x = Box::new(String::from("Hello"));
    /// let p = Box::into_raw(x);
    /// unsafe {
    ///     ptr::drop_in_place(p);
    ///     dealloc(p as *mut u8, Layout::new::<String>());
    /// }
    /// ```
    ///
    /// [memory layout]: self#memory-layout
    #[inline]
    pub fn into_raw(b: Self) -> *mut T {
        let ptr = b.0.as_ptr();
        mem::forget(b);
        ptr
    }

    /// Consumes and leaks the `Box`, returning a mutable reference,
    /// `&'a mut T`. Note that the type `T` must outlive the chosen lifetime
    /// `'a`. If the type has only static references, or none at all, then this
    /// may be chosen to be `'static`.
    ///
    /// This function is mainly useful for data that lives for the remainder of
    /// the program's life. Dropping the returned reference will cause a memory
    /// leak. If this is not acceptable, the reference should first be wrapped
    /// with the [`Box::from_raw`] function producing a `Box`. This `Box` can
    /// then be dropped which will properly destroy `T` and release the
    /// allocated memory.
    ///
    /// Note: this is an associated function, which means that you have
    /// to call it as `Box::leak(b)` instead of `b.leak()`. This
    /// is so that there is no conflict with a method on the inner type.
    ///
    /// # Examples
    ///
    /// Simple usage:
    ///
    /// ```
    /// let x = Box::new(41);
    /// let static_ref: &'static mut usize = Box::leak(x);
    /// *static_ref += 1;
    /// assert_eq!(*static_ref, 42);
    /// ```
    ///
    /// Unsized data:
    ///
    /// ```
    /// let x = vec![1, 2, 3].into_boxed_slice();
    /// let static_ref = Box::leak(x);
    /// static_ref[0] = 4;
    /// assert_eq!(*static_ref, [4, 2, 3]);
    /// ```
    #[inline]
    pub fn leak_in<'a>(b: Self, _: &'a Bump) -> &'a mut T {
        unsafe { &mut *mem::ManuallyDrop::new(b).0.as_ptr() }
    }
}

unsafe impl<T: ?Sized> Drop for Box<T> {
    fn drop(&mut self) {
        mem::drop(*self.0)
    }
}

impl<T: ?Sized + PartialEq> PartialEq for Box<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&**self, &**other)
    }
    #[inline]
    fn ne(&self, other: &Self) -> bool {
        PartialEq::ne(&**self, &**other)
    }
}
impl<T: ?Sized + PartialOrd> PartialOrd for Box<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&**self, &**other)
    }
    #[inline]
    fn lt(&self, other: &Self) -> bool {
        PartialOrd::lt(&**self, &**other)
    }
    #[inline]
    fn le(&self, other: &Self) -> bool {
        PartialOrd::le(&**self, &**other)
    }
    #[inline]
    fn ge(&self, other: &Self) -> bool {
        PartialOrd::ge(&**self, &**other)
    }
    #[inline]
    fn gt(&self, other: &Self) -> bool {
        PartialOrd::gt(&**self, &**other)
    }
}
impl<T: ?Sized + Ord> Ord for Box<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&**self, &**other)
    }
}
impl<T: ?Sized + Eq> Eq for Box<T> {}

impl<T: ?Sized + Hash> Hash for Box<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (**self).hash(state);
    }
}

impl<T: ?Sized + Hasher> Hasher for Box<T> {
    fn finish(&self) -> u64 {
        (**self).finish()
    }
    fn write(&mut self, bytes: &[u8]) {
        (**self).write(bytes)
    }
    fn write_u8(&mut self, i: u8) {
        (**self).write_u8(i)
    }
    fn write_u16(&mut self, i: u16) {
        (**self).write_u16(i)
    }
    fn write_u32(&mut self, i: u32) {
        (**self).write_u32(i)
    }
    fn write_u64(&mut self, i: u64) {
        (**self).write_u64(i)
    }
    fn write_u128(&mut self, i: u128) {
        (**self).write_u128(i)
    }
    fn write_usize(&mut self, i: usize) {
        (**self).write_usize(i)
    }
    fn write_i8(&mut self, i: i8) {
        (**self).write_i8(i)
    }
    fn write_i16(&mut self, i: i16) {
        (**self).write_i16(i)
    }
    fn write_i32(&mut self, i: i32) {
        (**self).write_i32(i)
    }
    fn write_i64(&mut self, i: i64) {
        (**self).write_i64(i)
    }
    fn write_i128(&mut self, i: i128) {
        (**self).write_i128(i)
    }
    fn write_isize(&mut self, i: isize) {
        (**self).write_isize(i)
    }
}

impl From<Box<str>> for Box<[u8]> {
    /// Converts a `Box<str>` into a `Box<[u8]>`
    ///
    /// This conversion does not allocate on the heap and happens in place.
    ///
    /// # Examples
    /// ```rust
    /// // create a Box<str> which will be used to create a Box<[u8]>
    /// let boxed: Box<str> = Box::from("hello");
    /// let boxed_str: Box<[u8]> = Box::from(boxed);
    ///
    /// // create a &[u8] which will be used to create a Box<[u8]>
    /// let slice: &[u8] = &[104, 101, 108, 108, 111];
    /// let boxed_slice = Box::from(slice);
    ///
    /// assert_eq!(boxed_slice, boxed_str);
    /// ```
    #[inline]
    fn from(s: Box<str>) -> Self {
        let raw = Box::into_raw_with_allocator(s);
        unsafe { Box::from_raw_in(raw as *mut [u8]) }
    }
}

impl<T, const N: usize> TryFrom<Box<[T]>> for Box<[T; N]> {
    type Error = Box<[T]>;

    /// Attempts to convert a `Box<[T]>` into a `Box<[T; N]>`.
    ///
    /// The conversion occurs in-place and does not require a
    /// new memory allocation.
    ///
    /// # Errors
    ///
    /// Returns the old `Box<[T]>` in the `Err` variant if
    /// `boxed_slice.len()` does not equal `N`.
    fn try_from(boxed_slice: Box<[T]>) -> Result<Self, Self::Error> {
        if boxed_slice.len() == N {
            Ok(unsafe { Box::from_raw(Box::into_raw(boxed_slice) as *mut [T; N]) })
        } else {
            Err(boxed_slice)
        }
    }
}

impl Box<dyn Any> {
    #[inline]
    /// Attempt to downcast the box to a concrete type.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::any::Any;
    ///
    /// fn print_if_string(value: Box<dyn Any>) {
    ///     if let Ok(string) = value.downcast::<String>() {
    ///         println!("String ({}): {}", string.len(), string);
    ///     }
    /// }
    ///
    /// let my_string = "Hello World".to_string();
    /// print_if_string(Box::new(my_string));
    /// print_if_string(Box::new(0i8));
    /// ```
    pub fn downcast<T: Any>(self) -> Result<Box<T>, Self> {
        if self.is::<T>() {
            unsafe {
                let (raw, alloc): (*mut dyn Any, _) = Box::into_raw_with_allocator(self);
                Ok(Box::from_raw_in(raw as *mut T, alloc))
            }
        } else {
            Err(self)
        }
    }
}

impl Box<dyn Any + Send> {
    #[inline]
    /// Attempt to downcast the box to a concrete type.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::any::Any;
    ///
    /// fn print_if_string(value: Box<dyn Any + Send>) {
    ///     if let Ok(string) = value.downcast::<String>() {
    ///         println!("String ({}): {}", string.len(), string);
    ///     }
    /// }
    ///
    /// let my_string = "Hello World".to_string();
    /// print_if_string(Box::new(my_string));
    /// print_if_string(Box::new(0i8));
    /// ```
    pub fn downcast<T: Any>(self) -> Result<Box<T>, Self> {
        if self.is::<T>() {
            unsafe {
                let (raw, alloc): (*mut (dyn Any + Send), _) = Box::into_raw_with_allocator(self);
                Ok(Box::from_raw_in(raw as *mut T, alloc))
            }
        } else {
            Err(self)
        }
    }
}

impl Box<dyn Any + Send + Sync> {
    #[inline]
    /// Attempt to downcast the box to a concrete type.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::any::Any;
    ///
    /// fn print_if_string(value: Box<dyn Any + Send + Sync>) {
    ///     if let Ok(string) = value.downcast::<String>() {
    ///         println!("String ({}): {}", string.len(), string);
    ///     }
    /// }
    ///
    /// let my_string = "Hello World".to_string();
    /// print_if_string(Box::new(my_string));
    /// print_if_string(Box::new(0i8));
    /// ```
    pub fn downcast<T: Any>(self) -> Result<Box<T>, Self> {
        if self.is::<T>() {
            unsafe {
                let (raw, alloc): (*mut (dyn Any + Send + Sync), _) =
                    Box::into_raw_with_allocator(self);
                Ok(Box::from_raw_in(raw as *mut T, alloc))
            }
        } else {
            Err(self)
        }
    }
}

impl<T: fmt::Display + ?Sized> fmt::Display for Box<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<T: fmt::Debug + ?Sized> fmt::Debug for Box<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T: ?Sized> fmt::Pointer for Box<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // It's not possible to extract the inner Uniq directly from the Box,
        // instead we cast it to a *const which aliases the NonNull
        let ptr: *const T = &**self;
        fmt::Pointer::fmt(&ptr, f)
    }
}

impl<T: ?Sized> Deref for Box<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &**self
    }
}

impl<T: ?Sized> DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut **self
    }
}

impl<I: Iterator + ?Sized> Iterator for Box<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        (**self).next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (**self).size_hint()
    }
    fn nth(&mut self, n: usize) -> Option<I::Item> {
        (**self).nth(n)
    }
    fn last(self) -> Option<I::Item> {
        BoxIter::last(self)
    }
}

trait BoxIter {
    type Item;
    fn last(self) -> Option<Self::Item>;
}

impl<I: Iterator + ?Sized> BoxIter for Box<I> {
    type Item = I::Item;
    fn last(self) -> Option<I::Item> {
        #[inline]
        fn some<T>(_: Option<T>, x: T) -> Option<T> {
            Some(x)
        }

        self.fold(None, some)
    }
}

impl<I: DoubleEndedIterator + ?Sized> DoubleEndedIterator for Box<I> {
    fn next_back(&mut self) -> Option<I::Item> {
        (**self).next_back()
    }
    fn nth_back(&mut self, n: usize) -> Option<I::Item> {
        (**self).nth_back(n)
    }
}
impl<I: ExactSizeIterator + ?Sized> ExactSizeIterator for Box<I> {
    fn len(&self) -> usize {
        (**self).len()
    }
}

impl<I: FusedIterator + ?Sized> FusedIterator for Box<I> {}

impl<T: ?Sized> borrow::Borrow<T> for Box<T> {
    fn borrow(&self) -> &T {
        &**self
    }
}

impl<T: ?Sized> borrow::BorrowMut<T> for Box<T> {
    fn borrow_mut(&mut self) -> &mut T {
        &mut **self
    }
}

impl<T: ?Sized> AsRef<T> for Box<T> {
    fn as_ref(&self) -> &T {
        &**self
    }
}

impl<T: ?Sized> AsMut<T> for Box<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut **self
    }
}
