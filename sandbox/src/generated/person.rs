//! Hand-written code for Person message.
//!
//! This represents our ideal API for the generated code using the Closed Struct approach.
//!
//! Uses trait-based field operations for type-safe, scalable code generation.

use puroro::{
    error::Error,
    field_ops::{ExplicitOptional, Field, FieldType, ImplicitOptional, MessageField, StringField},
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

    // Message field getters
    fn address(&self) -> Option<&Address>; // Message fields always return Option, even for ImplicitOptional
    fn profile(&self) -> Option<&Profile>;

    // Presence checks (for Proto3 optional semantics)
    fn has_name(&self) -> bool;
    fn has_age(&self) -> bool;
    fn has_email(&self) -> bool;
    fn has_score(&self) -> bool;
    fn has_address(&self) -> bool;
    fn has_profile(&self) -> bool;
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

    // Message field setters
    fn set_address(&mut self, v: &Address);
    fn set_profile(&mut self, v: &Profile);

    // Builder-style methods for nested message construction
    fn address_mut(&mut self) -> &mut Address;
    fn profile_mut(&mut self) -> &mut Profile;
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

    // Message field clearers
    fn clear_address(&mut self);
    fn clear_profile(&mut self);
}

/// Fallible immutable trait for Person message.
///
/// This trait provides read-only access to Person fields with error handling.
/// Use this for implementations that may fail during field access (e.g., lazy deserialization, validation).
/// All methods in this trait are fallible for consistency.
pub trait PersonTry {
    // Getters (fallible) - using try_ prefix following Rust conventions
    fn try_name(&self) -> Result<&str, Error>;
    fn try_age(&self) -> Result<i32, Error>;
    fn try_email(&self) -> Result<Option<&str>, Error>;
    fn try_score(&self) -> Result<Option<i32>, Error>;

    // Presence checks (fallible) - may fail when reading metadata
    fn try_has_name(&self) -> Result<bool, Error>;
    fn try_has_age(&self) -> Result<bool, Error>;
    fn try_has_email(&self) -> Result<bool, Error>;
    fn try_has_score(&self) -> Result<bool, Error>;
}

/// Fallible append-only trait for Person message.
///
/// This trait extends PersonTry with append operations that may fail.
/// Use this for implementations that validate data on write but don't need destructive operations.
/// All methods in this trait are fallible for consistency.
pub trait PersonAppendTry: PersonTry {
    // Setters (fallible) - using try_ prefix
    fn try_set_name(&mut self, v: &str) -> Result<(), Error>;
    fn try_set_age(&mut self, v: i32) -> Result<(), Error>;
    fn try_set_email(&mut self, v: &str) -> Result<(), Error>;
    fn try_set_score(&mut self, v: i32) -> Result<(), Error>;
}

/// Fallible fully mutable trait for Person message.
///
/// This trait extends PersonAppendTry with destructive operations that may fail.
/// Use this for lazy or validated implementations that need full mutation support.
/// All methods in this trait are fallible for consistency.
pub trait PersonTryMut: PersonAppendTry {
    // Clear methods (fallible) - may fail in validated or persistent implementations
    fn try_clear_name(&mut self) -> Result<(), Error>;
    fn try_clear_age(&mut self) -> Result<(), Error>;
    fn try_clear_email(&mut self) -> Result<(), Error>;
    fn try_clear_score(&mut self) -> Result<(), Error>;
}

// ============================================================================
// Address Message
// ============================================================================

/// Address message implementation
#[derive(Debug, Clone, PartialEq)]
pub struct Address {
    street: FieldType<StringField, ImplicitOptional, 1, 1>,
    city: FieldType<StringField, ImplicitOptional, 2, 1>,
    zip_code: FieldType<i32, ImplicitOptional, 3, 1>,
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
    bio: FieldType<StringField, ImplicitOptional, 1, 1>,
    website: FieldType<StringField, ExplicitOptional<0>, 2, 1>,
    reputation: FieldType<i32, ImplicitOptional, 3, 1>,
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
    // For 2 explicit optional fields (email, score): ⌈2/8⌉ = 1 byte (stack-allocated)
    // Message fields use heap allocation with Option<Box<M>> for presence tracking
    _shared: SharedFields<1>,

    // Exclusive fields ordered by size (descending)
    // String: 24 bytes (3 words on 64-bit)
    // Direct field types with explicit parameters for clarity
    // Format: FieldType<T, L, FIELD_NUMBER, SHARED_BYTES_LEN>
    name: FieldType<StringField, ImplicitOptional, 1, 1>, // Field 1, implicit presence, 1 byte shared
    email: FieldType<StringField, ExplicitOptional<0>, 3, 1>, // Field 3, explicit presence, bit 0, 1 byte shared

    // Message fields: use heap allocation with Option<Box<M>> for presence tracking
    // No presence bits needed - Option<Box<M>> handles presence directly
    address: FieldType<MessageField<Address>, ImplicitOptional, 6, 1>, // Field 6, heap-allocated presence
    profile: FieldType<MessageField<Profile>, ImplicitOptional, 7, 1>, // Field 7, heap-allocated presence

    // Scalar fields: 4 bytes
    age: FieldType<i32, ImplicitOptional, 2, 1>, // Field 2, implicit presence, 1 byte shared
    score: FieldType<i32, ExplicitOptional<1>, 5, 1>, // Field 5, explicit presence, bit 1, 1 byte shared
}

impl PersonImpl {
    /// Creates a new Person with default values.
    pub fn new() -> Self {
        Self {
            name: Default::default(),
            email: Default::default(),
            address: Default::default(),
            profile: Default::default(),
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
    fn address(&self) -> Option<&Address> {
        self.address.get(&self._shared)
    }

    #[inline]
    fn profile(&self) -> Option<&Profile> {
        self.profile.get(&self._shared)
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
    fn has_address(&self) -> bool {
        // ImplicitOptional fields check if value is not equal to default
        self.address.is_present(&self._shared)
    }

    #[inline]
    fn has_profile(&self) -> bool {
        // ExplicitOptional fields check presence via Field trait
        self.profile.is_present(&self._shared)
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
    fn set_address(&mut self, v: &Address) {
        self.address.set(&mut self._shared, v);
    }

    #[inline]
    fn set_profile(&mut self, v: &Profile) {
        self.profile.set(&mut self._shared, v);
    }

    #[inline]
    fn address_mut(&mut self) -> &mut Address {
        // Ensure field is allocated and marked as present
        if self.address.data.0.is_none() {
            self.address.data.0 = Some(Box::new(Address::default()));
        }
        self.address.data.0.as_mut().unwrap().as_mut()
    }

    #[inline]
    fn profile_mut(&mut self) -> &mut Profile {
        // Ensure field is allocated and marked as present
        if self.profile.data.0.is_none() {
            self.profile.data.0 = Some(Box::new(Profile::default()));
        }
        self.profile.data.0.as_mut().unwrap().as_mut()
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
    fn clear_address(&mut self) {
        self.address.clear(&mut self._shared);
    }

    #[inline]
    fn clear_profile(&mut self) {
        self.profile.clear(&mut self._shared);
    }
}

// Standard implementation also implements fallible traits
// (infallible operations always succeed, so they can be wrapped in Ok())
impl PersonTry for PersonImpl {
    #[inline]
    fn try_name(&self) -> Result<&str, Error> {
        Ok(self.name.get(&self._shared))
    }

    #[inline]
    fn try_age(&self) -> Result<i32, Error> {
        Ok(self.age.get(&self._shared))
    }

    #[inline]
    fn try_email(&self) -> Result<Option<&str>, Error> {
        Ok(self.email.get(&self._shared))
    }

    #[inline]
    fn try_score(&self) -> Result<Option<i32>, Error> {
        Ok(self.score.get(&self._shared))
    }

    #[inline]
    fn try_has_name(&self) -> Result<bool, Error> {
        Ok(self.name.is_present(&self._shared))
    }

    #[inline]
    fn try_has_age(&self) -> Result<bool, Error> {
        Ok(self.age.is_present(&self._shared))
    }

    #[inline]
    fn try_has_email(&self) -> Result<bool, Error> {
        Ok(self.email.is_present(&self._shared))
    }

    #[inline]
    fn try_has_score(&self) -> Result<bool, Error> {
        Ok(self.score.is_present(&self._shared))
    }
}

impl PersonAppendTry for PersonImpl {
    #[inline]
    fn try_set_name(&mut self, v: &str) -> Result<(), Error> {
        self.set_name(v);
        Ok(())
    }

    #[inline]
    fn try_set_age(&mut self, v: i32) -> Result<(), Error> {
        self.set_age(v);
        Ok(())
    }

    #[inline]
    fn try_set_email(&mut self, v: &str) -> Result<(), Error> {
        self.set_email(v);
        Ok(())
    }

    #[inline]
    fn try_set_score(&mut self, v: i32) -> Result<(), Error> {
        self.set_score(v);
        Ok(())
    }
}

impl PersonTryMut for PersonImpl {
    #[inline]
    fn try_clear_name(&mut self) -> Result<(), Error> {
        self.clear_name();
        Ok(())
    }

    #[inline]
    fn try_clear_age(&mut self) -> Result<(), Error> {
        self.clear_age();
        Ok(())
    }

    #[inline]
    fn try_clear_email(&mut self) -> Result<(), Error> {
        self.clear_email();
        Ok(())
    }

    #[inline]
    fn try_clear_score(&mut self) -> Result<(), Error> {
        self.clear_score();
        Ok(())
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
    fn test_wrapper_types() {
        // Test StringField wrapper
        let string_field = StringField("Hello".to_string());
        assert_eq!(string_field.0, "Hello");

        // Test MessageField wrapper
        let address = Address::new();
        let message_field = MessageField(Some(Box::new(address)));
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
