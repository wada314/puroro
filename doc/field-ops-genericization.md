# Field Operations Genericization

## Problem

As we add more protobuf features, `field.rs` will have function explosion:

```rust
// Scalar types
set_i32, set_i64, set_u32, set_u64, set_f32, set_f64, set_bool, set_string, set_bytes

// Repeated fields
add_i32, add_string, add_message, ...
get_repeated_i32, get_repeated_string, ...
clear_repeated_i32, ...

// Map fields
insert_map_string_i32, insert_map_i32_string, ...
get_map_string_i32, ...

// Nested messages
set_message, get_message, has_message, clear_message, mut_message

// Oneofs
set_oneof_variant_a, set_oneof_variant_b, ...
which_oneof, ...

// Enums
set_enum, get_enum, ...
```

This is **unsustainable**:
- ❌ Too many functions
- ❌ Naming confusion
- ❌ Hard to maintain
- ❌ Difficult to discover the right function

## Solution: Generic Type-Based Dispatch

### Core Idea

Define a single generic type that encodes:
1. Field value type (i32, String, Vec<T>, etc.)
2. Field attributes (singular, repeated, map, oneof)
3. Other metadata

Then use trait implementations or type-level dispatch to select the right behavior.

## Approach 1: Trait-Based with Sealed Trait (DEPRECATED)

**Note**: This approach was initially implemented but later replaced with separate traits due to lifetime issues.

```rust
// Core trait for field operations (REMOVED - had lifetime issues)
pub trait FieldOps: private::Sealed {
    type Value;  // ← Cannot express lifetimes like &str
    type Storage;
    
    fn set<const BYTES: usize>(...);
    fn get(storage: &Self::Storage) -> Self::Value;  // ← Cannot return &str
    fn clear<const BYTES: usize>(...);
}
```

**Problem**: Cannot express lifetime relationships (e.g., `&str` from `&String`).

**Solution**: Split into separate traits with GATs (Generic Associated Types).

## Current Implementation: Split Traits with GATs

**Final Design**: Separate traits for each operation to handle lifetime issues.

```rust
// Separate traits for each operation
pub trait FieldSet {
    type Storage;
    type Value;
    fn set<const BYTES: usize>(...);
}

pub trait FieldGet {
    type Storage;
    type Value<'a> where Self: 'a;  // ← GAT for lifetime support
    fn get<'a>(storage: &'a Self::Storage) -> Self::Value<'a>;
}

pub trait FieldClear {
    type Storage;
    fn clear<const BYTES: usize>(...);
}

// Implementation for String fields with lifetime support
impl FieldGet for Field<String, ImplicitOptional> {
    type Storage = String;
    type Value<'a> = &'a str;  // ← Can return &str with proper lifetime
    
    fn get<'a>(storage: &'a Self::Storage) -> Self::Value<'a> {
        storage.as_str()  // ← Returns &str
    }
}
```

## Decision

**Chosen**: Split Traits with GATs (Current Implementation)

**Reasons**:
1. ✅ **Lifetime Support**: Can return `&str` from `&String`
2. ✅ **Type Safety**: Compiler enforces correct usage
3. ✅ **Extensibility**: Easy to add new field types
4. ✅ **Generic**: Works with any field type combination
5. ✅ **Stable Rust**: Uses stable GAT feature

**Trade-offs**:
- ⚠️ More complex than single trait approach
- ⚠️ Requires understanding of GATs
- ⚠️ Generated code is more verbose

## Implementation Status

- ✅ Core trait definitions (FieldSet, FieldGet, FieldClear)
- ✅ Scalar types (i32, i64, u32, u64, f32, f64, bool)
- ✅ String fields with lifetime support
- ✅ Repeated fields
- ✅ ImplicitOptional and ExplicitOptional presence semantics
- ⏸️ Map fields
- ⏸️ Oneof fields
- ⏸️ Nested messages
- ⏸️ Enum fields

## Generated Code Example

```rust
// Type aliases for field descriptors
type NameField = Field<String, ImplicitOptional>;
type AgeField = Field<i32, ImplicitOptional>;
type HobbiesField = Field<String, Repeated>;

// Generated struct
pub struct PersonImpl {
    _shared: SharedFields<1>,
    name: String,    // <NameField as FieldSet>::Storage
    age: i32,        // <AgeField as FieldSet>::Storage
    hobbies: Vec<String>, // <HobbiesField as FieldSet>::Storage
}

// Generated implementation
impl PersonAppend for PersonImpl {
    fn set_name(&mut self, v: &str) {
        NameField::set(&mut self._shared, IDX_NAME, &mut self.name, v);
    }
    
    fn set_age(&mut self, v: i32) {
        AgeField::set(&mut self._shared, IDX_AGE, &mut self.age, v);
    }
}

impl Person for PersonImpl {
    fn name(&self) -> &str {
        NameField::get(&self.name)  // Returns &str
    }
    
    fn age(&self) -> i32 {
        AgeField::get(&self.age)  // Returns i32
    }
}
```

## Approach 2: Generic Functions with Type Markers

```rust
// Zero-sized marker types
pub struct Singular;
pub struct Repeated;
pub struct Map;

// Generic set function
pub fn set<const BYTES: usize, T, K>(
    shared: &mut SharedFields<BYTES>,
    bit_index: usize,
    storage: &mut T::Storage,
    value: T::Value,
) where
    T: FieldKind<K>,  // K is Singular/Repeated/Map
{
    T::set_impl(shared, bit_index, storage, value);
}

// Trait for field kinds
pub trait FieldKind<K> {
    type Storage;
    type Value;
    
    fn set_impl<const BYTES: usize>(
        shared: &mut SharedFields<BYTES>,
        bit_index: usize,
        storage: &mut Self::Storage,
        value: Self::Value,
    );
}

// Implementations
impl FieldKind<Singular> for String {
    type Storage = String;
    type Value = &str;  // Hmm, lifetime issues?
}

impl<T: FieldKind<Singular>> FieldKind<Repeated> for Vec<T> {
    type Storage = Vec<T::Storage>;
    type Value = T::Value;
}

// Generated code
field::set::<StringField, Singular>(&mut self._shared, IDX, &mut self.name, v);
field::set::<StringField, Repeated>(&mut self._shared, IDX, &mut self.hobbies, v);
```

**Issues:**
- ⚠️ Lifetime problems with `type Value = &str`
- ⚠️ Still requires type annotations
- ⚠️ Not much simpler than Approach 1

## Approach 3: Specialization (Requires Nightly)

```rust
#![feature(specialization)]

// Generic default implementation
pub fn set<T>(shared: &mut SharedFields, bit_index: usize, storage: &mut T, value: T) {
    // Default: just assign
    *storage = value;
    shared.has_bits_mut().set(bit_index, true);
}

// Specialized for String
impl SpecializedSet for String {
    fn set(shared: &mut SharedFields, bit_index: usize, storage: &mut String, value: &str) {
        value.clone_into(storage);  // clone_into instead of assignment
        shared.has_bits_mut().set(bit_index, true);
    }
}

// Specialized for Vec<T>
impl<T> SpecializedSet for Vec<T> {
    fn set(shared: &mut SharedFields, bit_index: usize, storage: &mut Vec<T>, value: T) {
        storage.push(value);  // push instead of assignment
        shared.has_bits_mut().set(bit_index, true);
    }
}
```

**Pros:**
- ✅ Clean syntax in generated code
- ✅ Automatic dispatch based on type

**Cons:**
- ❌ Requires nightly (specialization is unstable)
- ❌ We want to support stable Rust

## Approach 4: Enum Dispatch

```rust
// Define operation kinds
pub enum FieldOp {
    SetScalar,
    SetString,
    AddRepeated,
    InsertMap,
    // ...
}

// Single function with operation parameter
pub fn field_op<T>(
    op: FieldOp,
    shared: &mut SharedFields,
    bit_index: usize,
    storage: &mut T,
    value: ...,  // Problem: different types per operation
) {
    match op {
        FieldOp::SetScalar => ...
        FieldOp::SetString => ...
        // ...
    }
}
```

**Issues:**
- ❌ Value type differs per operation (can't be generic)
- ❌ Runtime dispatch overhead
- ❌ Type safety lost

## Approach 5: Trait Object (Dynamic Dispatch)

```rust
pub trait FieldValue {
    fn set(&self, shared: &mut SharedFields, bit_index: usize, storage: &mut dyn Any);
}

impl FieldValue for &str {
    fn set(&self, shared: &mut SharedFields, bit_index: usize, storage: &mut dyn Any) {
        let storage = storage.downcast_mut::<String>().unwrap();
        self.clone_into(storage);
        shared.has_bits_mut().set(bit_index, true);
    }
}
```

**Issues:**
- ❌ Runtime overhead (vtable, downcast)
- ❌ Unsafe or unwrap() needed
- ❌ Not zero-cost

## Approach 6: Keep Separate Functions (Current)

```rust
// Simple, explicit functions
pub fn set_string(...) { ... }
pub fn set_scalar<T: Copy>(...) { ... }
pub fn add_repeated<T>(...) { ... }
pub fn insert_map<K, V>(...) { ... }

// Generated code is clear
field::set_string(&mut self._shared, IDX, &mut self.name, v);
field::add_repeated(&mut self._shared, IDX, &mut self.hobbies, v);
```

**Pros:**
- ✅ Simple and explicit
- ✅ Works on stable Rust
- ✅ No complex type system
- ✅ Clear what each function does

**Cons:**
- ⚠️ Function count grows
- ⚠️ Some naming overlap possible

### Mitigation: Organized Naming

```rust
// Singular fields
pub fn set_scalar<T: Copy>(...) { ... }
pub fn set_string(...) { ... }
pub fn set_bytes(...) { ... }
pub fn set_message<T: Message>(...) { ... }
pub fn set_enum(...) { ... }

// Repeated fields  
pub fn repeated_add<T>(...) { ... }
pub fn repeated_get(...) { ... }
pub fn repeated_clear(...) { ... }
pub fn repeated_len(...) { ... }

// Map fields
pub fn map_insert<K, V>(...) { ... }
pub fn map_get<K, V>(...) { ... }
pub fn map_remove<K>(...) { ... }

// Oneof fields
pub fn oneof_set_variant<T>(...) { ... }
pub fn oneof_which(...) { ... }
pub fn oneof_clear(...) { ... }
```

## My Analysis

### Current Reality

For protobuf, we need to support:
- ~10 scalar types (i32, i64, u32, u64, f32, f64, bool, string, bytes, message)
- Repeated fields (for each type)
- Map fields (various key/value combinations)
- Oneof fields
- Enums

This could result in **~50-100 functions** in the worst case.

### Best Practical Solution: Hybrid Approach

**For Stable Rust:**

1. **Use generics where possible:**
   ```rust
   pub fn set_scalar<const BYTES: usize, T: Copy>(...) { ... }
   // Covers: i32, i64, u32, u64, f32, f64, bool
   ```

2. **Separate functions for complex types:**
   ```rust
   pub fn set_string<const BYTES: usize>(...) { ... }
   pub fn set_bytes<const BYTES: usize>(...) { ... }
   pub fn set_message<const BYTES: usize, T: Message>(...) { ... }
   ```

3. **Namespace by prefix:**
   ```rust
   pub fn repeated_add<const BYTES: usize, T>(...) { ... }
   pub fn map_insert<const BYTES: usize, K, V>(...) { ... }
   ```

**Total estimate:** ~20-30 well-organized functions (manageable)

### Future with Specialization

When specialization stabilizes, we could refactor to:
```rust
pub fn set<T>(...) { ... }  // Generic with specialized impls
```

But for now, explicit functions are the pragmatic choice.

## Decision (2025-10-21)

**DECIDED: Use trait-based approach with Field<T, K> type descriptors.**

Implemented in `puroro/src/field_ops.rs`.

## Recommendation (Original - Now Superseded)

**For now: Keep separate, well-named functions (Approach 6).**

Reasons:
1. ✅ Works on stable Rust
2. ✅ Clear and explicit
3. ✅ ~20-30 functions is manageable
4. ✅ Good IDE support (autocomplete)
5. ✅ Easy to understand

**Mitigation strategies:**
- Use generic functions where type system allows (set_scalar<T>)
- Clear naming prefixes (repeated_*, map_*, oneof_*)
- Good module documentation
- Consider sub-modules if it grows too large (field::repeated, field::map)

**Future migration path:**
- When specialization stabilizes, refactor to unified generic API
- Current explicit functions can be kept as compatibility layer

## Open Questions

1. Should we pre-emptively add sub-modules? (field::singular, field::repeated, field::map)
2. How many generic parameters is too many? (set_map<BYTES, K, V> seems okay)
3. Should we explore trait-based approach more deeply?

Would you like me to prototype a trait-based approach to see if it's cleaner than separate functions?

