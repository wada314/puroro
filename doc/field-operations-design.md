# Field Operations Design

## Overview

This document explains the field operations pattern used in Puroro's generated code,
separating shared state from exclusive field storage.

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

## API Design Evolution

### Version 1: Direct Implementation (Initial)

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

### Version 2: SharedFields Pattern (Current)

```rust
impl PersonAppend for PersonImpl {
    #[inline]
    fn set_name(&mut self, v: &str) {
        // Pass shared + bit index + exclusive field directly
        field::set_string(&mut self._shared, IDX_NAME, &mut self.name, v);
    }
    
    #[inline]
    fn set_age(&mut self, v: i32) {
        // Single line - clean and direct
        field::set_scalar(&mut self._shared, IDX_AGE, &mut self.age, v);
    }
}
```

**Benefits:**
- Logic centralized in `puroro::field`
- **Single line per operation** (simplest possible)
- No intermediate wrapper types needed
- Adding features to `SharedFields`: just update the struct and library functions

### Library Code (puroro crate)

```rust
// puroro/src/field.rs

#[inline]
pub fn set_string<const BYTES: usize>(
    shared: &mut SharedFields<BYTES>,
    bit_index: usize,
    storage: &mut String,
    value: &str,
) {
    value.clone_into(storage);  // Reuse allocation
    shared.has_bits_mut().set(bit_index, true);
}

#[inline]
pub fn set_scalar<const BYTES: usize, T: Copy>(
    shared: &mut SharedFields<BYTES>,
    bit_index: usize,
    storage: &mut T,
    value: T,
) {
    *storage = value;
    shared.has_bits_mut().set(bit_index, true);
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

### Critical Design Principle: Shared State Lives Inside SharedFields

**All** shared state is stored **inside** `SharedFields`, not passed as function parameters.

This means:
- ✅ Allocator is stored IN the message
- ✅ Bool_bits is IN SharedFields
- ✅ Unknown_fields is IN SharedFields
- ✅ Function signatures remain stable

### Allocator Support (Future)

```rust
// Add allocator to SharedFields
pub struct SharedFields<const BYTES: usize, A: Allocator = Global> {
    has_bits: BitArray<[u8; BYTES]>,
    allocator: A,  // Stored in message!
}

// Function signature automatically adapts
pub fn set_string<const BYTES: usize, A: Allocator>(
    shared: &mut SharedFields<BYTES, A>,  // Compiler infers A
    bit_index: usize,
    storage: &mut String,  // Or allocator-aware String
    value: &str,
) {
    // Can use shared.allocator internally
    value.clone_into(storage);
    shared.has_bits_mut().set(bit_index, true);
}

// Generated code doesn't change!
field::set_string(&mut self._shared, IDX_NAME, &mut self.name, v);
```

**Key advantage:** No need to thread allocator through every function call.

### Boolean Packing (Future)

```rust
// Add bool_bits to SharedFields
pub struct SharedFields<const BYTES: usize, const BOOL_BYTES: usize> {
    has_bits: BitArray<[u8; BYTES]>,
    bool_bits: BitArray<[u8; BOOL_BYTES]>,  // Added
}

// New function for boolean fields
pub fn set_bool<const BYTES: usize, const BOOL_BYTES: usize>(
    shared: &mut SharedFields<BYTES, BOOL_BYTES>,
    bit_index: usize,
    value: bool,
) {
    shared.has_bits_mut().set(bit_index, true);
    shared.bool_bits_mut().set(bit_index, value);
}
```

### Unknown Fields (Future)

```rust
pub struct SharedFields<const BYTES: usize, A: Allocator = Global> {
    has_bits: BitArray<[u8; BYTES]>,
    allocator: A,
    unknown_fields: Vec<u8, A>,  // Using the same allocator
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

## BitArr for All Messages

### Design Decision: Use BitArr (Fixed-Size, Stack-Allocated)

We use `bitvec`'s `BitArr!` for **all** messages, with field count known at compile time.

**Rationale:**
- **Zero heap overhead**: Stack-allocated, same efficiency as u32
- **No artificial limits**: Supports any number of fields
- **Simplicity**: One implementation pattern
- **Best of both worlds**: u32 performance + unlimited fields

### Implementation

```rust
use puroro::{
    field,
    shared::SharedFields,
};

pub struct PersonImpl {
    // Shared fields wrapper: For 3 fields, ⌈3/8⌉ = 1 byte
    _shared: SharedFields<1>,
    // Exclusive fields
    name: String,
    age: i32,
}

impl PersonAppend for PersonImpl {
    fn set_name(&mut self, v: &str) {
        // Pass shared fields + bit index + exclusive field directly
        field::set_string(&mut self._shared, 0, &mut self.name, v);
    }
}
```

**Key insights:**
1. Generic const parameter is **BYTES** (not bits), so we can use `BitArray<[u8; BYTES]>` directly
2. No `FieldContext` needed - pass `SharedFields` directly for simplicity

### Memory Impact

**Small message (3 fields):**
- Old u32 approach: 56 bytes (u32: 4 bytes)
- **BitArr approach: 56 bytes** (BitArr!(for 3, in u8): 1 byte) ✅
- **No overhead!**

**Large message (100 fields):**
- Hypothetical u64 array: 8-16 bytes
- **BitArr approach: 13 bytes** (BitArr!(for 100, in u8): 13 bytes) ✅
- **Still efficient!**

**Perfect solution:**
- ✅ Same memory efficiency as u32 for small messages
- ✅ No field count limitations
- ✅ Stack-allocated (no heap overhead)
- ✅ Scales efficiently for large messages

## Summary

The Field Operations pattern with SharedFields:
- ✅ Separates shared state from exclusive storage
- ✅ Centralizes logic in library code  
- ✅ **Single-line operations** (simplest possible generated code)
- ✅ Reduces code size significantly
- ✅ Enables future optimizations (allocators, bool packing, etc.)
- ✅ Maintains full inlining for performance
- ✅ **No field count limitations** (BitArr supports unlimited fields)
- ✅ **Zero heap overhead** (stack-allocated, fixed size)
- ✅ **Best of both worlds** (u32 efficiency + unlimited fields)
- ✅ **No wrapper types** (SharedFields passed directly)
- ✅ **Simple implementation** (one pattern for all message sizes)

