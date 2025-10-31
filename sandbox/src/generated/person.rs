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
    Message,
};

/// Infallible immutable trait for Person message.
///
/// This trait provides read-only access to Person fields without error handling.
/// Use this for implementations that guarantee valid data (e.g., fully deserialized messages).
pub trait Person {
    // Getters
    fn name(&self) -> &str;
    fn age(&self) -> i32;
    fn email(&self) -> Option<&str>;
    fn score(&self) -> Option<i32>;

    // Enum field getters
    fn status(&self) -> Result<Status, i32>;
    fn secondary_status(&self) -> Result<Option<Status>, i32>;

    // Message field getters
    fn address(&self) -> Option<&Address>; // Message fields always return Option, even for ImplicitOptional

    // Presence checks (for optional semantics)
    fn has_name(&self) -> bool;
    fn has_age(&self) -> bool;
    fn has_email(&self) -> bool;
    fn has_score(&self) -> bool;
    fn has_status(&self) -> bool;
    fn has_secondary_status(&self) -> bool;
    fn has_address(&self) -> bool;
}

/// Infallible append-only trait for Person message.
///
/// This trait extends Person with append operations (set/add/insert) but no destructive operations.
/// Use this for most common use cases where you only need to add data, not clear it.
/// This provides type-level safety against accidental data loss.
pub trait PersonAppend: Person {
    // Setters - append new values
    fn set_name(&mut self, v: &str);
    fn set_age(&mut self, v: i32);
    fn set_email(&mut self, v: &str);
    fn set_score(&mut self, v: i32);

    // Enum field setters
    fn set_status(&mut self, v: Status);
    fn set_secondary_status(&mut self, v: Status);

    // Message field setters
    fn set_address(&mut self, v: &Address);

    // Builder-style methods for nested message construction
    fn address_mut(&mut self) -> &mut Address;
}

/// Infallible fully mutable trait for Person message.
///
/// This trait extends PersonAppend with destructive operations (clear).
/// Use this only when you need to delete or clear data.
pub trait PersonMut: PersonAppend {
    // Clear methods - destructive operations
    fn clear_name(&mut self);
    fn clear_age(&mut self);
    fn clear_email(&mut self);
    fn clear_score(&mut self);

    // Enum field clearers
    fn clear_status(&mut self);
    fn clear_secondary_status(&mut self);

    // Message field clearers
    fn clear_address(&mut self);
}

// ============================================================================
// Status Enum
// ============================================================================

/// Status enum implementation (proto3 + allow_alias = false)
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
// Address Message
// ============================================================================

/// Address message implementation
#[derive(Debug, Clone, PartialEq)]
pub struct Address {
    street: FieldStorage<StringFieldWrapper, ImplicitOptional, 1, 1>,
    city: FieldStorage<StringFieldWrapper, ImplicitOptional, 2, 1>,
    zip_code: FieldStorage<i32, ImplicitOptional, 3, 1>,
    _shared: SharedFields<1>,
}

impl Address {
    pub fn new() -> Self {
        Self {
            street: Default::default(),
            city: Default::default(),
            zip_code: Default::default(),
            _shared: SharedFields::new(),
        }
    }

    pub fn street(&self) -> &str {
        self.street.get(&self._shared)
    }

    pub fn city(&self) -> &str {
        self.city.get(&self._shared)
    }

    pub fn zip_code(&self) -> i32 {
        self.zip_code.get(&self._shared)
    }

    pub fn set_street(&mut self, v: &str) {
        self.street.set(&mut self._shared, v);
    }

    pub fn set_city(&mut self, v: &str) {
        self.city.set(&mut self._shared, v);
    }

    pub fn set_zip_code(&mut self, v: i32) {
        self.zip_code.set(&mut self._shared, v);
    }
}

impl Default for Address {
    fn default() -> Self {
        Self::new()
    }
}

impl Message for Address {
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
// Profile Message
// ============================================================================

/// Profile message implementation
#[derive(Debug, Clone, PartialEq)]
pub struct Profile {
    bio: FieldStorage<StringFieldWrapper, ImplicitOptional, 1, 1>,
    website: FieldStorage<StringFieldWrapper, ExplicitOptional<0>, 2, 1>,
    reputation: FieldStorage<i32, ImplicitOptional, 3, 1>,
    _shared: SharedFields<1>,
}

impl Profile {
    pub fn new() -> Self {
        Self {
            bio: Default::default(),
            website: Default::default(),
            reputation: Default::default(),
            _shared: SharedFields::new(),
        }
    }

    pub fn bio(&self) -> &str {
        self.bio.get(&self._shared)
    }

    pub fn website(&self) -> Option<&str> {
        self.website.get(&self._shared)
    }

    pub fn reputation(&self) -> i32 {
        self.reputation.get(&self._shared)
    }

    pub fn set_bio(&mut self, v: &str) {
        self.bio.set(&mut self._shared, v);
    }

    pub fn set_website(&mut self, v: &str) {
        self.website.set(&mut self._shared, v);
    }

    pub fn set_reputation(&mut self, v: i32) {
        self.reputation.set(&mut self._shared, v);
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self::new()
    }
}

impl Message for Profile {
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
    address: FieldStorage<MessageFieldWrapper<Address>, ImplicitOptional, 6, 1>, // Field 6, heap-allocated presence
    profile: FieldStorage<MessageFieldWrapper<Profile>, ImplicitOptional, 7, 1>, // Field 7, heap-allocated presence

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
            profile: Default::default(),
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
    #[inline]
    fn name(&self) -> &str {
        self.name.get(&self._shared)
    }

    #[inline]
    fn age(&self) -> i32 {
        self.age.get(&self._shared)
    }

    #[inline]
    fn email(&self) -> Option<&str> {
        self.email.get(&self._shared)
    }

    #[inline]
    fn score(&self) -> Option<i32> {
        self.score.get(&self._shared)
    }

    #[inline]
    fn status(&self) -> Result<Status, i32> {
        Status::from_wire(self.status.get(&self._shared))
    }

    #[inline]
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

    #[inline]
    fn address(&self) -> Option<&Address> {
        self.address.get(&self._shared)
    }

    #[inline]
    fn has_name(&self) -> bool {
        // ImplicitOptional fields check if value is not equal to default
        self.name.is_present(&self._shared)
    }

    #[inline]
    fn has_age(&self) -> bool {
        // ImplicitOptional fields check if value is not equal to default
        self.age.is_present(&self._shared)
    }

    #[inline]
    fn has_email(&self) -> bool {
        // ExplicitOptional fields check presence via Field trait
        self.email.is_present(&self._shared)
    }

    #[inline]
    fn has_score(&self) -> bool {
        // ExplicitOptional fields check presence via Field trait
        self.score.is_present(&self._shared)
    }

    #[inline]
    fn has_status(&self) -> bool {
        // ImplicitOptional fields check if value is not equal to default
        self.status.is_present(&self._shared)
    }

    #[inline]
    fn has_secondary_status(&self) -> bool {
        // ExplicitOptional fields check presence via Field trait
        self.secondary_status.is_present(&self._shared)
    }

    #[inline]
    fn has_address(&self) -> bool {
        // ImplicitOptional fields check if value is not equal to default
        self.address.is_present(&self._shared)
    }
}

impl PersonAppend for PersonImpl {
    #[inline]
    fn set_name(&mut self, v: &str) {
        self.name.set(&mut self._shared, v);
    }

    #[inline]
    fn set_age(&mut self, v: i32) {
        self.age.set(&mut self._shared, v);
    }

    #[inline]
    fn set_email(&mut self, v: &str) {
        self.email.set(&mut self._shared, v);
    }

    #[inline]
    fn set_score(&mut self, v: i32) {
        self.score.set(&mut self._shared, v);
    }

    #[inline]
    fn set_status(&mut self, v: Status) {
        self.status.set(&mut self._shared, v.to_wire());
    }

    #[inline]
    fn set_secondary_status(&mut self, v: Status) {
        self.secondary_status.set(&mut self._shared, v.to_wire());
    }

    #[inline]
    fn set_address(&mut self, v: &Address) {
        self.address.set(&mut self._shared, v);
    }

    #[inline]
    fn address_mut(&mut self) -> &mut Address {
        // Ensure field is allocated and marked as present
        if self.address.data.0.is_none() {
            self.address.data.0 = Some(Box::new(Address::default()));
        }
        self.address.data.0.as_mut().unwrap().as_mut()
    }
}

impl PersonMut for PersonImpl {
    #[inline]
    fn clear_name(&mut self) {
        self.name.clear(&mut self._shared);
    }

    #[inline]
    fn clear_age(&mut self) {
        self.age.clear(&mut self._shared);
    }

    #[inline]
    fn clear_email(&mut self) {
        self.email.clear(&mut self._shared);
    }

    #[inline]
    fn clear_score(&mut self) {
        self.score.clear(&mut self._shared);
    }

    #[inline]
    fn clear_status(&mut self) {
        self.status.clear(&mut self._shared);
    }

    #[inline]
    fn clear_secondary_status(&mut self) {
        self.secondary_status.clear(&mut self._shared);
    }

    #[inline]
    fn clear_address(&mut self) {
        self.address.clear(&mut self._shared);
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
    use super::*;

    #[test]
    fn test_message_fields() {
        let mut person = PersonImpl::new();

        // Test setting message fields
        let mut address = Address::new();
        address.set_street("123 Main St");
        address.set_city("Anytown");
        address.set_zip_code(12345);

        person.set_address(&address);

        // Test getting message fields
        let retrieved_address = person.address().unwrap();
        assert_eq!(retrieved_address.street(), "123 Main St");
        assert_eq!(retrieved_address.city(), "Anytown");
        assert_eq!(retrieved_address.zip_code(), 12345);

        // Test presence checking
        assert!(person.has_address());

        // Test builder pattern
        person.address_mut().set_street("456 Oak Ave");
        assert_eq!(person.address().unwrap().street(), "456 Oak Ave");

        // Test optional message field
        let mut profile = Profile::new();
        profile.set_bio("Software developer");
        profile.set_reputation(100);

        person.set_profile(&profile);

        let retrieved_profile = person.profile().unwrap();
        assert_eq!(retrieved_profile.bio(), "Software developer");
        assert_eq!(retrieved_profile.reputation(), 100);
        assert!(person.has_profile());

        // Test clearing message fields
        person.clear_address();
        assert!(!person.has_address());

        person.clear_profile();
        assert!(!person.has_profile());
    }

    #[test]
    fn test_enum_fields() {
        let mut person = PersonImpl::new();

        // Test setting enum fields
        person.set_status(Status::Active);
        person.set_secondary_status(Status::Pending);

        // Test getting enum fields
        match person.status() {
            Ok(Status::Active) => println!("Status is Active"),
            Ok(status) => println!("Status is {:?}", status),
            Err(unknown) => println!("Unknown status: {}", unknown),
        }

        match person.secondary_status() {
            Ok(Some(Status::Pending)) => println!("Secondary status is Pending"),
            Ok(Some(status)) => println!("Secondary status is {:?}", status),
            Ok(None) => println!("No secondary status"),
            Err(unknown) => println!("Unknown secondary status: {}", unknown),
        }

        // Test presence checking
        assert!(person.has_status());
        assert!(person.has_secondary_status());

        // Test clearing enum fields
        person.clear_status();
        assert!(!person.has_status());

        person.clear_secondary_status();
        assert!(!person.has_secondary_status());
    }

    #[test]
    fn test_wrapper_types() {
        // Test StringFieldWrapper wrapper
        let string_field = StringFieldWrapper("Hello".to_string());
        assert_eq!(string_field.0, "Hello");

        // Test MessageFieldWrapper wrapper
        let address = Address::new();
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
