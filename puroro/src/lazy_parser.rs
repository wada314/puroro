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
use ::protobuf_core::{AsRefExtProtobuf, Field, ProtobufFieldSliceIterator};
use ::std::cell::Cell;
use ::std::cell::RefCell;
use ::std::iter::Peekable;
use ::std::rc::Rc;

/// Iterator over protobuf fields in slices.
///
/// Can be paused and resumed, making it easy to parse incrementally.
/// Uses flat_map approach to convert slice iterator to field iterator.
///
/// - `'slice`: Lifetime of the input slices (external data)
///
/// Instead of storing slices and creating an iterator from them, we store a list of iterators,
/// one per slice. When a slice is added via `add_slice()`, a new iterator is created and appended
/// to the list. When `next()` is called, we get the first iterator and try to get an item from it.
/// If the iterator is exhausted, we remove it and try the next one. This allows automatic
/// detection of newly added slices without iterator recreation nor self-referential structs.
pub struct FieldIterator<'slice, A: Allocator = Global> {
    /// List of field iterators, one per slice
    /// Each iterator is generated from its corresponding slice
    /// Using Peekable to allow checking if iterator has next item without consuming it
    /// ProtobufFieldSliceIterator already returns Field<&'slice [u8]>
    field_iterators: OnceList<Peekable<ProtobufFieldSliceIterator<'slice>>, A>,
}

impl<'slice, A: Allocator> FieldIterator<'slice, A>
where
    A: Allocator + Clone,
{
    /// Create a new FieldIterator from an initial slice
    /// Creates an iterator from the slice and stores it in the list
    pub fn new(initial_slice: &'slice [u8], alloc: A) -> Self
    where
        A: Allocator + Clone,
    {
        let field_iterators = OnceList::new_in(alloc.clone());
        // Create peekable iterator from the initial slice
        let iter = initial_slice.read_protobuf_fields().peekable();
        field_iterators.push(iter);
        Self { field_iterators }
    }

    /// Create a new FieldIterator from an iterator of slices
    /// Creates an iterator from each slice and stores them in the list
    pub fn from_slices<I>(slice_iter: I, alloc: A) -> Self
    where
        I: Iterator<Item = &'slice [u8]>,
        A: Allocator + Clone,
    {
        let field_iterators = OnceList::new_in(alloc.clone());
        for slice in slice_iter {
            // Create peekable iterator from slice
            let iter = slice.read_protobuf_fields().peekable();
            field_iterators.push(iter);
        }

        Self { field_iterators }
    }

    /// Add a slice to the field iterators list
    /// Creates a new iterator from the slice and appends it to the list
    pub fn add_slice(&self, slice: &'slice [u8])
    where
        A: Allocator + Clone,
    {
        // Create peekable iterator from slice
        let iter = slice.read_protobuf_fields().peekable();
        self.field_iterators.push(iter);
    }

    /// Check if there is at least one more slice (iterator) available
    ///
    /// Returns `true` if `field_iterators` is not empty.
    /// This returns `true` even if the slice is empty (iterator has no items).
    /// To check if there are actually items, use the iterator's `next()` method.
    ///
    /// This method does not require mutation and does not consume any items.
    pub fn has_next_slice(&self) -> bool {
        !self.field_iterators.is_empty()
    }
}

impl<'slice, A: Allocator> Iterator for FieldIterator<'slice, A> {
    type Item = Result<Field<&'slice [u8]>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(first_iter_mut) = self.field_iterators.first_mut() {
            // Try to get next item from the first iterator
            // Convert error type at this point (ProtobufError -> Error)
            if let Some(item) = first_iter_mut.next() {
                return Some(item.map_err(|e| Error::from(e)));
            }

            // First iterator is exhausted, remove it
            self.field_iterators.remove(|_| true);
            // Continue loop to try next iterator (if any)
        }
        None
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

        // Add the slice to the iterator
        // The new approach automatically detects new slices without recreation
        self.field_iter.add_slice(slice);

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
    /// it checks if there are more slices available (via `has_next_slice()`) or requests the parent
    /// parser state (if it exists) to continue parsing, which may add more slices to this state's
    /// `field_iter`. The method stops parent parsing when new slices are added to `field_iter`.
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

        // field_iter is exhausted - check if we have more slices available from parent
        //
        // Note: When field_iter is exhausted (next() returns None), has_next_slice() is guaranteed
        // to be false. So we can simply check if has_next_slice() becomes true to detect new slices.

        // Check if parent exists and can provide more slices
        let Some(ref parent_state) = self.parent_parser_state else {
            // No parent - iterator is truly exhausted
            // Mark as terminated since no more input will be available
            self.terminated.set(true);
            return Ok(None);
        };

        // Request parent to continue parsing, which may add more slices to this state
        // Stop when field_iter gets new slices (has_next_slice() becomes true)
        parent_state.borrow_mut().parse_until(|_field| {
            // Check if new slices were added to our field_iter
            // Since has_next_slice() is false when exhausted, becoming true means new slices added
            self.field_iter.has_next_slice()
        })?;

        // After parent parsing, new iterators should have been added automatically
        // The iterator will automatically see new items via first_mut() without recreation

        // Try again to get the next field from field_iter
        match self.field_iter.next() {
            Some(result) => Ok(Some(result?)),
            None => {
                // Iterator is still exhausted - check if parent can still provide slices
                // by checking if we have any slices available
                if !self.field_iter.has_next_slice() {
                    // No slices available - parent's iterator is also exhausted
                    // Mark as terminated since no more input will be available
                    self.terminated.set(true);
                }
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
