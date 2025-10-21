# SharedFields Extensibility Design

## Critical Design Principle

**All shared state lives INSIDE `SharedFields`, not as function parameters.**

This is the fundamental design principle that makes our field operations extensible.

## Why This Matters

### Bad Approach: Parameters Everywhere
```rust
// ❌ Bad: Passing shared state as parameters
pub fn set_string(
    has_bits: &mut BitSlice,
    bit_index: usize,
    allocator: &Allocator,        // ← Parameter
    bool_bits: &mut BitSlice,     // ← Parameter
    unknown_fields: &mut Vec<u8>, // ← Parameter
    storage: &mut String,
    value: &str,
) { ... }

// Generated code becomes a mess
field::set_string(
    self._has_bits.as_mut(),
    IDX_NAME,
    &self.allocator,        // Threading through
    self._bool_bits.as_mut(), // Threading through
    &mut self._unknown_fields, // Threading through
    &mut self.name,
    v
);
```

**Problems:**
- Every new shared field adds a parameter
- Every function signature changes
- All generated code must be regenerated
- Error-prone and verbose

### Good Approach: Encapsulated in SharedFields
```rust
// ✅ Good: Shared state lives inside SharedFields
pub struct SharedFields<const BYTES: usize, A: Allocator = Global> {
    has_bits: BitArray<[u8; BYTES]>,
    allocator: A,                    // Stored here
    bool_bits: BitArray<...>,        // Stored here
    unknown_fields: Vec<u8, A>,      // Stored here
}

pub fn set_string<const BYTES: usize, A: Allocator>(
    shared: &mut SharedFields<BYTES, A>,  // One parameter
    bit_index: usize,
    storage: &mut String,
    value: &str,
) {
    // Access shared.allocator, shared.bool_bits, etc. internally
    value.clone_into(storage);
    shared.has_bits_mut().set(bit_index, true);
}

// Generated code stays clean - just one line!
field::set_string(&mut self._shared, IDX_NAME, &mut self.name, v);
```

**Benefits:**
- Adding new shared fields = just update SharedFields struct
- Function signatures **automatically** adapt via generics
- Generated code **never** needs to change
- Clean and simple

## Extensibility Examples

### Adding Allocator Support

**Step 1:** Update SharedFields
```rust
pub struct SharedFields<const BYTES: usize, A: Allocator = Global> {
    has_bits: BitArray<[u8; BYTES]>,
    allocator: A,  // ← Added
}
```

**Step 2:** Field functions automatically pick it up
```rust
// Generic params automatically updated
pub fn set_string<const BYTES: usize, A: Allocator>(
    shared: &mut SharedFields<BYTES, A>,
    // ... rest unchanged
) {
    // Can now use shared.allocator
}
```

**Step 3:** Generated code doesn't change
```rust
// Still the same!
field::set_string(&mut self._shared, IDX_NAME, &mut self.name, v);
```

### Adding Boolean Packing

**Step 1:** Update SharedFields
```rust
pub struct SharedFields<const BYTES: usize, const BOOL_BYTES: usize> {
    has_bits: BitArray<[u8; BYTES]>,
    bool_bits: BitArray<[u8; BOOL_BYTES]>,  // ← Added
}
```

**Step 2:** Add new function
```rust
pub fn set_bool<const BYTES: usize, const BOOL_BYTES: usize>(
    shared: &mut SharedFields<BYTES, BOOL_BYTES>,
    bit_index: usize,
    value: bool,
) {
    shared.has_bits_mut().set(bit_index, true);
    shared.bool_bits_mut().set(bit_index, value);
}
```

**Step 3:** Generated code for booleans
```rust
// New pattern for bool fields
field::set_bool(&mut self._shared, IDX_IS_ACTIVE, true);
```

String fields still use the same pattern - no changes needed!

### Combining Both (Allocator + Bool Packing)

```rust
pub struct SharedFields<const BYTES: usize, const BOOL_BYTES: usize, A: Allocator = Global> {
    has_bits: BitArray<[u8; BYTES]>,
    bool_bits: BitArray<[u8; BOOL_BYTES]>,
    allocator: A,
    unknown_fields: Vec<u8, A>,  // Uses allocator
}

// Functions automatically adapt
pub fn set_string<const BYTES: usize, const BOOL_BYTES: usize, A: Allocator>(
    shared: &mut SharedFields<BYTES, BOOL_BYTES, A>,
    bit_index: usize,
    storage: &mut String,
    value: &str,
) { ... }

pub fn set_bool<const BYTES: usize, const BOOL_BYTES: usize, A: Allocator>(
    shared: &mut SharedFields<BYTES, BOOL_BYTES, A>,
    bit_index: usize,
    value: bool,
) { ... }

// Generated code stays simple
field::set_string(&mut self._shared, IDX_NAME, &mut self.name, v);
field::set_bool(&mut self._shared, IDX_IS_ACTIVE, true);
```

## Message Struct Evolution

### Current (Phase 1)
```rust
pub struct PersonImpl {
    _shared: SharedFields<1>,  // Just has_bits
    name: String,
    age: i32,
}
```

### With Allocator (Phase 2)
```rust
pub struct PersonImpl<A: Allocator = Global> {
    _shared: SharedFields<1, A>,  // has_bits + allocator
    name: String,  // Or Vec<u8, A> for allocator-aware
    age: i32,
}
```

### With Bool Packing (Phase 3)
```rust
pub struct PersonImpl {
    _shared: SharedFields<1, 1>,  // has_bits + bool_bits
    name: String,
    age: i32,
    // No exclusive field for: is_active, is_verified (stored in bool_bits)
}
```

### Full Featured (Phase 4)
```rust
pub struct PersonImpl<A: Allocator = Global> {
    _shared: SharedFields<1, 1, A>,  // has_bits + bool_bits + allocator
    name: String,
    age: i32,
}
```

## Key Takeaway

**SharedFields is the extensibility point.**

- All shared state goes inside SharedFields
- Field operation functions receive SharedFields as a single parameter
- Adding new shared features = update SharedFields + function implementations
- Generated code remains stable

This design makes future enhancements **trivial** instead of requiring
massive refactoring of function signatures and generated code.

## Comparison with Alternative Approaches

### Approach 1: Individual Parameters (❌ Bad)
```rust
fn set_field(..., has_bits, allocator, bool_bits, unknown_fields, ...)
// Adding one more shared field = update ALL functions + ALL generated code
```

### Approach 2: FieldContext Wrapper (❌ Unnecessary)
```rust
struct FieldContext { has_bits, bit_index }
fn set_field(ctx: FieldContext, ...)
// Extra wrapper type when SharedFields already exists
```

### Approach 3: SharedFields (✅ Optimal)
```rust
fn set_field(shared: &mut SharedFields, bit_index, ...)
// All shared state inside SharedFields
// Functions automatically adapt via generics
// Generated code never changes
```

**Approach 3 is the winner:** Simplicity + Extensibility + Stability

