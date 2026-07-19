//! Minimal mutator surface for repeated field containers.
//!
//! Generated `_mut` accessors can return [`RepeatedElementsMut`] (or
//! `impl `[`RepeatedContainerMut`]) so scalar, message, and string/bytes
//! repeated fields share one shape. The concrete element handle is
//! [`RepeatedElementMut::ElementMut`] (same family as singular `_mut` payloads).

use ::core::marker::PhantomData;
use ::core::ops::DerefMut;

use ::allocator_api2::alloc::Allocator;
use ::unmanaged::vec::VecGuard;

use crate::fields::wire::repeated_element::{
    RepeatedElement, RepeatedElementMerge, RepeatedElementMut,
};

/// Minimal mutator API for a repeated field container.
///
/// No `IndexMut`: out-of-range access goes through [`get_mut`](Self::get_mut).
/// [`push`](Self::push) appends a type-default empty element and returns a
/// mutable handle to it (C++ `add_foo()` style).
pub trait RepeatedContainerMut {
    /// Mutable handle for one element (`&mut i32`, [`StringGuard`](::unmanaged::string::StringGuard), …).
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

/// Growable view over a repeated field's element storage.
///
/// Obtained from [`RepeatedFieldMut::container_mut`](super::field::RepeatedFieldMut::container_mut).
/// Holds a [`VecGuard`] so push/growth keep the message allocator.
///
/// Methods are inherent so generated `labels_mut()` callers need not import
/// [`RepeatedContainerMut`] (the trait still exists for `impl Trait` bounds).
pub struct RepeatedElementsMut<'a, T, A>
where
    T: RepeatedElement,
    A: Allocator + Clone,
{
    values: VecGuard<'a, T::Element<A>, A>,
    _marker: PhantomData<T>,
}

impl<'a, T, A> RepeatedElementsMut<'a, T, A>
where
    T: RepeatedElement,
    A: Allocator + Clone,
{
    #[inline]
    pub(super) fn new(values: VecGuard<'a, T::Element<A>, A>) -> Self {
        Self {
            values,
            _marker: PhantomData,
        }
    }
}

impl<'a, T, A> RepeatedElementsMut<'a, T, A>
where
    T: RepeatedElementMut + RepeatedElementMerge<A>,
    A: Allocator + Clone,
{
    #[inline]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Appends a default / empty element and returns a mutator for it.
    #[inline]
    pub fn push(&mut self) -> T::ElementMut<'_, A> {
        let alloc = self.values.allocator().clone();
        self.values.push(T::default_element(alloc.clone()));
        let elem = self.values.last_mut().expect("element present after push");
        // SAFETY: `alloc` is a clone of the message allocator that owns this
        // vector and the new element.
        unsafe { T::element_mut(elem, alloc) }
    }

    /// Mutable handle for the element at `index`, or `None` if out of range.
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<T::ElementMut<'_, A>> {
        let alloc = self.values.allocator().clone();
        let elem = self.values.get_mut(index)?;
        // SAFETY: `alloc` owns this vector and every live element.
        Some(unsafe { T::element_mut(elem, alloc) })
    }

    /// Removes all elements (heap payloads are released).
    #[inline]
    pub fn clear(&mut self) {
        let alloc = self.values.allocator().clone();
        while let Some(elem) = self.values.pop() {
            // SAFETY: same allocator ownership as `RepeatedFieldMut::clear`.
            unsafe { T::deallocate_element(elem, alloc.clone()) };
        }
    }

    /// Removes the last element (and releases it). Returns whether one existed.
    #[inline]
    pub fn pop(&mut self) -> bool {
        let alloc = self.values.allocator().clone();
        match self.values.pop() {
            Some(elem) => {
                // SAFETY: same allocator ownership as `clear`.
                unsafe { T::deallocate_element(elem, alloc) };
                true
            }
            None => false,
        }
    }
}

impl<'a, T, A> RepeatedContainerMut for RepeatedElementsMut<'a, T, A>
where
    T: RepeatedElementMut + RepeatedElementMerge<A>,
    A: Allocator + Clone,
{
    type Mut<'m>
        = T::ElementMut<'m, A>
    where
        Self: 'm;

    #[inline]
    fn len(&self) -> usize {
        RepeatedElementsMut::len(self)
    }

    #[inline]
    fn push(&mut self) -> T::ElementMut<'_, A> {
        RepeatedElementsMut::push(self)
    }

    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<T::ElementMut<'_, A>> {
        RepeatedElementsMut::get_mut(self, index)
    }

    #[inline]
    fn clear(&mut self) {
        RepeatedElementsMut::clear(self);
    }

    #[inline]
    fn pop(&mut self) -> bool {
        RepeatedElementsMut::pop(self)
    }
}
