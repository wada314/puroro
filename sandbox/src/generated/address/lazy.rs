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

use super::traits::AddressTry;
use ::allocator_extras::{Allocator, Global};
use ::puroro::error::Error;
use ::puroro::lazy_slice_parser::MessageParserStateRef;
use ::puroro::protobuf_core::{Field, FieldValue};
use ::std::cell::Cell;
use ::std::rc::{Rc, Weak};

/// Lazy implementation of `Address` that deserializes fields on-demand.
///
/// Fields handled:
/// - `street` (field 1)
/// - `city` (field 2)
/// - `zip_code` (field 3)
///
/// - `'slice`: Lifetime of the input slices (external data)
pub struct AddressLazyImpl<'slice, A: Allocator = Global> {
    /// Owns parser state handle (wraps Rc<RefCell<...>>)
    parser_state: MessageParserStateRef<'slice, A>,

    /// Field 1: street (implicit presence string field)
    street: Cell<Option<&'slice str>>,

    /// Field 2: city (implicit presence string field)
    city: Cell<Option<&'slice str>>,

    /// Field 3: zip_code (implicit presence varint field)
    zip_code: Cell<i32>,
}

impl<'slice, A: Allocator + Clone + 'slice> AddressLazyImpl<'slice, A> {
    /// Create a new AddressLazyImpl from a slice.
    ///
    /// - For top-level messages: pass `parent_parser_state: None`
    /// - For child messages: pass `parent_parser_state: Some(parent_parser_state)`
    ///
    /// Note: Child holds strong Rc reference to parent's parser state (not message body)
    /// This avoids cycles: Parent Message Body → Parent Parser State → (Child holds Rc to this)
    /// Returns Rc<Self> - all methods use self: &Rc<Self>
    pub fn new(
        slice: &'slice [u8],
        alloc: A,
        parent_parser_state: Option<MessageParserStateRef<'slice, A>>,
    ) -> Rc<Self> {
        // Use Rc::new_cyclic with callback that handles Message Body
        // This is needed for both top-level and child messages because they may have their own child messages
        Rc::new_cyclic(move |weak: &Weak<Self>| {
            let message_body_weak = weak.clone();

            // Create initial callback that only handles Message Body (when it's alive)
            // This callback will be replaced in Drop::drop with one that handles child messages
            let closure = move |field: Field<&'slice [u8]>| -> Result<(), Error> {
                // Update via Message Body (should always succeed when this callback is active)
                if let Some(message_body) = message_body_weak.upgrade() {
                    // Rc implements Deref, so we can call update_field directly
                    let _ = message_body.update_field(field);
                }
                Ok(())
            };
            // Create parser state with initial callback
            let parser_state =
                MessageParserStateRef::create(slice, alloc.clone(), parent_parser_state, closure);

            // Create message body
            Self {
                parser_state,
                street: Cell::new(None),
                city: Cell::new(None),
                zip_code: Cell::new(0),
            }
        })
    }

    /// Add additional slice from parent
    pub(crate) fn add_slice(&self, slice: &'slice [u8]) -> Result<(), Error> {
        // Add slice to the parser state's field iterator
        // Terminated check is handled inside MessageParserStateRef::add_slice()
        self.parser_state.add_slice(slice)
    }

    /// Getter for street field
    pub fn street(&self) -> Result<&'slice str, Error> {
        self.ensure_all_fields_parsed()?;
        Ok(self.street.get().unwrap_or(""))
    }

    /// Getter for city field
    pub fn city(&self) -> Result<&'slice str, Error> {
        self.ensure_all_fields_parsed()?;
        Ok(self.city.get().unwrap_or(""))
    }

    /// Getter for zip_code field
    pub fn zip_code(&self) -> Result<i32, Error> {
        self.ensure_all_fields_parsed()?;
        Ok(self.zip_code.get())
    }

    /// Ensure all fields are parsed
    ///
    /// This will request parent's parser state to continue parsing if needed (for child messages).
    /// Then parses all collected slices to extract Address fields.
    ///
    /// This is a terminating operation - after this method completes, the message is marked as terminated
    /// and no additional slices can be added.
    fn ensure_all_fields_parsed(&self) -> Result<(), Error> {
        self.parser_state.ensure_all_fields_parsed_with_callback()
    }

    /// Update a field with parsed value
    fn update_field(&self, field: Field<&'slice [u8]>) -> Result<(), Error> {
        let field_num = field.field_number.as_u32();
        match (field_num, field.value) {
            (1, FieldValue::Len(data)) => {
                // street field - string
                let bytes = data.as_ref();
                let street_value = ::std::str::from_utf8(bytes)?;
                self.street.set(Some(street_value));
            }
            (2, FieldValue::Len(data)) => {
                // city field - string
                let bytes = data.as_ref();
                let city_value = ::std::str::from_utf8(bytes)?;
                self.city.set(Some(city_value));
            }
            (3, FieldValue::Varint(varint)) => {
                // zip_code field - varint
                let zip_code_value = varint.try_to_int32()?;
                self.zip_code.set(zip_code_value);
            }
            _ => {
                // Unknown field - ignore
            }
        }
        Ok(())
    }
}

impl<'slice, A: Allocator + Clone + 'slice> AddressTry for AddressLazyImpl<'slice, A> {
    fn try_street(&self) -> Result<&str, Error> {
        self.street()
    }
    fn try_city(&self) -> Result<&str, Error> {
        self.city()
    }
    fn try_zip_code(&self) -> Result<i32, Error> {
        self.zip_code()
    }
}
