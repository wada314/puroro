//! Hand-written code for Person message.
//!
//! This represents our ideal API for the generated code using the Closed Struct approach.

use puroro::{error::Error, Message};

/// Immutable trait for Person message.
///
/// This trait provides read-only access to Person fields.
/// Implementations can be optimized for immutable access patterns (e.g., zero-copy views).
pub trait Person {
    // Getters
    fn name(&self) -> &str;
    fn age(&self) -> i32;
    fn email(&self) -> &str;

    // Presence checks (for Proto3 optional semantics)
    fn has_name(&self) -> bool;
    fn has_age(&self) -> bool;
    fn has_email(&self) -> bool;
}

/// Mutable trait for Person message.
///
/// This trait extends Person with mutation capabilities.
/// Use this when you need to modify message fields.
pub trait PersonMut: Person {
    // Setters
    fn set_name(&mut self, v: impl Into<String>);
    fn set_age(&mut self, v: i32);
    fn set_email(&mut self, v: impl Into<String>);

    // Clear methods
    fn clear_name(&mut self);
    fn clear_age(&mut self);
    fn clear_email(&mut self);
}

/// Standard implementation of Person message.
///
/// Uses bitflags for efficient optional field tracking.
/// Memory layout: 4 bytes (bitflags) + fields without Option<T> overhead.
#[derive(Debug, Clone, PartialEq)]
pub struct PersonImpl {
    // Bitflags for tracking which fields have been explicitly set
    // Using u32 allows up to 32 fields per message
    _has_bits: u32,

    // Fields stored directly (no Option<T> wrapper)
    name: String,
    age: i32,
    email: String,
}

// Bit positions for each field
const HAS_NAME: u32 = 1 << 0;
const HAS_AGE: u32 = 1 << 1;
const HAS_EMAIL: u32 = 1 << 2;

impl PersonImpl {
    /// Creates a new Person with default values.
    pub fn new() -> Self {
        Self {
            _has_bits: 0,
            name: String::new(),
            age: 0,
            email: String::new(),
        }
    }
}

impl Default for PersonImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl Person for PersonImpl {
    fn name(&self) -> &str {
        &self.name
    }

    fn age(&self) -> i32 {
        self.age
    }

    fn email(&self) -> &str {
        &self.email
    }

    fn has_name(&self) -> bool {
        (self._has_bits & HAS_NAME) != 0
    }

    fn has_age(&self) -> bool {
        (self._has_bits & HAS_AGE) != 0
    }

    fn has_email(&self) -> bool {
        (self._has_bits & HAS_EMAIL) != 0
    }
}

impl PersonMut for PersonImpl {
    fn set_name(&mut self, v: impl Into<String>) {
        self.name = v.into();
        self._has_bits |= HAS_NAME;
    }

    fn set_age(&mut self, v: i32) {
        self.age = v;
        self._has_bits |= HAS_AGE;
    }

    fn set_email(&mut self, v: impl Into<String>) {
        self.email = v.into();
        self._has_bits |= HAS_EMAIL;
    }

    fn clear_name(&mut self) {
        self.name.clear();
        self._has_bits &= !HAS_NAME;
    }

    fn clear_age(&mut self) {
        self.age = 0;
        self._has_bits &= !HAS_AGE;
    }

    fn clear_email(&mut self) {
        self.email.clear();
        self._has_bits &= !HAS_EMAIL;
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
