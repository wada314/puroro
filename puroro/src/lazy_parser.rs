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
use ::std::cell::RefCell;
use ::std::rc::Rc;

/// Iterator over protobuf fields in slices.
///
/// Can be paused and resumed, making it easy to parse incrementally.
/// Uses flat_map approach to convert slice iterator to field iterator.
///
/// - `'slice`: Lifetime of the input slices (external data)
pub struct FieldIterator<'slice, A: Allocator = Global> {
    /// Field slices container
    field_slices: OnceList<&'slice [u8], A>,
    /// Flattened iterator over protobuf fields from all slices
    /// The slices it references live for 'slice
    field_iter: Box<dyn Iterator<Item = Result<Field<&'slice [u8]>, Error>> + 'slice, A>,
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
        let field_slices = OnceList::new_in(alloc.clone());
        field_slices.push(initial_slice);
        // Create field iterator from the initial slice
        // This will be updated later to recreate from field_slices
        let field_iter = initial_slice
            .read_protobuf_fields()
            .map(|result| result.map_err(|e| Error::from(e)));
        let boxed = Box::new_in(field_iter, alloc.clone());
        let field_iter: Box<dyn Iterator<Item = Result<Field<&'slice [u8]>, Error>> + 'slice, A> =
            ::allocator_api2::unsize_box!(boxed);
        Self {
            field_slices,
            field_iter,
        }
    }

    /// Create a new FieldIterator from an iterator of slices
    /// All slices are stored in OnceList and used to create the field iterator
    pub fn from_slices<I>(slice_iter: I, alloc: A) -> Self
    where
        I: Iterator<Item = &'slice [u8]>,
        A: Allocator + Clone,
    {
        let field_slices = OnceList::new_in(alloc.clone());
        // Collect slices into OnceList first
        for slice in slice_iter {
            field_slices.push(slice);
        }

        // Create field iterator from all slices using flat_map
        // We need to collect slices into a Vec to avoid lifetime issues
        // Note: This is a temporary solution - ideally we'd iterate directly from OnceList
        let slices_vec: Vec<&'slice [u8]> = field_slices.iter().copied().collect();
        let field_iter: Box<dyn Iterator<Item = Result<Field<&'slice [u8]>, Error>> + 'slice, A> =
            if slices_vec.is_empty() {
                let boxed = Box::new_in(std::iter::empty(), alloc.clone());
                ::allocator_api2::unsize_box!(boxed)
            } else {
                let iter = slices_vec.into_iter().flat_map(|slice| {
                    slice
                        .read_protobuf_fields()
                        .map(|result| result.map_err(|e| Error::from(e)))
                });
                let boxed = Box::new_in(iter, alloc.clone());
                ::allocator_api2::unsize_box!(boxed)
            };
        Self {
            field_slices,
            field_iter,
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
}

impl<'slice, A: Allocator> Iterator for FieldIterator<'slice, A> {
    type Item = Result<Field<&'slice [u8]>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.field_iter.next()
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
            // Parse fields until condition is met or iterator is exhausted
            // When next() returns None, it means the iterator is exhausted with currently available input.
            // In lazy parsing, this is temporary - more input might arrive from the parent message.
            for result in &mut self.field_iter {
                let field = result?;

                // Check if condition is met
                let should_stop = condition(&field);

                // Update field via callback
                (self.field_update_callback)(field)?;

                if should_stop {
                    // Condition met - stop parsing
                    // The iterator state is preserved, so we can continue from here later
                    return Ok(());
                }
            }

            // Iterator exhausted - next() returned None
            // If parent exists, request it to continue parsing
            if let Some(ref parent_state) = self.parent_parser_state {
                // Request parent to continue parsing, which may add more slices to this state
                parent_state.borrow_mut().parse_until(|_| false)?;

                // After parent continues parsing, check if iterator can yield more fields
                // If iterator is still exhausted (parent didn't add new slices), stop
                // We check this by trying to peek at the next field
                // If we can't get a field, iterator is still exhausted - stop
                if let Some(result) = self.field_iter.next() {
                    // Parent added new slices - process the field
                    let field = result?;
                    let should_stop = condition(&field);
                    (self.field_update_callback)(field)?;
                    if should_stop {
                        // Condition met - stop parsing
                        return Ok(());
                    }
                    // Continue loop to parse more fields from new slices
                } else {
                    // Iterator is still exhausted - parent didn't add new slices
                    // This means parent's iterator is also exhausted
                    return Ok(());
                }
            } else {
                // No parent - iterator is truly exhausted
                return Ok(());
            }
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
