# Remove has_bits Tracking for Repeated Fields

## Decision

Removed `has_bits` tracking for `Repeated` fields since they don't need presence tracking.

## Rationale

Repeated fields are always considered "present" even when empty (they contain an empty list), so they don't need explicit presence tracking via `has_bits`. This simplifies the implementation and correctly reflects the protobuf semantics.

## Changes Made

### 1. Updated FieldSet Implementation for Repeated Fields

**Before:**
```rust
impl<T: ScalarType> FieldSet for Field<T, Repeated> {
    fn set<const BYTES: usize>(...) {
        storage.push(value);
        shared.has_bits_mut().set(bit_index, true); // ❌ Unnecessary
    }
}
```

**After:**
```rust
impl<T: ScalarType> FieldSet for Field<T, Repeated> {
    fn set<const BYTES: usize>(...) {
        storage.push(value);
        // Note: Repeated fields don't need presence tracking
        // (they're always considered "present", even when empty)
    }
}
```

### 2. Updated FieldClear Implementation for Repeated Fields

**Before:**
```rust
impl<T: ScalarType> FieldClear for Field<T, Repeated> {
    fn clear<const BYTES: usize>(...) {
        storage.clear();
        shared.has_bits_mut().set(bit_index, false); // ❌ Unnecessary
    }
}
```

**After:**
```rust
impl<T: ScalarType> FieldClear for Field<T, Repeated> {
    fn clear<const BYTES: usize>(...) {
        storage.clear();
        // Note: Repeated fields don't need presence tracking
        // (they're always considered "present", even when empty)
    }
}
```

### 3. Updated String Repeated Field Implementations

**Before:**
```rust
impl Field<String, Repeated> {
    pub fn set<const BYTES: usize>(...) {
        storage.push(value.to_string());
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
impl Field<String, Repeated> {
    pub fn set<const BYTES: usize>(...) {
        storage.push(value.to_string());
        // Note: Repeated fields don't need presence tracking
        // (they're always considered "present", even when empty)
    }
    
    pub fn clear<const BYTES: usize>(...) {
        storage.clear();
        // Note: Repeated fields don't need presence tracking
        // (they're always considered "present", even when empty)
    }
}
```

### 4. Updated Tests

**Before:**
```rust
#[test]
fn test_repeated_i32() {
    ScoresField::set(&mut shared, 0, &mut storage, 10);
    assert!(shared.has_bits()[0]); // ❌ Always false now
    
    ScoresField::clear(&mut shared, 0, &mut storage);
    assert!(!shared.has_bits()[0]); // ❌ Always false now
}
```

**After:**
```rust
#[test]
fn test_repeated_i32() {
    ScoresField::set(&mut shared, 0, &mut storage, 10);
    // Repeated fields don't track presence in has_bits
    // (they're always considered "present", even when empty)
    
    ScoresField::clear(&mut shared, 0, &mut storage);
    // Repeated fields don't track presence in has_bits
}
```

## Protobuf Semantics

### Repeated Fields
- Always considered "present" (contain an empty list when no elements)
- No presence tracking needed
- Empty list is still a valid, "present" field

### Field Types Summary

| Field Type | Presence Tracking | has_bits Usage |
|------------|------------------|----------------|
| **ImplicitOptional** | ❌ Not needed | Not used |
| **ExplicitOptional** | ✅ Required | Used for presence |
| **Repeated** | ❌ Not needed | Not used |
| **Map** | ❌ Not needed | Not used |

## Benefits

1. ✅ **Correct Semantics**: Repeated fields correctly always considered "present"
2. ✅ **Simplified Implementation**: No unnecessary `has_bits` tracking for repeated fields
3. ✅ **Better Performance**: Fewer bit operations for repeated fields
4. ✅ **Consistent Design**: Only `ExplicitOptional` fields use `has_bits`

## Test Results

After updates:
- puroro field_ops: 7 tests ✅
- puroro shared: 3 tests ✅
- sandbox basic: 17 tests ✅

**Total: 27 tests ✅**

## Files Modified

- ✅ `puroro/src/field_ops.rs` - Removed has_bits tracking for Repeated fields

## Conclusion

✅ Successfully removed unnecessary has_bits tracking for Repeated fields  
✅ All tests passing  
✅ Correct protobuf semantics implemented  
✅ Consistent design: only ExplicitOptional uses has_bits  

The field system now correctly implements protobuf semantics where only `ExplicitOptional` fields require presence tracking via `has_bits`.
