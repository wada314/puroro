//! Lazy parser infrastructure for Protocol Buffer messages.
//!
//! This module provides the core parsing infrastructure for lazy deserialization:
//! - FieldIterator: Iterator over protobuf fields in slices
//! - MessageParserStateRef: Parser state that can be shared between message bodies and child messages
//! - Helper functions for wire format parsing (varint decoding, field tag parsing)

use crate::error::Error;
use ::allocator_extras::{Allocator, Global};
use ::once_list2::OnceList;
use ::protobuf_core::{AsRefExtProtobuf, Field, ProtobufFieldSliceIterator};
use ::std::cell::RefCell;
use ::std::iter::Peekable;
use ::std::rc::Rc;

/// Iterator over protobuf fields in slices.
///
/// Can be paused and resumed, making it easy to parse incrementally.
/// Internally stores a list of per-slice iterators so new slices can be appended
/// without recreating a single "global" iterator or using self-referential structs.
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
pub(crate) struct MessageParserStateInner<'slice, A: Allocator = Global> {
    /// FieldIterator - needs &mut self for Iterator::next()
    ///
    /// **State meanings:**
    /// - The iterator is always present and can be iterated. When `next()` returns `None`,
    ///   it means the iterator has been exhausted with the currently available input slices.
    ///   However, this does NOT mean the message parsing is complete - the parent message
    ///   might still have more input to provide. When `field_iter.next()` returns `None`,
    ///   the caller should request the parent message to continue parsing (via `parse_until_with_callback(|| false)`)
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
    /// - `true`: The message has been fully parsed via `ensure_all_fields_parsed_with_callback()` (a
    ///   terminating operation). No further input slices will be accepted, and the message
    ///   state is now immutable. Attempts to call `add_slice()` will return an error.
    ///
    /// **Key distinction**: `terminated = true` means "all fields have been parsed and
    /// no more input will be accepted", while `field_iter.next()` returning `None` only means
    /// "the current iterator is exhausted, but more input might arrive from the parent message".
    terminated: bool,
    /// Parent parser state - strong Rc<RefCell<...>> reference (no cycle!)
    /// Child needs parent's parser state to request continued parsing
    /// None for top-level messages, Some(...) for child messages
    parent_parser_state: Option<Rc<RefCell<MessageParserStateInner<'slice, A>>>>,
    /// Field update callback - handles field updates
    ///
    /// This callback is responsible for updating fields when iterating over field_iter.
    ///
    /// Initially, the callback captures a Weak reference to Message Body and calls update_field.
    /// When Message Body is dropped, Drop::drop updates this callback to one that captures
    /// child message Weak references and uses static match-case to update them directly.
    ///
    /// This design allows MessageParserStateInner to be generic across all message types,
    /// as it doesn't need to know the specific message type at compile time.
    /// Use allocator_api2::boxed::Box to use the allocator A for consistency with MessageParserStateInner's allocator.
    /// The callback captures Weak references by value.
    field_update_callback: Rc<dyn Fn(Field<&'slice [u8]>) -> Result<(), Error> + 'slice>,
}

impl<'slice, A: Allocator> MessageParserStateInner<'slice, A> {
    /// Create a new MessageParserStateInner with all parameters specified.
    ///
    /// `initial_slice` is the first byte slice that will be parsed as protobuf fields.
    ///
    /// - For top-level messages: pass `parent_parser_state: None`
    /// - For child messages: pass `parent_parser_state: Some(parent_parser_state)`
    pub(crate) fn new<F>(
        initial_slice: &'slice [u8],
        allocator: A,
        parent_parser_state: Option<Rc<RefCell<MessageParserStateInner<'slice, A>>>>,
        field_update_callback: F,
    ) -> Self
    where
        F: Fn(Field<&'slice [u8]>) -> Result<(), Error> + 'slice,
        A: Allocator + Clone,
    {
        Self {
            field_iter: FieldIterator::new(initial_slice, allocator.clone()),
            allocator: allocator.clone(),
            terminated: false,
            parent_parser_state,
            field_update_callback: Rc::new(field_update_callback),
        }
    }

    /// Get a reference to the allocator.
    pub fn allocator(&self) -> &A {
        &self.allocator
    }

    /// Add a slice to the field iterator
    /// This will append a new per-slice iterator to the underlying `FieldIterator`.
    ///
    /// **Important**: This method can only be called when `terminated = false`. Once
    /// `ensure_all_fields_parsed_with_callback()` has been called, this method will return an error.
    ///
    /// The existing iterators are not recreated; they are consumed in order.
    pub fn add_slice(&mut self, slice: &'slice [u8]) -> Result<(), Error>
    where
        A: Clone,
    {
        // Check if message has been terminated by ensure_all_fields_parsed_with_callback()
        // Once terminated, no new slices can be added to maintain consistency
        if self.terminated {
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
        F: Fn(Field<&'slice [u8]>) -> Result<(), Error> + 'slice,
        A: Allocator + Clone,
    {
        self.field_update_callback = Rc::new(callback);
    }
}

/// Cheaply cloneable handle to shared parser state (Rc<RefCell<...>>).
#[derive(Clone)]
pub struct MessageParserStateRef<'slice, A: Allocator = Global> {
    state: Rc<RefCell<MessageParserStateInner<'slice, A>>>,
}

impl<'slice, A: Allocator + Clone> MessageParserStateRef<'slice, A> {
    pub fn create<F>(
        initial_slice: &'slice [u8],
        allocator: A,
        parent_parser_state: Option<MessageParserStateRef<'slice, A>>,
        field_update_callback: F,
    ) -> Self
    where
        F: Fn(Field<&'slice [u8]>) -> Result<(), Error> + 'slice,
    {
        let parent_inner = parent_parser_state.map(|parent| parent.state.clone());
        let inner = MessageParserStateInner::new(
            initial_slice,
            allocator,
            parent_inner,
            field_update_callback,
        );
        Self {
            state: Rc::new(RefCell::new(inner)),
        }
    }

    pub(crate) fn from_inner(state: Rc<RefCell<MessageParserStateInner<'slice, A>>>) -> Self {
        Self { state }
    }

    pub fn add_slice(&self, slice: &'slice [u8]) -> Result<(), Error> {
        self.state.borrow_mut().add_slice(slice)
    }

    pub fn allocator(&self) -> A {
        self.state.borrow().allocator().clone()
    }

    pub fn set_field_update_callback<F>(&self, callback: F)
    where
        F: Fn(Field<&'slice [u8]>) -> Result<(), Error> + 'slice,
    {
        self.state.borrow_mut().set_field_update_callback(callback);
    }

    /// Get the next field from the iterator, requesting parent to parse if needed.
    ///
    /// This method uses short borrows on parser state so callbacks can run without
    /// overlapping with state borrows.
    fn next_field(&self) -> Result<Option<Field<&'slice [u8]>>, Error> {
        loop {
            let parent_state = {
                let mut state = self.state.borrow_mut();
                if let Some(result) = state.field_iter.next() {
                    return Ok(Some(result?));
                }
                state.parent_parser_state.clone()
            };

            let Some(parent_state) = parent_state else {
                let mut state = self.state.borrow_mut();
                state.terminated = true;
                return Ok(None);
            };

            MessageParserStateRef::from_inner(parent_state).parse_until_with_callback(|| {
                let state = self.state.borrow();
                state.field_iter.has_next_slice()
            })?;

            let has_next_slice = {
                let state = self.state.borrow();
                state.field_iter.has_next_slice()
            };

            if !has_next_slice {
                let mut state = self.state.borrow_mut();
                state.terminated = true;
                return Ok(None);
            }
        }
    }

    /// Parse fields until a certain condition is met.
    ///
    /// This function reads fields from the input slice until the condition closure returns `true`.
    /// The condition closure is evaluated after each field is processed via the field update callback,
    /// and should return `true` when parsing should stop.
    ///
    /// **Important**: If the iterator is exhausted and the condition is not met, this method will
    /// request the parent parser state (if it exists) to continue parsing. This ensures that
    /// lazy parsing works correctly with nested messages - when a child message needs more input,
    /// it can request its parent to continue parsing, which may in turn request its own parent.
    ///
    /// # Arguments
    /// * `condition` - A closure that returns `true` when parsing should stop
    ///
    /// # Returns
    /// * `Ok(())` - Parsing completed (either condition was met or iterator exhausted)
    /// * `Err(Error)` - An error occurred during parsing
    pub fn parse_until_with_callback<F>(&self, mut condition: F) -> Result<(), Error>
    where
        F: FnMut() -> bool,
    {
        while let Some(field) = self.next_field()? {
            let callback = {
                let state = self.state.borrow();
                state.field_update_callback.clone()
            };

            (callback)(field)?;

            if condition() {
                return Ok(());
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
    pub fn ensure_all_fields_parsed_with_callback(&self) -> Result<(), Error> {
        let parent_state = {
            let state = self.state.borrow_mut();
            if state.terminated {
                return Ok(());
            }
            state.parent_parser_state.clone()
        };

        if let Some(parent_state) = parent_state {
            MessageParserStateRef::from_inner(parent_state).parse_until_with_callback(|| false)?;
        }

        self.parse_until_with_callback(|| false)?;

        let mut state = self.state.borrow_mut();
        state.terminated = true;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{FieldIterator, MessageParserStateRef};
    use crate::error::Error;
    use ::allocator_extras::Global;
    use ::protobuf_core::{Field, FieldNumber, FieldValue, WriteExtProtobuf};
    use ::std::cell::RefCell;

    fn build_varint_field_bytes(field_number: u32, value: i32) -> Vec<u8> {
        let field = Field::<Vec<u8>>::new(
            FieldNumber::try_from(field_number).unwrap(),
            FieldValue::from_int32(value),
        );
        let mut bytes = Vec::new();
        bytes.write_protobuf_field(&field).unwrap();
        bytes
    }

    #[test]
    fn test_field_iterator_empty_slice() {
        let empty_slice: &[u8] = &[];
        let iter = FieldIterator::new(empty_slice, Global);

        // Should have at least one slice (even if empty)
        assert!(iter.has_next_slice());

        // But iterator should return None immediately
        let mut iter = iter;
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_field_iterator_single_field() {
        let encoded = build_varint_field_bytes(1, 42);
        let iter = FieldIterator::new(&encoded, Global);

        let mut iter = iter;
        let field_result = iter.next();
        assert!(field_result.is_some());

        let field = field_result.unwrap().unwrap();
        assert_eq!(field.field_number.as_u32(), 1);
        if let FieldValue::Varint(varint) = field.value {
            assert_eq!(varint.try_to_int32().unwrap(), 42);
        } else {
            panic!("Expected Varint field value");
        }

        // Should be exhausted now
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_field_iterator_multiple_fields() {
        let mut encoded = build_varint_field_bytes(1, 10);
        encoded.extend_from_slice(&build_varint_field_bytes(2, 20));
        encoded.extend_from_slice(&build_varint_field_bytes(3, 30));

        let iter = FieldIterator::new(&encoded, Global);
        let mut iter = iter;

        // First field
        let field1 = iter.next().unwrap().unwrap();
        assert_eq!(field1.field_number.as_u32(), 1);

        // Second field
        let field2 = iter.next().unwrap().unwrap();
        assert_eq!(field2.field_number.as_u32(), 2);

        // Third field
        let field3 = iter.next().unwrap().unwrap();
        assert_eq!(field3.field_number.as_u32(), 3);

        // Should be exhausted
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_field_iterator_add_slice() {
        let first_slice = build_varint_field_bytes(1, 10);
        let second_slice = build_varint_field_bytes(2, 20);

        let iter = FieldIterator::new(&first_slice, Global);
        iter.add_slice(&second_slice);

        let mut iter = iter;

        // First field from first slice
        let field1 = iter.next().unwrap().unwrap();
        assert_eq!(field1.field_number.as_u32(), 1);

        // Second field from second slice
        let field2 = iter.next().unwrap().unwrap();
        assert_eq!(field2.field_number.as_u32(), 2);

        // Should be exhausted
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_field_iterator_from_slices() {
        let slice1 = build_varint_field_bytes(1, 10);
        let slice2 = build_varint_field_bytes(2, 20);
        let slice3 = build_varint_field_bytes(3, 30);

        let slices = vec![slice1.as_slice(), slice2.as_slice(), slice3.as_slice()];
        let iter = FieldIterator::from_slices(slices.into_iter(), Global);

        let mut iter = iter;

        let field1 = iter.next().unwrap().unwrap();
        assert_eq!(field1.field_number.as_u32(), 1);

        let field2 = iter.next().unwrap().unwrap();
        assert_eq!(field2.field_number.as_u32(), 2);

        let field3 = iter.next().unwrap().unwrap();
        assert_eq!(field3.field_number.as_u32(), 3);

        assert!(iter.next().is_none());
    }

    #[test]
    fn test_field_iterator_has_next_slice() {
        let iter = FieldIterator::new(&[], Global);
        assert!(iter.has_next_slice());

        // After iterator is exhausted, the empty iterator is removed from the list
        // so has_next_slice() should return false
        let mut iter = iter;
        assert!(iter.next().is_none());
        assert!(!iter.has_next_slice()); // No more slices after exhaustion
    }

    #[test]
    fn test_message_parser_state_ref_basic() {
        let encoded = build_varint_field_bytes(1, 42);
        let captured_field = RefCell::new(None::<Field<&[u8]>>);

        let captured_field_clone = &captured_field;
        let parser_state = MessageParserStateRef::create(&encoded, Global, None, |field| {
            *captured_field_clone.borrow_mut() = Some(field);
            Ok(())
        });

        // Parse until callback returns true
        parser_state
            .parse_until_with_callback(|| captured_field.borrow().is_some())
            .unwrap();

        let field_num = {
            let borrowed = captured_field.borrow();
            assert!(borrowed.is_some());
            borrowed.as_ref().unwrap().field_number.as_u32()
        };
        assert_eq!(field_num, 1);
    }

    #[test]
    fn test_message_parser_state_ref_add_slice() {
        let first_slice = build_varint_field_bytes(1, 10);
        let second_slice = build_varint_field_bytes(2, 20);
        let captured_fields = RefCell::new(Vec::new());

        let captured_fields_clone = &captured_fields;
        let parser_state = MessageParserStateRef::create(&first_slice, Global, None, |field| {
            captured_fields_clone
                .borrow_mut()
                .push(field.field_number.as_u32());
            Ok(())
        });

        // Add second slice
        parser_state.add_slice(&second_slice).unwrap();

        // Parse all fields
        parser_state.parse_until_with_callback(|| false).unwrap();

        assert_eq!(*captured_fields.borrow(), vec![1, 2]);
    }

    #[test]
    fn test_message_parser_state_ref_terminated_error() {
        let encoded = build_varint_field_bytes(1, 42);
        let parser_state = MessageParserStateRef::create(&encoded, Global, None, |_field| Ok(()));

        // Terminate the message
        parser_state
            .ensure_all_fields_parsed_with_callback()
            .unwrap();

        // Attempting to add slice after termination should fail
        let result = parser_state.add_slice(&[]);
        assert!(result.is_err());
        if let Err(Error::MessageTerminated) = result {
            // Expected error
        } else {
            panic!("Expected MessageTerminated error");
        }
    }

    #[test]
    fn test_message_parser_state_ref_parse_until_condition() {
        let mut encoded = build_varint_field_bytes(1, 10);
        encoded.extend_from_slice(&build_varint_field_bytes(2, 20));
        encoded.extend_from_slice(&build_varint_field_bytes(3, 30));
        encoded.extend_from_slice(&build_varint_field_bytes(4, 40));

        let captured_fields = RefCell::new(Vec::new());
        let captured_fields_clone = &captured_fields;
        let parser_state = MessageParserStateRef::create(&encoded, Global, None, |field| {
            let field_num = field.field_number.as_u32();
            captured_fields_clone.borrow_mut().push(field_num);
            Ok(())
        });

        // Parse until we encounter field 3
        parser_state
            .parse_until_with_callback(|| captured_fields.borrow().last() == Some(&3))
            .unwrap();

        // Should have parsed fields 1, 2, and 3
        assert_eq!(*captured_fields.borrow(), vec![1, 2, 3]);

        // Field 4 should not have been parsed yet
        // (We can continue parsing to get field 4)
    }

    #[test]
    fn test_message_parser_state_ref_ensure_all_fields_parsed() {
        let mut encoded = build_varint_field_bytes(1, 10);
        encoded.extend_from_slice(&build_varint_field_bytes(2, 20));
        encoded.extend_from_slice(&build_varint_field_bytes(3, 30));

        let captured_fields = RefCell::new(Vec::new());
        let captured_fields_clone = &captured_fields;
        let parser_state = MessageParserStateRef::create(&encoded, Global, None, |field| {
            captured_fields_clone
                .borrow_mut()
                .push(field.field_number.as_u32());
            Ok(())
        });

        // Ensure all fields are parsed
        parser_state
            .ensure_all_fields_parsed_with_callback()
            .unwrap();

        // All fields should be captured
        assert_eq!(*captured_fields.borrow(), vec![1, 2, 3]);

        // Second call should be idempotent
        parser_state
            .ensure_all_fields_parsed_with_callback()
            .unwrap();
        assert_eq!(*captured_fields.borrow(), vec![1, 2, 3]); // Should not have changed
    }

    #[test]
    fn test_message_parser_state_ref_empty_message() {
        let empty_slice: &[u8] = &[];
        let capture_called = RefCell::new(false);

        let capture_called_clone = &capture_called;
        let parser_state = MessageParserStateRef::create(empty_slice, Global, None, |_field| {
            *capture_called_clone.borrow_mut() = true;
            Ok(())
        });

        // Parse all fields (none should be found)
        parser_state
            .ensure_all_fields_parsed_with_callback()
            .unwrap();

        // Callback should not have been called
        assert!(!*capture_called.borrow());
    }
}
