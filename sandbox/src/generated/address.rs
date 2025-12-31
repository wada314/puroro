//! Hand-written code for Address message.
//!
//! This represents a submessage of Person message.
//! Uses trait-based field operations for type-safe, scalable code generation.
//!
//! This code is (supposed to be) generated from `sandbox/protos/person.proto`.

use ::allocator_api2::vec::Vec as AllocVec;
use ::allocator_extras::{Allocator, Global};
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

// AddressLazyImpl will be re-implemented in Phase 3
// Old implementation removed - will be replaced with new design based on MessageParserState
