//! Lazy parser infrastructure for Protocol Buffer messages.
//!
//! This module provides the core parsing infrastructure for lazy deserialization:
//! - FieldIterator: Iterator over protobuf fields in slices
//! - MessageParserState: Parser state that can be shared between message bodies and child messages
//! - Helper functions for wire format parsing (varint decoding, field tag parsing)

use ::allocator_extras::{Allocator, Global};
use once_list2::OnceList;
use puroro::error::Error;

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
            return Err(Error::InvalidWireFormat(
                "Varint too long".to_string(),
            ));
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
/// Tracks the current slice and position within that slice.
pub struct FieldIterator<'a> {
    /// Iterator over the slices
    /// This iterator maintains its own state
    slice_iter: std::boxed::Box<dyn Iterator<Item = &'a [u8]> + 'a>,
    /// Current slice being parsed
    current_slice: Option<&'a [u8]>,
    /// Current position within the current slice
    position: usize,
}

impl<'a> FieldIterator<'a> {
    /// Create a new FieldIterator from any iterator over slices
    /// The iterator is created once and maintains its state - no need to recreate it
    pub fn new(slice_iter: std::boxed::Box<dyn Iterator<Item = &'a [u8]> + 'a>) -> Self {
        Self {
            slice_iter,
            current_slice: None,
            position: 0,
        }
    }
}

impl<'a> Iterator for FieldIterator<'a> {
    type Item = Result<(u32, u32, &'a [u8]), Error>; // field_num, wire_type, value_slice

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Get current slice or advance to next slice
            if let Some(slice) = self.current_slice {
                if self.position < slice.len() {
                    // Parse field tag
                    let remaining = &slice[self.position..];
                    let (field_num, wire_type, tag_bytes) = match decode_field_tag(remaining) {
                        Ok(tag) => tag,
                        Err(e) => return Some(Err(e)),
                    };

                    let value_start = self.position + tag_bytes;

                    // Extract value slice based on wire type
                    let value_slice = match wire_type {
                        0 => {
                            // Varint - read varint to find its length
                            let (_, varint_bytes) = match decode_varint(&remaining[tag_bytes..]) {
                                Ok(v) => v,
                                Err(e) => return Some(Err(e)),
                            };
                            let value_end = value_start + varint_bytes;
                            if value_end > slice.len() {
                                return Some(Err(Error::InvalidWireFormat(
                                    "Varint extends beyond slice".to_string(),
                                )));
                            }
                            &slice[value_start..value_end]
                        }
                        2 => {
                            // Length-delimited - read length prefix (varint)
                            let (length, length_bytes) =
                                match decode_varint(&remaining[tag_bytes..]) {
                                    Ok(v) => v,
                                    Err(e) => return Some(Err(e)),
                                };
                            let value_start_with_length = value_start + length_bytes;
                            let value_end = value_start_with_length + length as usize;
                            if value_end > slice.len() {
                                // Value might span multiple slices - we'll handle this later
                                // For now, return the available portion
                                &slice[value_start_with_length..]
                            } else {
                                &slice[value_start_with_length..value_end]
                            }
                        }
                        _ => {
                            return Some(Err(Error::InvalidWireFormat(format!(
                                "Unsupported wire type: {}",
                                wire_type
                            ))));
                        }
                    };

                    // Update position
                    // For wire type 2 (length-delimited), we already computed length_bytes above
                    // value_slice is just the value part (without length prefix)
                    // So we need to account for: tag_bytes + length_bytes + value_length
                    if wire_type == 2 {
                        // For length-delimited, we already computed length_bytes above (in the match arm)
                        // Re-decode to get length_bytes for position calculation
                        let (_, length_bytes) = match decode_varint(&remaining[tag_bytes..]) {
                            Ok(v) => v,
                            Err(e) => return Some(Err(e)),
                        };
                        self.position += tag_bytes + length_bytes + value_slice.len();
                    } else {
                        // For varint, value_slice contains the entire varint
                        self.position += tag_bytes + value_slice.len();
                    }

                    return Some(Ok((field_num, wire_type, value_slice)));
                }
            }

            // Current slice exhausted, move to next slice using iterator
            self.current_slice = self.slice_iter.next();
            self.position = 0;

            if self.current_slice.is_none() {
                // All slices exhausted
                return None;
            }
        }
    }
}

/// Parser state for a message.
///
/// Generic across all message types - does not need to know the specific message type.
/// The entire State is wrapped in RefCell (it's a state, so it should be mutable).
pub struct MessageParserState<'a, A: Allocator = Global> {
    /// FieldIterator - needs &mut self for Iterator::next()
    /// Use std::boxed::Box (not allocator_api2::Box) since FieldIterator doesn't need allocator-aware Box
    pub field_iter: Option<FieldIterator<'a>>,
    pub allocator: A,
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
    /// Use std::boxed::Box (not allocator_api2::Box) for consistency
    pub field_update_callback: Option<std::boxed::Box<dyn FnMut(u32, u32, &'a [u8]) -> Result<(), Error> + 'a>>,
}

impl<'a, A: Allocator + Clone> MessageParserState<'a, A> {
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
                Some(Ok((fnum, wire_type, value_slice))) => {
                    // Update field via callback (handles both Message Body and child messages)
                    if let Some(ref mut callback) = self.field_update_callback {
                        let _ = callback(fnum, wire_type, value_slice);
                        // Ignore errors - Message Body or child might be dropped
                    }
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

