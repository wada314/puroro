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
use ::once_list2::OnceList;

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
    _field_number: u32,
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
        field_number: u32,
        list: &'message OnceList<T, A>,
    ) -> Self {
        Self {
            parent_parser_state,
            _field_number: field_number,
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
        // Check if we already have enough elements
        if self.list.iter().count() >= needed {
            return Ok(());
        }

        // Parse until we have enough elements for this field
        // The condition checks if we've reached the target count
        // If the parent's iterator is exhausted, parse_until will automatically
        // request the parent's parent to continue parsing
        self.parent_parser_state.parse_until_with_callback(|| {
            // Stop when we have enough elements for this field.
            // Note: This is checked after each field is processed via the callback.
            self.list.iter().count() >= needed
        })?;

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
    inner_iter: ::std::boxed::Box<dyn Iterator<Item = T> + 'iter>,
}

impl<'iter, 'slice, 'message, T, A> LazyRepeatedIter<'iter, 'slice, 'message, T, A>
where
    T: Clone + 'message,
    A: Allocator + Clone + 'slice,
{
    fn new(lazy_repeated: &'iter LazyRepeated<'slice, 'message, T, A>) -> Self {
        let iter = lazy_repeated.list.iter().cloned();
        Self {
            lazy_repeated,
            inner_iter: ::std::boxed::Box::new(iter),
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
        // Try to get next element from the current iterator
        if let Some(item) = self.inner_iter.next() {
            return Some(item);
        }

        // Current iterator exhausted - check if we need to parse more
        // Try to continue parsing and see if we get more elements
        let previous_count = self.lazy_repeated.list.iter().count();
        // Try to parse at least one more element
        let _ = self.lazy_repeated.ensure_at_least(previous_count + 1);

        // Check if we got new elements after parsing
        let new_total = self.lazy_repeated.list.iter().count();
        if new_total <= previous_count {
            // No progress made - parser exhausted or no more elements
            return None;
        }

        // Recreate the iterator to include newly parsed elements
        // The new iterator will iterate over all elements, but we've already consumed
        // all elements from the old iterator, so we just get the new ones
        let iter = self.lazy_repeated.list.iter().cloned();
        self.inner_iter = ::std::boxed::Box::new(iter);
        // Skip the elements we've already seen (all previous_count of them)
        for _ in 0..previous_count {
            self.inner_iter.next();
        }

        // Now get the next element (which should be newly parsed)
        self.inner_iter.next()
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
        self.list.iter().count()
    }

    fn is_empty(&self) -> bool {
        // Parse just enough to know if at least one exists.
        let _ = self.ensure_at_least(1);
        self.list.iter().next().is_none()
    }

    fn get(&self, index: usize) -> Option<Self::Item> {
        let _ = self.ensure_at_least(index.saturating_add(1));
        self.list.iter().nth(index).cloned()
    }

    fn iter_box(&self) -> ::allocator_api2::boxed::Box<dyn Iterator<Item = Self::Item> + 'message> {
        // For the initial boilerplate, materialize by fully parsing and collecting.
        let _ = self.ensure_fully_parsed();
        let owned: ::std::vec::Vec<T> = self.list.iter().cloned().collect();
        let it = owned.into_iter();
        let boxed = ::allocator_api2::boxed::Box::new(it);
        let boxed_dyn: ::allocator_api2::boxed::Box<dyn Iterator<Item = T> + 'message> =
            ::allocator_api2::unsize_box!(boxed);
        boxed_dyn
    }
}
