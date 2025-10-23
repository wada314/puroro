# Naming Consistency Update: Optional → ImplicitOptional

## Decision

Renamed `Optional` to `ImplicitOptional` for clarity and consistency with `ExplicitOptional`.

## Rationale

For naming consistency, if we use `ExplicitOptional` for proto3 explicit presence fields, then the proto3 implicit presence fields should be named `ImplicitOptional` to make the distinction clear.

## Changes Made

### 1. Updated Field Label Names

**Before:**
```rust
pub struct Optional;           // proto3 implicit presence
pub struct ExplicitOptional;   // proto3 explicit presence
```

**After:**
```rust
pub struct ImplicitOptional;   // proto3 implicit presence
pub struct ExplicitOptional;   // proto3 explicit presence
```

### 2. Updated All Implementations

- ✅ `FieldSet` implementations: `Field<T, Optional>` → `Field<T, ImplicitOptional>`
- ✅ `FieldGet` implementations: `Field<T, Optional>` → `Field<T, ImplicitOptional>`
- ✅ `FieldClear` implementations: `Field<T, Optional>` → `Field<T, ImplicitOptional>`
- ✅ String field implementations: `Field<String, Optional>` → `Field<String, ImplicitOptional>`

### 3. Updated Type Aliases

**Before:**
```rust
type NameField = Field<String, Optional>;
type AgeField = Field<i32, Optional>;
```

**After:**
```rust
type NameField = Field<String, ImplicitOptional>;
type AgeField = Field<i32, ImplicitOptional>;
```

### 4. Updated All Tests

- ✅ `test_optional_i32` → `test_implicit_optional_i32`
- ✅ `test_optional_string` → `test_implicit_optional_string`
- ✅ `test_multiple_scalar_types` updated to use `ImplicitOptional`
- ✅ All test type aliases updated

### 5. Updated Documentation

- ✅ Module documentation updated
- ✅ Field descriptor examples updated
- ✅ Usage pattern comments updated

## Final Field Label System

```rust
// Proto3 implicit presence (default)
string name = 1;                    // Field<String, ImplicitOptional>

// Proto3 explicit presence
optional string email = 2;          // Field<String, ExplicitOptional>

// Repeated fields
repeated string hobbies = 3;        // Field<String, Repeated>

// Map fields (syntactic sugar)
map<string, int32> scores = 4;      // Field<(String, i32), Map>
```

## Benefits

1. ✅ **Clear Naming**: `ImplicitOptional` vs `ExplicitOptional` makes the distinction obvious
2. ✅ **Consistent Terminology**: Both optional types follow the same naming pattern
3. ✅ **Self-Documenting**: The names clearly indicate the presence semantics
4. ✅ **Future-Proof**: Clear distinction for proto2 compatibility (if needed)

## Test Results

After updates:
- puroro field_ops: 7 tests ✅
- puroro shared: 3 tests ✅
- sandbox basic: 17 tests ✅

**Total: 27 tests ✅**

## Files Modified

- ✅ `puroro/src/field_ops.rs` - Updated all implementations and tests
- ✅ `sandbox/src/generated/person.rs` - Updated type aliases

## Conclusion

✅ Successfully renamed `Optional` to `ImplicitOptional` for clarity  
✅ All tests passing  
✅ Consistent naming throughout the codebase  
✅ Clear distinction between proto3 presence semantics  

The field label system now has clear, consistent naming that makes the proto3 presence semantics obvious at the type level.
