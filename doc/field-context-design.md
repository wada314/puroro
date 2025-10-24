# Field Operations Design

**Last Updated**: 2025-10-21  
**Status**: Implemented in `puroro/src/field_ops.rs`

## Overview

This document explains the field operations pattern used in Puroro's generated code,
separating shared state from exclusive field storage using a trait-based approach.

## Current Implementation: Trait-Based Field Operations

The current implementation uses a trait-based approach where field operations are
handled through the `Field` trait, eliminating the need for intermediate context types.

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
    
    fn set(&mut self, shared: &mut Self::SharedFields, value: Self::Value<'_>);
    fn get<'a>(&'a self, shared: &'a Self::SharedFields) -> Self::GetValue<'a>;
    fn clear(&mut self, shared: &mut Self::SharedFields);
    fn is_present(&self, shared: &Self::SharedFields) -> bool;
}
```

**SharedFields**: Wrapper for shared state
```rust
pub struct SharedFields<const BYTES: usize> {
    has_bits: BitArray<[u8; BYTES]>,
}
```

## Generated Code Pattern

### Current Implementation

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
```

### Benefits of Trait-Based Approach

1. **Type Safety**: Compile-time validation of field operations
2. **Single Line Operations**: Each field operation is one line
3. **Centralized Logic**: Field operations handled by trait implementations
4. **Extensible**: Easy to add new field types by implementing the Field trait
5. **No Context Types**: Eliminates need for intermediate wrapper types

## Memory Layout Optimization

### SharedFields with BitArray

The `SharedFields` struct uses `BitArray` for presence tracking:

```rust
pub struct SharedFields<const BYTES: usize> {
    has_bits: BitArray<[u8; BYTES]>,
}
```

**Benefits:**
- **Stack-allocated**: No heap overhead
- **Scalable**: Supports unlimited fields (⌈fields/8⌉ bytes)
- **Efficient**: Same performance as u32 for small messages
- **Type-safe**: Compiler enforces correct byte count

### Memory Impact

**Small message (3 fields):**
- BitArray approach: 1 byte (⌈3/8⌉ = 1)
- **No overhead compared to u32 approach**

**Large message (100 fields):**
- BitArray approach: 13 bytes (⌈100/8⌉ = 13)
- **Still efficient and scalable**

## Future Extensions

### Allocator Support

When allocator support is added, it will be stored inside `SharedFields`:

```rust
pub struct SharedFields<const BYTES: usize, A: Allocator = Global> {
    has_bits: BitArray<[u8; BYTES]>,
    allocator: A,  // Stored in message
}
```

**Key advantage**: Function signatures don't change when adding allocator support.

### Boolean Field Packing

Boolean fields can be packed into a separate BitArray:

```rust
pub struct SharedFields<const BYTES: usize, const BOOL_BYTES: usize> {
    has_bits: BitArray<[u8; BYTES]>,
    bool_bits: BitArray<[u8; BOOL_BYTES]>,  // Packed boolean values
}
```

### Unknown Fields

Unknown fields can be stored in SharedFields:

```rust
pub struct SharedFields<const BYTES: usize, A: Allocator = Global> {
    has_bits: BitArray<[u8; BYTES]>,
    allocator: A,
    unknown_fields: Vec<u8, A>,  // Raw wire format bytes
}
```

## Design Evolution

### From FieldContext to Trait-Based

**Previous approach (FieldContext):**
```rust
let ctx = FieldContext::new(&mut self._has_bits, HAS_NAME);
field::set_string(ctx, &mut self.name, v);
```

**Current approach (Trait-based):**
```rust
self.name.set(&mut self._shared, v);
```

**Benefits of evolution:**
- ✅ Simpler generated code (one line vs two)
- ✅ Better type safety (trait bounds)
- ✅ No intermediate types needed
- ✅ More idiomatic Rust

## Summary

The current trait-based field operations approach:
- ✅ **Type-safe**: Compile-time validation of all field operations
- ✅ **Simple**: Single-line operations in generated code
- ✅ **Efficient**: Stack-allocated SharedFields with BitArray
- ✅ **Scalable**: Supports unlimited fields with minimal overhead
- ✅ **Extensible**: Easy to add new field types and features
- ✅ **Maintainable**: Centralized logic in trait implementations
- ✅ **Future-proof**: SharedFields can be extended without changing function signatures

