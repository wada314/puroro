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

/// Decode a varint-encoded value from a byte slice.
///
/// Returns the decoded value and the number of bytes consumed.
/// This is a simplified implementation - in production, use protobuf-core's varint decoding.
pub fn decode_varint(bytes: &[u8]) -> Result<(u64, usize), Error> {
    let mut result = 0u64;
    let mut shift = 0;
    let mut consumed = 0;

    for &byte in bytes {
        consumed += 1;
        result |= ((byte & 0x7F) as u64) << shift;

        if (byte & 0x80) == 0 {
            return Ok((result, consumed));
        }

        shift += 7;
        if shift >= 64 {
            return Err(Error::InvalidWireFormat("Varint too long".to_string()));
        }
    }

    Err(Error::InvalidWireFormat("Incomplete varint".to_string()))
}

/// Decode a field tag (varint-encoded field number and wire type) from a byte slice.
///
/// Returns (field_number, wire_type, bytes_consumed).
/// Wire type is encoded in the lower 3 bits of the tag.
/// Field number is encoded in the upper bits.
pub fn decode_field_tag(bytes: &[u8]) -> Result<(u32, u32, usize), Error> {
    let (tag, consumed) = decode_varint(bytes)?;
    let wire_type = (tag & 0x7) as u32;
    let field_number = (tag >> 3) as u32;

    if field_number == 0 {
        return Err(Error::InvalidWireFormat(
            "Field number cannot be zero".to_string(),
        ));
    }

    Ok((field_number, wire_type, consumed))
}

/// Parse a varint-encoded i32 value from a byte slice.
///
/// This is a convenience function that decodes a varint and casts it to i32.
pub fn parse_varint(bytes: &[u8]) -> Result<i32, Error> {
    let (value, _) = decode_varint(bytes)?;
    // Cast u64 to i32 (varint encoding uses zigzag encoding for signed integers,
    // but for simplicity in Phase 1, we just cast)
    Ok(value as i32)
}

/// Parse a UTF-8 string from a length-delimited field value slice.
///
/// The value_slice is the actual value bytes (without the length prefix).
/// This function validates UTF-8 and returns a String.
pub fn parse_string(bytes: &[u8]) -> Result<String, Error> {
    String::from_utf8(bytes.to_vec()).map_err(|e| Error::InvalidUtf8(e))
}

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
    field_iter: std::boxed::Box<dyn Iterator<Item = Result<Field<&'slice [u8]>, Error>> + 'slice>,
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
        Self {
            field_slices,
            field_iter: std::boxed::Box::new(field_iter),
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
        let field_iter: std::boxed::Box<
            dyn Iterator<Item = Result<Field<&'slice [u8]>, Error>> + 'slice,
        > = if slices_vec.is_empty() {
            std::boxed::Box::new(std::iter::empty())
        } else {
            let iter = slices_vec.into_iter().flat_map(|slice| {
                slice
                    .read_protobuf_fields()
                    .map(|result| result.map_err(|e| Error::from(e)))
            });
            std::boxed::Box::new(iter)
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
    /// Use std::boxed::Box (not allocator_api2::Box) since FieldIterator doesn't need allocator-aware Box
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
