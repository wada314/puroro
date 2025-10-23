//! Generic field operations using trait-based dispatch.
//!
//! This module provides a unified interface for all field types,
//! using generic type parameters to encode field attributes
//! (labels and presence semantics).
//!
//! # Protobuf Field Labels
//!
//! Protobuf defines three field labels:
//! - **Optional**: Field may or may not be present (proto3 implicit presence)
//! - **Explicit Optional**: Field may or may not be present (proto3 explicit presence)
//! - **Repeated**: Field may appear zero or more times
//! - **Map**: Key-value pairs (syntactic sugar for repeated fields)
//!
//! # Presence Semantics
//!
//! - **Implicit Presence (proto3 default)**: Field is always considered "present" with default value
//! - **Explicit Presence (proto3 optional)**: Field has explicit presence tracking

use crate::shared::SharedFields;
use std::marker::PhantomData;

// ============================================================================
// Field Label Markers (Zero-Sized Types)
// ============================================================================

/// Marker trait for protobuf field labels.
///
/// This is a sealed trait - users cannot implement it.
/// Represents the protobuf field label (optional, repeated, map).
pub trait FieldLabel: private::Sealed {}

/// Marker for optional fields with implicit presence (proto3 default).
///
/// These fields are always considered "present" with their default value.
/// No presence tracking is needed.
pub struct Optional;

/// Marker for optional fields with explicit presence (proto3 optional).
///
/// These fields have explicit presence tracking via has_bits.
/// The field may be absent (not set) or present (set to a value).
pub struct ExplicitOptional;

/// Marker for repeated fields.
///
/// These fields may appear zero or more times.
pub struct Repeated;

/// Marker for map fields.
///
/// These are syntactic sugar for repeated fields with key-value pairs.
pub struct Map;

impl FieldLabel for Optional {}
impl FieldLabel for ExplicitOptional {}
impl FieldLabel for Repeated {}
impl FieldLabel for Map {}

// Sealed trait pattern
mod private {
    pub trait Sealed {}
    impl Sealed for super::Optional {}
    impl Sealed for super::ExplicitOptional {}
    impl Sealed for super::Repeated {}
    impl Sealed for super::Map {}
}

// ============================================================================
// Field Descriptor Type
// ============================================================================

/// Type-level descriptor for a protobuf field.
///
/// Encodes both the value type (T) and field label (L).
///
/// # Type Parameters
/// - `T`: The value type (i32, String, etc.)
/// - `L`: The field label (Optional, ExplicitOptional, Repeated, Map)
///
/// # Examples
/// ```ignore
/// type NameField = Field<String, Optional>;         // proto3 implicit presence
/// type EmailField = Field<String, ExplicitOptional>; // proto3 explicit presence
/// type HobbiesField = Field<String, Repeated>;
/// type ScoresField = Field<(String, i32), Map>;
/// ```
pub struct Field<T, L: FieldLabel> {
    _phantom: PhantomData<(T, L)>,
}

// ============================================================================
// Field Operations Trait
// ============================================================================

/// Generic operations for protobuf fields.
///
/// This trait defines the core operations (set, get, clear) for all field types.
/// The behavior is determined by the implementing type.
pub trait FieldOps {
    /// The storage type for this field (how it's stored in the struct)
    type Storage;

    /// The value type for this field (what users pass in)
    type Value;

    /// Sets the field value.
    fn set<const BYTES: usize>(
        shared: &mut SharedFields<BYTES>,
        bit_index: usize,
        storage: &mut Self::Storage,
        value: Self::Value,
    );

    /// Gets the field value.
    fn get(storage: &Self::Storage) -> Self::Value;

    /// Clears the field value.
    fn clear<const BYTES: usize>(
        shared: &mut SharedFields<BYTES>,
        bit_index: usize,
        storage: &mut Self::Storage,
    );
}

// ============================================================================
// Split Set/Get/Clear Traits (Solution to Lifetime Issues)
// ============================================================================

/// Trait for setting field values.
///
/// Note: For types with lifetime issues (like &str),
/// implementations may use inherent methods instead of this trait.
pub trait FieldSet {
    /// The storage type for this field in the struct.
    type Storage;

    /// The value type that users pass in.
    type Value;

    /// Sets the field value and marks it as present.
    fn set<const BYTES: usize>(
        shared: &mut SharedFields<BYTES>,
        bit_index: usize,
        storage: &mut Self::Storage,
        value: Self::Value,
    );
}

/// Trait for getting field values (with lifetimes using GATs).
pub trait FieldGet {
    /// The storage type for this field in the struct.
    type Storage;

    /// The value type returned by get (may borrow from storage).
    type Value<'a>
    where
        Self: 'a;

    /// Gets the field value.
    fn get<'a>(storage: &'a Self::Storage) -> Self::Value<'a>;
}

/// Trait for clearing field values.
pub trait FieldClear {
    /// The storage type for this field in the struct.
    type Storage;

    /// Clears the field value and marks it as not present.
    fn clear<const BYTES: usize>(
        shared: &mut SharedFields<BYTES>,
        bit_index: usize,
        storage: &mut Self::Storage,
    );
}

// ============================================================================
// Implementations for Field<String, Optional> and Field<String, ExplicitOptional>
// ============================================================================

// Note: We can't use `type Value = &str` directly because of lifetime issues.
// Instead, we make the set() method generic over the value type.
impl Field<String, Optional> {
    /// Sets a string field value (proto3 implicit presence).
    #[inline]
    pub fn set<const BYTES: usize>(
        shared: &mut SharedFields<BYTES>,
        bit_index: usize,
        storage: &mut String,
        value: &str, // Can accept any &str!
    ) {
        value.clone_into(storage);
        // Note: Optional fields don't strictly need presence tracking in proto3
        // (they're always considered "present" with default value)
        // But we still track for consistency with explicit optional fields
        shared.has_bits_mut().set(bit_index, true);
    }

    /// Clears a string field (proto3 implicit presence).
    #[inline]
    pub fn clear<const BYTES: usize>(
        shared: &mut SharedFields<BYTES>,
        bit_index: usize,
        storage: &mut String,
    ) {
        storage.clear();
        shared.has_bits_mut().set(bit_index, false);
    }
}

impl Field<String, ExplicitOptional> {
    /// Sets a string field value (proto3 explicit presence).
    #[inline]
    pub fn set<const BYTES: usize>(
        shared: &mut SharedFields<BYTES>,
        bit_index: usize,
        storage: &mut String,
        value: &str, // Can accept any &str!
    ) {
        value.clone_into(storage);
        shared.has_bits_mut().set(bit_index, true);
    }

    /// Clears a string field (proto3 explicit presence).
    #[inline]
    pub fn clear<const BYTES: usize>(
        shared: &mut SharedFields<BYTES>,
        bit_index: usize,
        storage: &mut String,
    ) {
        storage.clear();
        shared.has_bits_mut().set(bit_index, false);
    }
}

impl FieldGet for Field<String, Optional> {
    type Storage = String;
    type Value<'a> = &'a str;

    #[inline]
    fn get<'a>(storage: &'a Self::Storage) -> Self::Value<'a> {
        storage.as_str()
    }
}

impl FieldGet for Field<String, ExplicitOptional> {
    type Storage = String;
    type Value<'a> = &'a str;

    #[inline]
    fn get<'a>(storage: &'a Self::Storage) -> Self::Value<'a> {
        storage.as_str()
    }
}

// ============================================================================
// Generic Implementations for all Copy Types
// ============================================================================

/// Helper trait to identify scalar (Copy) types for singular fields
pub trait ScalarType: Copy + Default {}
impl ScalarType for i32 {}
impl ScalarType for i64 {}
impl ScalarType for u32 {}
impl ScalarType for u64 {}
impl ScalarType for f32 {}
impl ScalarType for f64 {}
impl ScalarType for bool {}
// Add more as needed

impl<T: ScalarType> FieldSet for Field<T, Optional> {
    type Storage = T;
    type Value = T;

    #[inline]
    fn set<const BYTES: usize>(
        shared: &mut SharedFields<BYTES>,
        bit_index: usize,
        storage: &mut Self::Storage,
        value: Self::Value,
    ) {
        *storage = value;
        // Note: Optional fields don't need presence tracking in proto3
        // (they're always considered "present" with default value)
        // But we still track for consistency with explicit optional fields
        shared.has_bits_mut().set(bit_index, true);
    }
}

impl<T: ScalarType> FieldSet for Field<T, ExplicitOptional> {
    type Storage = T;
    type Value = T;

    #[inline]
    fn set<const BYTES: usize>(
        shared: &mut SharedFields<BYTES>,
        bit_index: usize,
        storage: &mut Self::Storage,
        value: Self::Value,
    ) {
        *storage = value;
        shared.has_bits_mut().set(bit_index, true);
    }
}

impl<T: ScalarType> FieldGet for Field<T, Optional> {
    type Storage = T;
    type Value<'a>
        = T
    where
        T: 'a;

    #[inline]
    fn get<'a>(storage: &'a Self::Storage) -> Self::Value<'a> {
        *storage
    }
}

impl<T: ScalarType> FieldGet for Field<T, ExplicitOptional> {
    type Storage = T;
    type Value<'a>
        = T
    where
        T: 'a;

    #[inline]
    fn get<'a>(storage: &'a Self::Storage) -> Self::Value<'a> {
        *storage
    }
}

impl<T: ScalarType> FieldClear for Field<T, Optional> {
    type Storage = T;

    #[inline]
    fn clear<const BYTES: usize>(
        shared: &mut SharedFields<BYTES>,
        bit_index: usize,
        storage: &mut Self::Storage,
    ) {
        *storage = T::default();
        shared.has_bits_mut().set(bit_index, false);
    }
}

impl<T: ScalarType> FieldClear for Field<T, ExplicitOptional> {
    type Storage = T;

    #[inline]
    fn clear<const BYTES: usize>(
        shared: &mut SharedFields<BYTES>,
        bit_index: usize,
        storage: &mut Self::Storage,
    ) {
        *storage = T::default();
        shared.has_bits_mut().set(bit_index, false);
    }
}

// ============================================================================
// Implementations for Repeated Fields
// ============================================================================

/// Repeated fields for scalar types
impl<T: ScalarType> FieldSet for Field<T, Repeated> {
    type Storage = Vec<T>;
    type Value = T;

    #[inline]
    fn set<const BYTES: usize>(
        shared: &mut SharedFields<BYTES>,
        bit_index: usize,
        storage: &mut Self::Storage,
        value: Self::Value,
    ) {
        // For repeated fields, "set" means "add"
        storage.push(value);
        shared.has_bits_mut().set(bit_index, true);
    }
}

impl<T: ScalarType> FieldGet for Field<T, Repeated> {
    type Storage = Vec<T>;
    type Value<'a>
        = &'a [T]
    where
        T: 'a;

    #[inline]
    fn get<'a>(storage: &'a Self::Storage) -> Self::Value<'a> {
        storage.as_slice()
    }
}

impl<T: ScalarType> FieldClear for Field<T, Repeated> {
    type Storage = Vec<T>;

    #[inline]
    fn clear<const BYTES: usize>(
        shared: &mut SharedFields<BYTES>,
        bit_index: usize,
        storage: &mut Self::Storage,
    ) {
        storage.clear();
        shared.has_bits_mut().set(bit_index, false);
    }
}

/// Repeated String fields
impl Field<String, Repeated> {
    /// Adds a string value to a repeated field.
    #[inline]
    pub fn set<const BYTES: usize>(
        shared: &mut SharedFields<BYTES>,
        bit_index: usize,
        storage: &mut Vec<String>,
        value: &str, // Can accept any &str
    ) {
        storage.push(value.to_string());
        shared.has_bits_mut().set(bit_index, true);
    }

    /// Clears a repeated string field.
    #[inline]
    pub fn clear<const BYTES: usize>(
        shared: &mut SharedFields<BYTES>,
        bit_index: usize,
        storage: &mut Vec<String>,
    ) {
        storage.clear();
        shared.has_bits_mut().set(bit_index, false);
    }
}

impl FieldGet for Field<String, Repeated> {
    type Storage = Vec<String>;
    type Value<'a>
        = &'a [String]
    where
        String: 'a;

    #[inline]
    fn get<'a>(storage: &'a Self::Storage) -> Self::Value<'a> {
        storage.as_slice()
    }
}

// ============================================================================
// Usage Pattern in Generated Code
// ============================================================================

/*
// Type aliases for clarity
type NameField = Field<String, Optional>;         // proto3 implicit presence
type AgeField = Field<i32, Optional>;             // proto3 implicit presence
type EmailField = Field<String, ExplicitOptional>; // proto3 explicit presence
type HobbiesField = Field<String, Repeated>;

impl PersonAppend for PersonImpl {
    fn set_name(&mut self, v: &str) {
        NameField::set(&mut self._shared, IDX_NAME, &mut self.name, v);
    }

    fn set_age(&mut self, v: i32) {
        AgeField::set(&mut self._shared, IDX_AGE, &mut self.age, v);
    }
}

impl Person for PersonImpl {
    fn name(&self) -> &str {
        NameField::get(&self.name)
    }

    fn age(&self) -> i32 {
        AgeField::get(&self.age)
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optional_i32() {
        type AgeField = Field<i32, Optional>;

        let mut shared = SharedFields::<1>::new();
        let mut storage: i32 = 0;

        AgeField::set(&mut shared, 0, &mut storage, 42);

        assert_eq!(storage, 42);
        assert!(shared.has_bits()[0]);

        assert_eq!(AgeField::get(&storage), 42);

        AgeField::clear(&mut shared, 0, &mut storage);
        assert_eq!(storage, 0);
        assert!(!shared.has_bits()[0]);
    }

    #[test]
    fn test_optional_string() {
        type NameField = Field<String, Optional>;

        let mut shared = SharedFields::<1>::new();
        let mut storage = String::new();

        NameField::set(&mut shared, 0, &mut storage, "Alice");

        assert_eq!(storage, "Alice");
        assert!(shared.has_bits()[0]);

        assert_eq!(NameField::get(&storage), "Alice");

        NameField::clear(&mut shared, 0, &mut storage);
        assert_eq!(storage, "");
        assert!(!shared.has_bits()[0]);
    }

    #[test]
    fn test_repeated_i32() {
        type ScoresField = Field<i32, Repeated>;

        let mut shared = SharedFields::<1>::new();
        let mut storage = Vec::new();

        ScoresField::set(&mut shared, 0, &mut storage, 10);
        ScoresField::set(&mut shared, 0, &mut storage, 20);
        ScoresField::set(&mut shared, 0, &mut storage, 30);

        assert_eq!(storage, vec![10, 20, 30]);
        assert!(shared.has_bits()[0]);

        assert_eq!(ScoresField::get(&storage), &[10, 20, 30]);

        ScoresField::clear(&mut shared, 0, &mut storage);
        assert_eq!(storage.len(), 0);
        assert!(!shared.has_bits()[0]);
    }

    #[test]
    fn test_repeated_string() {
        type HobbiesField = Field<String, Repeated>;

        let mut shared = SharedFields::<1>::new();
        let mut storage = Vec::new();

        HobbiesField::set(&mut shared, 0, &mut storage, "reading");
        HobbiesField::set(&mut shared, 0, &mut storage, "coding");

        assert_eq!(storage.len(), 2);
        assert_eq!(storage[0], "reading");
        assert_eq!(storage[1], "coding");
        assert!(shared.has_bits()[0]);

        let hobbies = HobbiesField::get(&storage);
        assert_eq!(hobbies.len(), 2);
    }

    #[test]
    fn test_multiple_scalar_types() {
        type I64Field = Field<i64, Optional>;
        type F32Field = Field<f32, Optional>;
        type BoolField = Field<bool, Optional>;

        let mut shared = SharedFields::<1>::new();

        let mut i64_val: i64 = 0;
        I64Field::set(&mut shared, 0, &mut i64_val, 12345);
        assert_eq!(I64Field::get(&i64_val), 12345);

        let mut f32_val: f32 = 0.0;
        F32Field::set(&mut shared, 0, &mut f32_val, 3.14);
        assert_eq!(F32Field::get(&f32_val), 3.14);

        let mut bool_val: bool = false;
        BoolField::set(&mut shared, 0, &mut bool_val, true);
        assert_eq!(BoolField::get(&bool_val), true);
    }

    #[test]
    fn test_explicit_optional_i32() {
        type AgeField = Field<i32, ExplicitOptional>;

        let mut shared = SharedFields::<1>::new();
        let mut storage: i32 = 0;

        AgeField::set(&mut shared, 0, &mut storage, 42);

        assert_eq!(storage, 42);
        assert!(shared.has_bits()[0]);

        assert_eq!(AgeField::get(&storage), 42);

        AgeField::clear(&mut shared, 0, &mut storage);
        assert_eq!(storage, 0);
        assert!(!shared.has_bits()[0]);
    }

    #[test]
    fn test_explicit_optional_string() {
        type NameField = Field<String, ExplicitOptional>;

        let mut shared = SharedFields::<1>::new();
        let mut storage = String::new();

        NameField::set(&mut shared, 0, &mut storage, "Bob");

        assert_eq!(storage, "Bob");
        assert!(shared.has_bits()[0]);

        assert_eq!(NameField::get(&storage), "Bob");

        NameField::clear(&mut shared, 0, &mut storage);
        assert_eq!(storage, "");
        assert!(!shared.has_bits()[0]);
    }
}
