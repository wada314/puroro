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
/// - `'message`: Lifetime of the iterator itself (tied to MessageParserState's lifetime)
/// - `'slice: 'message`: Slices must outlive the iterator
pub struct FieldIterator<'slice, 'message, A: Allocator = Global>
where
    'slice: 'message,
{
    /// Field slices container
    field_slices: OnceList<&'slice [u8], A>,
    /// Flattened iterator over protobuf fields from all slices
    /// The iterator itself lives for 'message, but the slices it references live for 'slice
    field_iter: std::boxed::Box<dyn Iterator<Item = Result<Field<&'slice [u8]>, Error>> + 'message>,
}

impl<'slice, 'message, A: Allocator> FieldIterator<'slice, 'message, A>
where
    'slice: 'message,
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
}

impl<'slice, 'message, A: Allocator> Iterator for FieldIterator<'slice, 'message, A>
where
    'slice: 'message,
{
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
/// - `'message`: Lifetime of the callback and iterator (tied to MessageParserState's lifetime)
/// - `'slice: 'message`: Slices must outlive the message parser state
pub struct MessageParserState<'slice, 'message, A: Allocator = Global>
where
    'slice: 'message,
{
    /// FieldIterator - needs &mut self for Iterator::next()
    /// Use std::boxed::Box (not allocator_api2::Box) since FieldIterator doesn't need allocator-aware Box
    field_iter: Option<FieldIterator<'slice, 'message>>,
    allocator: A,
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
    /// The callback itself lives for 'message, but the slices it receives live for 'slice
    field_update_callback: Box<dyn FnMut(Field<&'slice [u8]>) -> Result<(), Error> + 'message, A>,
}

impl<'slice, 'message, A: Allocator> MessageParserState<'slice, 'message, A>
where
    'slice: 'message,
{
    /// Create a new MessageParserState with all parameters specified.
    ///
    /// `slice_iter` is an iterator over byte slices that will be parsed as protobuf fields.
    pub fn new<I>(
        slice_iter: I,
        allocator: A,
        field_update_callback: Box<
            dyn FnMut(Field<&'slice [u8]>) -> Result<(), Error> + 'message,
            A,
        >,
    ) -> Self
    where
        I: Iterator<Item = &'slice [u8]> + 'message,
    {
        Self {
            field_iter: Some(FieldIterator::new(slice_iter)),
            allocator,
            field_update_callback,
        }
    }

    /// Set the field iterator from a slice iterator.
    pub fn set_field_iter_from_slices<I>(&mut self, slice_iter: I)
    where
        I: Iterator<Item = &'slice [u8]> + 'message,
    {
        self.field_iter = Some(FieldIterator::new(slice_iter));
    }

    /// Take the field iterator, leaving None in its place.
    /// This is needed for RefCell borrow management when parsing fields.
    pub fn take_field_iter(&mut self) -> Option<FieldIterator<'slice, 'message>> {
        self.field_iter.take()
    }

    /// Set the field iterator.
    /// This is needed for RefCell borrow management when parsing fields.
    pub fn set_field_iter(&mut self, field_iter: Option<FieldIterator<'slice, 'message>>) {
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
    pub fn set_field_update_callback(
        &mut self,
        callback: Box<dyn FnMut(Field<&'slice [u8]>) -> Result<(), Error> + 'message, A>,
    ) {
        self.field_update_callback = callback;
    }
}

impl<'slice, 'message, A: Allocator + Clone> MessageParserState<'slice, 'message, A>
where
    'slice: 'message,
{
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
        let mut field_iter = match self.field_iter.take() {
            Some(iter) => iter,
            None => return Ok(()), // Already exhausted
        };

        // Parse until iterator is exhausted, updating ALL fields via callback
        loop {
            match field_iter.next() {
                Some(Ok(field)) => {
                    // Update field via callback (handles both Message Body and child messages)
                    let _ = (self.field_update_callback)(field);
                    // Ignore errors - Message Body or child might be dropped
                }
                Some(Err(e)) => {
                    // Restore iterator before returning error
                    self.field_iter = Some(field_iter);
                    return Err(e);
                }
                None => {
                    // Iterator exhausted
                    self.field_iter = None;
                    return Ok(());
                }
            }
        }
    }
}
