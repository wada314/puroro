//! Hand-written code for Address message.
//!
//! This represents a submessage of Person message.
//! Uses trait-based field operations for type-safe, scalable code generation.
//!
//! This code is (supposed to be) generated from `sandbox/protos/person.proto`.

use ::allocator_api2::boxed::Box;
use ::allocator_api2::vec::Vec as AllocVec;
use ::allocator_extras::{Allocator, Global};
use once_list2::OnceList;
use puroro::{
    Message,
    error::Error,
    field_ops::{FieldOperations, FieldStorage, ImplicitOptional, StringFieldWrapper},
    shared::SharedFields,
};

// ============================================================================
// Address Traits and Implementation
// ============================================================================

/// Flexible view trait for Address message (not dyn-compatible).
///
/// Code generation note: MUST NOT reference implementation struct names. Trait-only abstraction.
pub trait Address: DynAddress {}

// Blanket implementation for references
impl<T: Address> Address for &T {}
impl<T: Address> Address for &mut T {}
impl<T: AddressMut> AddressMut for &mut T {}

// Blanket implementation for Option
impl<T: DynAddress> DynAddress for Option<T> {
    fn street(&self) -> &str {
        self.as_ref().map(|v| v.street()).unwrap_or("")
    }

    fn city(&self) -> &str {
        self.as_ref().map(|v| v.city()).unwrap_or("")
    }

    fn zip_code(&self) -> i32 {
        self.as_ref().map(|v| v.zip_code()).unwrap_or(0)
    }
}

impl<T: Address> Address for Option<T> {}

// Blanket implementation for Box
impl<T: Address> Address for Box<T> {}
impl<T: AddressMut> AddressMut for Box<T> {}

// Blanket implementation for Option
impl<T: AddressMut> AddressMut for Option<T> {}

/// Flexible view fully mutable trait for Address message (not dyn-compatible).
///
/// Code generation note: MUST NOT reference implementation struct names. Trait-only abstraction.
pub trait AddressMut: Address + DynAddressMut {}

/// Dyn-compatible immutable trait for Address message.
///
/// Code generation note: This trait MUST be dyn-compatible. Do not use `impl Trait` here.
pub trait DynAddress {
    fn street(&self) -> &str;
    fn city(&self) -> &str;
    fn zip_code(&self) -> i32;
}

// Blanket implementation for references
impl<T: DynAddress> DynAddress for &T {
    fn street(&self) -> &str {
        (*self).street()
    }

    fn city(&self) -> &str {
        (*self).city()
    }

    fn zip_code(&self) -> i32 {
        (*self).zip_code()
    }
}

impl<T: DynAddress> DynAddress for &mut T {
    fn street(&self) -> &str {
        (**self).street()
    }

    fn city(&self) -> &str {
        (**self).city()
    }

    fn zip_code(&self) -> i32 {
        (**self).zip_code()
    }
}

// Blanket implementation for Box
impl<T: DynAddress> DynAddress for Box<T> {
    fn street(&self) -> &str {
        (**self).street()
    }

    fn city(&self) -> &str {
        (**self).city()
    }

    fn zip_code(&self) -> i32 {
        (**self).zip_code()
    }
}

/// Dyn-compatible fully mutable trait for Address message.
///
/// Code generation note: This trait MUST be dyn-compatible. Do not use `impl Trait` here.
pub trait DynAddressMut: DynAddress {
    // Setters - sample: set_street (others follow same pattern: field.set(&mut self._shared, v))
    fn set_street(&mut self, v: &str);

    // Clear methods - sample: clear_street (others follow same pattern: field.clear(&mut self._shared))
    fn clear_street(&mut self);
}

// Blanket implementation for references
impl<T: DynAddressMut> DynAddressMut for &mut T {
    fn set_street(&mut self, v: &str) {
        (**self).set_street(v)
    }
    fn clear_street(&mut self) {
        (**self).clear_street()
    }
}

// Blanket implementation for Box
impl<T: DynAddressMut> DynAddressMut for Box<T> {
    fn set_street(&mut self, v: &str) {
        (**self).set_street(v)
    }
    fn clear_street(&mut self) {
        (**self).clear_street()
    }
}

// Blanket implementation for Option
impl<T: DynAddressMut> DynAddressMut for Option<T> {
    fn set_street(&mut self, v: &str) {
        if let Some(t) = self {
            t.set_street(v);
        }
    }
    fn clear_street(&mut self) {
        if let Some(t) = self {
            t.clear_street();
        }
    }
}

/// Address message implementation
#[derive(Debug, Clone)]
pub struct AddressImpl<A: Allocator = Global> {
    street: FieldStorage<StringFieldWrapper<A>, ImplicitOptional, 1, 1, A>,
    city: FieldStorage<StringFieldWrapper<A>, ImplicitOptional, 2, 1, A>,
    zip_code: FieldStorage<i32, ImplicitOptional, 3, 1, A>,
    _shared: SharedFields<1, A>,
}

impl<A> AddressImpl<A>
where
    A: Allocator + Clone,
{
    pub fn new_in(alloc: A) -> Self {
        Self {
            street: FieldStorage::new(StringFieldWrapper::new_in(alloc.clone())),
            city: FieldStorage::new(StringFieldWrapper::new_in(alloc.clone())),
            zip_code: Default::default(),
            _shared: SharedFields::new_in(alloc),
        }
    }
}

impl AddressImpl<Global> {
    pub fn new() -> Self {
        Self::new_in(Global)
    }
}

impl<A> Default for AddressImpl<A>
where
    A: Allocator + Clone + Default,
{
    fn default() -> Self {
        Self::new_in(A::default())
    }
}

impl<A> PartialEq for AddressImpl<A>
where
    A: Allocator,
{
    fn eq(&self, other: &Self) -> bool {
        self.street == other.street
            && self.city == other.city
            && self.zip_code == other.zip_code
            && self._shared == other._shared
    }
}

impl<A> Eq for AddressImpl<A> where A: Allocator {}

impl<A: Allocator + Clone> Address for AddressImpl<A> {}

impl<A: Allocator + Clone> DynAddress for AddressImpl<A> {
    fn street(&self) -> &str {
        self.street.get(&self._shared)
    }
    fn city(&self) -> &str {
        self.city.get(&self._shared)
    }
    fn zip_code(&self) -> i32 {
        self.zip_code.get(&self._shared)
    }
}

impl<A: Allocator + Clone> AddressMut for AddressImpl<A> {}

impl<A: Allocator + Clone> DynAddressMut for AddressImpl<A> {
    // set_* methods - sample implementation (others follow same pattern: field.set(&mut self._shared, v))
    fn set_street(&mut self, v: &str) {
        self.street.set(&mut self._shared, v)
    }

    // clear_* methods - sample implementation (others follow same pattern: field.clear(&mut self._shared))
    fn clear_street(&mut self) {
        self.street.clear(&mut self._shared)
    }
}

impl<A: Allocator + Clone> Message for AddressImpl<A> {
    fn parse_from_bytes_in<B>(_bytes: &[u8], _alloc: B) -> Result<Self, Error>
    where
        B: Allocator + Clone,
    {
        todo!("Parsing not yet implemented")
    }

    fn write_to_bytes_in<B>(&self, _alloc: B) -> Result<AllocVec<u8, B>, Error>
    where
        B: Allocator + Clone,
    {
        todo!("Serialization not yet implemented")
    }

    fn compute_size(&self) -> usize {
        todo!("Size computation not yet implemented")
    }
}

// ============================================================================
// AddressLazyImpl Structure (Lazy Implementation - Phase 3)
// ============================================================================

use puroro::lazy_parser::MessageParserState;
use puroro::protobuf_core::{Field, FieldValue};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

/// Lazy implementation of Address message that deserializes fields on-demand.
///
/// Phase 3: Basic implementation with street, city, zip_code fields.
pub struct AddressLazyImpl<'a, A: Allocator = Global> {
    /// Owns parser state via Rc<RefCell<...>> - State itself is mutable
    parser_state: Rc<RefCell<MessageParserState<'a, A>>>,

    /// Parent parser state - strong Rc<RefCell<...>> reference (no cycle!)
    /// Child needs parent's parser state to request continued parsing
    parent_parser_state: Rc<RefCell<MessageParserState<'a, A>>>,

    /// Field slices collected so far (from parent)
    /// These are length-delimited value slices (not including field tags)
    /// Using &'a [u8] to support Field type which returns Cow<'a, [u8]> for Len values
    field_slices: OnceList<&'a [u8], A>,

    /// Field 1: street (implicit presence string field)
    street: RefCell<String>,

    /// Field 2: city (implicit presence string field)
    city: RefCell<String>,

    /// Field 3: zip_code (implicit presence varint field)
    zip_code: Cell<i32>,
}

impl<'a, A: Allocator + Clone + 'a> AddressLazyImpl<'a, A> {
    /// Create from first slice, with parent parser state reference
    ///
    /// Note: Child holds strong Rc reference to parent's parser state (not message body)
    /// This avoids cycles: Parent Message Body → Parent Parser State → (Child holds Rc to this)
    /// Returns Rc<Self> - all methods use self: &Rc<Self>
    pub(crate) fn new_from_parent(
        first_slice: &'a [u8],
        parent_parser_state: Rc<RefCell<MessageParserState<'a, A>>>,
        alloc: A,
    ) -> Rc<Self> {
        let field_slices = OnceList::new_in(alloc.clone());
        field_slices.push(first_slice);

        let alloc_clone = alloc.clone();
        // Create a dummy callback for child messages (not used, but required by MessageParserState)
        let closure = move |_field: Field<&'a [u8]>| -> Result<(), Error> { Ok(()) };
        let boxed = Box::new_in(closure, alloc_clone.clone());
        let callback: Box<dyn FnMut(Field<&'a [u8]>) -> Result<(), Error> + 'a, A> =
            ::allocator_api2::unsize_box!(boxed);

        let parser_state = Rc::new(RefCell::new(MessageParserState::new(
            std::iter::once(first_slice),
            alloc_clone.clone(),
            callback,
        )));

        Rc::new(Self {
            parser_state,
            parent_parser_state,
            field_slices,
            street: RefCell::new(String::new()),
            city: RefCell::new(String::new()),
            zip_code: Cell::new(0),
        })
    }

    /// Add additional slice from parent
    pub(crate) fn add_slice(self: &Rc<Self>, slice: &'a [u8]) -> Result<(), Error> {
        self.field_slices.push(slice);
        // Note: field_iter needs to be recreated when parsing
        Ok(())
    }

    /// Getter for street field
    pub fn street(self: &Rc<Self>) -> std::cell::Ref<'_, String> {
        let _ = self.ensure_all_fields_parsed();
        self.street.borrow()
    }

    /// Getter for city field
    pub fn city(self: &Rc<Self>) -> std::cell::Ref<'_, String> {
        let _ = self.ensure_all_fields_parsed();
        self.city.borrow()
    }

    /// Getter for zip_code field
    pub fn zip_code(self: &Rc<Self>) -> i32 {
        let _ = self.ensure_all_fields_parsed();
        self.zip_code.get()
    }

    /// Ensure all fields are parsed
    ///
    /// This will request parent's parser state to continue parsing if needed.
    /// Then parses all collected slices to extract Address fields.
    fn ensure_all_fields_parsed(self: &Rc<Self>) -> Result<(), Error> {
        // Request parent's parser state to continue parsing
        // This works even if parent Message Body is dropped, because:
        // 1. Child holds strong Rc reference to parent's Parser State
        // 2. continue_parsing_for_children doesn't require parent Message Body to be alive
        // 3. Callbacks (field_update_callback) already add slices to child messages via add_slice
        self.parent_parser_state
            .borrow_mut()
            .continue_parsing_for_children()?;

        // Check if we've already parsed (field_iter is None and we've parsed before)
        // We check if field_iter was previously Some (now None) vs never created (was None)
        // For simplicity, we always recreate the iterator from field_slices
        // This ensures we parse all slices even if field_iter state is lost

        // Now parse our own fields from all collected slices
        // Since field_slices stores &'a [u8], we can use them directly
        let slices: Vec<&'a [u8]> = self.field_slices.iter().copied().collect();

        // Always recreate iterator to ensure we parse all slices
        let mut parser_state = self.parser_state.borrow_mut();
        parser_state.set_field_iter_from_slices(slices.into_iter());
        drop(parser_state);

        // Parse until exhausted
        loop {
            let mut parser_state = self.parser_state.borrow_mut();
            let mut field_iter = match parser_state.take_field_iter() {
                Some(iter) => iter,
                None => break, // Already parsed
            };

            match field_iter.next() {
                Some(Ok(field)) => {
                    // Store iterator back before calling update_field
                    parser_state.set_field_iter(Some(field_iter));
                    drop(parser_state);
                    self.update_field(field)?;
                }
                Some(Err(e)) => {
                    parser_state.set_field_iter(Some(field_iter));
                    return Err(e);
                }
                None => {
                    parser_state.set_field_iter(None);
                    break;
                }
            }
        }

        Ok(())
    }

    /// Update a field with parsed value
    fn update_field(self: &Rc<Self>, field: Field<&'a [u8]>) -> Result<(), Error> {
        let field_num = field.field_number.as_u32();
        match field_num {
            1 => {
                // street field - string
                if let FieldValue::Len(data) = field.value {
                    let street_value =
                        String::from_utf8(data.to_vec()).map_err(|e| Error::InvalidUtf8(e))?;
                    *self.street.borrow_mut() = street_value;
                }
            }
            2 => {
                // city field - string
                if let FieldValue::Len(data) = field.value {
                    let city_value =
                        String::from_utf8(data.to_vec()).map_err(|e| Error::InvalidUtf8(e))?;
                    *self.city.borrow_mut() = city_value;
                }
            }
            3 => {
                // zip_code field - varint
                if let FieldValue::Varint(varint) = field.value {
                    let zip_code_value = varint.try_to_int32()?;
                    self.zip_code.set(zip_code_value);
                }
            }
            _ => {
                // Unknown field - ignore
            }
        }
        Ok(())
    }
}
