// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::super::address::AddressLazyImpl;
use ::allocator_extras::{Allocator, Global};
use ::once_list2::OnceListWithTailLen as OnceList;
use ::puroro::error::Error;
use ::puroro::lazy_parser::MessageParserStateRef;
use ::puroro::protobuf_core::{Field, FieldValue};
use ::puroro::repeated_lazy::LazyRepeated;
use ::std::cell::{Cell, OnceCell};
use ::std::rc::{Rc, Weak};

/// Lazy implementation of Person message that deserializes fields on-demand.
///
/// Currently this implementation handles:
/// - `age` (field 2)
/// - `scores` (field 10)
/// - `address` (field 6)
/// - `addresses` (field 9)
///
/// - `'slice`: Lifetime of the input slices (external data)
pub struct PersonLazyImpl<'slice, A: Allocator + Clone + 'slice = Global> {
    /// Owns parser state handle (wraps Rc<RefCell<...>>)
    parser_state: MessageParserStateRef<'slice, A>,

    /// Field 2: age (implicit presence varint field)
    /// Copy type - Cell is sufficient
    age: Cell<i32>,

    /// Field 10: scores (repeated varint field)
    /// OnceList has built-in interior mutability - no RefCell needed
    scores: OnceList<i32, A>,

    /// Field 6: address (scalar message field)
    /// Scalar message field cache.
    ///
    /// This is set at most once (first occurrence) and then reused; `OnceCell` provides
    /// interior mutability without dynamic borrow checks.
    address: OnceCell<Rc<AddressLazyImpl<'slice, A>>>,

    /// Field 9: addresses (repeated message field)
    /// OnceList has built-in interior mutability - no RefCell needed
    /// Use Rc so each element is cheaply cloneable and can be returned/shared without borrowing.
    addresses: OnceList<Rc<AddressLazyImpl<'slice, A>>, A>,
}

impl<'slice, A: Allocator + Clone + 'slice> PersonLazyImpl<'slice, A> {
    /// Create a new PersonLazyImpl from a slice.
    ///
    /// - For top-level messages: pass `parent_parser_state: None`
    /// - For child messages: pass `parent_parser_state: Some(parent_parser_state)`
    ///
    /// Note: Child holds strong Rc reference to parent's parser state (not message body)
    /// This avoids cycles: Parent Message Body → Parent Parser State → (Child holds Rc to this)
    /// Returns Rc<Self> - all methods use self: &'message Rc<Self>
    pub fn new(
        slice: &'slice [u8],
        alloc: A,
        parent_parser_state: Option<MessageParserStateRef<'slice, A>>,
    ) -> Rc<Self> {
        // Use Rc::new_cyclic with callback that handles Message Body
        // This is needed for both top-level and child messages because they may have their own child messages
        Rc::new_cyclic(move |weak: &Weak<Self>| {
            let message_body_weak = weak.clone();

            // Create initial callback that handles all fields
            // This callback will be replaced in Drop::drop with one that only handles child messages
            let closure = move |field: Field<&'slice [u8]>| -> Result<(), Error> {
                // Update via Message Body (should always succeed when this callback is active)
                if let Some(message_body) = message_body_weak.upgrade() {
                    // Rc implements Deref, so we can call update_field directly
                    message_body.update_field(field)?;
                }
                Ok(())
            };
            // Create parser state with initial callback
            let parser_state =
                MessageParserStateRef::create(slice, alloc.clone(), parent_parser_state, closure);

            // Create message body
            Self {
                parser_state,
                // Initialize fields with default values
                age: Cell::new(0),
                scores: OnceList::new_in(alloc.clone()),
                address: OnceCell::new(),
                addresses: OnceList::new_in(alloc.clone()),
            }
        })
    }

    /// Add additional slice from parent
    #[allow(dead_code)] // Used when PersonLazyImpl is used as a child message
    pub(crate) fn add_slice(&self, slice: &'slice [u8]) -> Result<(), Error> {
        // Add slice to the parser state's field iterator
        // Terminated check is handled inside MessageParserStateRef::add_slice()
        self.parser_state.add_slice(slice)
    }

    /// Getter for age field
    /// Returns the final confirmed value after parsing is complete
    pub fn age(&self) -> i32 {
        // Ensure all fields are parsed before returning value
        let _ = self.ensure_all_fields_parsed();
        self.age.get() // Cell - returns Copy value (final confirmed)
    }

    /// Getter for scores field
    /// Returns a LazyRepeated adapter that enables on-demand parsing
    pub fn scores(&self) -> LazyRepeated<'slice, '_, i32, A> {
        LazyRepeated::new(self.parser_state.clone(), &self.scores)
    }

    /// Getter for address field
    /// Returns the parsed child message if present, or `None` otherwise.
    ///
    /// Note: Currently this calls `ensure_all_fields_parsed()` so the child has received all
    /// available slices before it is returned.
    pub fn address(&self) -> Option<Rc<AddressLazyImpl<'slice, A>>> {
        let _ = self.ensure_all_fields_parsed();
        self.address.get().cloned()
    }

    /// Getter for addresses field
    /// Returns a LazyRepeated adapter that enables on-demand parsing
    pub fn addresses(&self) -> LazyRepeated<'slice, '_, Rc<AddressLazyImpl<'slice, A>>, A> {
        LazyRepeated::new(self.parser_state.clone(), &self.addresses)
    }

    /// Ensure all fields are parsed
    ///
    /// This will request parent's parser state to continue parsing if needed (for child messages).
    /// Then parses all collected slices to extract Person fields.
    ///
    /// This is a terminating operation - after this method completes, the message is marked as terminated
    /// and no additional slices can be added.
    fn ensure_all_fields_parsed(&self) -> Result<(), Error> {
        self.parser_state.ensure_all_fields_parsed_with_callback()
    }

    /// Update a field with parsed value
    /// Called during parsing to update field values
    fn update_field(&self, field: Field<&'slice [u8]>) -> Result<(), Error> {
        let field_num = field.field_number.as_u32();
        match (field_num, field.value) {
            (2, FieldValue::Varint(varint)) => {
                // age field - varint
                let age_value = varint.try_to_int32()?;
                self.age.set(age_value);
            }
            (6, FieldValue::Len(data)) => {
                // address field - scalar message field (length-delimited)
                if let Some(addr) = self.address.get() {
                    // Child already exists - add slice to it
                    addr.add_slice(data)?;
                } else {
                    // First occurrence - create child with first slice
                    let allocator = self.parser_state.allocator();
                    let parent_parser_state = Some(self.parser_state.clone());
                    let child = AddressLazyImpl::new(data, allocator, parent_parser_state);

                    // This should succeed because we are in the "unset" branch. If it ever fails
                    // (e.g., via re-entrancy), fall back to updating the existing child.
                    if self.address.set(child).is_err() {
                        if let Some(addr) = self.address.get() {
                            addr.add_slice(data)?;
                        }
                    }
                }
            }
            (9, FieldValue::Len(data)) => {
                // addresses field - repeated message field (length-delimited)
                // For repeated message fields, each occurrence is a separate message
                // Create a new AddressLazyImpl for this occurrence
                let data_slice = data.as_ref();
                let allocator = self.parser_state.allocator();
                let parent_parser_state = Some(self.parser_state.clone());
                let child = AddressLazyImpl::new(data_slice, allocator, parent_parser_state);
                // Use OnceList's built-in interior mutability
                self.addresses.push(child); // push() takes &self
            }
            (10, FieldValue::Varint(varint)) => {
                // scores field - repeated varint
                // Use OnceList's built-in interior mutability
                let score_value = varint.try_to_int32()?;
                self.scores.push(score_value); // push() takes &self
            }
            // Other fields are currently ignored.
            _ => {
                // Unknown field - ignore
            }
        }
        Ok(())
    }
}

impl<'slice, A: Allocator + Clone + 'slice> Drop for PersonLazyImpl<'slice, A> {
    fn drop(&mut self) {
        // When Message Body is dropped, update callback to handle child messages
        // Extract child Weak references from fields
        let address_weak = self.address.get().map(Rc::downgrade);

        // Create new callback that captures child Weak references and uses static match-case
        let closure = move |field: Field<&'slice [u8]>| -> Result<(), Error> {
            // Static match-case for each child message field
            let field_num = field.field_number.as_u32();
            match field_num {
                6 => {
                    // address field - should always have Len wire type according to .proto definition
                    let FieldValue::Len(data) = field.value else {
                        return Err(Error::unexpected_wire_type(field_num, "Len", &field.value));
                    };
                    if let Some(addr) = address_weak.as_ref().and_then(Weak::upgrade) {
                        // addr is Rc<AddressLazyImpl>, add_slice takes &self
                        addr.add_slice(data)?;
                    }
                }
                // Add other child message fields here as needed (e.g., profile field 7)
                _ => {
                    // Not a child message field - ignore
                }
            }
            Ok(())
        };

        // Update callback in parser state
        self.parser_state.set_field_update_callback(closure);
    }
}
