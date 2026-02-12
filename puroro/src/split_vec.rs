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

//! Reference-separated access: a trait and handles for holding element references
//! (`&T` or `&mut T`) while pushing, by splitting access into separate handles.
//!
//! # Split API
//!
//! - **[`ReadHandle`]**: `get(i)` and `get_mut(i)`. The borrow checker ensures `get` and
//!   `get_mut` are never used such that a shared and exclusive reference are alive at once.
//! - **[`AppendHandle`]**: `push(value)` only. Does not invalidate existing `&T` or `&mut T`.
//! - **[`CountHandle`]**: `count()` only. Can be called regardless of other handle use.
//!
//! Implemented for [`orx_split_vec::SplitVec`] and [`once_list2::OnceListWithTailLen`].

use std::marker::PhantomData;

/// Trait for address-stable growable containers that support splitting into read, push, and count handles.
///
/// Implementors must ensure that `push` does not invalidate existing element references.
pub trait RefSeparatedVec {
    /// Element type.
    type Item;

    /// Number of elements.
    fn count(&self) -> usize;

    /// Reference to the element at index `i`, or `None` if out of range.
    fn get(&self, i: usize) -> Option<&Self::Item>;

    /// Mutable reference to the element at index `i`, or `None` if out of range.
    fn get_mut(&mut self, i: usize) -> Option<&mut Self::Item>;

    /// Appends an element. Must not invalidate existing `&Self::Item` or `&mut Self::Item`.
    fn push(&mut self, value: Self::Item);

    /// Splits into read, push, and count handles. While the returned handles are alive,
    /// the container must not be used for structural mutation (e.g. clear).
    fn as_split(&mut self) -> (ReadHandle<'_, Self>, AppendHandle<'_, Self>, CountHandle<'_, Self>)
    where
        Self: Sized,
    {
        let ptr = self as *mut Self;
        (
            ReadHandle {
                ptr,
                _marker: PhantomData,
            },
            AppendHandle {
                ptr,
                _marker: PhantomData,
            },
            CountHandle {
                ptr: ptr as *const Self,
                _marker: PhantomData,
            },
        )
    }
}

// --- Handles (unsafe: raw pointers used to allow split borrows) ---

/// Read and single-element mutable access: `get` and `get_mut`.
pub struct ReadHandle<'a, C: ?Sized> {
    ptr: *mut C,
    _marker: PhantomData<&'a C>,
}

impl<'a, C> ReadHandle<'a, C>
where
    C: RefSeparatedVec + ?Sized,
{
    /// Returns a reference to the element at index `i`, or `None` if out of range.
    pub fn get(&self, i: usize) -> Option<&'a C::Item> {
        unsafe { (*self.ptr).get(i) }
    }

    /// Returns a mutable reference to the element at index `i`, or `None` if out of range.
    pub fn get_mut(&mut self, i: usize) -> Option<&'a mut C::Item> {
        unsafe { (*self.ptr).get_mut(i) }
    }
}

/// Push-only access. Does not invalidate existing refs from [`ReadHandle`].
pub struct AppendHandle<'a, C: ?Sized> {
    ptr: *mut C,
    _marker: PhantomData<&'a mut C>,
}

impl<'a, C> AppendHandle<'a, C>
where
    C: RefSeparatedVec + ?Sized,
{
    /// Appends an element. Existing element references remain valid.
    pub fn push(&mut self, value: C::Item) {
        unsafe {
            (*self.ptr).push(value);
        }
    }
}

/// Count-only access. Can be used regardless of other handle state.
pub struct CountHandle<'a, C: ?Sized> {
    ptr: *const C,
    _marker: PhantomData<&'a C>,
}

impl<'a, C> CountHandle<'a, C>
where
    C: RefSeparatedVec + ?Sized,
{
    /// Returns the number of elements.
    pub fn count(&self) -> usize {
        unsafe { (*self.ptr).count() }
    }
}

// --- Implementations ---

impl<T, G> RefSeparatedVec for orx_split_vec::SplitVec<T, G>
where
    G: orx_split_vec::Growth,
{
    type Item = T;

    fn count(&self) -> usize {
        orx_split_vec::PinnedVec::len(self)
    }

    fn get(&self, i: usize) -> Option<&T> {
        let len = orx_split_vec::PinnedVec::len(self);
        (i < len).then(|| &self[i])
    }

    fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        let len = orx_split_vec::PinnedVec::len(self);
        (i < len).then(|| &mut self[i])
    }

    fn push(&mut self, value: T) {
        orx_split_vec::PinnedVec::push(self, value);
    }
}

/// Implemented for `OnceListCore<T, A, C>` for any cache mode `C: once_list2::CacheMode<T, A>`
/// (e.g. `NoCache`, `WithLen`, `WithTail`, `WithTailLen`).
impl<T, A, C> RefSeparatedVec for once_list2::OnceListCore<T, A, C>
where
    A: ::allocator_api2::alloc::Allocator + Clone,
    C: once_list2::CacheMode<T, A>,
{
    type Item = T;

    fn count(&self) -> usize {
        self.len()
    }

    fn get(&self, i: usize) -> Option<&T> {
        self.iter().nth(i)
    }

    fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        self.iter_mut().nth(i)
    }

    fn push(&mut self, value: T) {
        once_list2::OnceListCore::push(self, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orx_split_vec_append_while_holding_ref() {
        let mut vec = orx_split_vec::SplitVec::<i32>::new();
        vec.push(1);
        let (read, mut append, count) = vec.as_split();
        let r = read.get(0).unwrap();
        assert_eq!(count.count(), 1);
        append.push(2);
        assert_eq!(*r, 1);
        assert_eq!(count.count(), 2);
    }

    #[test]
    fn orx_split_vec_get_mut_and_push() {
        let mut vec = orx_split_vec::SplitVec::<i32>::new();
        vec.push(10);
        vec.push(20);
        let (mut read, mut append, count) = vec.as_split();
        let m = read.get_mut(0).unwrap();
        *m = 11;
        append.push(30);
        assert_eq!(*m, 11);
        assert_eq!(count.count(), 3);
    }

    #[test]
    fn once_list_append_while_holding_ref() {
        let list = once_list2::OnceListWithTailLen::<i32>::new();
        list.push(1);
        let mut list = list;
        let (read, mut append, count) = list.as_split();
        let r = read.get(0).unwrap();
        assert_eq!(count.count(), 1);
        append.push(2);
        assert_eq!(*r, 1);
        assert_eq!(count.count(), 2);
    }

    #[test]
    fn once_list_get_mut_and_push() {
        let list = once_list2::OnceListWithTailLen::<i32>::new();
        list.push(10);
        list.push(20);
        let mut list = list;
        let (mut read, mut append, count) = list.as_split();
        let m = read.get_mut(0).unwrap();
        *m = 11;
        append.push(30);
        assert_eq!(*m, 11);
        assert_eq!(count.count(), 3);
    }
}
