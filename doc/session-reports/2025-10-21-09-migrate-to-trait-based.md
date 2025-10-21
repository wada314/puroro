# Migration to Trait-Based Field Operations

## Decision

Fully migrated to trait-based field operations. Removed old function-based approach.

## What Changed

### Deleted
- ❌ `puroro/src/field.rs` - Old function-based approach (set_scalar, set_string, etc.)
- ❌ Reference to field module in `puroro/src/lib.rs`

### Updated
- ✅ `sandbox/src/generated/person.rs` - Now uses trait-based operations
- ✅ `puroro/src/lib.rs` - Removed field module export
- ✅ `doc/field-operations-design.md` - Added note about current implementation
- ✅ `doc/field-ops-genericization.md` - Marked decision

## Final Design

### Type Descriptors
```rust
// Type aliases describe field attributes at type level
type NameField = Field<String, Singular>;
type AgeField = Field<i32, Singular>;
type HobbiesField = Field<String, Repeated>;
```

### Struct Definition
```rust
pub struct PersonImpl {
    _shared: SharedFields<1>,
    name: String,    // NameField documents this
    email: String,   // EmailField documents this
    age: i32,        // AgeField documents this
}
```

### Generated Code Pattern
```rust
impl PersonAppend for PersonImpl {
    fn set_name(&mut self, v: &str) {
        NameField::set(&mut self._shared, IDX_NAME, &mut self.name, v);
    }
}

impl Person for PersonImpl {
    fn name(&self) -> &str {
        NameField::get(&self.name)
    }
}

impl PersonMut for PersonImpl {
    fn clear_name(&mut self) {
        NameField::clear(&mut self._shared, IDX_NAME, &mut self.name);
    }
}
```

## Implementation in field_ops.rs

### Core Traits

**FieldSet** - For setting values
```rust
pub trait FieldSet {
    type Storage;
    type Value;
    fn set<const BYTES: usize>(...);
}
```

**FieldGet** - For getting values (with GATs)
```rust
pub trait FieldGet {
    type Storage;
    type Value<'a> where Self: 'a;  // GAT!
    fn get<'a>(storage: &'a Storage) -> Value<'a>;
}
```

**FieldClear** - For clearing values
```rust
pub trait FieldClear {
    type Storage;
    fn clear<const BYTES: usize>(...);
}
```

### Implementations

**Generic for all scalar types:**
```rust
impl<T: ScalarType> FieldSet for Field<T, Singular> { ... }
impl<T: ScalarType> FieldGet for Field<T, Singular> { ... }
impl<T: ScalarType> FieldClear for Field<T, Singular> { ... }

impl<T: ScalarType> FieldSet for Field<T, Repeated> { ... }
impl<T: ScalarType> FieldGet for Field<T, Repeated> { ... }
impl<T: ScalarType> FieldClear for Field<T, Repeated> { ... }
```

**Special handling for String (lifetime issues):**
```rust
impl Field<String, Singular> {
    // Inherent method instead of FieldSet trait
    pub fn set<const BYTES: usize>(..., value: &str) { ... }
    pub fn clear<const BYTES: usize>(...) { ... }
}

impl FieldGet for Field<String, Singular> {
    type Value<'a> = &'a str;  // GAT works here
    fn get<'a>(...) -> &'a str { ... }
}
```

## Benefits Realized

### Scalability
- ✅ Adding new scalar type: Add to `ScalarType` trait (1 line)
- ✅ Adding repeated variant: Already supported generically
- ✅ Adding map fields: Implement `Field<(K, V), Map>` (3 trait impls)
- ❌ Without traits: Would need 3+ functions per type

### Code Organization
- ✅ ~8 trait implementations instead of ~30 functions
- ✅ Clear type-level documentation
- ✅ Compiler enforces correctness

### Extensibility
- ✅ New field kinds: Add marker type + trait impls
- ✅ New value types: Add to ScalarType or specific impl
- ✅ Future features (bool packing): Add to SharedFields, update trait impls

## Migration Notes

### Lifetime Handling

**Problem:** `&str` has lifetime dependency on `&String`

**Solutions implemented:**
1. **Set operations:** Use inherent methods with explicit `&str` parameter
   ```rust
   impl Field<String, Singular> {
       pub fn set(..., value: &str) { ... }
   }
   ```

2. **Get operations:** Use GATs (Generic Associated Types)
   ```rust
   impl FieldGet for Field<String, Singular> {
       type Value<'a> = &'a str;
       fn get<'a>(...) -> &'a str { ... }
   }
   ```

This hybrid approach (inherent + trait) works well on stable Rust.

## Test Results

After migration:
- puroro field_ops: 5 tests ✅ (trait-based)
- puroro shared: 3 tests ✅
- sandbox basic: 17 tests ✅

**Total: 25 tests ✅** (vs 32 before, but field.rs tests removed)

**Memory: 56 bytes ✅**

## Files Deleted

- ❌ `puroro/src/field.rs` - Replaced by field_ops.rs

## Files Modified

- ✅ `puroro/src/lib.rs` - Removed field module
- ✅ `sandbox/src/generated/person.rs` - Uses trait-based operations
- ✅ `doc/field-operations-design.md` - Added implementation note
- ✅ `doc/field-ops-genericization.md` - Marked decision

## Next Steps

Extend field_ops.rs with:
1. ⏸️ Map fields: `Field<(K, V), Map>`
2. ⏸️ Oneof fields: `Field<OneofType, Oneof>`
3. ⏸️ Enum fields: `Field<EnumType, Singular>`
4. ⏸️ Nested messages: `Field<MessageType, Singular>`
5. ⏸️ Bytes fields: `Field<Vec<u8>, Singular>`

All can be added as trait implementations without function explosion!

## Conclusion

✅ Successfully migrated to trait-based approach  
✅ Function-based approach removed  
✅ All tests passing  
✅ Ready for future expansion  

The trait-based design scales much better and provides type safety.
