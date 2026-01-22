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

//! Lazy wrapper for repeated fields enabling on-demand parsing.
//!
//! This type is intended to be a common return type for all message repeated
//! field getters in lazy implementations. It delegates storage to `OnceList`
//! while coordinating with the parent's `MessageParserStateRef` to advance parsing
//! just enough to satisfy access patterns (indexing/peek/length).

use crate::error::Error;
use crate::lazy_parser::MessageParserStateRef;
use crate::repeated::Repeated;
use ::allocator_extras::Allocator;
use ::once_list2::OnceListWithTailLen as OnceList;
/// A lazy, on-demand parsing adapter over a repeated field.
///
/// - `T`: element type contained in the repeated field (should be `Clone`).
/// - `A`: allocator used by the underlying `OnceList`.
/// - `'slice`: lifetime of the parent's parsing input slices.
/// - `'message`: lifetime of the borrowed `OnceList` inside the parent message.
pub struct LazyRepeated<'slice, 'message, T, A>
where
    T: Clone + 'message,
    A: Allocator + Clone + 'slice,
{
    parent_parser_state: MessageParserStateRef<'slice, A>,
    list: &'message OnceList<T, A>,
}

impl<'slice, 'message, T, A> LazyRepeated<'slice, 'message, T, A>
where
    T: Clone + 'message,
    A: Allocator + Clone + 'slice,
{
    /// Create a new `LazyRepeated` adapter.
    #[inline]
    pub fn new(
        parent_parser_state: MessageParserStateRef<'slice, A>,
        list: &'message OnceList<T, A>,
    ) -> Self {
        Self {
            parent_parser_state,
            list,
        }
    }

    /// Ensure the repeated field has at least `needed` elements parsed.
    ///
    /// This will request the parent to continue parsing until either:
    /// - the required number of elements are available, or
    /// - the parent input is exhausted.
    ///
    /// If the parent's iterator is exhausted, `parse_until` will automatically request
    /// the parent's parent to continue parsing, so this method only needs to call `parse_until`.
    fn ensure_at_least(&self, needed: usize) -> Result<(), Error> {
        while self.list.len() < needed {
            // No more elements right now; advance the parent by one field and retry.
            // If the parent is exhausted, we cannot make further progress.
            if !self.parent_parser_state.parse_one_field_with_callback()? {
                break;
            }
        }
        Ok(())
    }

    /// Ensure the parent is fully parsed (used for operations that require total length).
    fn ensure_fully_parsed(&self) -> Result<(), Error> {
        // Parse all remaining fields until iterator is exhausted
        // Use a condition that always returns false to parse all fields
        let _ = self.parent_parser_state.parse_until_with_callback(|| false);
        Ok(())
    }

    /// Return an iterator over the elements in the repeated field.
    ///
    /// The iterator's `next()` method triggers parsing on-demand when needed.
    pub fn iter(&self) -> LazyRepeatedIter<'_, 'slice, 'message, T, A>
    where
        T: 'message,
    {
        LazyRepeatedIter::new(self)
    }
}

/// Iterator over `LazyRepeated` that triggers parsing on-demand in `next()`.
pub struct LazyRepeatedIter<'iter, 'slice, 'message, T, A>
where
    T: Clone + 'message,
    A: Allocator + Clone + 'slice,
{
    lazy_repeated: &'iter LazyRepeated<'slice, 'message, T, A>,
    inner_iter: ::std::iter::Cloned<::once_list2::Iter<'message, T, A>>,
}

impl<'iter, 'slice, 'message, T, A> LazyRepeatedIter<'iter, 'slice, 'message, T, A>
where
    T: Clone + 'message,
    A: Allocator + Clone + 'slice,
{
    fn new(lazy_repeated: &'iter LazyRepeated<'slice, 'message, T, A>) -> Self {
        Self {
            lazy_repeated,
            inner_iter: lazy_repeated.list.iter().cloned(),
        }
    }
}

impl<'iter, 'slice, 'message, T, A> Iterator for LazyRepeatedIter<'iter, 'slice, 'message, T, A>
where
    T: Clone + 'message,
    A: Allocator + Clone + 'slice,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // Fast path: already-parsed elements.
        if let Some(item) = self.inner_iter.next() {
            return Some(item);
        }

        // We are at the end of what is currently available. Advance the parent parser one field at
        // a time until this repeated field receives at least one new element, or the parent is
        // exhausted.
        loop {
            let progressed = self
                .lazy_repeated
                .parent_parser_state
                .parse_one_field_with_callback()
                .ok()?;

            if !progressed {
                return None;
            }

            if let Some(item) = self.inner_iter.next() {
                return Some(item);
            }
        }
    }
}

#[allow(missing_docs)]
impl<'slice, 'message, T, A> Repeated<'message> for LazyRepeated<'slice, 'message, T, A>
where
    T: Clone + 'message,
    A: Allocator + Clone + 'slice,
{
    type Item = T;

    fn len(&self) -> usize {
        // Length requires knowing all elements; fall back to full parse.
        let _ = self.ensure_fully_parsed();
        self.list.len()
    }

    fn is_empty(&self) -> bool {
        // Parse just enough to know if at least one exists.
        let _ = self.ensure_at_least(1);
        self.list.first().is_none()
    }

    fn get(&self, index: usize) -> Option<Self::Item> {
        let _ = self.ensure_at_least(index.saturating_add(1));
        self.list.iter().nth(index).cloned()
    }

    fn iter_box(&self) -> ::allocator_api2::boxed::Box<dyn Iterator<Item = Self::Item> + 'message> {
        // For the initial boilerplate, fully parse, then return a non-allocating iterator.
        let _ = self.ensure_fully_parsed();
        let it = self.list.iter().cloned();
        let boxed = ::allocator_api2::boxed::Box::new(it);
        let boxed_dyn: ::allocator_api2::boxed::Box<dyn Iterator<Item = T> + 'message> =
            ::allocator_api2::unsize_box!(boxed);
        boxed_dyn
    }
}
