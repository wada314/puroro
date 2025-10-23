# ExplicitOptional Get Method with Presence Check

## Decision

Added presence check to `get` methods for `ExplicitOptional` fields, returning default values when not present.

## Rationale

`ExplicitOptional` fields have explicit presence tracking via `has_bits`. When getting values, we must check if the field is present and return the default value if not set, rather than returning the stored value directly.

## Changes Made

### 1. Added Presence-Checking Get Methods for Scalar Types

**Before:**
```rust
impl<T: ScalarType> FieldGet for Field<T, ExplicitOptional> {
    fn get<'a>(storage: &'a Self::Storage) -> Self::Value<'a> {
        *storage  // ❌ Always returns stored value, ignoring presence
    }
}
```

**After:**
```rust
impl<T: ScalarType> Field<T, ExplicitOptional> {
    /// Gets the field value, checking presence first.
    /// 
    /// Returns the default value if the field is not present.
    #[inline]
    pub fn get<const BYTES: usize>(
        shared: &SharedFields<BYTES>,
        bit_index: usize,
        storage: &T,
    ) -> T {
        if shared.has_bits()[bit_index] {
            *storage
        } else {
            T::default()  // ✅ Returns default when not present
        }
    }
}
```

### 2. Added Presence-Checking Get Methods for String Types

**Before:**
```rust
impl FieldGet for Field<String, ExplicitOptional> {
    fn get<'a>(storage: &'a Self::Storage) -> Self::Value<'a> {
        storage.as_str()  // ❌ Always returns stored value, ignoring presence
    }
}
```

**After:**
```rust
impl Field<String, ExplicitOptional> {
    /// Gets the field value, checking presence first.
    /// 
    /// Returns an empty string if the field is not present.
    #[inline]
    pub fn get<'a, const BYTES: usize>(
        shared: &'a SharedFields<BYTES>,
        bit_index: usize,
        storage: &'a String,
    ) -> &'a str {
        if shared.has_bits()[bit_index] {
            storage.as_str()
        } else {
            ""  // ✅ Returns empty string when not present
        }
    }
}
```

### 3. Updated Tests to Verify Presence Check Behavior

**Before:**
```rust
#[test]
fn test_explicit_optional_i32() {
    AgeField::set(&mut shared, 0, &mut storage, 42);
    assert_eq!(AgeField::get(&storage), 42);  // ❌ Old API
}
```

**After:**
```rust
#[test]
fn test_explicit_optional_i32() {
    // Initially not set - should return default value
    assert_eq!(AgeField::get(&shared, 0, &storage), 0);
    assert!(!shared.has_bits()[0]);

    AgeField::set(&mut shared, 0, &mut storage, 42);
    
    // Now set - should return actual value
    assert_eq!(AgeField::get(&shared, 0, &storage), 42);
    assert!(shared.has_bits()[0]);

    AgeField::clear(&mut shared, 0, &mut storage);
    
    // After clear - should return default value again
    assert_eq!(AgeField::get(&shared, 0, &storage), 0);
}
```

## Protobuf Semantics

### ExplicitOptional Fields
- Have explicit presence tracking via `has_bits`
- `get()` method checks presence before returning value
- Returns default value when not present
- Returns actual stored value when present

### Field Types Summary

| Field Type | Get Behavior | Presence Check |
|------------|-------------|----------------|
| **ImplicitOptional** | Always returns stored value | ❌ Not needed |
| **ExplicitOptional** | Checks presence, returns default if not set | ✅ Required |
| **Repeated** | Always returns slice/vector | ❌ Not needed |
| **Map** | Always returns map | ❌ Not needed |

## API Changes

### New Get Method Signature
```rust
// For scalar types
Field<T, ExplicitOptional>::get<const BYTES: usize>(
    shared: &SharedFields<BYTES>,
    bit_index: usize,
    storage: &T,
) -> T

// For String type
Field<String, ExplicitOptional>::get<'a, const BYTES: usize>(
    shared: &'a SharedFields<BYTES>,
    bit_index: usize,
    storage: &'a String,
) -> &'a str
```

### Usage Example
```rust
// Check if field is present and get value
let value = ExplicitOptionalField::get(&self._shared, IDX_FIELD, &self.field);
// Returns default value if not present, actual value if present
```

## Benefits

1. ✅ **Correct Semantics**: `ExplicitOptional` fields correctly check presence before returning values
2. ✅ **Type Safety**: Compiler enforces presence checking for explicit optional fields
3. ✅ **Consistent API**: Clear distinction between implicit and explicit presence semantics
4. ✅ **Performance**: Minimal overhead for presence checking

## Test Results

After updates:
- puroro field_ops: 7 tests ✅
- puroro shared: 3 tests ✅
- sandbox basic: 17 tests ✅

**Total: 27 tests ✅**

## Files Modified

- ✅ `puroro/src/field_ops.rs` - Added presence-checking get methods for ExplicitOptional fields

## Conclusion

✅ Successfully added presence checking to ExplicitOptional get methods  
✅ All tests passing  
✅ Correct protobuf semantics implemented  
✅ Clear API distinction between field types  

The field system now correctly implements protobuf semantics where `ExplicitOptional` fields check presence before returning values, ensuring proper default value handling.
