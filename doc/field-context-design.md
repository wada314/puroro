# Field Context Design

## Overview

This document explains the "Field Context" pattern used in Puroro's generated code.

## Motivation

Generated protobuf code contains repetitive logic for:
- Setting field values
- Tracking presence (has_bits)
- Clearing fields
- Future: Boolean value packing, allocator handling, etc.

By separating **shared state** from **exclusive field storage**, we can:
1. **Improve generated code readability** - Logic moves to library code
2. **Reduce code size** - Common logic in one place
3. **Improve maintainability** - Bug fixes in one place
4. **Enable future optimizations** - Easy to add allocator support, bool packing, etc.

## Terminology

### Shared Fields

Fields that are shared across all message fields:
- `_has_bits: u32` - Presence tracking
- `_bool_bits: u32` - Packed boolean values (future)
- Allocator reference (future)
- Unknown fields storage (future)

### Exclusive Fields

Fields that correspond 1:1 to proto message fields:
- `name: String` - For proto `string name = 1`
- `age: i32` - For proto `int32 age = 2`
- `email: String` - For proto `string email = 3`

Note: Boolean proto fields will NOT create exclusive fields - they'll be packed into `_bool_bits`.

## Implementation Pattern

### Generated Code (Before - Direct Implementation)

```rust
impl PersonAppend for PersonImpl {
    #[inline]
    fn set_name(&mut self, v: &str) {
        v.clone_into(&mut self.name);  // Reuse allocation
        self._has_bits |= HAS_NAME;
    }
    
    #[inline]
    fn set_age(&mut self, v: i32) {
        self.age = v;
        self._has_bits |= HAS_AGE;
    }
}
```

**Issues:**
- Logic repeated for each field
- Hard to add features (allocator, validation, etc.)
- Code size grows linearly with field count

### Generated Code (After - Field Context Pattern)

```rust
impl PersonAppend for PersonImpl {
    #[inline]
    fn set_name(&mut self, v: &str) {
        // Pair: shared state + exclusive field
        let ctx = FieldContext::new(&mut self._has_bits, HAS_NAME);
        field::set_string(ctx, &mut self.name, v);
    }
    
    #[inline]
    fn set_age(&mut self, v: i32) {
        // Pair: shared state + exclusive field  
        let ctx = FieldContext::new(&mut self._has_bits, HAS_AGE);
        field::set_scalar(ctx, &mut self.age, v);
    }
}
```

**Benefits:**
- Logic centralized in `puroro::field`
- Easy to read: "create context + call library function"
- Adding allocator support: just update `FieldContext` and library functions

### Library Code (puroro crate)

```rust
// puroro/src/field.rs

pub struct FieldContext<'a> {
    pub has_bits: &'a mut u32,
    pub has_bit_mask: u32,
}

#[inline]
pub fn set_string(mut ctx: FieldContext, storage: &mut String, value: &str) {
    value.clone_into(storage);  // Reuse allocation
    ctx.mark_set();
}

#[inline]
pub fn set_scalar<T: Copy>(mut ctx: FieldContext, storage: &mut T, value: T) {
    *storage = value;
    ctx.mark_set();
}
```

## Boolean Field Optimization

Boolean fields are memory-inefficient as struct fields:

```rust
// ❌ Without optimization: 1 byte + 3 padding = 4 bytes per bool
struct Message {
    is_active: bool,    // 4 bytes (with padding)
    is_verified: bool,  // 4 bytes (with padding)
    is_admin: bool,     // 4 bytes (with padding)
    // Total: 12 bytes for 3 bools
}
```

Instead, we pack them into a bit array:

```rust
// ✅ With optimization: 4 bytes for up to 32 bools
struct Message {
    _bool_bits: u32,  // 4 bytes for ALL boolean values!
    // No exclusive fields for booleans
}

// Library functions
#[inline]
pub fn set_bool_packed(
    has_bits: &mut u32,
    bool_bits: &mut u32, 
    has_mask: u32,
    bool_mask: u32,
    value: bool,
) {
    *has_bits |= has_mask;  // Mark as set
    if value {
        *bool_bits |= bool_mask;
    } else {
        *bool_bits &= !bool_mask;
    }
}
```

## Future Extensions

### Allocator Support

```rust
pub struct FieldContext<'a, A: Allocator = Global> {
    pub has_bits: &'a mut u32,
    pub has_bit_mask: u32,
    pub allocator: &'a A,  // Added
}

// Usage in generated code stays almost the same
let ctx = FieldContext::new(&mut self._has_bits, HAS_NAME, &self.allocator);
field::set_string(ctx, &mut self.name, v);
```

### Unknown Fields

```rust
pub struct FieldContext<'a> {
    pub has_bits: &'a mut u32,
    pub has_bit_mask: u32,
    pub unknown_fields: &'a mut Vec<u8>,  // Added
}
```

## Design Decisions

### Why not a mega-generic function?

We could create one ultra-generic function:

```rust
pub fn set_field<T, const IS_STRING: bool, const IS_REPEATED: bool, ...>(
    ctx: FieldContext,
    storage: &mut T,
    value: impl Into<T>,
) { ... }
```

**Decision: Start simple, add generics only when patterns emerge**

Current approach:
- `set_scalar<T>` - For Copy types
- `set_string` - For string fields
- `set_bytes` - For bytes fields  
- `set_bool_packed` - For boolean fields
- (Future) `add_repeated<T>` - For repeated fields
- (Future) `set_message<T>` - For nested messages

### Why direct access for read-only?

For `has_*()` methods:

```rust
// Option A: Use FieldContext (complex)
fn has_name(&self) -> bool {
    let ctx = FieldContext { has_bits: ???, ... };  // Can't create &mut from &self
    field::has_string(&ctx)
}

// Option B: Direct access (chosen)
fn has_name(&self) -> bool {
    (self._has_bits & HAS_NAME) != 0
}
```

**Decision: Direct access for read-only operations**

Creating `FieldContext` requires `&mut`, which we don't have in `&self` methods. Direct bit checking is clearer and more efficient.

### Naming: Why "Context"?

Alternatives considered:
- `FieldMetadata` - Not accurate (allocator isn't metadata)
- `FieldState` - Not accurate (allocator isn't state)
- `FieldShared` - Accurate but less idiomatic
- `FieldContext` - **Chosen** - Provides "context" for field operations

## Code Size Impact

Example with 100 fields:

**Before (direct implementation):**
```
100 fields × ~50 lines each = 5000 lines of repetitive code
```

**After (field context):**
```
100 fields × ~3 lines each = 300 lines
+ Library code in puroro crate = ~200 lines
Total: 500 lines (10x reduction)
```

## Summary

The Field Context pattern:
- ✅ Separates shared state from exclusive storage
- ✅ Centralizes logic in library code  
- ✅ Improves generated code readability
- ✅ Reduces code size significantly
- ✅ Enables future optimizations (allocators, bool packing, etc.)
- ✅ Maintains full inlining for performance

