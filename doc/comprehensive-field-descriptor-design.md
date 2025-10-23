//! Comprehensive Field Descriptor Design
//!
//! This document outlines the design for a unified field descriptor that contains
//! all information needed for a protobuf field in a single type.

use crate::shared::SharedFields;
use std::marker::PhantomData;

// ============================================================================
// Comprehensive Field Descriptor
// ============================================================================

/// Comprehensive field descriptor containing all protobuf field information.
///
/// This trait provides a unified interface for all field metadata including:
/// - Field type (i32, String, Vec<T>, etc.)
/// - Field label (ImplicitOptional, ExplicitOptional, Repeated, Map)
/// - Default value
/// - Field number
/// - Presence semantics
/// - Storage type
/// - Value type
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

    /// The field label (ImplicitOptional, ExplicitOptional, Repeated, Map)
    type Label = L;

    /// Creates the default value for this field
    fn default_value() -> Self::Storage;

    /// Sets the field value
    fn set<const BYTES: usize>(
        shared: &mut SharedFields<BYTES>,
        storage: &mut Self::Storage,
        value: Self::Value,
    );

    /// Gets the field value
    fn get<'a>(storage: &'a Self::Storage) -> Self::GetValue<'a>;

    /// Clears the field value
    fn clear<const BYTES: usize>(
        shared: &mut SharedFields<BYTES>,
        storage: &mut Self::Storage,
    );

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
// Field Descriptor Implementation Macro
// ============================================================================

/// Macro to implement FieldDescriptor for common field types
macro_rules! impl_field_descriptor {
    // Scalar types with ImplicitOptional
    ($storage_type:ty, $value_type:ty, $get_type:ty, $default:expr, $field_num:literal, $bit_idx:literal) => {
        impl FieldDescriptor<$value_type, ImplicitOptional, $field_num, $bit_idx> for Field<$value_type, ImplicitOptional> {
            type Storage = $storage_type;
            type Value = $value_type;
            type GetValue<'a> = $get_type;

            fn default_value() -> Self::Storage {
                $default
            }

            fn set<const BYTES: usize>(
                _shared: &mut SharedFields<BYTES>,
                storage: &mut Self::Storage,
                value: Self::Value,
            ) {
                *storage = value;
                // ImplicitOptional fields don't need presence tracking
            }

            fn get<'a>(storage: &'a Self::Storage) -> Self::GetValue<'a> {
                *storage
            }

            fn clear<const BYTES: usize>(
                _shared: &mut SharedFields<BYTES>,
                storage: &mut Self::Storage,
            ) {
                *storage = Self::default_value();
                // ImplicitOptional fields don't need presence tracking
            }

            fn is_present<const BYTES: usize>(_shared: &SharedFields<BYTES>) -> bool {
                true // ImplicitOptional fields are always present
            }
        }
    };

    // String types with ImplicitOptional
    ($field_num:literal, $bit_idx:literal) => {
        impl FieldDescriptor<String, ImplicitOptional, $field_num, $bit_idx> for Field<String, ImplicitOptional> {
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

            fn get<'a>(storage: &'a Self::Storage) -> Self::GetValue<'a> {
                storage.as_str()
            }

            fn clear<const BYTES: usize>(
                _shared: &mut SharedFields<BYTES>,
                storage: &mut Self::Storage,
            ) {
                storage.clear();
                // ImplicitOptional fields don't need presence tracking
            }

            fn is_present<const BYTES: usize>(_shared: &SharedFields<BYTES>) -> bool {
                true // ImplicitOptional fields are always present
            }
        }
    };
}

// ============================================================================
// Example Usage
// ============================================================================

/*
// Define field descriptors with all information
type NameField = Field<String, ImplicitOptional>;
type AgeField = Field<i32, ImplicitOptional>;
type EmailField = Field<String, ImplicitOptional>;

// Implement the descriptors
impl_field_descriptor!(1, 0); // name field: field_number=1, bit_index=0
impl_field_descriptor!(i32, i32, i32, 0, 2, 1); // age field: field_number=2, bit_index=1
impl_field_descriptor!(3, 2); // email field: field_number=3, bit_index=2

// Usage in generated struct
pub struct PersonImpl {
    _shared: SharedFields<1>,
    name: <NameField as FieldDescriptor<String, ImplicitOptional, 1, 0>>::Storage,
    age: <AgeField as FieldDescriptor<i32, ImplicitOptional, 2, 1>>::Storage,
    email: <EmailField as FieldDescriptor<String, ImplicitOptional, 3, 2>>::Storage,
}

// Generated implementation
impl Person for PersonImpl {
    fn name(&self) -> &str {
        NameField::get(&self.name)
    }
    
    fn age(&self) -> i32 {
        AgeField::get(&self.age)
    }
    
    fn email(&self) -> &str {
        EmailField::get(&self.email)
    }
}

impl PersonAppend for PersonImpl {
    fn set_name(&mut self, v: &str) {
        NameField::set(&mut self._shared, &mut self.name, v);
    }
    
    fn set_age(&mut self, v: i32) {
        AgeField::set(&mut self._shared, &mut self.age, v);
    }
    
    fn set_email(&mut self, v: &str) {
        EmailField::set(&mut self._shared, &mut self.email, v);
    }
}
*/

// ============================================================================
// Benefits of This Approach
// ============================================================================

/*
1. **Single Source of Truth**: All field information is in one place
   - Field type, label, number, bit index, default value
   - No need for separate constants or type aliases

2. **Type Safety**: Compiler enforces correct usage
   - Wrong field number or bit index won't compile
   - Type mismatches are caught at compile time

3. **Self-Documenting**: Field descriptor contains all metadata
   - Easy to understand field properties
   - Clear relationship between field number and bit index

4. **Extensible**: Easy to add new field types
   - Just implement FieldDescriptor for new types
   - No need to modify multiple places

5. **Code Generation Friendly**: Simple to generate
   - One implementation per field
   - Clear mapping from protobuf schema to Rust code

6. **Performance**: Zero runtime cost
   - All information is compile-time constants
   - Inlined operations with no overhead
*/
