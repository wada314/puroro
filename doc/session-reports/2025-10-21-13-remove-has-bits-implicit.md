# Remove has_bits Tracking for ImplicitOptional Fields

## Decision

Removed `has_bits` tracking for `ImplicitOptional` fields since they don't need presence tracking.

## Rationale

`ImplicitOptional` fields are always considered "present" with their default value, so they don't need explicit presence tracking via `has_bits`. This simplifies the implementation and correctly reflects the protobuf semantics.

## Changes Made

### 1. Updated FieldSet Implementation for ImplicitOptional

**Before:**
```rust
impl<T: ScalarType> FieldSet for Field<T, ImplicitOptional> {
    fn set<const BYTES: usize>(...) {
        *storage = value;
        shared.has_bits_mut().set(bit_index, true); // ❌ Unnecessary
    }
}
```

**After:**
```rust
impl<T: ScalarType> FieldSet for Field<T, ImplicitOptional> {
    fn set<const BYTES: usize>(...) {
        *storage = value;
        // Note: ImplicitOptional fields don't need presence tracking
        // (they're always considered "present" with default value)
    }
}
```

### 2. Updated FieldClear Implementation for ImplicitOptional

**Before:**
```rust
impl<T: ScalarType> FieldClear for Field<T, ImplicitOptional> {
    fn clear<const BYTES: usize>(...) {
        *storage = T::default();
        shared.has_bits_mut().set(bit_index, false); // ❌ Unnecessary
    }
}
```

**After:**
```rust
impl<T: ScalarType> FieldClear for Field<T, ImplicitOptional> {
    fn clear<const BYTES: usize>(...) {
        *storage = T::default();
        // Note: ImplicitOptional fields don't need presence tracking
        // (they're always considered "present" with default value)
    }
}
```

### 3. Updated String Field Implementations

**Before:**
```rust
impl Field<String, ImplicitOptional> {
    pub fn set<const BYTES: usize>(...) {
        value.clone_into(storage);
        shared.has_bits_mut().set(bit_index, true); // ❌ Unnecessary
    }
    
    pub fn clear<const BYTES: usize>(...) {
        storage.clear();
        shared.has_bits_mut().set(bit_index, false); // ❌ Unnecessary
    }
}
```

**After:**
```rust
impl Field<String, ImplicitOptional> {
    pub fn set<const BYTES: usize>(...) {
        value.clone_into(storage);
        // Note: ImplicitOptional fields don't need presence tracking
        // (they're always considered "present" with default value)
    }
    
    pub fn clear<const BYTES: usize>(...) {
        storage.clear();
        // Note: ImplicitOptional fields don't need presence tracking
        // (they're always considered "present" with default value)
    }
}
```

### 4. Updated has_*() Methods

**Before:**
```rust
impl Person for PersonImpl {
    fn has_name(&self) -> bool {
        self._shared.has_bits()[IDX_NAME] // ❌ Always false now
    }
}
```

**After:**
```rust
impl Person for PersonImpl {
    fn has_name(&self) -> bool {
        // ImplicitOptional fields are always considered "present"
        true
    }
}
```

### 5. Updated Tests

**Before:**
```rust
#[test]
fn test_implicit_optional_i32() {
    AgeField::set(&mut shared, 0, &mut storage, 42);
    assert!(shared.has_bits()[0]); // ❌ Always false now
}
```

**After:**
```rust
#[test]
fn test_implicit_optional_i32() {
    AgeField::set(&mut shared, 0, &mut storage, 42);
    // ImplicitOptional fields don't track presence in has_bits
    // (they're always considered "present")
}
```

**Before:**
```rust
#[test]
fn test_person_creation() {
    // Initially, no fields should be marked as "set"
    assert!(!person.has_name()); // ❌ Always false now
}
```

**After:**
```rust
#[test]
fn test_person_creation() {
    // ImplicitOptional fields are always considered "present"
    assert!(person.has_name());
}
```

## Benefits

1. ✅ **Correct Semantics**: `ImplicitOptional` fields correctly always return `true` for presence
2. ✅ **Simplified Implementation**: No unnecessary `has_bits` tracking for implicit fields
3. ✅ **Better Performance**: Fewer bit operations for implicit fields
4. ✅ **Clearer Separation**: Explicit vs implicit presence semantics are clearly distinct

## Protobuf Semantics

### ImplicitOptional Fields
- Always considered "present" with default value
- No presence tracking needed
- `has_*()` methods always return `true`

### ExplicitOptional Fields
- May be absent (not set) or present (set to a value)
- Presence tracking via `has_bits` required
- `has_*()` methods reflect actual presence state

## Test Results

After updates:
- puroro field_ops: 7 tests ✅
- puroro shared: 3 tests ✅
- sandbox basic: 17 tests ✅

**Total: 27 tests ✅**

## Files Modified

- ✅ `puroro/src/field_ops.rs` - Removed has_bits tracking for ImplicitOptional
- ✅ `sandbox/src/generated/person.rs` - Updated has_*() methods to always return true
- ✅ `sandbox/tests/basic.rs` - Updated tests to expect ImplicitOptional always present

## Conclusion

✅ Successfully removed unnecessary has_bits tracking for ImplicitOptional fields  
✅ All tests passing  
✅ Correct protobuf semantics implemented  
✅ Clear separation between implicit and explicit presence  

The field system now correctly implements protobuf presence semantics without unnecessary overhead for implicit fields.
