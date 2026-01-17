//! Lazy parser infrastructure for Protocol Buffer messages.
//!
//! This module provides the core parsing infrastructure for lazy deserialization:
//! - FieldIterator: Iterator over protobuf fields in slices
//! - MessageParserState: Parser state that can be shared between message bodies and child messages
//! - Helper functions for wire format parsing (varint decoding, field tag parsing)

use crate::error::Error;
use ::allocator_api2::boxed::Box;
use ::allocator_extras::{Allocator, Global};
use ::once_list2::OnceList;
use ::protobuf_core::{AsRefExtProtobuf, Field};
use ::std::cell::Cell;
use ::std::cell::OnceCell;
use ::std::cell::RefCell;
use ::std::rc::Rc;

/// Iterator over protobuf fields in slices.
///
/// Can be paused and resumed, making it easy to parse incrementally.
/// Uses flat_map approach to convert slice iterator to field iterator.
///
/// - `'slice`: Lifetime of the input slices (external data)
/// Iterator over protobuf fields from multiple slices.
///
/// TODO: Currently, `field_iter` is created from a snapshot of `field_slices` at construction time.
/// When new slices are added via `add_slice()`, the iterator doesn't automatically see them.
/// This requires manual recreation of the iterator (as done in `get_next_field`).
/// We should refactor `FieldIterator` so that `next()` can automatically detect and include
/// newly added slices from `field_slices`, eliminating the need for manual iterator recreation.
/// This would simplify `get_next_field` and make the iterator more robust.
///
/// **Self-referential struct problem**: If we try to make `field_iter` directly reference `field_slices`,
/// we get a self-referential struct which Rust's borrow checker doesn't allow. Potential solutions:
/// 1. Use `Rc<OnceList<...>>` to share `field_slices` (adds reference counting overhead)
/// 2. Store iterator state manually (slice index + field offset) instead of a boxed iterator
/// 3. Keep current `Vec` approach but optimize recreation logic
/// 4. Use `Pin` (complex, may require unstable APIs)
///
/// Solution: Using `Rc<OnceList<...>>` for `field_slices` and `OnceCell<Box<dyn Iterator<...>>>`
/// for `field_iter` allows lazy initialization without self-referential struct issues.
/// The iterator is created on first `next()` call, directly from `field_slices` without
/// needing to collect to Vec first. This also allows automatic detection of newly added slices
/// (though iterator recreation would still be needed after new slices are added).
pub struct FieldIterator<'slice, A: Allocator = Global> {
    /// Field slices container (wrapped in Rc to allow sharing)
    field_slices: Rc<OnceList<&'slice [u8], A>>,
    /// Allocator for lazy iterator initialization
    allocator: A,
    /// Flattened iterator over protobuf fields from all slices
    /// Lazily initialized from field_slices to avoid Vec collection
    field_iter: OnceCell<Box<dyn Iterator<Item = Result<Field<&'slice [u8]>, Error>> + 'slice, A>>,
}

impl<'slice, A: Allocator> FieldIterator<'slice, A>
where
    A: Allocator + Clone,
{
    /// Create a new FieldIterator from an initial slice
    /// The slice is stored in OnceList and used to create the field iterator
    pub fn new(initial_slice: &'slice [u8], alloc: A) -> Self
    where
        A: Allocator + Clone,
    {
        let field_slices = Rc::new(OnceList::new_in(alloc.clone()));
        field_slices.push(initial_slice);
        Self {
            field_slices,
            allocator: alloc,
            field_iter: OnceCell::new(),
        }
    }

    /// Create a new FieldIterator from an iterator of slices
    /// All slices are stored in OnceList and used to create the field iterator
    pub fn from_slices<I>(slice_iter: I, alloc: A) -> Self
    where
        I: Iterator<Item = &'slice [u8]>,
        A: Allocator + Clone,
    {
        let field_slices = Rc::new(OnceList::new_in(alloc.clone()));
        // Collect slices into OnceList first
        for slice in slice_iter {
            field_slices.push(slice);
        }

        Self {
            field_slices,
            allocator: alloc,
            field_iter: OnceCell::new(),
        }
    }

    /// Add a slice to the field slices container
    pub fn add_slice(&self, slice: &'slice [u8])
    where
        A: Allocator + Clone,
    {
        self.field_slices.push(slice);
    }

    /// Get an iterator over the field slices
    /// This is used to recreate the field iterator after adding new slices
    pub fn field_slices_iter(&self) -> impl Iterator<Item = &'slice [u8]> {
        self.field_slices.iter().copied()
    }

    /// Get the number of slices in the field slices container.
    ///
    /// This can be used to detect if new slices have been added during parent parsing.
    pub fn field_slices_count(&self) -> usize {
        self.field_slices.iter().count()
    }
}

impl<'slice, A: Allocator> Iterator for FieldIterator<'slice, A>
where
    A: Allocator + Clone,
{
    type Item = Result<Field<&'slice [u8]>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        // Lazily initialize iterator on first access if not already initialized
        // Even with Rc and OnceCell, we still need to collect to Vec to own the slices
        // and satisfy the 'slice lifetime requirement for the iterator
        if self.field_iter.get().is_none() {
            // Collect slices to Vec to transfer ownership and satisfy lifetime requirements
            let slices_vec: Vec<&'slice [u8]> = self.field_slices.iter().copied().collect();
            let alloc_clone = self.allocator.clone();
            let iter = slices_vec.into_iter().flat_map(|slice| {
                slice
                    .read_protobuf_fields()
                    .map(|result| result.map_err(|e| Error::from(e)))
            });
            let boxed = Box::new_in(iter, alloc_clone.clone());
            let field_iter: Box<
                dyn Iterator<Item = Result<Field<&'slice [u8]>, Error>> + 'slice,
                A,
            > = ::allocator_api2::unsize_box!(boxed);
            // Initialize OnceCell - this only works once
            let _ = self.field_iter.set(field_iter);
        }

        // get_mut() gives us &mut Box<...>, then as_mut() gives us &mut dyn Iterator
        self.field_iter
            .get_mut()
            .map(|iter_box| iter_box.as_mut().next())
            .flatten()
    }
}

/// Parser state for a message.
///
/// Generic across all message types - does not need to know the specific message type.
/// The entire State is wrapped in RefCell (it's a state, so it should be mutable).
///
/// - `'slice`: Lifetime of the input slices (external data)
pub struct MessageParserState<'slice, A: Allocator = Global> {
    /// FieldIterator - needs &mut self for Iterator::next()
    ///
    /// **State meanings:**
    /// - The iterator is always present and can be iterated. When `next()` returns `None`,
    ///   it means the iterator has been exhausted with the currently available input slices.
    ///   However, this does NOT mean the message parsing is complete - the parent message
    ///   might still have more input to provide. When `field_iter.next()` returns `None`,
    ///   the caller should request the parent message to continue parsing (via `parse_until(|_| false)`)
    ///   before assuming the message is fully parsed.
    ///
    /// **Important**: In lazy parsing, input slices might arrive incrementally. Therefore,
    /// `field_iter.next()` returning `None` only means "no more fields available *right now*", not
    /// "all fields have been parsed". The iterator can be extended with new slices via `add_slice()`.
    field_iter: FieldIterator<'slice, A>,
    allocator: A,
    /// Flag indicating whether the message has been terminated by a terminating operation.
    ///
    /// **State meanings:**
    /// - `false`: The message is still accepting new input slices. Parsing operations can
    ///   be performed incrementally (e.g., `parse_until` for conditional parsing).
    /// - `true`: The message has been fully parsed via `ensure_all_fields_parsed()` (a
    ///   terminating operation). No further input slices will be accepted, and the message
    ///   state is now immutable. Attempts to call `add_slice()` will return an error.
    ///
    /// **Key distinction**: `terminated = true` means "all fields have been parsed and
    /// no more input will be accepted", while `field_iter.next()` returning `None` only means
    /// "the current iterator is exhausted, but more input might arrive from the parent message".
    terminated: Cell<bool>,
    /// Parent parser state - strong Rc<RefCell<...>> reference (no cycle!)
    /// Child needs parent's parser state to request continued parsing
    /// None for top-level messages, Some(...) for child messages
    parent_parser_state: Option<Rc<RefCell<MessageParserState<'slice, A>>>>,
    /// Field update callback - handles field updates
    ///
    /// This callback is responsible for updating fields when iterating over field_iter.
    ///
    /// Initially, the callback captures a Weak reference to Message Body and calls update_field.
    /// When Message Body is dropped, Drop::drop updates this callback to one that captures
    /// child message Weak references and uses static match-case to update them directly.
    ///
    /// This design allows MessageParserState to be generic across all message types,
    /// as it doesn't need to know the specific message type at compile time.
    /// Use allocator_api2::boxed::Box to use the allocator A for consistency with MessageParserState's allocator.
    /// The callback captures Weak references by value.
    field_update_callback: Box<dyn FnMut(Field<&'slice [u8]>) -> Result<(), Error> + 'slice, A>,
}

impl<'slice, A: Allocator> MessageParserState<'slice, A> {
    /// Create a new MessageParserState with all parameters specified.
    ///
    /// `initial_slice` is the first byte slice that will be parsed as protobuf fields.
    ///
    /// - For top-level messages: pass `parent_parser_state: None`
    /// - For child messages: pass `parent_parser_state: Some(parent_parser_state)`
    pub fn new<F>(
        initial_slice: &'slice [u8],
        allocator: A,
        parent_parser_state: Option<Rc<RefCell<MessageParserState<'slice, A>>>>,
        field_update_callback: F,
    ) -> Self
    where
        F: FnMut(Field<&'slice [u8]>) -> Result<(), Error> + 'slice,
        A: Allocator + Clone,
    {
        let boxed = Box::new_in(field_update_callback, allocator.clone());
        let callback: Box<dyn FnMut(Field<&'slice [u8]>) -> Result<(), Error> + 'slice, A> =
            ::allocator_api2::unsize_box!(boxed);
        Self {
            field_iter: FieldIterator::new(initial_slice, allocator.clone()),
            allocator,
            terminated: Cell::new(false),
            parent_parser_state,
            field_update_callback: callback,
        }
    }

    /// Set the field iterator from a slice iterator.
    pub fn set_field_iter_from_slices<I>(&mut self, slice_iter: I)
    where
        I: Iterator<Item = &'slice [u8]>,
        A: Clone,
    {
        self.field_iter = FieldIterator::from_slices(slice_iter, self.allocator.clone());
    }

    /// Get a reference to the allocator.
    pub fn allocator(&self) -> &A {
        &self.allocator
    }

    /// Get a reference to the parent parser state, if any.
    pub fn parent_parser_state(&self) -> Option<&Rc<RefCell<MessageParserState<'slice, A>>>> {
        self.parent_parser_state.as_ref()
    }

    /// Add a slice to the field iterator
    /// This will add the slice to the underlying FieldIterator and recreate the field iterator
    ///
    /// **Important**: This method can only be called when `terminated = false`. Once
    /// `ensure_all_fields_parsed()` has been called, this method will return an error.
    ///
    /// If `field_iter` is `None` (exhausted), we recreate it with the new slice, allowing
    /// the iterator to continue parsing with the additional input.
    pub fn add_slice(&mut self, slice: &'slice [u8]) -> Result<(), Error>
    where
        A: Clone,
    {
        // Check if message has been terminated by ensure_all_fields_parsed()
        // Once terminated, no new slices can be added to maintain consistency
        if self.terminated.get() {
            return Err(Error::MessageTerminated);
        }

        // Add the slice to the iterator and recreate from all slices
        // This allows continuing parsing when more input arrives from the parent message
        self.field_iter.add_slice(slice);
        let slices_iter = self.field_iter.field_slices_iter();
        self.field_iter = FieldIterator::from_slices(slices_iter, self.allocator.clone());

        Ok(())
    }

    /// Set the field update callback.
    pub fn set_field_update_callback<F>(&mut self, callback: F)
    where
        F: FnMut(Field<&'slice [u8]>) -> Result<(), Error> + 'slice,
        A: Allocator + Clone,
    {
        let boxed = Box::new_in(callback, self.allocator.clone());
        let new_callback: Box<dyn FnMut(Field<&'slice [u8]>) -> Result<(), Error> + 'slice, A> =
            ::allocator_api2::unsize_box!(boxed);
        self.field_update_callback = new_callback;
    }

    /// Get the next field from the iterator, requesting parent to parse if needed.
    ///
    /// This method tries to get the next field from `field_iter`. If the iterator is exhausted,
    /// it requests the parent parser state (if it exists) to continue parsing, which may add
    /// more slices to this state's `field_iter`. The method stops parent parsing when new slices
    /// are added to `field_iter` (detected by slice count increase).
    ///
    /// Returns:
    /// - `Ok(Some(field))` - A field was successfully retrieved
    /// - `Ok(None)` - No more fields available (iterator and parent are exhausted)
    /// - `Err(error)` - An error occurred during parsing
    fn get_next_field(&mut self) -> Result<Option<Field<&'slice [u8]>>, Error>
    where
        A: Clone,
    {
        // Step 1: Try to get the next field from field_iter
        if let Some(result) = self.field_iter.next() {
            return Ok(Some(result?));
        }

        // field_iter is exhausted - try to get more from parent
        let Some(ref parent_state) = self.parent_parser_state else {
            // No parent - iterator is truly exhausted
            // Mark as terminated since no more input will be available
            self.terminated.set(true);
            return Ok(None);
        };

        // Remember the number of slices before requesting parent to parse
        let slices_count_before = self.field_iter.field_slices_count();

        // Request parent to continue parsing, which may add more slices to this state
        // Stop when field_iter gets new slices (indicated by slice count increase)
        parent_state.borrow_mut().parse_until(|_field| {
            // Check if new slices were added to our field_iter
            // We can access self.field_iter.field_slices_count() here because
            // parent_state.borrow_mut() doesn't conflict with self.field_iter
            let slices_count_after = self.field_iter.field_slices_count();
            slices_count_after > slices_count_before
        })?;

        // After parent parsing, check if field_iter has new slices
        // TODO: See FieldIterator struct documentation - we need to manually recreate the iterator
        // when new slices are added because the current implementation doesn't automatically detect
        // changes to field_slices. Once FieldIterator is refactored to detect new slices automatically,
        // this manual recreation can be removed.
        let slices_count_after = self.field_iter.field_slices_count();
        if slices_count_after > slices_count_before {
            // New slices were added - recreate the iterator to include them
            let slices_iter = self.field_iter.field_slices_iter();
            self.field_iter = FieldIterator::from_slices(slices_iter, self.allocator.clone());
        }

        // Try again to get the next field from field_iter
        match self.field_iter.next() {
            Some(result) => Ok(Some(result?)),
            None => {
                // Iterator is still exhausted - parent didn't add new slices
                // This means parent's iterator is also exhausted
                // Mark as terminated since no more input will be available
                self.terminated.set(true);
                Ok(None)
            }
        }
    }

    /// Internal method for parsing fields with configurable behavior.
    ///
    /// This is the common implementation used by `parse_until` and `ensure_all_fields_parsed`.
    ///
    /// **Important**: If the iterator is exhausted and the condition is not met, this method will
    /// automatically request the parent parser state (if it exists) to continue parsing. This ensures
    /// that lazy parsing works correctly with nested messages - when a child message needs more input,
    /// it can request its parent to continue parsing, which may in turn request its own parent.
    ///
    /// # Parameters
    /// * `condition` - Closure that takes a reference to a field and returns `true` when parsing should stop.
    ///   Pass `|_| false` if you want to parse all available fields.
    fn parse_fields_internal<F>(&mut self, mut condition: F) -> Result<(), Error>
    where
        F: FnMut(&Field<&'slice [u8]>) -> bool,
        A: Clone,
    {
        loop {
            // Step 1: Get the next field (handles parent parsing if needed)
            let field = match self.get_next_field()? {
                Some(field) => field,
                None => {
                    // No more fields available - iterator and parent are exhausted
                    return Ok(());
                }
            };

            // Step 2: Process the field
            let should_stop = condition(&field);
            (self.field_update_callback)(field)?;

            if should_stop {
                // Condition met - stop parsing
                return Ok(());
            }

            // Continue loop to get and process the next field
        }
    }

    /// Parse fields until a certain condition is met.
    ///
    /// This function reads fields from the input slice until the condition closure returns `true`.
    /// The condition closure receives a reference to the current field and should return `true` when parsing should stop.
    ///
    /// **Important**: If the iterator is exhausted and the condition is not met, this method will
    /// request the parent parser state (if it exists) to continue parsing. This ensures that
    /// lazy parsing works correctly with nested messages - when a child message needs more input,
    /// it can request its parent to continue parsing, which may in turn request its own parent.
    ///
    /// # Arguments
    /// * `condition` - A closure that takes a reference to a field and returns `true` when parsing should stop
    ///
    /// # Returns
    /// * `Ok(())` - Parsing completed (either condition was met or iterator exhausted)
    /// * `Err(Error)` - An error occurred during parsing
    pub fn parse_until<F>(&mut self, condition: F) -> Result<(), Error>
    where
        F: FnMut(&Field<&'slice [u8]>) -> bool,
        A: Clone,
    {
        // parse_fields_internal handles requesting parent to continue parsing
        // if the iterator is exhausted, so we just call it directly
        self.parse_fields_internal(condition)
    }
}

impl<'slice, A: Allocator + Clone> MessageParserState<'slice, A> {
    /// Ensure all fields are parsed
    ///
    /// This will request parent's parser state to continue parsing if needed (for child messages).
    /// Then parses all collected slices to extract message fields.
    ///
    /// This is a terminating operation - after this method completes, the message is marked as terminated
    /// and no additional slices can be added.
    pub fn ensure_all_fields_parsed(&mut self) -> Result<(), Error>
    where
        A: Clone,
    {
        // If already terminated, return early (idempotent operation)
        if self.terminated.get() {
            return Ok(());
        }

        // Request parent's parser state to continue parsing if this is a child message
        // This works even if parent Message Body is dropped, because:
        // 1. Child holds strong Rc reference to parent's Parser State
        // 2. parse_until doesn't require parent Message Body to be alive
        // 3. Callbacks (field_update_callback) already add slices to child messages via add_slice
        if let Some(ref parent_state) = self.parent_parser_state {
            parent_state.borrow_mut().parse_until(|_| false)?;
        }

        // Parse all fields until iterator is exhausted
        // Use a constant function that always returns false to parse all fields
        self.parse_fields_internal(|_| false)?;

        // Mark message as terminated after parsing all fields
        // This prevents adding new slices which would cause inconsistent behavior
        self.terminated.set(true);

        Ok(())
    }
}
