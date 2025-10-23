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

/// Type-level descriptor for a protobuf field that holds actual data.
///
/// Encodes both the value type (T) and field label (L), and stores the actual field data.
///
/// # Type Parameters
/// - `T`: The value type (i32, String, etc.)
/// - `L`: The field label (ImplicitOptional, ExplicitOptional, Repeated, Map)
/// - `const FIELD_NUMBER`: The protobuf field number
/// - `const BIT_INDEX`: The bit index for presence tracking
///
/// # Examples
/// ```ignore
/// type NameField = FieldType<String, ImplicitOptional, 1, 0>;  // implicit presence
/// type EmailField = FieldType<String, ExplicitOptional, 3, 2>; // explicit presence
/// type HobbiesField = FieldType<String, Repeated, 4, 3>;
/// type ScoresField = FieldType<(String, i32), Map, 5, 4>;
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct FieldType<T, L: FieldLabel, const FIELD_NUMBER: u32, const BIT_INDEX: usize> {
    /// The actual field data
    pub data: T,
    _phantom: PhantomData<L>,
}

impl<T, L: FieldLabel, const FIELD_NUMBER: u32, const BIT_INDEX: usize>
    FieldType<T, L, FIELD_NUMBER, BIT_INDEX>
{
    /// Creates a new FieldType with the given data
    pub fn new(data: T) -> Self {
        Self {
            data,
            _phantom: PhantomData,
        }
    }
}

/// Default implementation for FieldType
///
/// This implementation allows FieldType to be used with Default::default(),
/// making it easier to initialize PersonImpl and other message structures.
///
/// # Allocator Considerations
///
/// When allocators are introduced, this Default implementation may become
/// problematic for types that require custom allocators (e.g., String with
/// a specific allocator). In such cases, we may need to:
/// - Use a different initialization pattern
/// - Provide allocator-aware constructors
/// - Use lazy initialization
/// - Require explicit initialization in the message constructor
///
/// For now, this implementation works well with standard library types
/// that have their own Default implementations.
impl<T: Default, L: FieldLabel, const FIELD_NUMBER: u32, const BIT_INDEX: usize> Default
    for FieldType<T, L, FIELD_NUMBER, BIT_INDEX>
{
    fn default() -> Self {
        Self {
            data: T::default(),
            _phantom: PhantomData,
        }
    }
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
    /// The value type for this field (what users pass in)
    type Value<'a>;

    /// The return type for get operations (may borrow from self)
    type GetValue<'a>
    where
        Self: 'a;

    /// The protobuf field type
    const FIELD_TYPE: ProtobufFieldType;

    /// The protobuf field number
    const FIELD_NUMBER: u32 = FIELD_NUMBER;

    /// The bit index for presence tracking in SharedFields
    const BIT_INDEX: usize = BIT_INDEX;

    /// Sets the field value
    fn set<const BYTES: usize>(&mut self, shared: &mut SharedFields<BYTES>, value: Self::Value<'_>);

    /// Gets the field value
    fn get<'a, const BYTES: usize>(&'a self, shared: &'a SharedFields<BYTES>)
        -> Self::GetValue<'a>;

    /// Clears the field value
    fn clear<const BYTES: usize>(&mut self, shared: &mut SharedFields<BYTES>);

    /// Checks if the field is present (for ExplicitOptional fields)
    fn is_present<const BYTES: usize>(&self, shared: &SharedFields<BYTES>) -> bool;

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
// FieldType Implementations
// ============================================================================

/// Implementation for String fields with ImplicitOptional
impl<const FIELD_NUMBER: u32, const BIT_INDEX: usize>
    Field<String, ImplicitOptional, FIELD_NUMBER, BIT_INDEX>
    for FieldType<String, ImplicitOptional, FIELD_NUMBER, BIT_INDEX>
{
    type Value<'a> = &'a str; // Accept any &str
    type GetValue<'a> = &'a str;

    const FIELD_TYPE: ProtobufFieldType = ProtobufFieldType::String;

    fn set<const BYTES: usize>(
        &mut self,
        _shared: &mut SharedFields<BYTES>,
        value: Self::Value<'_>,
    ) {
        value.clone_into(&mut self.data);
        // ImplicitOptional fields don't need presence tracking
    }

    fn get<'a, const BYTES: usize>(
        &'a self,
        _shared: &'a SharedFields<BYTES>,
    ) -> Self::GetValue<'a> {
        self.data.as_str()
    }

    fn clear<const BYTES: usize>(&mut self, _shared: &mut SharedFields<BYTES>) {
        self.data.clear();
        // ImplicitOptional fields don't need presence tracking
    }

    fn is_present<const BYTES: usize>(&self, _shared: &SharedFields<BYTES>) -> bool {
        true // ImplicitOptional fields are always present
    }
}

/// Implementation for String fields with ExplicitOptional
impl<const FIELD_NUMBER: u32, const BIT_INDEX: usize>
    Field<String, ExplicitOptional, FIELD_NUMBER, BIT_INDEX>
    for FieldType<String, ExplicitOptional, FIELD_NUMBER, BIT_INDEX>
{
    type Value<'a> = &'a str; // Accept any &str
    type GetValue<'a> = Option<&'a str>;

    const FIELD_TYPE: ProtobufFieldType = ProtobufFieldType::String;

    fn set<const BYTES: usize>(
        &mut self,
        shared: &mut SharedFields<BYTES>,
        value: Self::Value<'_>,
    ) {
        value.clone_into(&mut self.data);
        shared.has_bits_mut().set(BIT_INDEX, true);
    }

    fn get<'a, const BYTES: usize>(
        &'a self,
        shared: &'a SharedFields<BYTES>,
    ) -> Self::GetValue<'a> {
        if shared.has_bits()[BIT_INDEX] {
            Some(self.data.as_str())
        } else {
            None
        }
    }

    fn clear<const BYTES: usize>(&mut self, shared: &mut SharedFields<BYTES>) {
        self.data.clear();
        shared.has_bits_mut().set(BIT_INDEX, false);
    }

    fn is_present<const BYTES: usize>(&self, shared: &SharedFields<BYTES>) -> bool {
        shared.has_bits()[BIT_INDEX]
    }
}

/// Implementation for scalar types with ImplicitOptional
impl<T: ScalarType, const FIELD_NUMBER: u32, const BIT_INDEX: usize>
    Field<T, ImplicitOptional, FIELD_NUMBER, BIT_INDEX>
    for FieldType<T, ImplicitOptional, FIELD_NUMBER, BIT_INDEX>
{
    type Value<'a> = T;
    type GetValue<'a>
        = T
    where
        T: 'a;

    const FIELD_TYPE: ProtobufFieldType = ProtobufFieldType::Int32; // Default to Int32

    fn set<const BYTES: usize>(
        &mut self,
        _shared: &mut SharedFields<BYTES>,
        value: Self::Value<'_>,
    ) {
        self.data = value;
        // ImplicitOptional fields don't need presence tracking
    }

    fn get<'a, const BYTES: usize>(
        &'a self,
        _shared: &'a SharedFields<BYTES>,
    ) -> Self::GetValue<'a> {
        self.data
    }

    fn clear<const BYTES: usize>(&mut self, _shared: &mut SharedFields<BYTES>) {
        self.data = T::default();
        // ImplicitOptional fields don't need presence tracking
    }

    fn is_present<const BYTES: usize>(&self, _shared: &SharedFields<BYTES>) -> bool {
        true // ImplicitOptional fields are always present
    }
}

/// Implementation for scalar types with ExplicitOptional
impl<T: ScalarType, const FIELD_NUMBER: u32, const BIT_INDEX: usize>
    Field<T, ExplicitOptional, FIELD_NUMBER, BIT_INDEX>
    for FieldType<T, ExplicitOptional, FIELD_NUMBER, BIT_INDEX>
{
    type Value<'a> = T;
    type GetValue<'a>
        = Option<T>
    where
        T: 'a;

    const FIELD_TYPE: ProtobufFieldType = ProtobufFieldType::Int32; // Default to Int32

    fn set<const BYTES: usize>(
        &mut self,
        shared: &mut SharedFields<BYTES>,
        value: Self::Value<'_>,
    ) {
        self.data = value;
        shared.has_bits_mut().set(BIT_INDEX, true);
    }

    fn get<'a, const BYTES: usize>(
        &'a self,
        shared: &'a SharedFields<BYTES>,
    ) -> Self::GetValue<'a> {
        if shared.has_bits()[BIT_INDEX] {
            Some(self.data)
        } else {
            None
        }
    }

    fn clear<const BYTES: usize>(&mut self, shared: &mut SharedFields<BYTES>) {
        self.data = T::default();
        shared.has_bits_mut().set(BIT_INDEX, false);
    }

    fn is_present<const BYTES: usize>(&self, shared: &SharedFields<BYTES>) -> bool {
        shared.has_bits()[BIT_INDEX]
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
