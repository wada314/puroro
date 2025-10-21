//! Hand-written code for Person message.
//!
//! This represents our ideal API for the generated code using the Closed Struct approach.

use puroro::{error::Error, Message};

/// Infallible immutable trait for Person message.
///
/// This trait provides read-only access to Person fields without error handling.
/// Use this for implementations that guarantee valid data (e.g., fully deserialized messages).
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
    fn try_email(&self) -> Result<&str, Error>;

    // Presence checks (fallible) - may fail when reading metadata
    fn try_has_name(&self) -> Result<bool, Error>;
    fn try_has_age(&self) -> Result<bool, Error>;
    fn try_has_email(&self) -> Result<bool, Error>;
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
}

/// Standard implementation of Person message.
///
/// Uses bitflags for efficient optional field tracking.
/// Fields are ordered by size (descending) to minimize padding.
/// Memory layout optimized for performance.
#[derive(Debug, Clone, PartialEq)]
pub struct PersonImpl {
    // Fields ordered by size (descending) for optimal memory layout
    // String: 24 bytes (3 words on 64-bit)
    name: String,
    email: String,

    // Smaller fields: 4 bytes each
    _has_bits: u32, // Bitflags for tracking which fields have been explicitly set
    age: i32,
}

// Bit positions for each field
const HAS_NAME: u32 = 1 << 0;
const HAS_AGE: u32 = 1 << 1;
const HAS_EMAIL: u32 = 1 << 2;

impl PersonImpl {
    /// Creates a new Person with default values.
    pub fn new() -> Self {
        Self {
            name: String::new(),
            email: String::new(),
            _has_bits: 0,
            age: 0,
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
        &self.name
    }

    #[inline]
    fn age(&self) -> i32 {
        self.age
    }

    #[inline]
    fn email(&self) -> &str {
        &self.email
    }

    #[inline]
    fn has_name(&self) -> bool {
        (self._has_bits & HAS_NAME) != 0
    }

    #[inline]
    fn has_age(&self) -> bool {
        (self._has_bits & HAS_AGE) != 0
    }

    #[inline]
    fn has_email(&self) -> bool {
        (self._has_bits & HAS_EMAIL) != 0
    }
}

impl PersonAppend for PersonImpl {
    #[inline]
    fn set_name(&mut self, v: &str) {
        v.clone_into(&mut self.name); // Reuse allocation
        self._has_bits |= HAS_NAME;
    }

    #[inline]
    fn set_age(&mut self, v: i32) {
        self.age = v;
        self._has_bits |= HAS_AGE;
    }

    #[inline]
    fn set_email(&mut self, v: &str) {
        v.clone_into(&mut self.email); // Reuse allocation
        self._has_bits |= HAS_EMAIL;
    }
}

impl PersonMut for PersonImpl {
    #[inline]
    fn clear_name(&mut self) {
        self.name.clear();
        self._has_bits &= !HAS_NAME;
    }

    #[inline]
    fn clear_age(&mut self) {
        self.age = 0;
        self._has_bits &= !HAS_AGE;
    }

    #[inline]
    fn clear_email(&mut self) {
        self.email.clear();
        self._has_bits &= !HAS_EMAIL;
    }
}

// Standard implementation also implements fallible traits
// (infallible operations always succeed, so they can be wrapped in Ok())
impl PersonTry for PersonImpl {
    #[inline]
    fn try_name(&self) -> Result<&str, Error> {
        Ok(&self.name)
    }

    #[inline]
    fn try_age(&self) -> Result<i32, Error> {
        Ok(self.age)
    }

    #[inline]
    fn try_email(&self) -> Result<&str, Error> {
        Ok(&self.email)
    }

    #[inline]
    fn try_has_name(&self) -> Result<bool, Error> {
        Ok(Person::has_name(self))
    }

    #[inline]
    fn try_has_age(&self) -> Result<bool, Error> {
        Ok(Person::has_age(self))
    }

    #[inline]
    fn try_has_email(&self) -> Result<bool, Error> {
        Ok(Person::has_email(self))
    }
}

impl PersonAppendTry for PersonImpl {
    #[inline]
    fn try_set_name(&mut self, v: &str) -> Result<(), Error> {
        PersonAppend::set_name(self, v);
        Ok(())
    }

    #[inline]
    fn try_set_age(&mut self, v: i32) -> Result<(), Error> {
        PersonAppend::set_age(self, v);
        Ok(())
    }

    #[inline]
    fn try_set_email(&mut self, v: &str) -> Result<(), Error> {
        PersonAppend::set_email(self, v);
        Ok(())
    }
}

impl PersonTryMut for PersonImpl {
    #[inline]
    fn try_clear_name(&mut self) -> Result<(), Error> {
        PersonMut::clear_name(self);
        Ok(())
    }

    #[inline]
    fn try_clear_age(&mut self) -> Result<(), Error> {
        PersonMut::clear_age(self);
        Ok(())
    }

    #[inline]
    fn try_clear_email(&mut self) -> Result<(), Error> {
        PersonMut::clear_email(self);
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
