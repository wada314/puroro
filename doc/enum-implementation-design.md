# Protobuf Enum Implementation Design

**Last Updated**: 2025-10-21  
**Status**: Design Decision Document  
**Related**: `field-operations-design.md`, `comprehensive-field-descriptor-design.md`

## Overview

This document records the comprehensive design discussion and final decisions for implementing protobuf enums in Puroro. The discussion covered proto2 closed enums, proto3 open enums, and the `allow_alias` option, resulting in a unified implementation strategy.

## Problem Statement

Protobuf enums have two main variants:
- **proto2 closed enums**: Unknown values are rejected and stored in unknown fields
- **proto3 open enums**: Unknown values are accepted and stored directly in the field

Additionally, both variants support the `allow_alias` option, which allows multiple enum names to map to the same integer value.

## Design Requirements

1. **Type Safety**: Compile-time guarantees for known enum values
2. **Forward Compatibility**: Support for future enum value additions
3. **Performance**: Zero-cost abstractions where possible
4. **Usability**: Intuitive API for users
5. **Consistency**: Unified approach across proto2/proto3 and allow_alias variants

## Explored Approaches

### Approach 1: Rust enum + `#[repr(i32)]` + `#[non_exhaustive]`

```rust
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MyEnum {
    Unspecified = 0,
    ValueA = 1,
    ValueB = 2,
}
```

**Pros:**
- Highest performance (zero-cost conversion)
- Excellent usability
- Complete forward compatibility
- Type safety

**Cons:**
- Cannot handle `allow_alias` (same integer value for multiple variants)
- Cannot handle proto3 open enums

### Approach 2: Rust enum + `From`/`Into` + `#[non_exhaustive]`

```rust
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MyEnum {
    Unspecified,
    ValueA,
    ValueB,
}

impl From<i32> for MyEnum {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Unspecified,
            1 => Self::ValueA,
            2 => Self::ValueB,
            _ => panic!("Unknown enum value: {}", value),
        }
    }
}
```

**Pros:**
- Supports `allow_alias`
- Excellent usability
- Complete forward compatibility
- Type safety

**Cons:**
- Small conversion cost
- Cannot handle proto3 open enums

### Approach 3: Rust enum + `Unknown` variant + `#[non_exhaustive]`

```rust
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MyEnum {
    Unspecified,
    ValueA,
    ValueB,
    Unknown(i32),
}
```

**Pros:**
- Supports proto3 open enums
- Supports `allow_alias` (no integer values specified)
- Good usability
- Complete forward compatibility

**Cons:**
- Ambiguous state problem (`ValueA` vs `Unknown(1)`)
- Small conversion cost

### Approach 4: `Result<Enum, i32>`

```rust
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MyEnum {
    Unspecified = 0,
    ValueA = 1,
    ValueB = 2,
}

type MyEnumResult = Result<MyEnum, i32>;
```

**Pros:**
- Type safety (known values only in enum type)
- Complete forward compatibility
- Supports all cases

**Cons:**
- Ambiguous state problem (`Ok(ValueA)` vs `Err(1)`)
- Always requires error handling
- Implementation complexity
- Usability concerns

### Approach 5: Newtype Wrapper + Constants

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MyEnum(i32);

impl MyEnum {
    pub const UNSPECIFIED: Self = Self(0);
    pub const VALUE_A: Self = Self(1);
    pub const VALUE_B: Self = Self(2);
}
```

**Pros:**
- Supports all cases
- Highest performance
- No ambiguous states

**Cons:**
- Lower usability
- Limited pattern matching

## Key Insights

### 1. `#[non_exhaustive]` is Essential

The `#[non_exhaustive]` attribute is crucial for forward compatibility. Without it, removing unused enum variants causes compilation errors even when the user code doesn't reference those variants.

### 2. `allow_alias` Behavior

Existing protobuf implementations (C++, Python) follow a consistent pattern:
- **Storage**: Always stores the integer value
- **Retrieval**: Returns the first defined name for a given integer value
- **Round-trip**: Name information is not preserved, only the integer value

### 3. Ambiguous State Problem

When users can manually construct ambiguous states (e.g., `ValueA` and `Unknown(1)` for the same value, or `Ok(ValueA)` and `Err(1)`), it creates data inconsistency issues. This problem affects approaches using `Unknown` variants or `Result` types. However, this can be mitigated by designing getter/setter APIs properly to prevent users from directly constructing these ambiguous states.

### 4. Integer Storage Strategy

The most straightforward approach for `allow_alias` support is to store integer values directly in message structs, converting to enum types only when users invoke getter methods.

## Final Design Decision

### Unified Implementation Strategy

**Core Principle**: Store enum values as `i32` in message structs, convert to enum types only when needed.

### Case-by-Case Implementation

#### Case 1: proto2 + allow_alias = false
```rust
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MyProto2Enum {
    Unspecified = 0,
    ValueA = 1,
    ValueB = 2,
}
```

#### Case 2: proto2 + allow_alias = true
```rust
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MyProto2Enum {
    Unspecified,
    ValueA,
    ValueB,  // allow_alias
}
```

#### Case 3: proto3 + allow_alias = false
```rust
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MyProto3Enum {
    Unspecified = 0,
    ValueA = 1,
    ValueB = 2,
}

// Wrapped in Result where needed
pub type MyProto3EnumResult = Result<MyProto3Enum, i32>;
```

#### Case 4: proto3 + allow_alias = true
```rust
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MyProto3Enum {
    Unspecified,
    ValueA,
    ValueB,  // allow_alias
}

// Wrapped in Result where needed
pub type MyProto3EnumResult = Result<MyProto3Enum, i32>;
```

### Field Implementation Pattern

```rust
pub struct MyMessage {
    enum_field: i32,  // Always store as i32
}

impl MyMessage {
    // Set with known enum value (type-safe)
    pub fn set_enum_field(&mut self, value: MyEnum) {
        self.enum_field = value.to_wire();
    }
    
    // Get as Result<MyEnum, i32> (handles unknown values)
    pub fn enum_field(&self) -> Result<MyEnum, i32> {
        MyEnum::from_wire(self.enum_field)
    }
    
    // Get as i32 (raw value access)
    pub fn enum_field_as_i32(&self) -> i32 {
        self.enum_field
    }
}
```

## Field Usage Examples

### proto2 Usage
```rust
pub struct MyProto2Message {
    enum_field: i32,  // Store as i32
}

impl MyProto2Message {
    // Type-safe setter
    pub fn set_enum_field(&mut self, value: MyProto2Enum) {
        self.enum_field = value.to_wire();
    }
    
    // Getter handles unknown values
    pub fn enum_field(&self) -> Result<MyProto2Enum, i32> {
        MyProto2Enum::from_wire(self.enum_field)
    }
    
    // Raw i32 access
    pub fn enum_field_as_i32(&self) -> i32 {
        self.enum_field
    }
}
```

### proto3 Usage
```rust
pub struct MyProto3Message {
    enum_field: i32,  // Store as i32
}

impl MyProto3Message {
    // Type-safe setter (only accepts defined enum values)
    pub fn set_enum_field(&mut self, value: MyProto3Enum) {
        self.enum_field = value.to_wire();
    }
    
    // Getter handles unknown values (proto3 open enum requirement)
    pub fn enum_field(&self) -> Result<MyProto3Enum, i32> {
        MyProto3Enum::from_wire(self.enum_field)
    }
    
    // Raw i32 access
    pub fn enum_field_as_i32(&self) -> i32 {
        self.enum_field
    }
}
```

## Implementation Benefits

### 1. Type Safety
- Setters only accept defined enum values (prevents invalid assignments)
- Getters handle unknown values gracefully with `Result<Enum, i32>`
- Compile-time type checking for enum assignments

### 2. Consistency
- All enum fields stored as `i32`
- Same pattern for getter/setter methods
- Unified API across all cases

### 3. Simplicity
- Simple field type (`i32`)
- No complex type conversions
- Memory efficient

### 4. Flexibility
- Handles both known and unknown values appropriately
- Users can access raw `i32` values when needed
- Simple serialization

### 5. `allow_alias` Support
- Integer values stored internally
- First defined name returned on retrieval
- Consistent with existing implementations

## Code Generation Strategy

```rust
fn generate_enum(enum_descriptor: &EnumDescriptorProto, syntax: Syntax) -> String {
    let allow_alias = enum_descriptor.options().allow_alias();
    
    match (syntax, allow_alias) {
        (Syntax::Proto2, false) => generate_proto2_enum_with_repr(enum_descriptor),
        (Syntax::Proto2, true) => generate_proto2_enum_without_repr(enum_descriptor),
        (Syntax::Proto3, false) => generate_proto3_enum_with_repr(enum_descriptor),
        (Syntax::Proto3, true) => generate_proto3_enum_without_repr(enum_descriptor),
        (Syntax::Editions, _) => {
            if is_open_enum(enum_descriptor) {
                generate_proto3_enum_without_repr(enum_descriptor)
            } else {
                generate_proto2_enum_without_repr(enum_descriptor)
            }
        }
    }
}
```

## Compatibility Analysis

### Forward Compatibility
- ✅ **Adding new values**: `#[non_exhaustive]` ensures compatibility
- ✅ **Removing unused values**: No compilation errors for unused variants
- ✅ **Unknown values**: Properly handled in all cases

### Backward Compatibility
- ✅ **Existing code**: Continues to work without changes
- ✅ **Serialization**: Integer values preserved correctly
- ✅ **Deserialization**: Proper enum value resolution

## Conclusion

The unified integer storage strategy provides the best balance of:
- **Consistency**: Same pattern across all enum cases
- **Simplicity**: Straightforward implementation
- **Flexibility**: Handles all protobuf enum requirements
- **Type Safety**: Appropriate type conversions
- **Performance**: Efficient memory usage and operations

This design ensures that Puroro's enum implementation is both robust and user-friendly while maintaining compatibility with existing protobuf implementations.

## C++ Official Implementation Compatibility

This design aligns with the [C++ official implementation](https://protobuf.dev/reference/cpp/cpp-generated/#enum_field):

- **Setters**: Only accept defined enum values (type-safe)
- **Getters**: Handle unknown values gracefully (proto3 open enum requirement)  
- **Storage**: Always store as `i32` internally

The C++ implementation aborts in debug builds when trying to set unknown enum values, which validates our approach of only providing type-safe setters.

### Key Design Decisions

1. **Type Safety**: Setters only accept `Status` enum values, preventing invalid assignments
2. **Proto3 Compatibility**: Getters return `Result<Status, i32>` to handle unknown values
3. **Storage Efficiency**: Always store as `i32` for optimal memory usage
4. **Forward Compatibility**: `#[non_exhaustive]` ensures new enum values can be added

## References

- [Protobuf Enum Documentation](https://protobuf.dev/programming-guides/enum/)
- [C++ Generated Code Guide - Enum Fields](https://protobuf.dev/reference/cpp/cpp-generated/#enum_field)
- [Buf Technologies: Dangers of Enum Aliases](https://buf.build/blog/totw-6-dangers-of-enum-aliases)
- [Protobuf Editions Features](https://protobuf.dev/editions/features/)
