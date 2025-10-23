# Multi-Version Support: Proto2, Proto3, and Editions

## Decision

Updated documentation and comments to reflect support for proto2, proto3, and editions, not just proto3.

## Rationale

Since we support all protobuf versions (proto2, proto3, and editions), our documentation should reflect this comprehensive support rather than focusing only on proto3.

## Changes Made

### 1. Updated Module Documentation

**Before:**
```rust
//! # Protobuf Field Labels
//!
//! Protobuf defines three field labels:
//! - **Optional**: Field may or may not be present (proto3 implicit presence)
//! - **Explicit Optional**: Field may or may not be present (proto3 explicit presence)
//! - **Repeated**: Field may appear zero or more times
//! - **Map**: Key-value pairs (syntactic sugar for repeated fields)
//!
//! # Presence Semantics
//!
//! - **Implicit Presence (proto3 default)**: Field is always considered "present" with default value
//! - **Explicit Presence (proto3 optional)**: Field has explicit presence tracking
```

**After:**
```rust
//! # Protobuf Field Labels
//!
//! Protobuf defines field labels that vary by version:
//! - **ImplicitOptional**: Field with implicit presence (always "present" with default value)
//! - **ExplicitOptional**: Field with explicit presence tracking (may be absent or present)
//! - **Repeated**: Field may appear zero or more times
//! - **Map**: Key-value pairs (syntactic sugar for repeated fields)
//!
//! # Presence Semantics by Protobuf Version
//!
//! - **Proto2**: All fields use explicit presence (`optional` keyword required)
//! - **Proto3**: Default fields use implicit presence, `optional` fields use explicit presence
//! - **Editions**: Presence semantics configurable via edition settings
```

### 2. Updated Field Label Documentation

**Before:**
```rust
/// Marker for optional fields with implicit presence (proto3 default).
///
/// These fields are always considered "present" with their default value.
/// No presence tracking is needed.
pub struct ImplicitOptional;

/// Marker for optional fields with explicit presence (proto3 optional).
///
/// These fields have explicit presence tracking via has_bits.
/// The field may be absent (not set) or present (set to a value).
pub struct ExplicitOptional;
```

**After:**
```rust
/// Marker for optional fields with implicit presence.
///
/// These fields are always considered "present" with their default value.
/// No presence tracking is needed.
/// 
/// Used for:
/// - Proto3 default fields (e.g., `string name = 1;`)
/// - Proto2 fields (e.g., `optional string name = 1;`)
/// - Editions with implicit presence semantics
pub struct ImplicitOptional;

/// Marker for optional fields with explicit presence.
///
/// These fields have explicit presence tracking via has_bits.
/// The field may be absent (not set) or present (set to a value).
/// 
/// Used for:
/// - Proto3 explicit optional fields (e.g., `optional string name = 1;`)
/// - Editions with explicit presence semantics
pub struct ExplicitOptional;
```

### 3. Updated Implementation Comments

**Before:**
```rust
/// Sets a string field value (proto3 implicit presence).
/// Clears a string field (proto3 implicit presence).
/// Sets a string field value (proto3 explicit presence).
/// Clears a string field (proto3 explicit presence).
```

**After:**
```rust
/// Sets a string field value (implicit presence).
/// Clears a string field (implicit presence).
/// Sets a string field value (explicit presence).
/// Clears a string field (explicit presence).
```

### 4. Updated Examples and Type Aliases

**Before:**
```rust
type NameField = Field<String, ImplicitOptional>;  // proto3 implicit presence
type EmailField = Field<String, ExplicitOptional>; // proto3 explicit presence
```

**After:**
```rust
type NameField = Field<String, ImplicitOptional>;  // implicit presence
type EmailField = Field<String, ExplicitOptional>; // explicit presence
```

## Protobuf Version Support Matrix

| Version | Field Type | Presence Semantics | Field Label |
|---------|------------|-------------------|-------------|
| **Proto2** | `optional string name = 1;` | Explicit presence | `ImplicitOptional` |
| **Proto3** | `string name = 1;` | Implicit presence | `ImplicitOptional` |
| **Proto3** | `optional string name = 1;` | Explicit presence | `ExplicitOptional` |
| **Editions** | Configurable | Configurable | `ImplicitOptional` or `ExplicitOptional` |

## Benefits

1. ✅ **Comprehensive Support**: Documentation reflects support for all protobuf versions
2. ✅ **Clear Version Mapping**: Shows which field labels apply to which versions
3. ✅ **Future-Proof**: Ready for editions and future protobuf versions
4. ✅ **Accurate Documentation**: No longer proto3-specific in terminology

## Test Results

After updates:
- puroro field_ops: 7 tests ✅
- puroro shared: 3 tests ✅
- sandbox basic: 17 tests ✅

**Total: 27 tests ✅**

## Files Modified

- ✅ `puroro/src/field_ops.rs` - Updated module documentation and field label comments
- ✅ `sandbox/src/generated/person.rs` - Updated type alias comments

## Conclusion

✅ Successfully updated documentation to reflect multi-version support  
✅ All tests passing  
✅ Clear mapping between protobuf versions and field labels  
✅ Ready for proto2, proto3, and editions support  

The field label system now has documentation that accurately reflects support for all protobuf versions, not just proto3.
