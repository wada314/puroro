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
    field_iter: Option<FieldIterator<'slice, A>>,
    allocator: A,
    /// Flag indicating whether the message has been terminated by a terminating getter.
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
            field_iter: Some(FieldIterator::new(initial_slice, allocator.clone())),
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
        self.field_iter = Some(FieldIterator::from_slices(
            slice_iter,
            self.allocator.clone(),
        ));
    }

    /// Take the field iterator, leaving None in its place.
    /// This is needed for RefCell borrow management when parsing fields.
    pub fn take_field_iter(&mut self) -> Option<FieldIterator<'slice, A>> {
        self.field_iter.take()
    }

    /// Set the field iterator.
    /// This is needed for RefCell borrow management when parsing fields.
    pub fn set_field_iter(&mut self, field_iter: Option<FieldIterator<'slice, A>>) {
        self.field_iter = field_iter;
    }

    /// Check if the field iterator is exhausted (None).
    pub fn is_field_iter_exhausted(&self) -> bool {
        self.field_iter.is_none()
    }

    /// Get a reference to the allocator.
    pub fn allocator(&self) -> &A {
        &self.allocator
    }

    /// Add a slice to the field iterator
    /// This will add the slice to the underlying FieldIterator and recreate the field iterator
    pub fn add_slice(&mut self, slice: &'slice [u8]) -> Result<(), Error>
    where
        A: Clone,
    {
        // Check if message has been terminated by a terminating getter
        // Terminating getters (e.g., scalar field getters that check all slices) make the
        // message state immutable to maintain consistency.
        if self.terminated.get() {
            return Err(Error::MessageTerminated);
        }

        // If field_iter is None (already exhausted), we can't add more slices
        // This is a design decision - once exhausted, we don't allow adding more slices
        if self.field_iter.is_none() {
            return Err(Error::InvalidWireFormat(
                "Cannot add slice: field iterator is exhausted".to_string(),
            ));
        }

        // Take the field iterator, add the slice, and recreate the iterator
        if let Some(field_iter) = self.field_iter.take() {
            field_iter.add_slice(slice);
            // Recreate the field iterator from all slices
            let slices_iter = field_iter.field_slices_iter();
            self.field_iter = Some(FieldIterator::from_slices(
                slices_iter,
                self.allocator.clone(),
            ));
        }

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

    /// Parse fields until a certain condition is met.
    ///
    /// This function reads fields from the input slice until the condition closure returns `true`.
    /// The condition closure receives the current field and should return `true` when parsing should stop.
    ///
    /// # Arguments
    /// * `condition` - A closure that takes a field and returns `true` when parsing should stop
    ///
    /// # Returns
    /// * `Ok(())` - Parsing completed (either condition was met or iterator exhausted)
    /// * `Err(Error)` - An error occurred during parsing
    pub fn parse_until<F>(&mut self, mut condition: F) -> Result<(), Error>
    where
        F: FnMut(Field<&'slice [u8]>) -> bool,
    {
        // If iterator is None (already exhausted), we're done
        let mut field_iter = match self.field_iter.take() {
            Some(iter) => iter,
            None => return Ok(()), // Already parsed completely
        };

        // Parse until condition is met
        loop {
            match field_iter.next() {
                Some(Ok(field)) => {
                    // Check if condition is met (clone field for condition check)
                    let should_stop = condition(field.clone());
                    if should_stop {
                        // Store iterator back (not exhausted, condition was met)
                        self.field_iter = Some(field_iter);
                        return Ok(());
                    }

                    // Update field via callback
                    (self.field_update_callback)(field)?;
                }
                Some(Err(e)) => {
                    // Store iterator back before returning error
                    self.field_iter = Some(field_iter);
                    return Err(e);
                }
                None => {
                    // Iterator exhausted - store None to indicate parsing is complete
                    self.field_iter = None;
                    return Ok(());
                }
            }
        }
    }
}

impl<'slice, A: Allocator + Clone> MessageParserState<'slice, A> {
    /// Continue parsing and update all registered fields via the callback
    /// Called by child when it needs all slices - works even if parent Message Body is dropped
    ///
    /// **Key Design**: Uses a single callback mechanism that is updated when Message Body is dropped:
    /// - Initially: Callback captures Weak reference to Message Body and calls update_field
    /// - After Drop: Callback is replaced with one that captures child Weak references and uses static match-case
    ///
    /// **Important**: When Message Body is dropped, the callback updates ALL registered child message fields.
    /// This ensures that multiple scalar message child fields can all receive their slices when
    /// any one of them calls this method.
    pub fn continue_parsing_for_children(&mut self) -> Result<(), Error> {
        // Parse using the existing field iterator
        // If field_iter is None (already exhausted), we're done
        if let Some(ref mut field_iter) = self.field_iter {
            loop {
                match field_iter.next() {
                    Some(Ok(field)) => {
                        // Update field via callback (handles both Message Body and child messages)
                        let _ = (self.field_update_callback)(field);
                        // Ignore errors - Message Body or child might be dropped
                    }
                    Some(Err(e)) => {
                        return Err(e);
                    }
                    None => {
                        // Iterator exhausted, mark it as None
                        self.field_iter = None;
                        return Ok(());
                    }
                }
            }
        }
        Ok(())
    }

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
        // 2. continue_parsing_for_children doesn't require parent Message Body to be alive
        // 3. Callbacks (field_update_callback) already add slices to child messages via add_slice
        if let Some(ref parent_state) = self.parent_parser_state {
            parent_state.borrow_mut().continue_parsing_for_children()?;
        }

        // Parse using the existing field iterator
        // If field_iter is None (already exhausted or never created), we're done
        // Otherwise, we'll parse until exhausted
        if let Some(ref mut field_iter) = self.field_iter {
            loop {
                match field_iter.next() {
                    Some(Ok(field)) => {
                        // Update field via callback
                        (self.field_update_callback)(field)?;
                    }
                    Some(Err(e)) => {
                        return Err(e);
                    }
                    None => {
                        // Iterator exhausted, mark it as None
                        self.field_iter = None;
                        break;
                    }
                }
            }
        }

        // Mark message as terminated after parsing all fields
        // This prevents adding new slices which would cause inconsistent behavior
        self.terminated.set(true);

        Ok(())
    }
}
