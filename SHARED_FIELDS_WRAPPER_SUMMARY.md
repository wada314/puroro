# SharedFields Wrapper - Final Implementation

## Achievement

✅ **Perfect solution: SharedFields wrapper with BYTES parameter works on stable Rust!**

## Final Design

### Key Insight

Use **BYTES** (not BITS) as the generic const parameter:
- ✅ No const expressions like `(N + 7) / 8` needed
- ✅ Works on stable Rust (no `#![feature(generic_const_exprs)]`)
- ✅ `BitArray<[u8; BYTES]>` can be used directly

```rust
pub struct SharedFields<const BYTES: usize> {
    has_bits: BitArray<[u8; BYTES]>,  // ✅ Works!
    // Not: BitArray<[u8; (N + 7) / 8]>  // ❌ Requires nightly
}
```

## Implementation

### Core Library (`puroro/src/shared.rs`)

```rust
pub struct SharedFields<const BYTES: usize> {
    has_bits: BitArray<[u8; BYTES]>,
    
    // Future fields:
    // bool_bits: BitArray<[u8; BOOL_BYTES]>,
    // allocator: A,
    // _unknown_fields: Vec<u8, A>,
}

impl<const BYTES: usize> SharedFields<BYTES> {
    pub fn new() -> Self {
        Self {
            has_bits: BitArray::ZERO,
        }
    }
    
    pub fn has_bits_mut(&mut self) -> &mut BitSlice<u8, Lsb0> {
        self.has_bits.as_mut_bitslice()
    }
    
    pub fn has_bits(&self) -> &BitSlice<u8, Lsb0> {
        self.has_bits.as_bitslice()
    }
}
```

### Generated Code (`PersonImpl`)

```rust
use puroro::{
    field::{self, FieldContext},
    shared::SharedFields,
};

pub struct PersonImpl {
    // Shared fields: For 3 fields, ⌈3/8⌉ = 1 byte
    _shared: SharedFields<1>,
    
    // Exclusive fields
    name: String,
    email: String,
    age: i32,
}

impl PersonAppend for PersonImpl {
    fn set_name(&mut self, v: &str) {
        let ctx = FieldContext::new(self._shared.has_bits_mut(), IDX_NAME);
        field::set_string(ctx, &mut self.name, v);
    }
}
```

## Benefits

### 1. Clean Separation

**Shared fields (in wrapper):**
- `has_bits` - Presence tracking
- Future: `bool_bits`, `allocator`, `_unknown_fields`

**Exclusive fields (direct members):**
- `name`, `email`, `age` - 1:1 with proto fields

### 2. Extensibility

Adding new shared fields is easy:

```rust
pub struct SharedFields<const BYTES: usize, const BOOL_BYTES: usize> {
    has_bits: BitArray<[u8; BYTES]>,
    bool_bits: BitArray<[u8; BOOL_BYTES]>,  // Just add here!
}
```

Generated code barely changes:
```rust
_shared: SharedFields<1, 1>,  // 1 byte for presence, 1 byte for bools
```

### 3. Type Safety

```rust
// Compiler enforces correct byte count
let s1 = SharedFields::<1>::new();  // For ≤8 fields
let s2 = SharedFields::<2>::new();  // For 9-16 fields
let s3 = SharedFields::<13>::new(); // For 97-104 fields
```

### 4. Code Generator Simplicity

```rust
// Pseudo-code for code generator
let field_count = message.fields.len();
let bytes_needed = (field_count + 7) / 8;  // Round up division

generate_struct! {
    _shared: SharedFields<{bytes_needed}>,
    // ... exclusive fields
}
```

## Memory Layout

**PersonImpl structure:**
```
Offset | Field        | Size   | Notes
-------|--------------|--------|---------------------------
0      | name         | 24 B   | String (ptr, len, cap)
24     | email        | 24 B   | String (ptr, len, cap)
48     | _shared      | 1 B    | SharedFields<1>
49     | (padding)    | 3 B    | Align to 4 bytes
52     | age          | 4 B    | i32
-------|--------------|--------|---------------------------
Total: 56 bytes (same as u32 approach!)
```

**SharedFields internal:**
```
Offset | Field        | Size   | Notes
-------|--------------|--------|---------------------------
0      | has_bits     | 1 B    | BitArray<[u8; 1]>
-------|--------------|--------|---------------------------
Total: 1 byte
```

## Comparison: All Approaches

| Approach | Stable? | Memory | Field Limit | Shared Wrapper | Complexity |
|----------|---------|--------|-------------|----------------|------------|
| u32 direct | ✅ | 56 B | 32 | No | Low |
| BitVec | ✅ | 80 B | ∞ | No | Low |
| BitArr direct | ✅ | 56 B | ∞ | No | Low |
| **SharedFields<BYTES>** | ✅ | **56 B** | **∞** | **Yes** ✅ | **Low** ✅ |

**SharedFields wins:** Same efficiency + extensibility + encapsulation

## Code Generator Calculation

```rust
// Helper function for code generator
fn calculate_bytes_needed(field_count: usize) -> usize {
    (field_count + 7) / 8  // Ceiling division
}

// Examples:
// 1-8 fields   → 1 byte
// 9-16 fields  → 2 bytes
// 17-24 fields → 3 bytes
// 97-104 fields → 13 bytes
```

## Test Results

All tests passing:
- puroro field tests: 8 tests ✅
- puroro shared tests: 3 tests ✅
- sandbox basic tests: 17 tests ✅

**Total: 28 tests ✅**

**Memory confirmed: 56 bytes (same as u32!) ✅**

## Files Modified

### Core Library
- ✅ `puroro/src/shared.rs` - SharedFields wrapper with BYTES parameter
- ✅ `puroro/src/lib.rs` - Export shared module

### Sample Code
- ✅ `sandbox/src/generated/person.rs` - Use SharedFields<1>
- ✅ Removed `use bitvec::prelude::*;` (no longer needed in generated code)

### Documentation
- ✅ `doc/field-context-design.md` - Updated with SharedFields pattern
- ✅ `SHARED_FIELDS_WRAPPER_SUMMARY.md` - This document

## Conclusion

**Perfect encapsulation achieved:**

1. ✅ **Shared fields properly wrapped** in `SharedFields`
2. ✅ **Works on stable Rust** (BYTES parameter trick)
3. ✅ **Same memory efficiency** as u32 approach
4. ✅ **Unlimited fields** like BitVec
5. ✅ **Easy to extend** (add bool_bits, allocator, etc.)
6. ✅ **Simple code generation** (calculate bytes, use wrapper)

This is the ideal solution for generated protobuf code! ���
