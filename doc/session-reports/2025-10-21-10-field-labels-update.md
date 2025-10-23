# Field Labels Update: Proto3 Presence Semantics

## Decision

Updated `FieldKind` to `FieldLabel` to properly reflect protobuf field labels and support proto3 presence semantics.

## Changes Made

### 1. Renamed FieldKind to FieldLabel

**Before:**
```rust
pub trait FieldKind: private::Sealed {}
pub struct Singular;
pub struct Repeated;
pub struct Map;
```

**After:**
```rust
pub trait FieldLabel: private::Sealed {}
pub struct Optional;           // proto3 implicit presence
pub struct ExplicitOptional;   // proto3 explicit presence
pub struct Repeated;
pub struct Map;
```

### 2. Added Proto3 Presence Semantics Support

**Optional (proto3 implicit presence):**
- Fields are always considered "present" with default value
- No strict presence tracking needed (but we still track for consistency)

**ExplicitOptional (proto3 explicit presence):**
- Fields have explicit presence tracking via has_bits
- Field may be absent (not set) or present (set to a value)

### 3. Updated Field Descriptor Examples

**Before:**
```rust
type NameField = Field<String, Singular>;
type AgeField = Field<i32, Singular>;
```

**After:**
```rust
type NameField = Field<String, Optional>;         // proto3 implicit presence
type EmailField = Field<String, ExplicitOptional>; // proto3 explicit presence
type HobbiesField = Field<String, Repeated>;
```

### 4. Updated All Implementations

- ✅ `FieldSet` implementations for both `Optional` and `ExplicitOptional`
- ✅ `FieldGet` implementations for both `Optional` and `ExplicitOptional`
- ✅ `FieldClear` implementations for both `Optional` and `ExplicitOptional`
- ✅ String field implementations for both presence types
- ✅ All tests updated and passing

### 5. Updated Documentation

- ✅ Added comprehensive module documentation explaining protobuf labels
- ✅ Added presence semantics explanation
- ✅ Updated examples to show both presence types

## Protobuf Field Labels

### Standard Protobuf Labels
1. **LABEL_OPTIONAL**: Field may or may not be present
2. **LABEL_REQUIRED**: Field must be present (deprecated in proto3)
3. **LABEL_REPEATED**: Field may appear zero or more times

### Proto3 Presence Semantics
- **Implicit Presence**: Default in proto3, field always "present" with default value
- **Explicit Presence**: `optional` keyword in proto3, explicit presence tracking

### Our Implementation
```rust
// Proto3 implicit presence (default)
string name = 1;                    // Field<String, Optional>

// Proto3 explicit presence
optional string email = 2;          // Field<String, ExplicitOptional>

// Repeated fields
repeated string hobbies = 3;        // Field<String, Repeated>

// Map fields (syntactic sugar)
map<string, int32> scores = 4;      // Field<(String, i32), Map>
```

## Test Results

After updates:
- puroro field_ops: 7 tests ✅ (added 2 new tests for ExplicitOptional)
- puroro shared: 3 tests ✅
- sandbox basic: 17 tests ✅

**Total: 27 tests ✅**

## Files Modified

- ✅ `puroro/src/field_ops.rs` - Updated trait names and implementations
- ✅ `sandbox/src/generated/person.rs` - Updated type aliases
- ✅ All tests updated to use new naming

## Benefits

1. ✅ **Accurate Terminology**: Matches protobuf specification exactly
2. ✅ **Proto3 Support**: Proper support for both presence semantics
3. ✅ **Future-Proof**: Ready for proto2 compatibility (if needed)
4. ✅ **Clear Documentation**: Comprehensive explanation of presence semantics
5. ✅ **Type Safety**: Compiler enforces correct usage

## Next Steps

The field label system is now properly aligned with protobuf specifications:

1. ⏸️ Add proto2 support (if needed): `Field<T, Required>`
2. ⏸️ Add enum field support: `Field<EnumType, Optional>`
3. ⏸️ Add nested message support: `Field<MessageType, Optional>`
4. ⏸️ Add oneof support: `Field<OneofType, Oneof>`

All future field types can be added as trait implementations without changing the core design.

## Conclusion

✅ Successfully updated field labels to match protobuf terminology  
✅ Added proper proto3 presence semantics support  
✅ All tests passing  
✅ Ready for future protobuf feature expansion  

The field label system now accurately reflects protobuf specifications and supports both proto3 presence semantics.
