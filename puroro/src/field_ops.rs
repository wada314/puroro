//! Generic field operations using trait-based dispatch.
//!
//! This module provides a unified interface for all field types,
//! using generic type parameters to encode field attributes
//! (labels and presence semantics).
//!
//! # Protobuf Field Labels
//!
//! Protobuf defines field labels that vary by version:
//! - **ImplicitOptional**: Field with implicit presence (always "present" with default value)
//! - **ExplicitOptional**: Field with explicit presence tracking (may be absent or present)
//! - **Repeated**: Field may appear zero or more times
//! - **Map**: Key-value pairs (syntactic sugar for repeated fields)
//!
//! # Presence Semantics by Protobuf Version
//!
//! - **Proto2**: All fields use explicit presence (`optional` keyword required)
//! - **Proto3**: Default fields use implicit presence, `optional` fields use explicit presence
//! - **Editions**: Presence semantics configurable via edition settings

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

/// Marker for optional fields with implicit presence.
///
/// These fields are always considered "present" with their default value.
/// No presence tracking is needed.
///
/// Used for:
/// - Proto3 default fields (e.g., `string name = 1;`)
/// - Proto2 fields (e.g., `optional string name = 1;`)
/// - Editions with implicit presence semantics
pub struct ImplicitOptional;

/// Marker for optional fields with explicit presence.
///
/// These fields have explicit presence tracking via has_bits.
/// The field may be absent (not set) or present (set to a value).
///
/// Used for:
/// - Proto3 explicit optional fields (e.g., `optional string name = 1;`)
/// - Editions with explicit presence semantics
pub struct ExplicitOptional;

/// Marker for repeated fields.
///
/// These fields may appear zero or more times.
pub struct Repeated;

/// Marker for map fields.
///
/// These are syntactic sugar for repeated fields with key-value pairs.
pub struct Map;

impl FieldLabel for ImplicitOptional {}
impl FieldLabel for ExplicitOptional {}
impl FieldLabel for Repeated {}
impl FieldLabel for Map {}

// Sealed trait pattern
mod private {
    pub trait Sealed {}
    impl Sealed for super::ImplicitOptional {}
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
/// - `L`: The field label (ImplicitOptional, ExplicitOptional, Repeated, Map)
///
/// # Examples
/// ```ignore
/// type NameField = Field<String, ImplicitOptional>;  // implicit presence
/// type EmailField = Field<String, ExplicitOptional>; // explicit presence
/// type HobbiesField = Field<String, Repeated>;
/// type ScoresField = Field<(String, i32), Map>;
/// ```
pub struct Field<T, L: FieldLabel> {
    _phantom: PhantomData<(T, L)>,
}

// ============================================================================
// Comprehensive Field Descriptor (Unified Approach)
// ============================================================================

/// Comprehensive field descriptor containing all protobuf field information.
///
/// This trait provides metadata for field operations:
/// - Field number (for serialization)
/// - Bit index (for presence tracking)
/// - Default value (for initialization)
///
/// # Type Parameters
/// - `T`: The value type (i32, String, etc.)
/// - `L`: The field label (ImplicitOptional, ExplicitOptional, Repeated, Map)
/// - `const FIELD_NUMBER`: The protobuf field number
/// - `const BIT_INDEX`: The bit index for presence tracking
pub trait FieldDescriptor<T, L: FieldLabel, const FIELD_NUMBER: u32, const BIT_INDEX: usize> {
    /// The storage type for this field (how it's stored in the struct)
    type Storage;

    /// The value type for this field (what users pass in)
    type Value;

    /// The return type for get operations (may borrow from storage)
    type GetValue<'a>
    where
        Self: 'a;

    /// The protobuf field number
    const FIELD_NUMBER: u32 = FIELD_NUMBER;

    /// The bit index for presence tracking in SharedFields
    const BIT_INDEX: usize = BIT_INDEX;

    /// Creates the default value for this field
    fn default_value() -> Self::Storage;

    /// Sets the field value
    fn set<const BYTES: usize>(
        shared: &mut SharedFields<BYTES>,
        storage: &mut Self::Storage,
        value: Self::Value,
    );

    /// Gets the field value
    fn get<'a, const BYTES: usize>(
        shared: &'a SharedFields<BYTES>,
        storage: &'a Self::Storage,
    ) -> Self::GetValue<'a>;

    /// Clears the field value
    fn clear<const BYTES: usize>(shared: &mut SharedFields<BYTES>, storage: &mut Self::Storage);

    /// Checks if the field is present (for ExplicitOptional fields)
    fn is_present<const BYTES: usize>(shared: &SharedFields<BYTES>) -> bool;

    /// Gets the field number
    fn field_number() -> u32 {
        FIELD_NUMBER
    }

    /// Gets the bit index
    fn bit_index() -> usize {
        BIT_INDEX
    }
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
    ///
    /// For ExplicitOptional fields, this method should check presence
    /// and return default value if not present.
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
// FieldDescriptor Implementations
// ============================================================================

/// Implementation for String fields with ImplicitOptional
impl<const FIELD_NUMBER: u32, const BIT_INDEX: usize>
    FieldDescriptor<String, ImplicitOptional, FIELD_NUMBER, BIT_INDEX>
    for Field<String, ImplicitOptional>
{
    type Storage = String;
    type Value = &'static str; // Can accept any &str
    type GetValue<'a> = &'a str;

    fn default_value() -> Self::Storage {
        String::new()
    }

    fn set<const BYTES: usize>(
        _shared: &mut SharedFields<BYTES>,
        storage: &mut Self::Storage,
        value: Self::Value,
    ) {
        value.clone_into(storage);
        // ImplicitOptional fields don't need presence tracking
    }

    fn get<'a, const BYTES: usize>(
        _shared: &'a SharedFields<BYTES>,
        storage: &'a Self::Storage,
    ) -> Self::GetValue<'a> {
        storage.as_str()
    }

    fn clear<const BYTES: usize>(_shared: &mut SharedFields<BYTES>, storage: &mut Self::Storage) {
        storage.clear();
        // ImplicitOptional fields don't need presence tracking
    }

    fn is_present<const BYTES: usize>(_shared: &SharedFields<BYTES>) -> bool {
        true // ImplicitOptional fields are always present
    }
}

/// Implementation for scalar types with ImplicitOptional
impl<T: ScalarType, const FIELD_NUMBER: u32, const BIT_INDEX: usize>
    FieldDescriptor<T, ImplicitOptional, FIELD_NUMBER, BIT_INDEX> for Field<T, ImplicitOptional>
{
    type Storage = T;
    type Value = T;
    type GetValue<'a>
        = T
    where
        T: 'a;

    fn default_value() -> Self::Storage {
        T::default()
    }

    fn set<const BYTES: usize>(
        _shared: &mut SharedFields<BYTES>,
        storage: &mut Self::Storage,
        value: Self::Value,
    ) {
        *storage = value;
        // ImplicitOptional fields don't need presence tracking
    }

    fn get<'a, const BYTES: usize>(
        _shared: &'a SharedFields<BYTES>,
        storage: &'a Self::Storage,
    ) -> Self::GetValue<'a> {
        *storage
    }

    fn clear<const BYTES: usize>(_shared: &mut SharedFields<BYTES>, storage: &mut Self::Storage) {
        *storage = T::default();
        // ImplicitOptional fields don't need presence tracking
    }

    fn is_present<const BYTES: usize>(_shared: &SharedFields<BYTES>) -> bool {
        true // ImplicitOptional fields are always present
    }
}

/// Implementation for scalar types with ExplicitOptional
impl<T: ScalarType, const FIELD_NUMBER: u32, const BIT_INDEX: usize>
    FieldDescriptor<T, ExplicitOptional, FIELD_NUMBER, BIT_INDEX> for Field<T, ExplicitOptional>
{
    type Storage = T;
    type Value = T;
    type GetValue<'a>
        = Option<T>
    where
        T: 'a;

    fn default_value() -> Self::Storage {
        T::default()
    }

    fn set<const BYTES: usize>(
        shared: &mut SharedFields<BYTES>,
        storage: &mut Self::Storage,
        value: Self::Value,
    ) {
        *storage = value;
        shared.has_bits_mut().set(BIT_INDEX, true);
    }

    fn get<'a, const BYTES: usize>(
        shared: &'a SharedFields<BYTES>,
        storage: &'a Self::Storage,
    ) -> Self::GetValue<'a> {
        if shared.has_bits()[BIT_INDEX] {
            Some(*storage)
        } else {
            None
        }
    }

    fn clear<const BYTES: usize>(shared: &mut SharedFields<BYTES>, storage: &mut Self::Storage) {
        *storage = T::default();
        shared.has_bits_mut().set(BIT_INDEX, false);
    }

    fn is_present<const BYTES: usize>(shared: &SharedFields<BYTES>) -> bool {
        shared.has_bits()[BIT_INDEX]
    }
}

/// Implementation for repeated scalar types
impl<T: ScalarType, const FIELD_NUMBER: u32, const BIT_INDEX: usize>
    FieldDescriptor<T, Repeated, FIELD_NUMBER, BIT_INDEX> for Field<T, Repeated>
{
    type Storage = Vec<T>;
    type Value = T;
    type GetValue<'a>
        = &'a [T]
    where
        T: 'a;

    fn default_value() -> Self::Storage {
        Vec::new()
    }

    fn set<const BYTES: usize>(
        _shared: &mut SharedFields<BYTES>,
        storage: &mut Self::Storage,
        value: Self::Value,
    ) {
        // For repeated fields, "set" means "add"
        storage.push(value);
        // Repeated fields don't need presence tracking
    }

    fn get<'a, const BYTES: usize>(
        _shared: &'a SharedFields<BYTES>,
        storage: &'a Self::Storage,
    ) -> Self::GetValue<'a> {
        storage.as_slice()
    }

    fn clear<const BYTES: usize>(_shared: &mut SharedFields<BYTES>, storage: &mut Self::Storage) {
        storage.clear();
        // Repeated fields don't need presence tracking
    }

    fn is_present<const BYTES: usize>(_shared: &SharedFields<BYTES>) -> bool {
        true // Repeated fields are always considered "present", even when empty
    }
}

// ============================================================================
// Implementations for Field<String, ImplicitOptional> and Field<String, ExplicitOptional>
// ============================================================================

// Note: We can't use `type Value = &str` directly because of lifetime issues.
// Instead, we make the set() method generic over the value type.
impl Field<String, ImplicitOptional> {
    /// Sets a string field value (implicit presence).
    #[inline]
    pub fn set<const BYTES: usize>(
        _shared: &mut SharedFields<BYTES>,
        _bit_index: usize,
        storage: &mut String,
        value: &str, // Can accept any &str!
    ) {
        value.clone_into(storage);
        // Note: ImplicitOptional fields don't need presence tracking
        // (they're always considered "present" with default value)
    }

    /// Clears a string field (implicit presence).
    #[inline]
    pub fn clear<const BYTES: usize>(
        _shared: &mut SharedFields<BYTES>,
        _bit_index: usize,
        storage: &mut String,
    ) {
        storage.clear();
        // Note: ImplicitOptional fields don't need presence tracking
        // (they're always considered "present" with default value)
    }
}

impl Field<String, ExplicitOptional> {
    /// Sets a string field value (explicit presence).
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

    /// Clears a string field (explicit presence).
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

impl FieldGet for Field<String, ImplicitOptional> {
    type Storage = String;
    type Value<'a> = &'a str;

    #[inline]
    fn get<'a>(storage: &'a Self::Storage) -> Self::Value<'a> {
        storage.as_str()
    }
}

impl Field<String, ExplicitOptional> {
    /// Gets the field value, checking presence first.
    ///
    /// Returns an empty string if the field is not present.
    #[inline]
    pub fn get<'a, const BYTES: usize>(
        shared: &'a SharedFields<BYTES>,
        bit_index: usize,
        storage: &'a String,
    ) -> &'a str {
        if shared.has_bits()[bit_index] {
            storage.as_str()
        } else {
            ""
        }
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

impl<T: ScalarType> FieldSet for Field<T, ImplicitOptional> {
    type Storage = T;
    type Value = T;

    #[inline]
    fn set<const BYTES: usize>(
        _shared: &mut SharedFields<BYTES>,
        _bit_index: usize,
        storage: &mut Self::Storage,
        value: Self::Value,
    ) {
        *storage = value;
        // Note: ImplicitOptional fields don't need presence tracking
        // (they're always considered "present" with default value)
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

impl<T: ScalarType> FieldGet for Field<T, ImplicitOptional> {
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

impl<T: ScalarType> Field<T, ExplicitOptional> {
    /// Gets the field value, checking presence first.
    ///
    /// Returns the default value if the field is not present.
    #[inline]
    pub fn get<const BYTES: usize>(
        shared: &SharedFields<BYTES>,
        bit_index: usize,
        storage: &T,
    ) -> T {
        if shared.has_bits()[bit_index] {
            *storage
        } else {
            T::default()
        }
    }
}

impl<T: ScalarType> FieldClear for Field<T, ImplicitOptional> {
    type Storage = T;

    #[inline]
    fn clear<const BYTES: usize>(
        _shared: &mut SharedFields<BYTES>,
        _bit_index: usize,
        storage: &mut Self::Storage,
    ) {
        *storage = T::default();
        // Note: ImplicitOptional fields don't need presence tracking
        // (they're always considered "present" with default value)
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
        _shared: &mut SharedFields<BYTES>,
        _bit_index: usize,
        storage: &mut Self::Storage,
        value: Self::Value,
    ) {
        // For repeated fields, "set" means "add"
        storage.push(value);
        // Note: Repeated fields don't need presence tracking
        // (they're always considered "present", even when empty)
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
        _shared: &mut SharedFields<BYTES>,
        _bit_index: usize,
        storage: &mut Self::Storage,
    ) {
        storage.clear();
        // Note: Repeated fields don't need presence tracking
        // (they're always considered "present", even when empty)
    }
}

/// Repeated String fields
impl Field<String, Repeated> {
    /// Adds a string value to a repeated field.
    #[inline]
    pub fn set<const BYTES: usize>(
        _shared: &mut SharedFields<BYTES>,
        _bit_index: usize,
        storage: &mut Vec<String>,
        value: &str, // Can accept any &str
    ) {
        storage.push(value.to_string());
        // Note: Repeated fields don't need presence tracking
        // (they're always considered "present", even when empty)
    }

    /// Clears a repeated string field.
    #[inline]
    pub fn clear<const BYTES: usize>(
        _shared: &mut SharedFields<BYTES>,
        _bit_index: usize,
        storage: &mut Vec<String>,
    ) {
        storage.clear();
        // Note: Repeated fields don't need presence tracking
        // (they're always considered "present", even when empty)
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
type NameField = Field<String, ImplicitOptional>;  // implicit presence
type AgeField = Field<i32, ImplicitOptional>;     // implicit presence
type EmailField = Field<String, ExplicitOptional>; // explicit presence
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
    fn test_implicit_optional_i32() {
        type AgeField = Field<i32, ImplicitOptional>;

        let mut shared = SharedFields::<1>::new();
        let mut storage: i32 = 0;

        <AgeField as FieldSet>::set(&mut shared, 0, &mut storage, 42);

        assert_eq!(storage, 42);
        // ImplicitOptional fields don't track presence in has_bits
        // (they're always considered "present")

        assert_eq!(<AgeField as FieldGet>::get(&storage), 42);

        <AgeField as FieldClear>::clear(&mut shared, 0, &mut storage);
        assert_eq!(storage, 0);
        // ImplicitOptional fields don't track presence in has_bits
    }

    #[test]
    fn test_implicit_optional_string() {
        type NameField = Field<String, ImplicitOptional>;

        let mut shared = SharedFields::<1>::new();
        let mut storage = String::new();

        NameField::set(&mut shared, 0, &mut storage, "Alice");

        assert_eq!(storage, "Alice");
        // ImplicitOptional fields don't track presence in has_bits
        // (they're always considered "present")

        assert_eq!(<NameField as FieldGet>::get(&storage), "Alice");

        NameField::clear(&mut shared, 0, &mut storage);
        assert_eq!(storage, "");
        // ImplicitOptional fields don't track presence in has_bits
    }

    #[test]
    fn test_repeated_i32() {
        type ScoresField = Field<i32, Repeated>;

        let mut shared = SharedFields::<1>::new();
        let mut storage = Vec::new();

        <ScoresField as FieldSet>::set(&mut shared, 0, &mut storage, 10);
        <ScoresField as FieldSet>::set(&mut shared, 0, &mut storage, 20);
        <ScoresField as FieldSet>::set(&mut shared, 0, &mut storage, 30);

        assert_eq!(storage, vec![10, 20, 30]);
        // Repeated fields don't track presence in has_bits
        // (they're always considered "present", even when empty)

        assert_eq!(<ScoresField as FieldGet>::get(&storage), &[10, 20, 30]);

        <ScoresField as FieldClear>::clear(&mut shared, 0, &mut storage);
        assert_eq!(storage.len(), 0);
        // Repeated fields don't track presence in has_bits
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
        // Repeated fields don't track presence in has_bits
        // (they're always considered "present", even when empty)

        let hobbies = <HobbiesField as FieldGet>::get(&storage);
        assert_eq!(hobbies.len(), 2);
    }

    #[test]
    fn test_multiple_scalar_types() {
        type I64Field = Field<i64, ImplicitOptional>;
        type F32Field = Field<f32, ImplicitOptional>;
        type BoolField = Field<bool, ImplicitOptional>;

        let mut shared = SharedFields::<1>::new();

        let mut i64_val: i64 = 0;
        <I64Field as FieldSet>::set(&mut shared, 0, &mut i64_val, 12345);
        assert_eq!(<I64Field as FieldGet>::get(&i64_val), 12345);

        let mut f32_val: f32 = 0.0;
        <F32Field as FieldSet>::set(&mut shared, 0, &mut f32_val, 3.14);
        assert_eq!(<F32Field as FieldGet>::get(&f32_val), 3.14);

        let mut bool_val: bool = false;
        <BoolField as FieldSet>::set(&mut shared, 0, &mut bool_val, true);
        assert_eq!(<BoolField as FieldGet>::get(&bool_val), true);
    }

    #[test]
    fn test_explicit_optional_i32() {
        type AgeField = Field<i32, ExplicitOptional>;

        let mut shared = SharedFields::<1>::new();
        let mut storage: i32 = 0;

        // Initially not set - should return default value
        assert_eq!(AgeField::get(&shared, 0, &storage), 0);
        assert!(!shared.has_bits()[0]);

        <AgeField as FieldSet>::set(&mut shared, 0, &mut storage, 42);

        assert_eq!(storage, 42);
        assert!(shared.has_bits()[0]);

        // Now set - should return actual value
        assert_eq!(AgeField::get(&shared, 0, &storage), 42);

        <AgeField as FieldClear>::clear(&mut shared, 0, &mut storage);
        assert_eq!(storage, 0);
        assert!(!shared.has_bits()[0]);

        // After clear - should return default value again
        assert_eq!(AgeField::get(&shared, 0, &storage), 0);
    }

    #[test]
    fn test_field_descriptor_metadata() {
        type NameField = Field<String, ImplicitOptional>;
        type AgeField = Field<i32, ImplicitOptional>;
        type ScoresField = Field<i32, Repeated>;

        // Test field number and bit index constants
        assert_eq!(
            <NameField as FieldDescriptor<String, ImplicitOptional, 1, 0>>::FIELD_NUMBER,
            1
        );
        assert_eq!(
            <NameField as FieldDescriptor<String, ImplicitOptional, 1, 0>>::BIT_INDEX,
            0
        );

        assert_eq!(
            <AgeField as FieldDescriptor<i32, ImplicitOptional, 2, 1>>::FIELD_NUMBER,
            2
        );
        assert_eq!(
            <AgeField as FieldDescriptor<i32, ImplicitOptional, 2, 1>>::BIT_INDEX,
            1
        );

        assert_eq!(
            <ScoresField as FieldDescriptor<i32, Repeated, 3, 2>>::FIELD_NUMBER,
            3
        );
        assert_eq!(
            <ScoresField as FieldDescriptor<i32, Repeated, 3, 2>>::BIT_INDEX,
            2
        );

        // Test field number and bit index methods
        assert_eq!(
            <NameField as FieldDescriptor<String, ImplicitOptional, 1, 0>>::field_number(),
            1
        );
        assert_eq!(
            <NameField as FieldDescriptor<String, ImplicitOptional, 1, 0>>::bit_index(),
            0
        );

        assert_eq!(
            <AgeField as FieldDescriptor<i32, ImplicitOptional, 2, 1>>::field_number(),
            2
        );
        assert_eq!(
            <AgeField as FieldDescriptor<i32, ImplicitOptional, 2, 1>>::bit_index(),
            1
        );

        assert_eq!(
            <ScoresField as FieldDescriptor<i32, Repeated, 3, 2>>::field_number(),
            3
        );
        assert_eq!(
            <ScoresField as FieldDescriptor<i32, Repeated, 3, 2>>::bit_index(),
            2
        );
    }

    #[test]
    fn test_field_descriptor_default_values() {
        type NameField = Field<String, ImplicitOptional>;
        type AgeField = Field<i32, ImplicitOptional>;
        type ScoresField = Field<i32, Repeated>;

        // Test default values
        assert_eq!(
            <NameField as FieldDescriptor<String, ImplicitOptional, 1, 0>>::default_value(),
            ""
        );
        assert_eq!(
            <AgeField as FieldDescriptor<i32, ImplicitOptional, 2, 1>>::default_value(),
            0
        );
        assert_eq!(
            <ScoresField as FieldDescriptor<i32, Repeated, 3, 2>>::default_value(),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn test_field_descriptor_operations() {
        type NameField = Field<String, ImplicitOptional>;
        type AgeField = Field<i32, ImplicitOptional>;

        let mut shared = SharedFields::<1>::new();
        let mut name_storage =
            <NameField as FieldDescriptor<String, ImplicitOptional, 1, 0>>::default_value();
        let mut age_storage =
            <AgeField as FieldDescriptor<i32, ImplicitOptional, 2, 1>>::default_value();

        // Test set operations
        <NameField as FieldDescriptor<String, ImplicitOptional, 1, 0>>::set(
            &mut shared,
            &mut name_storage,
            "Alice",
        );
        <AgeField as FieldDescriptor<i32, ImplicitOptional, 2, 1>>::set(
            &mut shared,
            &mut age_storage,
            30,
        );

        // Test get operations
        assert_eq!(
            <NameField as FieldDescriptor<String, ImplicitOptional, 1, 0>>::get(
                &shared,
                &name_storage
            ),
            "Alice"
        );
        assert_eq!(
            <AgeField as FieldDescriptor<i32, ImplicitOptional, 2, 1>>::get(&shared, &age_storage),
            30
        );

        // Test clear operations
        <NameField as FieldDescriptor<String, ImplicitOptional, 1, 0>>::clear(
            &mut shared,
            &mut name_storage,
        );
        <AgeField as FieldDescriptor<i32, ImplicitOptional, 2, 1>>::clear(
            &mut shared,
            &mut age_storage,
        );

        assert_eq!(
            <NameField as FieldDescriptor<String, ImplicitOptional, 1, 0>>::get(
                &shared,
                &name_storage
            ),
            ""
        );
        assert_eq!(
            <AgeField as FieldDescriptor<i32, ImplicitOptional, 2, 1>>::get(&shared, &age_storage),
            0
        );
    }
}
