# Comprehensive Field Descriptor Design

**Last Updated**: 2025-10-21  
**Status**: Implemented in `puroro/src/field_ops.rs`

This document outlines the design for a unified field descriptor that contains
all information needed for a protobuf field in a single type.

## Overview

The field descriptor system in Puroro uses a trait-based approach that encodes
field metadata at the type level, providing compile-time guarantees about field
properties while maintaining runtime efficiency.

## Key Components

### Field Label Types

Protobuf field labels are represented as zero-sized marker types:

```rust
/// Marker for implicit optional fields (Proto3 default)
pub struct ImplicitOptional;

/// Marker for explicit optional fields (Proto3 `optional` keyword)
pub struct ExplicitOptional<const PRESENCE_BIT_INDEX: usize>;

/// Marker for repeated fields
pub struct Repeated;

/// Marker for map fields
pub struct Map;
```

### FieldType Struct

The `FieldType` struct holds actual field data and encodes type-level information:

```rust
pub struct FieldType<T, L: FieldLabel, const FIELD_NUMBER: u32, const SHARED_BYTES_LEN: usize> {
    pub data: T,
    _phantom: PhantomData<L>,
}
```

**Type Parameters:**
- `T`: The value type (i32, String, etc.)
- `L`: The field label (ImplicitOptional, ExplicitOptional<BIT_INDEX>, Repeated, Map)
- `FIELD_NUMBER`: The protobuf field number
- `SHARED_BYTES_LEN`: Number of bytes needed for SharedFields storage

### Field Trait

The `Field` trait provides operations for field handling:

```rust
pub trait Field<T, L: FieldLabel, const FIELD_NUMBER: u32, const SHARED_BYTES_LEN: usize> {
    type Value<'a>;
    type GetValue<'a> where Self: 'a;
    type SharedFields;
    
    const FIELD_TYPE: ProtobufFieldType;
    const FIELD_NUMBER: u32 = FIELD_NUMBER;
    
    fn set(&mut self, shared: &mut Self::SharedFields, value: Self::Value<'_>);
    fn get<'a>(&'a self, shared: &'a Self::SharedFields) -> Self::GetValue<'a>;
    fn clear(&mut self, shared: &mut Self::SharedFields);
    fn is_present(&self, shared: &Self::SharedFields) -> bool;
    fn field_number() -> u32;
    fn field_type() -> ProtobufFieldType;
}
```

## Implementation Examples

### String Fields

```rust
// ImplicitOptional String field
impl<const FIELD_NUMBER: u32, const SHARED_BYTES_LEN: usize>
    Field<String, ImplicitOptional, FIELD_NUMBER, SHARED_BYTES_LEN>
    for FieldType<String, ImplicitOptional, FIELD_NUMBER, SHARED_BYTES_LEN>
{
    type Value<'a> = &'a str;
    type GetValue<'a> = &'a str;
    type SharedFields = SharedFields<SHARED_BYTES_LEN>;
    
    const FIELD_TYPE: ProtobufFieldType = ProtobufFieldType::String;
    
    fn set(&mut self, _shared: &mut Self::SharedFields, value: Self::Value<'_>) {
        value.clone_into(&mut self.data);
        // ImplicitOptional fields don't need presence tracking
    }
    
    fn get<'a>(&'a self, _shared: &'a Self::SharedFields) -> Self::GetValue<'a> {
        self.data.as_str()
    }
    
    fn clear(&mut self, _shared: &mut Self::SharedFields) {
        self.data.clear();
    }
    
    fn is_present(&self, _shared: &Self::SharedFields) -> bool {
        !self.data.is_empty()
    }
}
```

### ExplicitOptional Fields

```rust
// ExplicitOptional String field with presence tracking
impl<const FIELD_NUMBER: u32, const PRESENCE_BIT_INDEX: usize, const SHARED_BYTES_LEN: usize>
    Field<String, ExplicitOptional<PRESENCE_BIT_INDEX>, FIELD_NUMBER, SHARED_BYTES_LEN>
    for FieldType<String, ExplicitOptional<PRESENCE_BIT_INDEX>, FIELD_NUMBER, SHARED_BYTES_LEN>
{
    type Value<'a> = &'a str;
    type GetValue<'a> = Option<&'a str>;
    type SharedFields = SharedFields<SHARED_BYTES_LEN>;
    
    const FIELD_TYPE: ProtobufFieldType = ProtobufFieldType::String;
    
    fn set(&mut self, shared: &mut Self::SharedFields, value: Self::Value<'_>) {
        value.clone_into(&mut self.data);
        shared.has_bits_mut().set(PRESENCE_BIT_INDEX, true);
    }
    
    fn get<'a>(&'a self, shared: &'a Self::SharedFields) -> Self::GetValue<'a> {
        if shared.is_field_present(PRESENCE_BIT_INDEX) {
            Some(self.data.as_str())
        } else {
            None
        }
    }
    
    fn clear(&mut self, shared: &mut Self::SharedFields) {
        self.data.clear();
        shared.has_bits_mut().set(PRESENCE_BIT_INDEX, false);
    }
    
    fn is_present(&self, shared: &Self::SharedFields) -> bool {
        shared.is_field_present(PRESENCE_BIT_INDEX)
    }
}
```

## Usage in Generated Code

### Message Struct Definition

```rust
pub struct PersonImpl {
    // Shared fields: For 2 explicit optional fields: ⌈2/8⌉ = 1 byte
    _shared: SharedFields<1>,
    
    // Field definitions with type-level metadata
    name: FieldType<String, ImplicitOptional, 1, 1>,           // Field 1, implicit presence
    email: FieldType<String, ExplicitOptional<0>, 3, 1>,       // Field 3, explicit presence, bit 0
    age: FieldType<i32, ImplicitOptional, 2, 1>,              // Field 2, implicit presence
    score: FieldType<i32, ExplicitOptional<1>, 5, 1>,          // Field 5, explicit presence, bit 1
}
```

### Trait Implementation

```rust
impl Person for PersonImpl {
    fn name(&self) -> &str {
        self.name.get(&self._shared)
    }
    
    fn email(&self) -> Option<&str> {
        self.email.get(&self._shared)
    }
    
    fn age(&self) -> i32 {
        self.age.get(&self._shared)
    }
    
    fn score(&self) -> Option<i32> {
        self.score.get(&self._shared)
    }
    
    fn has_email(&self) -> bool {
        self.email.is_present(&self._shared)
    }
    
    fn has_score(&self) -> bool {
        self.score.is_present(&self._shared)
    }
}

impl PersonAppend for PersonImpl {
    fn set_name(&mut self, v: &str) {
        self.name.set(&mut self._shared, v);
    }
    
    fn set_email(&mut self, v: &str) {
        self.email.set(&mut self._shared, v);
    }
    
    fn set_age(&mut self, v: i32) {
        self.age.set(&mut self._shared, v);
    }
    
    fn set_score(&mut self, v: i32) {
        self.score.set(&mut self._shared, v);
    }
}
```

## Benefits of This Approach

1. **Type Safety**: Compiler enforces correct usage
   - Wrong field number or bit index won't compile
   - Type mismatches are caught at compile time
   - Presence bit indices are encoded at type level

2. **Self-Documenting**: Field descriptor contains all metadata
   - Easy to understand field properties
   - Clear relationship between field number and bit index
   - Field type information available at compile time

3. **Extensible**: Easy to add new field types
   - Just implement Field trait for new types
   - No need to modify multiple places
   - SharedFields can be extended without changing function signatures

4. **Code Generation Friendly**: Simple to generate
   - One implementation per field type
   - Clear mapping from protobuf schema to Rust code
   - Type parameters encode all necessary information

5. **Performance**: Zero runtime cost
   - All information is compile-time constants
   - Inlined operations with no overhead
   - Stack-allocated SharedFields with BitArray

6. **Memory Efficient**: Optimized storage
   - BitArray for presence tracking (1 byte for ≤8 fields)
   - No Option<T> overhead for optional fields
   - Stack allocation for shared state

## Current Implementation Status

✅ **Implemented**:
- Field label types (ImplicitOptional, ExplicitOptional, Repeated, Map)
- FieldType struct with type-level metadata
- Field trait with comprehensive operations
- String field implementations (both ImplicitOptional and ExplicitOptional)
- Scalar field implementations (i32, i64, u32, u64, f32, f64, bool)
- SharedFields with BitArray for presence tracking
- Generated code integration in sandbox

🚧 **In Progress**:
- Repeated field implementations
- Map field implementations
- Message field implementations
- Enum field implementations

📋 **Future**:
- Allocator support integration
- Boolean field packing optimization
- Unknown fields preservation
- Advanced field types (oneof, extensions)
