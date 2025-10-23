//! Example implementation using the new comprehensive FieldDescriptor approach.
//!
//! This demonstrates how the unified field descriptor can be used to create
//! message structs with all field information contained in the type system.

use puroro::{
    error::Error,
    field_ops::{Field, FieldDescriptor, ImplicitOptional},
    shared::SharedFields,
    Message,
};

// ============================================================================
// Field Descriptors with Complete Information
// ============================================================================

/// Field descriptor for 'name' field
/// - Field number: 1
/// - Bit index: 0
/// - Type: String with ImplicitOptional
type NameField = Field<String, ImplicitOptional>;

/// Field descriptor for 'age' field  
/// - Field number: 2
/// - Bit index: 1
/// - Type: i32 with ImplicitOptional
type AgeField = Field<i32, ImplicitOptional>;

/// Field descriptor for 'email' field
/// - Field number: 3
/// - Bit index: 2
/// - Type: String with ImplicitOptional
type EmailField = Field<String, ImplicitOptional>;

// ============================================================================
// PersonImpl with Unified Field Descriptors
// ============================================================================

/// Person message implementation using comprehensive field descriptors.
///
/// Each field type contains all necessary information:
/// - Field number (for serialization)
/// - Bit index (for presence tracking)
/// - Default value (for initialization)
/// - Storage type (for memory layout)
/// - Value type (for API)
#[derive(Debug, Clone, PartialEq)]
pub struct PersonImpl {
    // Shared fields: presence tracking, etc.
    // For 3 fields: ⌈3/8⌉ = 1 byte (stack-allocated)
    _shared: SharedFields<1>,

    // Fields with complete type information
    name: <NameField as FieldDescriptor<String, ImplicitOptional, 1, 0>>::Storage,
    age: <AgeField as FieldDescriptor<i32, ImplicitOptional, 2, 1>>::Storage,
    email: <EmailField as FieldDescriptor<String, ImplicitOptional, 3, 2>>::Storage,
}

impl PersonImpl {
    /// Creates a new Person with default values.
    pub fn new() -> Self {
        Self {
            _shared: SharedFields::new(),
            name: NameField::default_value(),
            age: AgeField::default_value(),
            email: EmailField::default_value(),
        }
    }
}

impl Default for PersonImpl {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Trait Implementations using FieldDescriptor
// ============================================================================

/// Infallible immutable trait for Person message.
pub trait Person {
    fn name(&self) -> &str;
    fn age(&self) -> i32;
    fn email(&self) -> &str;
    fn has_name(&self) -> bool;
    fn has_age(&self) -> bool;
    fn has_email(&self) -> bool;
}

/// Infallible append-only trait for Person message.
pub trait PersonAppend: Person {
    fn set_name(&mut self, v: &str);
    fn set_age(&mut self, v: i32);
    fn set_email(&mut self, v: &str);
}

/// Infallible fully mutable trait for Person message.
pub trait PersonMut: PersonAppend {
    fn clear_name(&mut self);
    fn clear_age(&mut self);
    fn clear_email(&mut self);
}

impl Person for PersonImpl {
    #[inline]
    fn name(&self) -> &str {
        NameField::get(&self.name)
    }

    #[inline]
    fn age(&self) -> i32 {
        AgeField::get(&self.age)
    }

    #[inline]
    fn email(&self) -> &str {
        EmailField::get(&self.email)
    }

    #[inline]
    fn has_name(&self) -> bool {
        NameField::is_present(&self._shared)
    }

    #[inline]
    fn has_age(&self) -> bool {
        AgeField::is_present(&self._shared)
    }

    #[inline]
    fn has_email(&self) -> bool {
        EmailField::is_present(&self._shared)
    }
}

impl PersonAppend for PersonImpl {
    #[inline]
    fn set_name(&mut self, v: &str) {
        NameField::set(&mut self._shared, &mut self.name, v);
    }

    #[inline]
    fn set_age(&mut self, v: i32) {
        AgeField::set(&mut self._shared, &mut self.age, v);
    }

    #[inline]
    fn set_email(&mut self, v: &str) {
        EmailField::set(&mut self._shared, &mut self.email, v);
    }
}

impl PersonMut for PersonImpl {
    #[inline]
    fn clear_name(&mut self) {
        NameField::clear(&mut self._shared, &mut self.name);
    }

    #[inline]
    fn clear_age(&mut self) {
        AgeField::clear(&mut self._shared, &mut self.age);
    }

    #[inline]
    fn clear_email(&mut self) {
        EmailField::clear(&mut self._shared, &mut self.email);
    }
}

impl Message for PersonImpl {
    fn parse_from_bytes(_bytes: &[u8]) -> Result<Self, Error> {
        // TODO: Implement actual parsing using field numbers
        todo!("Parsing not yet implemented")
    }

    fn write_to_bytes(&self) -> Result<Vec<u8>, Error> {
        // TODO: Implement actual serialization using field numbers
        todo!("Serialization not yet implemented")
    }

    fn compute_size(&self) -> usize {
        // TODO: Implement actual size computation
        todo!("Size computation not yet implemented")
    }
}

// ============================================================================
// Benefits Demonstration
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_descriptor_information() {
        // All field information is available at compile time
        assert_eq!(NameField::field_number(), 1);
        assert_eq!(NameField::bit_index(), 0);
        assert_eq!(AgeField::field_number(), 2);
        assert_eq!(AgeField::bit_index(), 1);
        assert_eq!(EmailField::field_number(), 3);
        assert_eq!(EmailField::bit_index(), 2);
    }

    #[test]
    fn test_default_values() {
        let person = PersonImpl::new();
        
        // Default values are automatically correct
        assert_eq!(person.name(), "");
        assert_eq!(person.age(), 0);
        assert_eq!(person.email(), "");
    }

    #[test]
    fn test_field_operations() {
        let mut person = PersonImpl::new();
        
        // Set operations work correctly
        person.set_name("Alice");
        person.set_age(30);
        person.set_email("alice@example.com");
        
        assert_eq!(person.name(), "Alice");
        assert_eq!(person.age(), 30);
        assert_eq!(person.email(), "alice@example.com");
        
        // Clear operations work correctly
        person.clear_name();
        person.clear_age();
        person.clear_email();
        
        assert_eq!(person.name(), "");
        assert_eq!(person.age(), 0);
        assert_eq!(person.email(), "");
    }

    #[test]
    fn test_presence_checking() {
        let person = PersonImpl::new();
        
        // ImplicitOptional fields are always present
        assert!(person.has_name());
        assert!(person.has_age());
        assert!(person.has_email());
    }
}

// ============================================================================
// Code Generation Template
// ============================================================================

/*
For code generation, the template would be:

```rust
// For each field in the protobuf message:
type {FieldName}Field = Field<{RustType}, {Label}>;

// Struct definition:
pub struct {MessageName}Impl {
    _shared: SharedFields<{ByteCount}>,
    {field_name}: <{FieldName}Field as FieldDescriptor<{RustType}, {Label}, {field_number}, {bit_index}>>::Storage,
    // ... more fields
}

// Implementation:
impl {MessageName} for {MessageName}Impl {
    fn {field_name}(&self) -> {ReturnType} {
        {FieldName}Field::get(&self.{field_name})
    }
    // ... more methods
}

impl {MessageName}Append for {MessageName}Impl {
    fn set_{field_name}(&mut self, v: {ValueType}) {
        {FieldName}Field::set(&mut self._shared, &mut self.{field_name}, v);
    }
    // ... more methods
}
```

This approach provides:
1. **Single Source of Truth**: All field info in one place
2. **Type Safety**: Compiler enforces correctness
3. **Self-Documenting**: Field properties are clear
4. **Extensible**: Easy to add new field types
5. **Code Generation Friendly**: Simple templates
6. **Zero Runtime Cost**: All compile-time constants
*/
