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
// Protobuf Field Types
// ============================================================================

/// Protobuf field types as enum values
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtobufFieldType {
    // Scalar types
    /// 32-bit signed integer
    Int32,
    /// 64-bit signed integer
    Int64,
    /// 32-bit unsigned integer
    UInt32,
    /// 64-bit unsigned integer
    UInt64,
    /// 32-bit signed integer (zigzag encoded)
    SInt32,
    /// 64-bit signed integer (zigzag encoded)
    SInt64,
    /// 32-bit fixed-point number
    Fixed32,
    /// 64-bit fixed-point number
    Fixed64,
    /// 32-bit signed fixed-point number
    SFixed32,
    /// 64-bit signed fixed-point number
    SFixed64,
    /// 32-bit floating point number
    Float,
    /// 64-bit floating point number
    Double,
    /// Boolean value
    Bool,

    // String types
    /// UTF-8 encoded string
    String,
    /// Raw bytes
    Bytes,

    // Message types
    /// Nested message
    Message,

    // Enum types
    /// Enumeration value
    Enum,

    // Group types (deprecated)
    /// Group (deprecated)
    Group,
}

impl ProtobufFieldType {
    /// Returns the wire type for this field type
    pub const fn wire_type(self) -> u8 {
        match self {
            Self::Int32
            | Self::Int64
            | Self::UInt32
            | Self::UInt64
            | Self::SInt32
            | Self::SInt64
            | Self::Bool => 0, // Varint

            Self::Fixed64 | Self::SFixed64 | Self::Double => 1, // 64-bit

            Self::String | Self::Bytes | Self::Message | Self::Enum => 2, // Length-delimited

            Self::Fixed32 | Self::SFixed32 | Self::Float => 5, // 32-bit

            Self::Group => 3, // Start group (deprecated)
        }
    }
}

// ============================================================================
// Field Label Markers (Zero-Sized Types)
// ============================================================================

/// Marker trait for protobuf field labels.
///
/// This is a sealed trait - users cannot implement it.
/// Represents the protobuf field label (optional, repeated, map).
pub trait FieldLabel: private::Sealed {}

/// Marker for implicit optional fields (Proto3 default).
///
/// These fields are always considered "present" and return default values
/// when not explicitly set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImplicitOptional;

/// Marker for explicit optional fields (Proto3 `optional` keyword).
///
/// These fields track presence explicitly and return `Option<T>`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExplicitOptional;

/// Marker for repeated fields.
///
/// These fields can appear zero or more times.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Repeated;

/// Marker for map fields.
///
/// These are syntactic sugar for repeated fields of key-value pairs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Map;

impl FieldLabel for ImplicitOptional {}
impl FieldLabel for ExplicitOptional {}
impl FieldLabel for Repeated {}
impl FieldLabel for Map {}

mod private {
    pub trait Sealed {}
    impl Sealed for super::ImplicitOptional {}
    impl Sealed for super::ExplicitOptional {}
    impl Sealed for super::Repeated {}
    impl Sealed for super::Map {}
}

// ============================================================================
// Scalar Type Trait
// ============================================================================

/// Trait for scalar protobuf field types.
pub trait ScalarType: Default + Copy + Clone + PartialEq {}

impl ScalarType for i32 {}
impl ScalarType for i64 {}
impl ScalarType for u32 {}
impl ScalarType for u64 {}
impl ScalarType for f32 {}
impl ScalarType for f64 {}
impl ScalarType for bool {}

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
/// type NameField = FieldType<String, ImplicitOptional>;  // implicit presence
/// type EmailField = FieldType<String, ExplicitOptional>; // explicit presence
/// type HobbiesField = FieldType<String, Repeated>;
/// type ScoresField = FieldType<(String, i32), Map>;
/// ```
pub struct FieldType<T, L: FieldLabel> {
    _phantom: PhantomData<(T, L)>,
}

/// Comprehensive field descriptor containing all protobuf field information.
///
/// This type encodes all field metadata including:
/// - Value type (T)
/// - Field label (L)
/// - Field number
/// - Bit index for presence tracking
///
/// # Type Parameters
/// - `T`: The value type (i32, String, etc.)
/// - `L`: The field label (ImplicitOptional, ExplicitOptional, Repeated, Map)
/// - `const FIELD_NUMBER`: The protobuf field number
/// - `const BIT_INDEX`: The bit index for presence tracking
///
/// # Examples
/// ```ignore
/// type NameField = FieldDescriptor<String, ImplicitOptional, 1, 0>;
/// type AgeField = FieldDescriptor<i32, ImplicitOptional, 2, 1>;
/// type EmailField = FieldDescriptor<String, ExplicitOptional, 3, 2>;
/// ```
pub struct FieldDescriptor<T, L: FieldLabel, const FIELD_NUMBER: u32, const BIT_INDEX: usize> {
    _phantom: PhantomData<(T, L)>,
}

// ============================================================================
// Field Trait (Unified Approach)
// ============================================================================

/// Comprehensive field trait containing all protobuf field information.
///
/// This trait provides metadata and operations for field handling:
/// - Field number (for serialization)
/// - Bit index (for presence tracking)
/// - Protobuf field type (for wire format)
/// - Default value (for initialization)
/// - Get/Set/Clear operations
///
/// # Type Parameters
/// - `T`: The value type (i32, String, etc.)
/// - `L`: The field label (ImplicitOptional, ExplicitOptional, Repeated, Map)
/// - `const FIELD_NUMBER`: The protobuf field number
/// - `const BIT_INDEX`: The bit index for presence tracking
pub trait Field<T, L: FieldLabel, const FIELD_NUMBER: u32, const BIT_INDEX: usize> {
    /// The storage type for this field (how it's stored in the struct)
    type Storage;

    /// The value type for this field (what users pass in)
    type Value;

    /// The return type for get operations (may borrow from storage)
    type GetValue<'a>
    where
        Self: 'a;

    /// The protobuf field type
    const FIELD_TYPE: ProtobufFieldType;

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

    /// Gets the protobuf field type
    fn field_type() -> ProtobufFieldType {
        Self::FIELD_TYPE
    }

    /// Gets the wire type for this field
    fn wire_type() -> u8 {
        Self::FIELD_TYPE.wire_type()
    }
}

// ============================================================================
// FieldDescriptor Implementations
// ============================================================================

/// Specialized implementation for String fields with ImplicitOptional
impl<const FIELD_NUMBER: u32, const BIT_INDEX: usize>
    Field<String, ImplicitOptional, FIELD_NUMBER, BIT_INDEX>
    for FieldDescriptor<String, ImplicitOptional, FIELD_NUMBER, BIT_INDEX>
{
    type Storage = String;
    type Value = &'static str; // Can accept any &str
    type GetValue<'a> = &'a str;

    const FIELD_TYPE: ProtobufFieldType = ProtobufFieldType::String;

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

/// Specialized implementation for String fields with ExplicitOptional
impl<const FIELD_NUMBER: u32, const BIT_INDEX: usize>
    Field<String, ExplicitOptional, FIELD_NUMBER, BIT_INDEX>
    for FieldDescriptor<String, ExplicitOptional, FIELD_NUMBER, BIT_INDEX>
{
    type Storage = String;
    type Value = String; // For FieldDescriptor compatibility
    type GetValue<'a> = Option<&'a str>;

    const FIELD_TYPE: ProtobufFieldType = ProtobufFieldType::String;

    fn default_value() -> Self::Storage {
        String::new()
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
            Some(storage.as_str())
        } else {
            None
        }
    }

    fn clear<const BYTES: usize>(shared: &mut SharedFields<BYTES>, storage: &mut Self::Storage) {
        storage.clear();
        shared.has_bits_mut().set(BIT_INDEX, false);
    }

    fn is_present<const BYTES: usize>(shared: &SharedFields<BYTES>) -> bool {
        shared.has_bits()[BIT_INDEX]
    }
}

/// Specialized implementation for scalar types with ImplicitOptional
impl<T: ScalarType, const FIELD_NUMBER: u32, const BIT_INDEX: usize>
    Field<T, ImplicitOptional, FIELD_NUMBER, BIT_INDEX>
    for FieldDescriptor<T, ImplicitOptional, FIELD_NUMBER, BIT_INDEX>
{
    type Storage = T;
    type Value = T;
    type GetValue<'a>
        = T
    where
        T: 'a;

    const FIELD_TYPE: ProtobufFieldType = ProtobufFieldType::Int32; // Default to Int32

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

/// Specialized implementation for scalar types with ExplicitOptional
impl<T: ScalarType, const FIELD_NUMBER: u32, const BIT_INDEX: usize>
    Field<T, ExplicitOptional, FIELD_NUMBER, BIT_INDEX>
    for FieldDescriptor<T, ExplicitOptional, FIELD_NUMBER, BIT_INDEX>
{
    type Storage = T;
    type Value = T;
    type GetValue<'a>
        = Option<T>
    where
        T: 'a;

    const FIELD_TYPE: ProtobufFieldType = ProtobufFieldType::Int32; // Default to Int32

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

// ============================================================================
// Convenience Methods for String Fields
// ============================================================================

/// Convenience methods for String fields with ImplicitOptional
impl FieldType<String, ImplicitOptional> {
    /// Sets a string field from a string slice
    #[inline]
    pub fn set<const BYTES: usize>(
        _shared: &mut SharedFields<BYTES>,
        _bit_index: usize,
        storage: &mut String,
        value: &str,
    ) {
        value.clone_into(storage);
        // ImplicitOptional fields don't need presence tracking
    }

    /// Clears a string field
    #[inline]
    pub fn clear<const BYTES: usize>(
        _shared: &mut SharedFields<BYTES>,
        _bit_index: usize,
        storage: &mut String,
    ) {
        storage.clear();
        // ImplicitOptional fields don't need presence tracking
    }

    /// Gets a string field value
    #[inline]
    pub fn get<'a, const BYTES: usize>(
        _shared: &'a SharedFields<BYTES>,
        _bit_index: usize,
        storage: &'a String,
    ) -> &'a str {
        storage.as_str()
    }
}

/// Convenience methods for String fields with ExplicitOptional
impl FieldType<String, ExplicitOptional> {
    /// Sets a string field from a string slice
    #[inline]
    pub fn set<const BYTES: usize>(
        shared: &mut SharedFields<BYTES>,
        bit_index: usize,
        storage: &mut String,
        value: &str,
    ) {
        value.clone_into(storage);
        shared.has_bits_mut().set(bit_index, true);
    }

    /// Clears a string field
    #[inline]
    pub fn clear<const BYTES: usize>(
        shared: &mut SharedFields<BYTES>,
        bit_index: usize,
        storage: &mut String,
    ) {
        storage.clear();
        shared.has_bits_mut().set(bit_index, false);
    }

    /// Gets a string field value
    #[inline]
    pub fn get<'a, const BYTES: usize>(
        shared: &'a SharedFields<BYTES>,
        bit_index: usize,
        storage: &'a String,
    ) -> Option<&'a str> {
        if shared.has_bits()[bit_index] {
            Some(storage.as_str())
        } else {
            None
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protobuf_field_type_wire_types() {
        assert_eq!(ProtobufFieldType::Int32.wire_type(), 0);
        assert_eq!(ProtobufFieldType::String.wire_type(), 2);
        assert_eq!(ProtobufFieldType::Fixed32.wire_type(), 5);
    }

    #[test]
    fn test_field_descriptor_constants() {
        type NameField = FieldDescriptor<String, ImplicitOptional, 1, 0>;
        type AgeField = FieldDescriptor<i32, ImplicitOptional, 2, 1>;
        type EmailField = FieldDescriptor<String, ExplicitOptional, 3, 2>;

        assert_eq!(NameField::FIELD_NUMBER, 1);
        assert_eq!(NameField::BIT_INDEX, 0);
        assert_eq!(NameField::FIELD_TYPE, ProtobufFieldType::String);

        assert_eq!(AgeField::FIELD_NUMBER, 2);
        assert_eq!(AgeField::BIT_INDEX, 1);
        assert_eq!(AgeField::FIELD_TYPE, ProtobufFieldType::Int32);

        assert_eq!(EmailField::FIELD_NUMBER, 3);
        assert_eq!(EmailField::BIT_INDEX, 2);
        assert_eq!(EmailField::FIELD_TYPE, ProtobufFieldType::String);
    }

    #[test]
    fn test_field_methods() {
        type NameField = FieldDescriptor<String, ImplicitOptional, 1, 0>;
        type EmailField = FieldDescriptor<String, ExplicitOptional, 3, 2>;

        assert_eq!(NameField::field_number(), 1);
        assert_eq!(NameField::bit_index(), 0);
        assert_eq!(NameField::field_type(), ProtobufFieldType::String);
        assert_eq!(NameField::wire_type(), 2);

        assert_eq!(EmailField::field_number(), 3);
        assert_eq!(EmailField::bit_index(), 2);
        assert_eq!(EmailField::field_type(), ProtobufFieldType::String);
        assert_eq!(EmailField::wire_type(), 2);
    }
}
