//! Hand-written code for Person message.
//!
//! This represents our ideal API for the generated code using the Closed Struct approach.
//!
//! Uses trait-based field operations for type-safe, scalable code generation.

use puroro::{
    error::Error,
    field_ops::{
        ExplicitOptional, FieldOperations, FieldStorage, ImplicitOptional, MessageFieldWrapper,
        StringFieldWrapper,
    },
    shared::SharedFields,
    view::ViewCow,
    Message,
};
use std::ops::{Deref, DerefMut};

/// Flexible view trait for Person message (not dyn-compatible).
///
/// Code generation note: This trait MUST NOT reference any implementation struct names (e.g., PersonImpl, AddressImpl).
/// Use only trait names and `impl Trait` syntax to maintain abstraction.
pub trait Person: DynPerson {
    // Methods that delegate to DynPerson (default implementations)
    #[inline]
    fn name(&self) -> &str {
        DynPerson::name(self)
    }
    #[inline]
    fn age(&self) -> i32 {
        DynPerson::age(self)
    }
    #[inline]
    fn email(&self) -> Option<&str> {
        DynPerson::email(self)
    }
    #[inline]
    fn score(&self) -> Option<i32> {
        DynPerson::score(self)
    }
    #[inline]
    fn status(&self) -> Result<Status, i32> {
        DynPerson::status(self)
    }
    #[inline]
    fn secondary_status(&self) -> Result<Option<Status>, i32> {
        DynPerson::secondary_status(self)
    }

    // Presence checks (delegating to DynPerson)
    #[inline]
    fn has_name(&self) -> bool {
        DynPerson::has_name(self)
    }

    // Methods with custom implementations (must be implemented)
    // NOTE: Must return `impl Address`, not a concrete struct type
    fn address(&self) -> impl Address + use<'_, Self>;
}

/// Dyn-compatible immutable trait for Person message.
///
/// Code generation note: This trait MUST be dyn-compatible. Do not use `impl Trait` here;
/// use `ViewCow` or other dyn-compatible return types for message fields.
pub trait DynPerson {
    // Getters
    fn name(&self) -> &str;
    fn age(&self) -> i32;
    fn email(&self) -> Option<&str>;
    fn score(&self) -> Option<i32>;

    // Enum field getters
    fn status(&self) -> Result<Status, i32>;
    fn secondary_status(&self) -> Result<Option<Status>, i32>;

    // Message field getters
    fn address(&self) -> Option<ViewCow<'_, dyn DynAddress>>; // Message fields always return Option, even for ImplicitOptional

    // Presence checks (for optional semantics) - sample: has_name (others follow same pattern)
    fn has_name(&self) -> bool;
}

/// Flexible view append-only trait for Person message (not dyn-compatible).
///
/// Code generation note: MUST NOT reference implementation struct names. All methods must use trait types only.
pub trait PersonAppend: Person + DynPersonAppend {
    // Methods that delegate to DynPersonAppend (default implementations)
    #[inline]
    fn set_name(&mut self, v: &str) {
        DynPersonAppend::set_name(self, v)
    }

    // Methods with custom implementations (must be implemented)
    // NOTE: Must return `impl AddressAppend`, not a concrete struct type
    fn address_mut(&mut self) -> impl AddressAppend + use<'_, Self>;
}

/// Dyn-compatible append-only trait for Person message.
///
/// Code generation note: This trait MUST be dyn-compatible. Do not use `impl Trait` here.
pub trait DynPersonAppend: DynPerson {
    // Setters - sample: set_name (others follow same pattern: field.set(&mut self._shared, v) or field.set(&mut self._shared, v.to_wire()))
    fn set_name(&mut self, v: &str);
}

/// Flexible view fully mutable trait for Person message (not dyn-compatible).
///
/// Code generation note: MUST NOT reference implementation struct names. All methods must use trait types only.
pub trait PersonMut: PersonAppend + DynPersonMut {
    // Clear methods (delegating to DynPersonMut)
    #[inline]
    fn clear_name(&mut self) {
        DynPersonMut::clear_name(self)
    }

    // Methods with custom implementations (must be implemented)
    // NOTE: Must return `impl AddressMut`, not a concrete struct type
    fn address_mut(&mut self) -> impl AddressMut + use<'_, Self>;
}

/// Dyn-compatible fully mutable trait for Person message.
///
/// Code generation note: This trait MUST be dyn-compatible. Do not use `impl Trait` here.
pub trait DynPersonMut: DynPersonAppend {
    // Clear methods - sample: clear_name (others follow same pattern: field.clear(&mut self._shared))
    fn clear_name(&mut self);

    // Builder-style methods for nested message construction
    fn address_mut(&mut self) -> &mut dyn DynAddressMut;
}

// ============================================================================
// Status Enum
// ============================================================================

/// Status enum implementation
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Status {
    /// Default value (must be 0)
    Unspecified = 0,
    Active = 1,
    Inactive = 2,
    Pending = 3,
}

impl Status {
    /// Convert from wire format (i32) - known values only
    pub fn from_wire(value: i32) -> Result<Self, i32> {
        match value {
            0 => Ok(Self::Unspecified),
            1 => Ok(Self::Active),
            2 => Ok(Self::Inactive),
            3 => Ok(Self::Pending),
            unknown => Err(unknown), // Unknown values are errors
        }
    }

    /// Convert to wire format (i32)
    pub fn to_wire(self) -> i32 {
        self as i32
    }
}

impl TryFrom<i32> for Status {
    type Error = i32;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::from_wire(value)
    }
}

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
impl<T: AddressAppend> AddressAppend for &mut T {}
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
impl<T: AddressAppend> AddressAppend for Box<T> {}
impl<T: AddressMut> AddressMut for Box<T> {}

// Blanket implementation for Option
impl<T: AddressAppend> AddressAppend for Option<T> {}
impl<T: AddressMut> AddressMut for Option<T> {}

/// Flexible view append-only trait for Address message (not dyn-compatible).
///
/// Code generation note: MUST NOT reference implementation struct names. Trait-only abstraction.
pub trait AddressAppend: Address + DynAddressAppend {}

/// Flexible view fully mutable trait for Address message (not dyn-compatible).
///
/// Code generation note: MUST NOT reference implementation struct names. Trait-only abstraction.
pub trait AddressMut: AddressAppend + DynAddressMut {}

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

/// Dyn-compatible append-only trait for Address message.
///
/// Code generation note: This trait MUST be dyn-compatible. Do not use `impl Trait` here.
pub trait DynAddressAppend: DynAddress {
    // Setters - sample: set_street (others follow same pattern: field.set(&mut self._shared, v))
    fn set_street(&mut self, v: &str);
}

// Blanket implementation for references
impl<T: DynAddressAppend> DynAddressAppend for &mut T {
    fn set_street(&mut self, v: &str) {
        (**self).set_street(v)
    }
}

// Blanket implementation for Box
impl<T: DynAddressAppend> DynAddressAppend for Box<T> {
    fn set_street(&mut self, v: &str) {
        (**self).set_street(v)
    }
}

// Blanket implementation for Option
impl<T: DynAddressAppend> DynAddressAppend for Option<T> {
    fn set_street(&mut self, v: &str) {
        if let Some(t) = self {
            t.set_street(v);
        }
    }
}

/// Dyn-compatible fully mutable trait for Address message.
///
/// Code generation note: This trait MUST be dyn-compatible. Do not use `impl Trait` here.
pub trait DynAddressMut: DynAddressAppend {
    // Clear methods - sample: clear_street (others follow same pattern: field.clear(&mut self._shared))
    fn clear_street(&mut self);
}

// Blanket implementation for references
impl<T: DynAddressMut> DynAddressMut for &mut T {
    fn clear_street(&mut self) {
        (**self).clear_street()
    }
}

// Blanket implementation for Box
impl<T: DynAddressMut> DynAddressMut for Box<T> {
    fn clear_street(&mut self) {
        (**self).clear_street()
    }
}

// Blanket implementation for Option
impl<T: DynAddressMut> DynAddressMut for Option<T> {
    fn clear_street(&mut self) {
        if let Some(t) = self {
            t.clear_street();
        }
    }
}

/// Address message implementation
#[derive(Debug, Clone, PartialEq)]
pub struct AddressImpl {
    street: FieldStorage<StringFieldWrapper, ImplicitOptional, 1, 1>,
    city: FieldStorage<StringFieldWrapper, ImplicitOptional, 2, 1>,
    zip_code: FieldStorage<i32, ImplicitOptional, 3, 1>,
    _shared: SharedFields<1>,
}

impl AddressImpl {
    pub fn new() -> Self {
        Self {
            street: Default::default(),
            city: Default::default(),
            zip_code: Default::default(),
            _shared: SharedFields::new(),
        }
    }
}

impl Default for AddressImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl Address for AddressImpl {}

impl DynAddress for AddressImpl {
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

impl AddressAppend for AddressImpl {}

impl DynAddressAppend for AddressImpl {
    // set_* methods - sample implementation (others follow same pattern: field.set(&mut self._shared, v))
    fn set_street(&mut self, v: &str) {
        self.street.set(&mut self._shared, v)
    }
}

impl AddressMut for AddressImpl {}

impl DynAddressMut for AddressImpl {
    // clear_* methods - sample implementation (others follow same pattern: field.clear(&mut self._shared))
    fn clear_street(&mut self) {
        self.street.clear(&mut self._shared)
    }
}

impl Message for AddressImpl {
    fn parse_from_bytes(_bytes: &[u8]) -> Result<Self, Error> {
        todo!("Parsing not yet implemented")
    }

    fn write_to_bytes(&self) -> Result<Vec<u8>, Error> {
        todo!("Serialization not yet implemented")
    }

    fn compute_size(&self) -> usize {
        todo!("Size computation not yet implemented")
    }
}

// ============================================================================
// PersonImpl Structure
// ============================================================================

/// Standard implementation of Person message.
///
/// Uses trait-based field operations for type-safe code generation.
/// Fields are ordered by size (descending) to minimize padding.
/// Memory layout optimized for performance.
#[derive(Debug, Clone, PartialEq)]
pub struct PersonImpl {
    // Shared fields: presence tracking, etc.
    // For 3 explicit optional fields (email, score, secondary_status): ⌈3/8⌉ = 1 byte (stack-allocated)
    // Message fields use heap allocation with Option<Box<M>> for presence tracking
    _shared: SharedFields<1>,

    // Exclusive fields ordered by size (descending)
    // String: 24 bytes (3 words on 64-bit)
    // Direct field types with explicit parameters for clarity
    // Format: FieldStorage<T, L, FIELD_NUMBER, SHARED_BYTES_LEN>
    name: FieldStorage<StringFieldWrapper, ImplicitOptional, 1, 1>, // Field 1, implicit presence, 1 byte shared
    email: FieldStorage<StringFieldWrapper, ExplicitOptional<0>, 3, 1>, // Field 3, explicit presence, bit 0, 1 byte shared

    // Message fields: use heap allocation with Option<Box<M>> for presence tracking
    // No presence bits needed - Option<Box<M>> handles presence directly
    address: FieldStorage<MessageFieldWrapper<AddressImpl>, ImplicitOptional, 6, 1>, // Field 6, heap-allocated presence

    // Enum fields: stored as i32
    status: FieldStorage<i32, ImplicitOptional, 4, 1>, // Field 4, implicit presence, 1 byte shared
    secondary_status: FieldStorage<i32, ExplicitOptional<2>, 8, 1>, // Field 8, explicit presence, bit 2, 1 byte shared

    // Scalar fields: 4 bytes
    age: FieldStorage<i32, ImplicitOptional, 2, 1>, // Field 2, implicit presence, 1 byte shared
    score: FieldStorage<i32, ExplicitOptional<1>, 5, 1>, // Field 5, explicit presence, bit 1, 1 byte shared
}

impl PersonImpl {
    /// Creates a new Person with default values.
    pub fn new() -> Self {
        Self {
            name: Default::default(),
            email: Default::default(),
            address: Default::default(),
            status: Default::default(),
            secondary_status: Default::default(),
            _shared: SharedFields::new(),
            age: Default::default(),
            score: Default::default(),
        }
    }
}

impl Default for PersonImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl Person for PersonImpl {
    fn address(&self) -> impl Address + use<'_> {
        self.address.get(&self._shared)
    }
    // All other methods use default implementations from the trait definition
}

impl PersonAppend for PersonImpl {
    fn address_mut(&mut self) -> impl AddressAppend + use<'_> {
        self.address.data.0.as_mut().map(|b| b.as_mut())
    }
    // All other methods use default implementations from the trait definition
}

impl PersonMut for PersonImpl {
    fn address_mut(&mut self) -> impl AddressMut + use<'_> {
        self.address.data.0.as_mut().map(|b| b.as_mut())
    }
    // All other methods use default implementations from the trait definition
}

impl DynPerson for PersonImpl {
    fn name(&self) -> &str {
        self.name.get(&self._shared)
    }
    fn age(&self) -> i32 {
        self.age.get(&self._shared)
    }
    fn email(&self) -> Option<&str> {
        self.email.get(&self._shared)
    }
    fn score(&self) -> Option<i32> {
        self.score.get(&self._shared)
    }
    fn status(&self) -> Result<Status, i32> {
        Status::from_wire(self.status.get(&self._shared))
    }

    fn secondary_status(&self) -> Result<Option<Status>, i32> {
        if self.secondary_status.is_present(&self._shared) {
            match Status::from_wire(self.secondary_status.get(&self._shared).unwrap_or(0)) {
                Ok(status) => Ok(Some(status)),
                Err(unknown) => Err(unknown),
            }
        } else {
            Ok(None)
        }
    }

    fn address(&self) -> Option<ViewCow<'_, dyn DynAddress>> {
        self.address
            .get(&self._shared)
            .map(|address| ViewCow::Borrowed(address as &dyn DynAddress))
    }

    // has_* methods - sample implementation (others follow same pattern: field.is_present(&self._shared))
    fn has_name(&self) -> bool {
        self.name.is_present(&self._shared)
    }
}

impl DynPersonAppend for PersonImpl {
    // set_* methods - sample implementation (others follow same pattern: field.set(&mut self._shared, v) or field.set(&mut self._shared, v.to_wire()))
    fn set_name(&mut self, v: &str) {
        self.name.set(&mut self._shared, v)
    }
}

impl DynPersonMut for PersonImpl {
    // clear_* methods - sample implementation (others follow same pattern: field.clear(&mut self._shared))
    fn clear_name(&mut self) {
        self.name.clear(&mut self._shared)
    }

    fn address_mut(&mut self) -> &mut dyn DynAddressMut {
        if self.address.data.0.is_none() {
            self.address.data.0 = Some(Box::new(AddressImpl::default()));
        }
        self.address.data.0.as_mut().unwrap().as_mut()
    }
}

impl Message for PersonImpl {
    fn parse_from_bytes(_bytes: &[u8]) -> Result<Self, Error> {
        // TODO: Implement actual parsing
        todo!("Parsing not yet implemented")
    }

    fn write_to_bytes(&self) -> Result<Vec<u8>, Error> {
        // TODO: Implement actual serialization
        todo!("Serialization not yet implemented")
    }

    fn compute_size(&self) -> usize {
        // TODO: Implement actual size computation
        todo!("Size computation not yet implemented")
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::{
        AddressImpl, DynAddress, DynAddressAppend, DynPerson, DynPersonMut, MessageFieldWrapper,
        Person, PersonAppend, PersonImpl, PersonMut, Status, StringFieldWrapper,
    };

    #[test]
    fn test_message_fields() {
        let mut person = PersonImpl::new();

        // Test setting message fields via mutable builder
        // Initialize first using DynPersonMut (destructive, but needed for testing)
        {
            DynPersonMut::address_mut(&mut person);
        }
        {
            let mut address_impl = PersonMut::address_mut(&mut person);
            // address_impl is Option<&mut AddressImpl> which implements AddressMut
            // For Option<T>, None case does nothing, so initialization is required
            address_impl.set_street("123 Main St");
        }

        // Test getting message fields
        {
            let address_impl = Person::address(&person);
            // address_impl is Option<&AddressImpl> which implements Address
            // For Option<T>, None case returns default values
            assert_eq!(address_impl.street(), "123 Main St");
        }

        // Test builder pattern
        {
            let mut address_impl = PersonMut::address_mut(&mut person);
            address_impl.set_street("456 Oak Ave");
        }
        {
            let address_impl = Person::address(&person);
            assert_eq!(address_impl.street(), "456 Oak Ave");
        }
    }

    #[test]
    fn test_enum_fields() {
        let mut person = PersonImpl::new();

        // Test setting enum fields (sample: only set_name is available in DynPersonAppend)
        PersonAppend::set_name(&mut person, "Test");

        // Test getting enum fields
        match Person::status(&person) {
            Ok(Status::Active) => println!("Status is Active"),
            Ok(status) => println!("Status is {:?}", status),
            Err(unknown) => println!("Unknown status: {}", unknown),
        }

        match Person::secondary_status(&person) {
            Ok(Some(Status::Pending)) => println!("Secondary status is Pending"),
            Ok(Some(status)) => println!("Secondary status is {:?}", status),
            Ok(None) => println!("No secondary status"),
            Err(unknown) => println!("Unknown secondary status: {}", unknown),
        }

        // Test presence checking (sample: only has_name is available in DynPerson)
        assert!(Person::has_name(&person));

        // Test clearing enum fields (sample: only clear_name is available in DynPersonMut)
        PersonMut::clear_name(&mut person);
        assert!(!Person::has_name(&person));
    }

    #[test]
    fn test_wrapper_types() {
        // Test StringFieldWrapper wrapper
        let string_field = StringFieldWrapper("Hello".to_string());
        assert_eq!(string_field.0, "Hello");

        // Test MessageFieldWrapper wrapper
        let address = AddressImpl::new();
        let message_field = MessageFieldWrapper(Some(Box::new(address)));
        assert_eq!(message_field.0.as_ref().unwrap().street(), "");
    }
}

// Example: Future lazy deserialization implementation
// This would deserialize fields on-demand when accessed
/*
pub struct PersonLazy {
    raw_bytes: Vec<u8>,
    // Cache for already-deserialized fields
    cached_name: Option<String>,
    cached_age: Option<i32>,
    cached_email: Option<String>,
}

impl PersonTry for PersonLazy {
    fn try_name(&self) -> Result<&str, Error> {
        // Deserialize on first access, then cache
        // May fail if raw_bytes are corrupted
        todo!("Lazy deserialization")
    }

    fn try_age(&self) -> Result<i32, Error> {
        // Deserialize on first access
        // May fail if raw_bytes are corrupted
        todo!("Lazy deserialization")
    }

    // ... etc
}
*/
