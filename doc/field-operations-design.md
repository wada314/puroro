# Field Operations Design

**Last Updated**: 2025-10-21  
**Status**: Implemented in `puroro/src/field_ops.rs`

## Overview

This document explains the field operations pattern used in Puroro's generated code,
separating shared state from exclusive field storage using a trait-based approach.

**See also:** `doc/field-context-design.md` for detailed implementation patterns

## Current Implementation: Trait-Based Field Operations

The current implementation uses a trait-based approach where field operations are
handled through the `Field` trait, providing type-safe operations with compile-time
metadata encoding.

### Key Components

**FieldType Struct**: Holds actual field data with type-level metadata
```rust
pub struct FieldType<T, L: FieldLabel, const FIELD_NUMBER: u32, const SHARED_BYTES_LEN: usize> {
    pub data: T,
    _phantom: PhantomData<L>,
}
```

**Field Trait**: Provides operations for field handling
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

**SharedFields**: Wrapper for shared state
```rust
pub struct SharedFields<const BYTES: usize> {
    has_bits: BitArray<[u8; BYTES]>,
}
```

## Field Operation Types

### ImplicitOptional Fields

Fields with implicit presence (Proto3 default behavior):

```rust
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
        // No presence tracking needed
    }
    
    fn get<'a>(&'a self, _shared: &'a Self::SharedFields) -> Self::GetValue<'a> {
        self.data.as_str()
    }
    
    fn clear(&mut self, _shared: &mut Self::SharedFields) {
        self.data.clear();
    }
    
    fn is_present(&self, _shared: &Self::SharedFields) -> bool {
        !self.data.is_empty()  // Present if not default value
    }
}
```

### ExplicitOptional Fields

Fields with explicit presence tracking (Proto3 `optional` keyword):

```rust
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

## Field Label Types

Protobuf field labels are represented as zero-sized marker types:

```rust
/// Marker trait for protobuf field labels
pub trait FieldLabel: private::Sealed {}

/// Marker for implicit optional fields (Proto3 default)
pub struct ImplicitOptional;

/// Marker for explicit optional fields (Proto3 `optional` keyword)
pub struct ExplicitOptional<const PRESENCE_BIT_INDEX: usize>;

/// Marker for repeated fields
pub struct Repeated;

/// Marker for map fields
pub struct Map;
```

## Protobuf Field Types

The `ProtobufFieldType` enum represents all supported protobuf field types:

```rust
pub enum ProtobufFieldType {
    // Scalar types
    Int32, Int64, UInt32, UInt64,
    SInt32, SInt64, Fixed32, Fixed64,
    SFixed32, SFixed64, Float, Double, Bool,
    
    // String types
    String, Bytes,
    
    // Message types
    Message, Enum, Group,
}
```

Each field type knows its wire type for serialization:

```rust
impl ProtobufFieldType {
    pub const fn wire_type(self) -> u8 {
        match self {
            Self::Int32 | Self::Int64 | Self::UInt32 | Self::UInt64 |
            Self::SInt32 | Self::SInt64 | Self::Bool => 0, // Varint
            
            Self::Fixed64 | Self::SFixed64 | Self::Double => 1, // 64-bit
            
            Self::String | Self::Bytes | Self::Message | Self::Enum => 2, // Length-delimited
            
            Self::Fixed32 | Self::SFixed32 | Self::Float => 5, // 32-bit
            
            Self::Group => 3, // Start group (deprecated)
        }
    }
}
```

## Implementation Status

### ✅ Completed Field Types

**String Fields**:
- ✅ ImplicitOptional (Proto3 default)
- ✅ ExplicitOptional (Proto3 `optional` keyword)

**Scalar Fields**:
- ✅ i32, i64, u32, u64 (ImplicitOptional and ExplicitOptional)
- ✅ f32, f64 (ImplicitOptional and ExplicitOptional)
- ✅ bool (ImplicitOptional and ExplicitOptional)

### 🚧 In Progress

**Complex Field Types**:
- 🚧 Repeated fields (Vec<T>)
- 🚧 Map fields (HashMap<K, V>)
- 🚧 Message fields (nested messages)
- 🚧 Enum fields

### 📋 Future

**Advanced Features**:
- 📋 Boolean field packing optimization
- 📋 Allocator support integration
- 📋 Unknown fields preservation
- 📋 Oneof field support

## Design Benefits

### Type Safety

The trait-based approach provides compile-time guarantees:

```rust
// Field number is encoded at type level
type NameField = FieldType<String, ImplicitOptional, 1, 1>;
type EmailField = FieldType<String, ExplicitOptional<0>, 3, 1>;

// Compiler enforces correct usage
assert_eq!(NameField::FIELD_NUMBER, 1);
assert_eq!(EmailField::FIELD_NUMBER, 3);
```

### Memory Efficiency

- **Stack allocation**: SharedFields uses BitArray for presence tracking
- **No Option<T> overhead**: ExplicitOptional fields use bit flags instead
- **Scalable**: Supports unlimited fields with minimal memory overhead

### Code Generation

The trait-based approach simplifies code generation:

```rust
// Generated code is simple and consistent
impl PersonAppend for PersonImpl {
    fn set_name(&mut self, v: &str) {
        self.name.set(&mut self._shared, v);
    }
    
    fn set_email(&mut self, v: &str) {
        self.email.set(&mut self._shared, v);
    }
}
```

## Summary

The trait-based field operations approach provides:

- ✅ **Type Safety**: Compile-time validation of field operations
- ✅ **Memory Efficiency**: Stack-allocated SharedFields with BitArray
- ✅ **Code Simplicity**: Single-line operations in generated code
- ✅ **Extensibility**: Easy to add new field types and features
- ✅ **Performance**: Zero runtime overhead for field operations
- ✅ **Scalability**: Supports unlimited fields with minimal overhead

This design forms the foundation for Puroro's efficient and type-safe protobuf implementation.

