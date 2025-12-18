# Lazy Parsing State Design

This document describes the finalized design for tracking "in-parsing" state in lazy Protocol Buffer message deserialization.

## Design Overview

Each message struct has its **own cursor** that tracks parsing progress through its own `field_slices`. When parsing for one field, we update ALL fields we encounter along the way. This allows us to pause parsing after finding the first occurrence of a field and resume later when needed.

**Critical Insight**: For scalar message fields (e.g., Person → Address → Location):
- Each message has its own cursor for parsing its own `field_slices`
- Scalar message fields hold a reference to the parent message
- When the child needs all slices, it asks the parent to continue parsing from the parent's cursor
- The parent continues parsing and collects all occurrences of the child's field number

## Final Recommended Design

### Core Pattern: Shared Enum with Thin Wrapper and Deref

**Key Insight**: We only need mutable access **from inside the struct during parsing**, not from outside!

- **External access**: Always `&self` (immutable reference)
- **Internal mutation**: Uses `RefCell::borrow_mut()` inside parsing methods
- **No external `&mut self`**: Users never need `&mut self` access

This means we can use:
- ✅ `Deref` for automatic method forwarding (perfect fit)
- ❌ `DerefMut` not needed (no external `&mut self` access)

### Implementation

```rust
// In puroro crate (shared across all message types)
pub enum LazyImplState<T> {
    /// Builder state - wrapped in RefCell for interior mutability during parsing
    Builder(RefCell<T>),
    /// Finalized state - direct ownership, no RefCell overhead
    Finalized(T),
}

impl<T> LazyImplState<T> {
    /// Get immutable reference to inner value (works for both states)
    pub fn inner(&self) -> &T {
        match self {
            LazyImplState::Builder(inner) => inner.borrow(),
            LazyImplState::Finalized(inner) => inner,
        }
    }
    
    /// Get mutable reference to inner value (only for Builder state, internal use only)
    /// 
    /// This explicitly calls `RefCell::borrow_mut()` to get `RefMut<'_, T>` which provides
    /// `&mut T`-like access. Returns `None` if already finalized (no mutation allowed).
    pub(crate) fn inner_mut(&self) -> Option<std::cell::RefMut<'_, T>> {
        match self {
            LazyImplState::Builder(inner) => {
                // Explicitly call borrow_mut() to get mutable access through RefCell
                Some(inner.borrow_mut())
            },
            LazyImplState::Finalized(_) => None,
        }
    }
    
    /// Finalize - move from Builder to Finalized state
    pub fn finalize(self) -> Self {
        match self {
            LazyImplState::Builder(inner) => {
                LazyImplState::Finalized(inner.into_inner())
            }
            LazyImplState::Finalized(_) => self,
        }
    }
    
    /// Check if finalized
    pub fn is_finalized(&self) -> bool {
        matches!(self, LazyImplState::Finalized(_))
    }
}

// Implement Deref for automatic method forwarding
impl<T> std::ops::Deref for LazyImplState<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

// Generated code for each message type
pub struct PersonLazyImpl<'a, A: Allocator = Global> {
    state: LazyImplState<PersonLazyImplInner<'a, A>>,
}

// Implement Deref to forward to inner struct
impl<'a, A: Allocator> std::ops::Deref for PersonLazyImpl<'a, A> {
    type Target = PersonLazyImplInner<'a, A>;
    fn deref(&self) -> &Self::Target {
        &*self.state  // Deref on LazyImplState forwards to inner
    }
}

// Inner struct with fields
pub struct PersonLazyImplInner<'a, A: Allocator = Global> {
    field_slices: OnceList<&'a [u8], A>,
    allocator: A,
    field_iter: Option<FieldIterator<'a, A>>,
    
    // Field types (same for both states):
    // - T for implicit presence fields (default values)
    // - Option<T> ONLY for explicit optional fields and scalar message fields
    name: String,  // Implicit presence - default to ""
    age: i32,  // Implicit presence - default to 0
    email: Option<String>,  // Explicit optional field
    status: i32,  // Implicit presence enum - default to 0
    score: Option<i32>,  // Explicit optional field
    address: Option<AddressLazyImpl<'a, A>>,  // Scalar message field
    secondary_status: Option<i32>,  // Explicit optional field
    scores: OnceList<i32, A>,  // Repeated field
    addresses: OnceList<AddressLazyImpl<'a, A>, A>,  // Repeated message field
}

// Methods on inner struct (accessed via Deref)
impl<'a, A: Allocator + Clone> PersonLazyImplInner<'a, A> {
    pub fn name(&self) -> &String { &self.name }
    pub fn age(&self) -> i32 { self.age }
    pub fn email(&self) -> Option<&String> { self.email.as_ref() }
    // ... other field accessors
}

// Internal parsing methods (use state.inner_mut() for mutation)
impl<'a, A: Allocator + Clone> PersonLazyImpl<'a, A> {
    fn update_field(&self, field_num: u32, value_slice: &'a [u8]) {
        if let Some(mut inner) = self.state.inner_mut() {
            match field_num {
                1 => inner.name = parse_string(value_slice)?,
                2 => inner.age = parse_varint(value_slice)?,
                // ...
            }
        }
    }
    
    // Wrapper methods for state management
    pub fn finalize(self) -> Self {
        Self { state: self.state.finalize() }
    }
    
    pub fn is_finalized(&self) -> bool {
        self.state.is_finalized()
    }
}

// Usage
let person = PersonLazyImpl { state: ... };
let name = person.name();  // Works via Deref!
let age = person.age();    // Works via Deref!
person.finalize();         // Wrapper method
```

### Benefits

1. **Automatic Method Forwarding**: `Deref` automatically forwards method calls to the inner struct
2. **Message-Specific Methods**: Can implement field accessors on `PersonLazyImplInner`
3. **Trait Implementation**: Can implement traits for `PersonLazyImpl` wrapper
4. **Clean API**: `person.name()` works directly via `Deref`
5. **Minimal Overhead**: Just one field indirection
6. **Interior Mutability**: Method signatures use `&self`, but internally get `&mut` access via `RefCell::borrow_mut()` - this is the interior mutability pattern
7. **No `DerefMut` Needed**: We don't need external mutable access (methods use `&self`, not `&mut self`)
8. **Shared Enum**: Single `LazyImplState<T>` type for all message types
9. **Zero RefCell Overhead After Finalization**: No `RefCell` wrapper in `Finalized` state

## Core Data Structure

```rust
use std::cell::{Cell, RefCell};

/// Iterator over protobuf fields in field_slices
/// Can be paused and resumed, making it easy to parse incrementally
pub struct FieldIterator<'a, A: Allocator> {
    /// Reference to the field_slices we're iterating over
    field_slices: &'a OnceList<&'a [u8], A>,
    /// Current slice index
    slice_index: usize,
    /// Current position within the current slice
    position: usize,
    /// Current slice being parsed (cached for efficiency)
    current_slice: Option<&'a [u8]>,
}

impl<'a, A: Allocator> Iterator for FieldIterator<'a, A> {
    type Item = Result<(u32, u32, &'a [u8]), Error>; // field_num, wire_type, value_slice
    
    fn next(&mut self) -> Option<Self::Item> {
        // Parse field tag and return field number, wire type, and value slice
        // ...
    }
}
```

**Key Points**:
- `FieldIterator` encapsulates the cursor logic - no manual `slice_index`/`position` tracking
- Iterator can be paused (stored) and resumed (recreated from position)
- Uses `RefCell<Option<FieldIterator>>` because `Iterator::next()` requires `&mut self`
- Much simpler parsing code - just iterate and match on field numbers

## Parsing Strategy

### 1. Scalar Non-Message Fields (e.g., `name`, `age`)

**Requirement**: Must parse entire message (to get last value, since later fields overwrite)

```rust
fn deserialize_field_1_name(&self) -> String {
    // Explicitly call inner_mut() to get RefMut (which calls borrow_mut() internally)
    // This gives us mutable access to the inner struct through RefCell
    let mut iter = {
        let mut inner = self.state.inner_mut().unwrap();  // borrow_mut() called here
        inner.field_iter.take()
            .unwrap_or_else(|| FieldIterator::new(&inner.field_slices))
    };
    
    // Parse entire message, updating all fields as we go
    let mut last_name = String::new();
    
    while let Some(result) = iter.next() {
        let (field_num, wire_type, value_slice) = result?;
        
        // Update ALL fields we encounter
        self.update_field(field_num, wire_type, value_slice)?;
        
        if field_num == 1 {
            last_name = parse_string(value_slice)?;
        }
    }
    
    // Save iterator state - explicitly call inner_mut() again to get RefMut
    {
        let mut inner = self.state.inner_mut().unwrap();  // borrow_mut() called here
        inner.field_iter = Some(iter);
    }
    // RefMut is dropped here, releasing the borrow
    
    last_name
}
```

### 2. Repeated Fields (e.g., `scores`, `addresses`)

**Requirement**: Parse only until first occurrence of target field, but still update other fields

```rust
fn ensure_scores_parsed(&self) {
    // Check if already have first item
    if self.scores.iter().next().is_some() {
        return; // Already parsed first item
    }
    
    // Explicitly call inner_mut() to get RefMut (calls borrow_mut() internally)
    let mut iter = {
        let mut inner = self.state.inner_mut().unwrap();  // borrow_mut() called here
        inner.field_iter.take()
            .unwrap_or_else(|| FieldIterator::new(&inner.field_slices))
    };
    
    // Parse until we find first occurrence of field 10 (scores)
    while let Some(result) = iter.next() {
        let (field_num, wire_type, value_slice) = result?;
        
        // Update ALL fields we encounter
        self.update_field(field_num, wire_type, value_slice)?;
        
        if field_num == 10 {
            // Found target field! Stop here
            // Explicitly call inner_mut() again to save iterator state
            {
                let mut inner = self.state.inner_mut().unwrap();  // borrow_mut() called here
                inner.field_iter = Some(iter);
            }
            return;
        }
    }
    
    // Iterator exhausted - save iterator state
    {
        let mut inner = self.state.inner_mut().unwrap();  // borrow_mut() called here
        inner.field_iter = Some(iter);
    }
}
```

### 3. Scalar Message Fields (e.g., `address`)

**Requirement**: Must collect all occurrences of the field number at the same nesting level

```rust
fn deserialize_field_6_address(&self) -> AddressLazyImpl<'a, A> {
    // Parse entire message - update_field will automatically collect all field 6 slices
    // to the child's field_slices as it encounters them
    self.parse_entire_message().unwrap();
    
    // Return the child (now with all slices collected)
    self.address.get().unwrap().clone()
}

fn update_field(&self, field_num: u32, wire_type: u32, value_slice: &'a [u8]) {
    if let Some(mut inner) = self.state.inner_mut() {
        match field_num {
            6 => { // address - scalar message field
                if inner.address.is_none() {
                    let addr = AddressLazyImpl::new_from_first_slice(
                        value_slice,
                        self,
                        6,
                        inner.allocator.clone(),
                    );
                    inner.address = Some(addr);
                } else {
                    // Push additional slice to existing child
                    inner.address.as_ref().unwrap().push_slice(value_slice);
                }
            }
            // ... other fields
        }
    }
}
```

## Interior Mutability Requirements

Since we're updating fields during parsing (which happens through `&self`), we need interior mutability:

| Field Type | Storage Type | Mutability Needed | Why |
|------------|--------------|-------------------|-----|
| **Field iterator** | `Option<FieldIterator>` (wrapped in `RefCell` via enum) | Mutable via `RefCell` | `Iterator::next()` requires `&mut self` |
| **Scalar fields** | Direct fields in inner struct | Update via `RefCell::borrow_mut()` | Can be updated multiple times during partial parsing |
| **Repeated fields** | `OnceList<T, A>` | Push multiple times | Built-in interior mutability via `&self` |
| **Scalar message fields** | `Option<AddressLazyImpl>` | Set once, then push slices | Child's `field_slices` has built-in interior mutability |

**Key Insight**: 
- Method signatures use `&self` (immutable reference) - external code doesn't need `&mut self`
- Internally, methods use `RefCell::borrow_mut()` to get `&mut` access - this is interior mutability
- The `RefMut<'_, T>` returned by `borrow_mut()` provides `&mut T`-like access, allowing field mutations
- This pattern allows mutation through an immutable reference (`&self`)

## Field Type Semantics

Following Protobuf semantics:

- **Implicit presence fields**: Use `T` directly (e.g., `String`, `i32`) with default values (`""`, `0`)
- **Explicit optional fields**: Use `Option<T>` (e.g., `Option<String>`, `Option<i32>`)
- **Scalar message fields**: Use `Option<MessageLazyImpl>` (may be absent)
- **Repeated fields**: Use `OnceList<T, A>` (can add items incrementally)

The same field types are used for both `Builder` and `Finalized` states, making conversion trivial (just move the struct).

## State Transition

```rust
impl<'a, A: Allocator + Clone> PersonLazyImpl<'a, A> {
    /// Finalize parsing - converts from Builder to Finalized state
    pub fn finalize(self) -> Self {
        Self {
            state: self.state.finalize(),
        }
    }
    
    /// Check if finalized
    pub fn is_finalized(&self) -> bool {
        self.state.is_finalized()
    }
}
```

**Key Benefit**: Since field types are the same for both states, the conversion is **trivial** - just move the inner struct directly (no `RefCell` needed for finalized state)!

**Note**: 
- **Builder state**: Uses `RefCell` for interior mutability (fields can be updated)
- **Finalized state**: No `RefCell` needed - fields are immutable, direct access via `&self`
- The enum variant prevents setter methods from being available (runtime check via match)
- This eliminates `RefCell` overhead completely after finalization!

## Alternative Approaches (Not Recommended)

### Const Generic Approach

Using `const IS_PARSING: bool` as a generic parameter was considered but rejected because:
- More complex type system
- Still need runtime enum matching anyway
- No significant benefits over simple enum approach

### Typestate with Different Inner Types

Using `PersonLazyImplInner<'a, BuilderState, A>` and `PersonLazyImplInner<'a, FinalizedState, A>` was considered but rejected because:
- Requires `unsafe` code for type conversion
- Complicates unified access methods
- Doesn't provide significant benefits over runtime enum matching
- Makes code generation more complex

### Type Alias Approach

Using `pub type PersonLazyImpl = LazyImplState<PersonLazyImplInner>` was considered but rejected because:
- Cannot implement message-specific methods
- Cannot implement traits for the alias
- Less type distinction

**Conclusion**: The thin wrapper with `Deref` approach provides the best balance of simplicity, type safety, and API ergonomics.

## Related Patterns: Rust Builder Pattern

Our design shares similarities with the Builder pattern in Rust:

### Standard Builder Pattern

```rust
struct PersonBuilder {
    name: Option<String>,
    age: Option<i32>,
}

impl PersonBuilder {
    fn build(self) -> Person {
        Person {
            name: self.name.unwrap(),
            age: self.age.unwrap(),
        }
    }
}
```

**Key Differences from Our Design:**
- **Standard Builder**: Uses separate types (`PersonBuilder` vs `Person`) - builder is consumed and cannot be reused
- **Our Design**: Uses a single enum type that can hold either state - allows state transition without consuming the original type
- **Standard Builder**: Builder and final product are completely separate types
- **Our Design**: Same inner struct types for both states, only the wrapper (enum) differs

### Why Our Design is Different

Our design differs from standard Builder patterns because:

1. **Lazy Parsing Requirements**: We need to parse incrementally while providing `&self` access to already-parsed fields
2. **Single Type API**: Users shouldn't need to know about builder vs finalized types - the same `PersonLazyImpl` type works for both
3. **Runtime State**: We can't use compile-time type states because parsing state is determined at runtime (we don't know when parsing is "complete" until we've parsed the entire message)
4. **Interior Mutability During Parsing**: We need `RefCell` during parsing because `Iterator::next()` requires `&mut self`, but we want to provide `&self` access to other fields

Our enum-based approach is a hybrid that combines:
- **Builder pattern's state management** (mutable during construction, immutable after)
- **Runtime state tracking** (enum variant tracks state)
- **Single type API** (same type for both states, unlike standard Builder)
- **Interior mutability** (RefCell only during parsing, eliminated after finalization)

### Relevant Resources

1. **Rust Design Patterns - Builder Pattern**
   - URL: https://rust-unofficial.github.io/patterns/patterns/creational/builder.html

2. **The Typestate Builder Pattern: A Case Study in Rust's Compile-Time Safety**
   - URL: https://mxncmr.com/blog/the-typestate-builder-pattern-a-case-study-in-rusts-compile-time-safety/

3. **Software Design Patterns in Rust - Chapter 6: Builder**
   - URL: https://sdpr.rantai.dev/docs/part-i/chapter-6/

4. **Builders in Rust | Shuttle**
   - URL: https://www.shuttle.dev/blog/2022/06/09/the-builder-pattern

5. **Rust builder pattern with types - DEV Community**
   - URL: https://dev.to/mindflavor/rust-builder-pattern-with-types-3chf
