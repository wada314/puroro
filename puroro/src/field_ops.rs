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
use ::allocator_api2::boxed::Box;
use ::allocator_api2::vec::Vec;
use ::allocator_extras::{Allocator, DefaultIn, Global, String as AllocString};
use ::std::marker::PhantomData;

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
///
/// **Note**: For message fields, use `SingularMessage` instead of `ImplicitOptional`.
/// Message fields use `Option<Box<M>>` for presence tracking, so they don't need
/// presence bits. `ImplicitOptional` is designed for scalar types that use presence
/// bits or default value checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImplicitOptional;

/// Marker for explicit optional fields (Proto3 `optional` keyword).
///
/// These fields track presence explicitly and return `Option<T>`.
/// The presence bit index is encoded at the type level for type safety.
///
/// **Note**: For message fields, use `SingularMessage` instead of `ExplicitOptional`.
/// Message fields use `Option<Box<M>>` for presence tracking, so they don't need
/// presence bits. `ExplicitOptional` is designed for scalar types that require
/// explicit presence bit tracking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExplicitOptional<const PRESENCE_BIT_INDEX: usize>;

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

/// Marker for singular (non-repeated) message fields.
///
/// Singular message fields use `Option<Box<M>>` for presence tracking,
/// so they don't need presence bits. This label is used for both
/// implicit and explicit optional message fields.
/// Repeated message fields use the `Repeated` label instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SingularMessage;

impl FieldLabel for ImplicitOptional {}
impl<const PRESENCE_BIT_INDEX: usize> FieldLabel for ExplicitOptional<PRESENCE_BIT_INDEX> {}
impl FieldLabel for Repeated {}
impl FieldLabel for Map {}
impl FieldLabel for SingularMessage {}

mod private {
    pub trait Sealed {}
    impl Sealed for super::ImplicitOptional {}
    impl<const PRESENCE_BIT_INDEX: usize> Sealed for super::ExplicitOptional<PRESENCE_BIT_INDEX> {}
    impl Sealed for super::Repeated {}
    impl Sealed for super::Map {}
    impl Sealed for super::SingularMessage {}
}

// ============================================================================
// Scalar Type Trait
// ============================================================================

/// Trait for Rust types that correspond to protobuf numerical field types.
///
/// This trait is implemented for Rust primitive types (i32, i64, u32, u64, f32, f64, bool)
/// that map to protobuf's scalar/numerical field types.
pub trait ScalarType: Default + Copy + Clone + PartialEq {}

impl ScalarType for i32 {}
impl ScalarType for i64 {}
impl ScalarType for u32 {}
impl ScalarType for u64 {}
impl ScalarType for f32 {}
impl ScalarType for f64 {}
impl ScalarType for bool {}

// ============================================================================
// Wrapper Types for Complex Protobuf Field Types
// ============================================================================

/// Wrapper type for String protobuf fields.
///
/// This wrapper is needed to distinguish String fields from scalar types
/// in the FieldOperations trait implementations, avoiding trait conflicts.
#[derive(Debug)]
pub struct StringFieldWrapper<A: Allocator = Global>(pub AllocString<A>);

/// Wrapper type for Bytes protobuf fields.
///
/// This wrapper is needed to distinguish Bytes fields from scalar types
/// in the FieldOperations trait implementations, avoiding trait conflicts.
#[derive(Debug)]
pub struct BytesFieldWrapper<A: Allocator = Global>(pub Vec<u8, A>);

/// Wrapper type for Message protobuf fields.
///
/// This wrapper is needed to distinguish Message fields from scalar types
/// in the FieldOperations trait implementations, avoiding trait conflicts.
/// Uses heap allocation with pointer null checks for presence tracking.
#[derive(Debug)]
pub struct MessageFieldWrapper<M, A: Allocator = Global> {
    value: Option<Box<M, A>>,
    allocator: A,
}

/// Convenience alias for `StringFieldWrapper` with the global allocator.
pub type StringFieldWrapperGlobal = StringFieldWrapper<Global>;

/// Convenience alias for `BytesFieldWrapper` with the global allocator.
pub type BytesFieldWrapperGlobal = BytesFieldWrapper<Global>;

/// Convenience alias for `MessageFieldWrapper` with the global allocator.
pub type MessageFieldWrapperGlobal<M> = MessageFieldWrapper<M, Global>;

/// Wrapper type for Enum protobuf fields.
///
/// This wrapper is needed to distinguish Enum fields from scalar types
/// in the FieldOperations trait implementations, avoiding trait conflicts.
#[derive(Debug, Clone, PartialEq)]
pub struct EnumFieldWrapper<E>(pub E);

// Default implementations for wrapper types
impl<A: Allocator + Default> Default for StringFieldWrapper<A> {
    fn default() -> Self {
        Self(AllocString::new_in(A::default()))
    }
}

impl<A: Allocator + Default> Default for BytesFieldWrapper<A> {
    fn default() -> Self {
        Self(Vec::new_in(A::default()))
    }
}

impl<M, A> Default for MessageFieldWrapper<M, A>
where
    A: Allocator + Default,
{
    fn default() -> Self {
        Self {
            value: None,
            allocator: A::default(),
        }
    }
}

impl<A: Allocator> StringFieldWrapper<A> {
    /// Creates a new string wrapper using the provided allocator.
    pub fn new_in(alloc: A) -> Self {
        Self(AllocString::new_in(alloc))
    }

    /// Consumes the wrapper and returns the underlying allocator-aware string.
    pub fn into_inner(self) -> AllocString<A> {
        self.0
    }

    /// Returns an immutable reference to the underlying string.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Returns a mutable reference to the underlying string.
    pub fn as_mut_string(&mut self) -> &mut AllocString<A> {
        &mut self.0
    }
}

impl BytesFieldWrapper<Global> {
    /// Creates a new wrapper using the global allocator.
    pub fn new() -> Self {
        Self::default()
    }
}

impl StringFieldWrapper<Global> {
    /// Creates a new wrapper using the global allocator.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<A: Allocator> BytesFieldWrapper<A> {
    /// Creates a new bytes wrapper using the provided allocator.
    pub fn new_in(alloc: A) -> Self {
        Self(Vec::new_in(alloc))
    }

    /// Consumes the wrapper and returns the underlying byte buffer.
    pub fn into_inner(self) -> Vec<u8, A> {
        self.0
    }

    /// Returns a reference to the underlying slice.
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl<M, A> MessageFieldWrapper<M, A>
where
    A: Allocator,
{
    /// Creates a new wrapper with the provided allocator.
    pub fn new_in(alloc: A) -> Self {
        Self {
            value: None,
            allocator: alloc,
        }
    }

    /// Returns a reference to the allocator used by this wrapper.
    pub fn allocator(&self) -> &A {
        &self.allocator
    }

    /// Replaces the allocator used for subsequent allocations.
    pub fn set_allocator(&mut self, allocator: A) {
        self.allocator = allocator;
    }

    /// Consumes the wrapper and returns the owned message if present.
    pub fn into_inner(self) -> Option<Box<M, A>> {
        self.value
    }

    /// Returns an immutable reference to the wrapped message if present.
    pub fn as_ref(&self) -> Option<&M> {
        self.value.as_ref().map(|boxed| boxed.as_ref())
    }

    /// Returns a mutable reference to the wrapped message if present.
    pub fn as_mut(&mut self) -> Option<&mut M> {
        self.value.as_mut().map(|boxed| boxed.as_mut())
    }

    /// Ensures the wrapper contains a value, creating one with the provided initializer.
    pub fn get_or_insert_with<F>(&mut self, init: F) -> &mut M
    where
        F: FnOnce() -> M,
        A: Clone,
    {
        if self.value.is_none() {
            let allocator = self.allocator.clone();
            self.value = Some(Box::new_in(init(), allocator));
        }
        self.value
            .as_mut()
            .map(|boxed| boxed.as_mut())
            .expect("initialized above")
    }

    /// Returns whether the wrapper currently holds a value.
    pub fn is_none(&self) -> bool {
        self.value.is_none()
    }
}

impl<M> MessageFieldWrapper<M, Global> {
    /// Creates a new wrapper using the global allocator.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<A: Allocator> DefaultIn<A> for StringFieldWrapper<A> {
    fn default_in(alloc: A) -> Self {
        Self::new_in(alloc)
    }
}

impl<A: Allocator> DefaultIn<A> for BytesFieldWrapper<A> {
    fn default_in(alloc: A) -> Self {
        Self::new_in(alloc)
    }
}

impl<M, A> DefaultIn<A> for MessageFieldWrapper<M, A>
where
    A: Allocator,
{
    fn default_in(alloc: A) -> Self {
        Self::new_in(alloc)
    }
}

impl<A: Allocator + Clone> Clone for StringFieldWrapper<A> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<A: Allocator> PartialEq for StringFieldWrapper<A> {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_str() == other.0.as_str()
    }
}

impl<A: Allocator> Eq for StringFieldWrapper<A> {}

impl<A: Allocator + Clone> Clone for BytesFieldWrapper<A> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<A: Allocator> PartialEq for BytesFieldWrapper<A> {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_slice() == other.0.as_slice()
    }
}

impl<A: Allocator> Eq for BytesFieldWrapper<A> {}

impl<M: Clone, A: Allocator + Clone> Clone for MessageFieldWrapper<M, A> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            allocator: self.allocator.clone(),
        }
    }
}

impl<M: PartialEq, A: Allocator> PartialEq for MessageFieldWrapper<M, A> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<M: Eq, A: Allocator> Eq for MessageFieldWrapper<M, A> {}

impl<E: Default> Default for EnumFieldWrapper<E> {
    fn default() -> Self {
        Self(E::default())
    }
}

// ============================================================================
// Field Descriptor Type
// ============================================================================

/// Type-level descriptor for a protobuf field that holds actual data.
///
/// Encodes both the value type (T) and field label (L), and stores the actual field data.
/// For ExplicitOptional fields, the presence bit index is encoded in the field label type.
///
/// # Type Parameters
/// - `T`: The value type (i32, StringFieldWrapper, etc.)
/// - `L`: The field label (ImplicitOptional, ExplicitOptional<BIT_INDEX>, Repeated, Map)
/// - `const FIELD_NUMBER`: The protobuf field number
/// - `const SHARED_BYTES_LEN`: The number of bytes for SharedFields storage
///
/// # Examples
/// ```ignore
/// type NameField = FieldStorage<StringFieldWrapper, ImplicitOptional, 1, 1>;           // implicit presence, 1 byte shared
/// type EmailField = FieldStorage<StringFieldWrapper, ExplicitOptional<2>, 3, 1>;        // explicit presence, bit 2, 1 byte shared
/// type HobbiesField = FieldStorage<StringFieldWrapper, Repeated, 4, 2>;                 // repeated field, 2 bytes shared
/// type ScoresField = FieldStorage<(StringFieldWrapper, i32), Map, 5, 1>;               // map field, 1 byte shared
/// ```
#[derive(Debug, Clone)]
pub struct FieldStorage<
    T,
    L: FieldLabel,
    const FIELD_NUMBER: u32,
    const SHARED_BYTES_LEN: usize,
    A: Allocator = Global,
> {
    /// The actual field data
    pub data: T,
    _phantom: PhantomData<(L, A)>,
}

impl<T, L: FieldLabel, const FIELD_NUMBER: u32, const SHARED_BYTES_LEN: usize, A: Allocator>
    FieldStorage<T, L, FIELD_NUMBER, SHARED_BYTES_LEN, A>
{
    /// Creates a new FieldStorage with the given data
    pub fn new(data: T) -> Self {
        Self {
            data,
            _phantom: PhantomData,
        }
    }

    /// Creates a new FieldStorage whose data is constructed using the provided allocator.
    pub fn new_in<F>(alloc: A, init: F) -> Self
    where
        F: FnOnce(A) -> T,
    {
        Self::new(init(alloc))
    }

    /// Creates a new FieldStorage with data obtained from the `DefaultIn` trait.
    pub fn default_in(alloc: A) -> Self
    where
        T: DefaultIn<A>,
    {
        Self::new(T::default_in(alloc))
    }

    /// Consumes the storage, returning the inner data.
    pub fn into_inner(self) -> T {
        self.data
    }

    /// Returns a mutable reference to the stored data.
    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

/// Default implementation for FieldStorage
///
/// This implementation allows FieldStorage to be used with Default::default(),
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
impl<
    T: Default,
    L: FieldLabel,
    const FIELD_NUMBER: u32,
    const SHARED_BYTES_LEN: usize,
    A: Allocator,
> Default for FieldStorage<T, L, FIELD_NUMBER, SHARED_BYTES_LEN, A>
{
    fn default() -> Self {
        Self {
            data: T::default(),
            _phantom: PhantomData,
        }
    }
}

impl<
    T: PartialEq,
    L: FieldLabel,
    const FIELD_NUMBER: u32,
    const SHARED_BYTES_LEN: usize,
    A: Allocator,
> PartialEq for FieldStorage<T, L, FIELD_NUMBER, SHARED_BYTES_LEN, A>
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T: Eq, L: FieldLabel, const FIELD_NUMBER: u32, const SHARED_BYTES_LEN: usize, A: Allocator> Eq
    for FieldStorage<T, L, FIELD_NUMBER, SHARED_BYTES_LEN, A>
{
}

// ============================================================================
// Repeated Scalar Implementations
// ============================================================================

/// Implementation for repeated i32 using allocator-aware Vec<i32, A>
impl<
        A: Allocator,
        const FIELD_NUMBER: u32,
        const SHARED_BYTES_LEN: usize,
    > FieldOperations<Vec<i32, A>, Repeated, FIELD_NUMBER, SHARED_BYTES_LEN>
    for FieldStorage<Vec<i32, A>, Repeated, FIELD_NUMBER, SHARED_BYTES_LEN, A>
{
    type SetValue<'a> = &'a [i32];
    type GetValue<'a>
        = &'a [i32]
    where
        A: 'a;
    type SharedFields = SharedFields<SHARED_BYTES_LEN, A>;

    const FIELD_TYPE: ProtobufFieldType = ProtobufFieldType::Int32;

    fn set(&mut self, _shared: &mut Self::SharedFields, value: Self::SetValue<'_>) {
        self.data.clear();
        self.data.extend_from_slice(value);
    }

    fn get<'a>(&'a self, _shared: &'a Self::SharedFields) -> Self::GetValue<'a> {
        self.data.as_slice()
    }

    fn clear(&mut self, _shared: &mut Self::SharedFields) {
        self.data.clear();
    }

    fn is_present(&self, _shared: &Self::SharedFields) -> bool {
        !self.data.is_empty()
    }
}

// ============================================================================
// Field Trait (Unified Approach)
// ============================================================================

/// Comprehensive field operations trait containing all protobuf field information.
///
/// This trait provides metadata and operations for field handling:
/// - Field number (for serialization)
/// - Presence bit index (for explicit presence tracking, only for ExplicitOptional fields)
/// - Protobuf field type (for wire format)
/// - Get/Set/Clear operations
///
/// # Type Parameters
/// - `T`: The value type (i32, StringFieldWrapper, etc.)
/// - `L`: The field label (ImplicitOptional, ExplicitOptional<BIT_INDEX>, Repeated, Map)
/// - `const FIELD_NUMBER`: The protobuf field number
/// - `const SHARED_BYTES_LEN`: The number of bytes for SharedFields storage
pub trait FieldOperations<T, L: FieldLabel, const FIELD_NUMBER: u32, const SHARED_BYTES_LEN: usize>
{
    /// The value type for this field (what users pass in when setting)
    type SetValue<'a>;

    /// The return type for get operations (may borrow from self)
    type GetValue<'a>
    where
        Self: 'a;

    /// The protobuf field type
    const FIELD_TYPE: ProtobufFieldType;

    /// The protobuf field number
    const FIELD_NUMBER: u32 = FIELD_NUMBER;

    /// Type alias for SharedFields to avoid repeating const generic parameters
    type SharedFields;

    /// Sets the field value
    fn set(&mut self, shared: &mut Self::SharedFields, value: Self::SetValue<'_>);

    /// Gets the field value
    fn get<'a>(&'a self, shared: &'a Self::SharedFields) -> Self::GetValue<'a>;

    /// Clears the field value
    fn clear(&mut self, shared: &mut Self::SharedFields);

    /// Checks if the field is present (for ExplicitOptional fields)
    fn is_present(&self, shared: &Self::SharedFields) -> bool;

    /// Gets the field number
    fn field_number() -> u32 {
        FIELD_NUMBER
    }

    /// Gets the protobuf field type
    fn field_type() -> ProtobufFieldType {
        Self::FIELD_TYPE
    }
}

// ============================================================================
// FieldType Implementations
// ============================================================================

/// Implementation for scalar types with ImplicitOptional
impl<T: ScalarType, A: Allocator + Clone, const FIELD_NUMBER: u32, const SHARED_BYTES_LEN: usize>
    FieldOperations<T, ImplicitOptional, FIELD_NUMBER, SHARED_BYTES_LEN>
    for FieldStorage<T, ImplicitOptional, FIELD_NUMBER, SHARED_BYTES_LEN, A>
{
    type SetValue<'a> = T;
    type GetValue<'a>
        = T
    where
        T: 'a,
        A: 'a;
    type SharedFields = SharedFields<SHARED_BYTES_LEN, A>;

    const FIELD_TYPE: ProtobufFieldType = ProtobufFieldType::Int32; // Default to Int32

    fn set(&mut self, _shared: &mut Self::SharedFields, value: Self::SetValue<'_>) {
        self.data = value;
        // ImplicitOptional fields don't need presence tracking
    }

    fn get<'a>(&'a self, _shared: &'a Self::SharedFields) -> Self::GetValue<'a> {
        self.data
    }

    fn clear(&mut self, _shared: &mut Self::SharedFields) {
        self.data = T::default();
        // ImplicitOptional fields don't need presence tracking
    }

    fn is_present(&self, _shared: &Self::SharedFields) -> bool {
        // ImplicitOptional fields are present only if not equal to default value
        self.data != T::default()
    }
}

/// Implementation for scalar types with ExplicitOptional
impl<
    T: ScalarType,
    A: Allocator + Clone,
    const FIELD_NUMBER: u32,
    const PRESENCE_BIT_INDEX: usize,
    const SHARED_BYTES_LEN: usize,
> FieldOperations<T, ExplicitOptional<PRESENCE_BIT_INDEX>, FIELD_NUMBER, SHARED_BYTES_LEN>
    for FieldStorage<T, ExplicitOptional<PRESENCE_BIT_INDEX>, FIELD_NUMBER, SHARED_BYTES_LEN, A>
{
    type SetValue<'a> = T;
    type GetValue<'a>
        = Option<T>
    where
        T: 'a,
        A: 'a;
    type SharedFields = SharedFields<SHARED_BYTES_LEN, A>;

    const FIELD_TYPE: ProtobufFieldType = ProtobufFieldType::Int32; // Default to Int32

    fn set(&mut self, shared: &mut Self::SharedFields, value: Self::SetValue<'_>) {
        self.data = value;
        shared.has_bits_mut().set(PRESENCE_BIT_INDEX, true);
    }

    fn get<'a>(&'a self, shared: &'a Self::SharedFields) -> Self::GetValue<'a> {
        if shared.is_field_present(PRESENCE_BIT_INDEX) {
            Some(self.data)
        } else {
            None
        }
    }

    fn clear(&mut self, shared: &mut Self::SharedFields) {
        self.data = T::default();
        shared.has_bits_mut().set(PRESENCE_BIT_INDEX, false);
    }

    fn is_present(&self, shared: &Self::SharedFields) -> bool {
        shared.is_field_present(PRESENCE_BIT_INDEX)
    }
}

// ============================================================================
// Wrapper Type Field Implementations
// ============================================================================

/// Implementation for StringFieldWrapper with ImplicitOptional
impl<A: Allocator + Clone, const FIELD_NUMBER: u32, const SHARED_BYTES_LEN: usize>
    FieldOperations<StringFieldWrapper<A>, ImplicitOptional, FIELD_NUMBER, SHARED_BYTES_LEN>
    for FieldStorage<StringFieldWrapper<A>, ImplicitOptional, FIELD_NUMBER, SHARED_BYTES_LEN, A>
{
    type SetValue<'a> = &'a str;
    type GetValue<'a>
        = &'a str
    where
        A: 'a;
    type SharedFields = SharedFields<SHARED_BYTES_LEN, A>;

    const FIELD_TYPE: ProtobufFieldType = ProtobufFieldType::String;

    fn set(&mut self, _shared: &mut Self::SharedFields, value: Self::SetValue<'_>) {
        self.data.0.clear();
        self.data.0.push_str(value);
        // ImplicitOptional fields don't need presence tracking
    }

    fn get<'a>(&'a self, _shared: &'a Self::SharedFields) -> Self::GetValue<'a> {
        self.data.0.as_str()
    }

    fn clear(&mut self, _shared: &mut Self::SharedFields) {
        self.data.0.clear();
        // ImplicitOptional fields don't need presence tracking
    }

    fn is_present(&self, _shared: &Self::SharedFields) -> bool {
        // ImplicitOptional fields are present only if not equal to default value (empty string)
        !self.data.0.is_empty()
    }
}

/// Implementation for StringFieldWrapper with ExplicitOptional
impl<
    A: Allocator + Clone,
    const FIELD_NUMBER: u32,
    const PRESENCE_BIT_INDEX: usize,
    const SHARED_BYTES_LEN: usize,
>
    FieldOperations<
        StringFieldWrapper<A>,
        ExplicitOptional<PRESENCE_BIT_INDEX>,
        FIELD_NUMBER,
        SHARED_BYTES_LEN,
    >
    for FieldStorage<
        StringFieldWrapper<A>,
        ExplicitOptional<PRESENCE_BIT_INDEX>,
        FIELD_NUMBER,
        SHARED_BYTES_LEN,
        A,
    >
{
    type SetValue<'a> = &'a str;
    type GetValue<'a>
        = Option<&'a str>
    where
        A: 'a;
    type SharedFields = SharedFields<SHARED_BYTES_LEN, A>;

    const FIELD_TYPE: ProtobufFieldType = ProtobufFieldType::String;

    fn set(&mut self, shared: &mut Self::SharedFields, value: Self::SetValue<'_>) {
        self.data.0.clear();
        self.data.0.push_str(value);
        shared.has_bits_mut().set(PRESENCE_BIT_INDEX, true);
    }

    fn get<'a>(&'a self, shared: &'a Self::SharedFields) -> Self::GetValue<'a> {
        if shared.is_field_present(PRESENCE_BIT_INDEX) {
            Some(self.data.0.as_str())
        } else {
            None
        }
    }

    fn clear(&mut self, shared: &mut Self::SharedFields) {
        self.data.0.clear();
        shared.has_bits_mut().set(PRESENCE_BIT_INDEX, false);
    }

    fn is_present(&self, shared: &Self::SharedFields) -> bool {
        shared.is_field_present(PRESENCE_BIT_INDEX)
    }
}

/// Implementation for MessageFieldWrapper with SingularMessage label
///
/// Uses heap allocation with pointer null checks for presence tracking.
/// No need for presence bits - the Option<Box<M>> handles presence directly.
/// This implementation is used for both implicit and explicit optional message fields.
/// Repeated message fields use the `Repeated` label instead.
impl<
    M: crate::Message,
    A: Allocator + Clone,
    const FIELD_NUMBER: u32,
    const SHARED_BYTES_LEN: usize,
> FieldOperations<MessageFieldWrapper<M, A>, SingularMessage, FIELD_NUMBER, SHARED_BYTES_LEN>
    for FieldStorage<MessageFieldWrapper<M, A>, SingularMessage, FIELD_NUMBER, SHARED_BYTES_LEN, A>
{
    type SetValue<'a> = ();
    type GetValue<'a>
        = Option<&'a M>
    where
        A: 'a,
        M: 'a;
    type SharedFields = SharedFields<SHARED_BYTES_LEN, A>;

    const FIELD_TYPE: ProtobufFieldType = ProtobufFieldType::Message;

    fn set(&mut self, _shared: &mut Self::SharedFields, _value: Self::SetValue<'_>) {
        // TODO: Implement message field setter when needed
        // Message fields are typically set via mutable accessors (e.g., address_mut())
    }

    fn get<'a>(&'a self, _shared: &'a Self::SharedFields) -> Self::GetValue<'a> {
        // Use Option<Box<M>> for presence checking
        self.data.value.as_ref().map(|boxed| boxed.as_ref())
    }

    fn clear(&mut self, _shared: &mut Self::SharedFields) {
        self.data.value = None;
        // No presence bit needed - Option<Box<M>> handles presence
    }

    fn is_present(&self, _shared: &Self::SharedFields) -> bool {
        // Use Option<Box<M>> for presence checking
        self.data.value.is_some()
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
    fn test_field_type_constants() {
        type NameField = FieldStorage<StringFieldWrapper, ImplicitOptional, 1, 1>;
        type AgeField = FieldStorage<i32, ImplicitOptional, 2, 1>;
        type EmailField = FieldStorage<StringFieldWrapper, ExplicitOptional<0>, 3, 1>;

        assert_eq!(NameField::FIELD_NUMBER, 1);
        assert_eq!(NameField::FIELD_TYPE, ProtobufFieldType::String);

        assert_eq!(AgeField::FIELD_NUMBER, 2);
        assert_eq!(AgeField::FIELD_TYPE, ProtobufFieldType::Int32);

        assert_eq!(EmailField::FIELD_NUMBER, 3);
        assert_eq!(EmailField::FIELD_TYPE, ProtobufFieldType::String);
    }

    #[test]
    fn test_field_methods() {
        type NameField = FieldStorage<StringFieldWrapper, ImplicitOptional, 1, 1>;
        type EmailField = FieldStorage<StringFieldWrapper, ExplicitOptional<0>, 3, 1>;

        assert_eq!(NameField::field_number(), 1);
        assert_eq!(NameField::field_type(), ProtobufFieldType::String);
        assert_eq!(NameField::field_type().wire_type(), 2);

        assert_eq!(EmailField::field_number(), 3);
        assert_eq!(EmailField::field_type(), ProtobufFieldType::String);
        assert_eq!(EmailField::field_type().wire_type(), 2);
    }
}
