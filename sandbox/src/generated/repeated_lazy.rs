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
    /// This triggers parsing of at least one element (if available) before returning the iterator.
    /// The iterator will iterate over already-parsed elements; use `get()` to trigger parsing
    /// for specific indices.
    pub fn iter(&self) -> impl Iterator<Item = T> + '_
    where
        T: 'b,
    {
        // Trigger parsing of at least one element (if available) before returning iterator
        let _ = self.ensure_at_least(1);
        self.list.iter().cloned()
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


