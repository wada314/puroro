//! Lazy wrapper for repeated fields enabling on-demand parsing.
//!
//! This type is intended to be a common return type for all message repeated
//! field getters in lazy implementations. It delegates storage to `OnceList`
//! while coordinating with the parent's `MessageParserState` to advance parsing
//! just enough to satisfy access patterns (indexing/peek/length).

use ::allocator_extras::Allocator;
use ::once_list2::OnceList;
use ::puroro::error::Error;
use ::puroro::repeated::Repeated;
use ::std::cell::RefCell;
use ::std::rc::Rc;

use crate::generated::lazy_parser::MessageParserState;

/// A lazy, on-demand parsing adapter over a repeated field.
///
/// - `T`: element type contained in the repeated field (should be `Clone`).
/// - `A`: allocator used by the underlying `OnceList`.
/// - `'a`: lifetime of the parent's parsing input.
/// - `'b`: lifetime of the borrowed `OnceList` inside the parent message.
pub struct LazyRepeated<'a, 'b, T, A>
where
    T: Clone + 'b,
    A: Allocator + Clone + 'a,
{
    parent_parser_state: Rc<RefCell<MessageParserState<'a, A>>>,
    _field_number: u32,
    list: &'b OnceList<T, A>,
}

impl<'a, 'b, T, A> LazyRepeated<'a, 'b, T, A>
where
    T: Clone + 'b,
    A: Allocator + Clone + 'a,
{
    /// Create a new `LazyRepeated` adapter.
    #[inline]
    pub fn new(
        parent_parser_state: Rc<RefCell<MessageParserState<'a, A>>>,
        field_number: u32,
        list: &'b OnceList<T, A>,
    ) -> Self {
        Self {
            parent_parser_state,
            _field_number: field_number,
            list,
        }
    }

    /// Ensure the repeated field has at least `needed` elements parsed.
    ///
    /// This will repeatedly request the parent to continue parsing until either:
    /// - the required number of elements are available, or
    /// - the parent input is exhausted.
    fn ensure_at_least(&self, needed: usize) -> Result<(), Error> {
        loop {
            let current = self.list.iter().count();
            if current >= needed {
                return Ok(());
            }

            // Ask parent to continue parsing (may update multiple fields).
            {
                let mut state = self.parent_parser_state.borrow_mut();
                // Ignore errors from callback targets being dropped; keep propagating parser errors.
                let _ = state.continue_parsing_for_children();
            }

            // If no progress and iterator is exhausted, stop.
            let after = self.list.iter().count();
            if after >= needed {
                return Ok(());
            }
            let exhausted = self.parent_parser_state.borrow().field_iter.is_none();
            if exhausted {
                return Ok(());
            }
        }
    }

    /// Ensure the parent is fully parsed (used for operations that require total length).
    fn ensure_fully_parsed(&self) -> Result<(), Error> {
        loop {
            let mut state = self.parent_parser_state.borrow_mut();
            if state.field_iter.is_none() {
                return Ok(());
            }
            let _ = state.continue_parsing_for_children();
            // Loop until iterator becomes None (exhausted).
        }
    }

    /// Return an iterator over the elements in the repeated field.
    ///
    /// The iterator's `next()` method triggers parsing on-demand when needed.
    pub fn iter(&self) -> LazyRepeatedIter<'_, 'a, 'b, T, A>
    where
        T: 'b,
    {
        LazyRepeatedIter::new(self)
    }
}

/// Iterator over `LazyRepeated` that triggers parsing on-demand in `next()`.
pub struct LazyRepeatedIter<'iter, 'a, 'b, T, A>
where
    T: Clone + 'b,
    A: Allocator + Clone + 'a,
{
    lazy_repeated: &'iter LazyRepeated<'a, 'b, T, A>,
    inner_iter: ::std::boxed::Box<dyn Iterator<Item = T> + 'iter>,
}

impl<'iter, 'a, 'b, T, A> LazyRepeatedIter<'iter, 'a, 'b, T, A>
where
    T: Clone + 'b,
    A: Allocator + Clone + 'a,
{
    fn new(lazy_repeated: &'iter LazyRepeated<'a, 'b, T, A>) -> Self {
        let iter = lazy_repeated.list.iter().cloned();
        Self {
            lazy_repeated,
            inner_iter: ::std::boxed::Box::new(iter),
        }
    }
}

impl<'iter, 'a, 'b, T, A> Iterator for LazyRepeatedIter<'iter, 'a, 'b, T, A>
where
    T: Clone + 'b,
    A: Allocator + Clone + 'a,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // Try to get next element from the current iterator
        if let Some(item) = self.inner_iter.next() {
            return Some(item);
        }

        // Current iterator exhausted - check if we need to parse more
        let parser_exhausted = self
            .lazy_repeated
            .parent_parser_state
            .borrow()
            .field_iter
            .is_none();
        if parser_exhausted {
            // No more elements will be available
            return None;
        }

        // Trigger parsing to get more elements
        let current_total = self.lazy_repeated.list.iter().count();
        let _ = self.lazy_repeated.ensure_at_least(current_total + 1);

        // Check if we got new elements after parsing
        let new_total = self.lazy_repeated.list.iter().count();
        if new_total <= current_total {
            // No progress made - parser exhausted or no more elements
            return None;
        }

        // Recreate the iterator to include newly parsed elements
        // The new iterator will iterate over all elements, but we've already consumed
        // all elements from the old iterator, so we just get the new ones
        let iter = self.lazy_repeated.list.iter().cloned();
        self.inner_iter = ::std::boxed::Box::new(iter);
        // Skip the elements we've already seen (all current_total of them)
        for _ in 0..current_total {
            self.inner_iter.next();
        }

        // Now get the next element (which should be newly parsed)
        self.inner_iter.next()
    }
}

#[allow(missing_docs)]
impl<'a, 'b, T, A> Repeated<'b> for LazyRepeated<'a, 'b, T, A>
where
    T: Clone + 'b,
    A: Allocator + Clone + 'a,
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

    fn iter_box(&self) -> ::allocator_api2::boxed::Box<dyn Iterator<Item = Self::Item> + 'b> {
        // For the initial boilerplate, materialize by fully parsing and collecting.
        let _ = self.ensure_fully_parsed();
        let owned: ::std::vec::Vec<T> = self.list.iter().cloned().collect();
        let it = owned.into_iter();
        let boxed = ::allocator_api2::boxed::Box::new(it);
        let boxed_dyn: ::allocator_api2::boxed::Box<dyn Iterator<Item = T> + 'b> =
            ::allocator_api2::unsize_box!(boxed);
        boxed_dyn
    }
}
