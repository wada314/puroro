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

### Core Pattern: Field-Level Interior Mutability

**Key Insight**: We should **not wrap the entire struct in `RefCell`**. Instead, we should use field-level interior mutability where needed:
- `OnceList` already has interior mutability (`push()` takes `&self`), so it doesn't need `RefCell` wrapping
- Scalar fields that need updates during parsing use `RefCell` or `Cell` at the field level
- This provides fine-grained protection and avoids borrow conflicts

- **External access**: Always `&self` (immutable reference)
- **Internal mutation**: Uses field-level `RefCell`/`Cell` or `OnceList`'s built-in interior mutability
- **No external `&mut self`**: Users never need `&mut self` access
- **RefCell safety**: Field getters that return `Ref` automatically ensure the entire input iterator is parsed before returning the `Ref`, preventing conflicts with `RefMut` used during parsing

### Implementation

**Note**: In the sample code below, `Box<T, A>` and `String<A>` are used for brevity. In the actual implementation:
- `Box<T, A>` should be `allocator_api2::boxed::Box<T, A>` (allocator-aware Box)
- `String<A>` should be `allocator_extras::String<A>` (allocator-aware String)
- All allocations should use the provided allocator `A`

```rust
use std::cell::{Cell, RefCell};

// Generated code for each message type
// No wrapper struct needed - PersonLazyImpl directly contains all fields
pub struct PersonLazyImpl<'a, A: Allocator = Global> {
    allocator: A,
    /// FieldIterator needs RefCell because Iterator::next() requires &mut self
    field_iter: RefCell<Option<FieldIterator<'a, Box<dyn Iterator<Item = &'a [u8]> + 'a, A>>>>,
    
    // Scalar fields that can be updated multiple times during parsing
    name: RefCell<String<A>>,  // Non-Copy type - needs RefCell
    age: Cell<i32>,  // Copy type - Cell is sufficient
    email: RefCell<Option<String<A>>>,  // Option<Non-Copy> - needs RefCell
    status: Cell<i32>,  // Copy type - Cell is sufficient
    score: Cell<Option<i32>>,  // Option<Copy> - Cell is sufficient
    secondary_status: Cell<Option<i32>>,  // Option<Copy> - Cell is sufficient
    
    // Scalar message field
    address: RefCell<Option<AddressLazyImpl<'a, A>>>,
    
    // Repeated fields - NO RefCell needed! OnceList already has interior mutability
    scores: OnceList<i32, A>,
    addresses: OnceList<AddressLazyImpl<'a, A>, A>,
}

// Methods on the struct
impl<'a, A: Allocator + Clone> PersonLazyImpl<'a, A> {
    // Field accessors for scalar fields
    pub fn name(&self) -> std::cell::Ref<'_, String<A>> {
        self.name.borrow()  // RefCell - returns Ref
    }
    
    pub fn age(&self) -> i32 {
        self.age.get()  // Cell - returns Copy value
    }
    
    pub fn email(&self) -> Option<std::cell::Ref<'_, String<A>>> {
        if self.email.borrow().is_some() {
            Some(std::cell::Ref::map(self.email.borrow(), |opt| opt.as_ref().unwrap()))
        } else {
            None
        }
    }
    
    pub fn score(&self) -> Option<i32> {
        self.score.get()  // Cell<Option<i32>> - returns Copy value
    }
    
    pub fn secondary_status(&self) -> Option<i32> {
        self.secondary_status.get()  // Cell<Option<i32>> - returns Copy value
    }
    
    // Field accessors for repeated fields - direct access, no RefCell!
    pub fn addresses(&self) -> &OnceList<AddressLazyImpl<'a, A>, A> {
        &self.addresses
    }
    
    pub fn scores(&self) -> &OnceList<i32, A> {
        &self.scores
    }
    
    // Parsing methods
    pub fn deserialize_field_1_name(&self) -> String<A> {
        // Get field iterator via RefCell
        let mut field_iter = self.field_iter.borrow_mut();
        let iter = field_iter.as_mut().expect("FieldIterator should be initialized");
        
        let mut last_name = String::new_in(self.allocator.clone());
        
        while let Some(result) = iter.next() {
            let (field_num, wire_type, value_slice) = result?;
            
            // Update ALL fields we encounter
            self.update_field(field_num, wire_type, value_slice)?;
            
            if field_num == 1 {
                last_name = parse_string(value_slice)?;
            }
        }
        
        last_name
    }
    
    fn update_field(&self, field_num: u32, wire_type: u32, value_slice: &'a [u8]) {
        match field_num {
            1 => *self.name.borrow_mut() = parse_string(value_slice)?,
            2 => self.age.set(parse_varint(value_slice)?),
            4 => {
                // Optional field with non-Copy type - use RefCell
                let email = parse_string(value_slice)?;
                *self.email.borrow_mut() = Some(email);
            },
            5 => {
                // Optional field with Copy type - use Cell
                let score = parse_varint(value_slice)?;
                self.score.set(Some(score));
            },
            7 => {
                // Optional field with Copy type - use Cell
                let status = parse_varint(value_slice)?;
                self.secondary_status.set(Some(status));
            },
            9 => {
                // Repeated field - use OnceList's built-in interior mutability
                let addr = parse_address(value_slice)?;
                self.addresses.push(addr);  // push() takes &self
            },
            10 => {
                // Repeated field - use OnceList's built-in interior mutability
                let score = parse_varint(value_slice)?;
                self.scores.push(score);  // push() takes &self
            },
            // ... other fields
        }
    }
}

impl<'a, A: Allocator + Clone> PersonLazyImpl<'a, A> {
    /// Create a new PersonLazyImpl from a single slice (for top-level messages)
    pub fn new(slice: &'a [u8], alloc: A) -> Self {
        let alloc_clone = alloc.clone();
        
        Self {
            allocator: alloc.clone(),
            field_iter: RefCell::new(Some(FieldIterator::new(
                Box::new_in(std::iter::once(slice), alloc)
            ))),
            // Initialize fields with default values
            name: RefCell::new(String::new_in(alloc.clone())),
            age: Cell::new(0),
            email: RefCell::new(None),
            status: Cell::new(0),
            score: Cell::new(None),
            address: RefCell::new(None),
            secondary_status: Cell::new(None),
            scores: OnceList::new_in(alloc_clone.clone()),
            addresses: OnceList::new_in(alloc_clone),
        }
    }
    
    /// Create a new PersonLazyImpl from multiple slices (for scalar message fields)
    pub fn new_from_slices(slices: impl Iterator<Item = &'a [u8]> + 'a, alloc: A) -> Self {
        let alloc_clone = alloc.clone();
        
        Self {
            allocator: alloc.clone(),
            field_iter: RefCell::new(Some(FieldIterator::new(
                Box::new_in(slices, alloc)
            ))),
            // Initialize fields with default values
            name: RefCell::new(String::new_in(alloc.clone())),
            age: Cell::new(0),
            email: RefCell::new(None),
            status: Cell::new(0),
            score: Cell::new(None),
            address: RefCell::new(None),
            secondary_status: Cell::new(None),
            scores: OnceList::new_in(alloc_clone.clone()),
            addresses: OnceList::new_in(alloc_clone),
        }
    }
}

// Usage
let person = PersonLazyImpl::new(bytes, alloc);
let name = person.name();  // Returns Ref<'_, String<A>>
let addresses = person.addresses();  // Returns &OnceList<...>
addresses.push(addr);  // Uses OnceList's built-in interior mutability
```

### Benefits

1. **Simple Structure**: No wrapper struct needed - direct use of the struct
2. **Message-Specific Methods**: Can implement field accessors directly on `PersonLazyImpl`
3. **Trait Implementation**: Can implement traits directly on `PersonLazyImpl`
4. **Fine-Grained Protection**: Each field is protected independently - no struct-level `RefCell`
5. **OnceList's Built-in Interior Mutability**: `OnceList` doesn't need `RefCell` wrapping - it already has `push(&self)`
6. **No Borrow Conflicts**: Reading from `OnceList` and calling `push()` both use `&self`, so no conflicts
7. **Clean API**: Direct access to repeated fields, `Ref`/`Cell::get()` for scalar fields

## Core Data Structure

```rust
use std::cell::{Cell, RefCell};

/// Iterator over protobuf fields in slices
/// Can be paused and resumed, making it easy to parse incrementally
pub struct FieldIterator<'a, I: Iterator<Item = &'a [u8]>> {
    /// Iterator over the slices
    /// This iterator maintains its own state
    slice_iter: I,
    /// Current slice being parsed
    current_slice: Option<&'a [u8]>,
    /// Current position within the current slice
    position: usize,
}

impl<'a, I: Iterator<Item = &'a [u8]>> FieldIterator<'a, I> {
    /// Create a new FieldIterator from any iterator over slices
    /// The iterator is created once and maintains its state - no need to recreate it
    pub fn new(slice_iter: I) -> Self {
        Self {
            slice_iter,
            current_slice: None,
            position: 0,
        }
    }
}

impl<'a, I: Iterator<Item = &'a [u8]>> Iterator for FieldIterator<'a, I> {
    type Item = Result<(u32, u32, &'a [u8]), Error>; // field_num, wire_type, value_slice
    
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Get current slice or advance to next slice
            if let Some(slice) = self.current_slice {
                if self.position < slice.len() {
                    // Parse field tag and return field number, wire type, and value slice
                    // ... parsing logic ...
                    // After parsing, update self.position
                    return Some(Ok((field_num, wire_type, value_slice)));
                }
            }
            
            // Current slice exhausted, move to next slice using iterator
            self.current_slice = self.slice_iter.next();
            self.position = 0;
            
            if self.current_slice.is_none() {
                // All slices exhausted
                return None;
            }
        }
    }
}
```

**Key Points**:
- `FieldIterator` accepts any `Iterator<Item = &'a [u8]>` - no need to convert to `OnceList` first
- For single slice: use `std::iter::once(slice)` wrapped in `Box::new_in(..., allocator)` (returns `Box<..., A>`)
- For multiple slices: pass the iterator directly, wrapped in `Box::new_in(..., allocator)` (returns `Box<..., A>`)
- Encapsulates cursor logic - tracks current slice and position within slice
- Iterator can be paused (stored) and resumed - once created, it maintains its state
- Stored in `RefCell<Option<FieldIterator>>` because `Iterator::next()` requires `&mut self`

## Parsing Strategy

### 1. Scalar Non-Message Fields (e.g., `name`, `age`)

**Requirement**: Must parse entire message (to get last value, since later fields overwrite)

```rust
impl<'a, A: Allocator + Clone> PersonLazyImpl<'a, A> {
    pub fn deserialize_field_1_name(&self) -> String<A> {
        // Get field iterator via RefCell
        let mut field_iter = self.field_iter.borrow_mut();
        let iter = field_iter.as_mut().expect("FieldIterator should be initialized");
        
        let mut last_name = String::new_in(self.allocator.clone());
        
        while let Some(result) = iter.next() {
            let (field_num, wire_type, value_slice) = result?;
            
            // Update ALL fields we encounter
            self.update_field(field_num, wire_type, value_slice)?;
            
            if field_num == 1 {
                last_name = parse_string(value_slice)?;
            }
        }
        
        last_name
    }
    
    fn update_field(&self, field_num: u32, wire_type: u32, value_slice: &'a [u8]) {
        match field_num {
            1 => *self.name.borrow_mut() = parse_string(value_slice)?,
            2 => self.age.set(parse_varint(value_slice)?),
            // ...
        }
    }
}
```

### 2. Repeated Fields (e.g., `scores`, `addresses`)

**Requirement**: Parse only until first occurrence of target field, but still update other fields

```rust
impl<'a, A: Allocator + Clone> PersonLazyImpl<'a, A> {
    pub fn ensure_scores_parsed(&self) {
        // Check if already have first item
        if self.scores.iter().next().is_some() {
            return; // Already parsed first item
        }
        
        // Get iterator via RefCell
        let mut field_iter = self.field_iter.borrow_mut();
        let iter = field_iter.as_mut().expect("FieldIterator should be initialized");
        
        // Parse until we find first occurrence of field 10 (scores)
        while let Some(result) = iter.next() {
            let (field_num, wire_type, value_slice) = result?;
            
            // Update ALL fields we encounter
            self.update_field(field_num, wire_type, value_slice)?;
            
            if field_num == 10 {
                // Found target field! Stop here
                return;
            }
        }
    }
    
    fn update_field(&self, field_num: u32, wire_type: u32, value_slice: &'a [u8]) {
        match field_num {
            10 => {
                // Use OnceList's built-in interior mutability
                let score = parse_varint(value_slice)?;
                self.scores.push(score);  // push() takes &self
            },
            // ... other fields
        }
    }
}
```

### 3. Scalar Message Fields (e.g., `address`)

**Requirement**: Must collect all occurrences of the field number at the same nesting level

```rust
impl<'a, A: Allocator + Clone> PersonLazyImpl<'a, A> {
    fn deserialize_field_6_address(&self) -> AddressLazyImpl<'a, A> {
        // Parse entire message - update_field will automatically collect all field 6 slices
        // to the child's field_slices as it encounters them
        // ... parsing logic ...
        
        // Return the child (now with all slices collected)
        self.address.borrow().as_ref().unwrap().clone()
    }
    
    fn update_field(&self, field_num: u32, wire_type: u32, value_slice: &'a [u8]) {
        match field_num {
            6 => { // address - scalar message field
                let mut address = self.address.borrow_mut();
                if address.is_none() {
                    let addr = AddressLazyImpl::new_from_first_slice(
                        value_slice,
                        6,
                        self.allocator.clone(),
                    );
                    *address = Some(addr);
                } else {
                    // Push additional slice to existing child
                    address.as_mut().unwrap().push_slice(value_slice);
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
| **Field iterator** | `RefCell<Option<FieldIterator>>` | Mutable via `RefCell` | `Iterator::next()` requires `&mut self` |
| **Scalar fields** (non-Copy) | `RefCell<T>` | Update via `RefCell::borrow_mut()` | Can be updated multiple times during partial parsing |
| **Scalar fields** (Copy) | `Cell<T>` | Update via `Cell::set()` | Can be updated multiple times, `Cell` is sufficient for `Copy` types |
| **Optional fields** (`Option<Copy>`) | `Cell<Option<T>>` | Update via `Cell::set()` | `Option<Copy>` is `Copy`, so `Cell` is sufficient |
| **Optional fields** (`Option<non-Copy>`) | `RefCell<Option<T>>` | Update via `RefCell::borrow_mut()` | `Option<non-Copy>` is not `Copy`, so `RefCell` is needed |
| **Repeated fields** | `OnceList<T, A>` | Push via `OnceList::push(&self)` | **Built-in interior mutability** - no `RefCell` needed! |
| **Scalar message fields** | `RefCell<Option<MessageLazyImpl>>` | Set once, then push slices | Child's `field_slices` has built-in interior mutability, but `Option<MessageLazyImpl>` is not `Copy` |

**Key Insights**: 
- `OnceList` already provides interior mutability via `push(&self)`, so it doesn't need `RefCell` wrapping
- Field-level `RefCell`/`Cell` provides fine-grained protection - each field is independent
- No struct-level `RefCell` means no coarse-grained protection issues
- Reading from `OnceList` and calling `push()` both use `&self`, so no borrow conflicts

## Field Type Semantics

Following Protobuf semantics:

- **Implicit presence fields**: Use `RefCell<T>` for non-`Copy` types or `Cell<T>` for `Copy` types (e.g., `RefCell<String<A>>`, `Cell<i32>`) with default values (`""`, `0`)
- **Explicit optional fields**: 
  - Use `Cell<Option<T>>` for `Option<Copy>` types (e.g., `Cell<Option<i32>>`)
  - Use `RefCell<Option<T>>` for `Option<non-Copy>` types (e.g., `RefCell<Option<String<A>>>`)
- **Scalar message fields**: Use `RefCell<Option<MessageLazyImpl>>` (may be absent)
- **Repeated fields**: Use `OnceList<T, A>` directly (can add items incrementally via `push(&self)`)

**Note**: In the sample code, `String<A>` is used for brevity. In the actual implementation, this should be `allocator_extras::String<A>` (allocator-aware String) with `String::new_in(allocator)` for initialization.

The same field types are used for both parsing and finalized states - no state transition needed.

## Why This Design Works

### No Struct-Level RefCell

The key insight is that **we don't need to wrap the entire struct in `RefCell`**:
- `OnceList` already has interior mutability (`push()` takes `&self`)
- We can use field-level `RefCell`/`Cell` for scalar fields that need updates
- This avoids the coarse-grained protection issues that occur when wrapping the entire struct

### OnceList's Built-in Interior Mutability

`OnceList` provides interior mutability through its `push(&self)` method:
- Reading existing items: `&self.addresses` → `&OnceList`, then `.iter().next()` directly
- Adding new items: `self.addresses.push(...)` → uses `OnceList::push(&self)` which takes `&self`
- No `RefCell` needed - both operations use `&self`

### Fine-Grained Protection

Each field is protected independently:
- Scalar fields wrapped in `RefCell` or `Cell` can be updated independently
- Repeated fields (`OnceList`) can be read and modified independently
- No conflicts between reading one field and updating another

### Critical: RefCell Borrow Safety and Field Value Semantics

**Important**: When using `RefCell`, we must ensure that immutable references (`Ref`) and mutable references (`RefMut`) are not active simultaneously. Additionally, we must distinguish between **unconfirmed intermediate values** (during parsing) and **confirmed final values** (after parsing is complete).

**The Problem**:
1. **Borrow conflict**: Field getters that return `Ref<'_, T>` (e.g., `name()`, `email()`) call `RefCell::borrow()`, but during parsing, `update_field()` calls `borrow_mut()` to update fields, causing a conflict.
2. **Value semantics**: Fields can have intermediate unconfirmed values during parsing (which may be overwritten by later fields), or final confirmed values after parsing is complete. We must ensure users only access confirmed values.

**The Solution (Best Approach)**:
- **When getting from a `RefCell` field, ensure the entire input iterator is parsed first**
- Field getters that return `Ref` should internally ensure that parsing is complete before returning the `Ref`
- This guarantees:
  1. No `RefMut` is active when we return `Ref` from field getters (borrow safety)
  2. The returned value is the **final confirmed field value**, not an intermediate unconfirmed value

**Field Value States**:
- **Intermediate unconfirmed value**: A value found during partial parsing that may be overwritten by later fields with the same field number. Accessing this value is unsafe because it's not the final value.
- **Final confirmed field value**: The value after parsing is complete (iterator exhausted). This is the correct, final value that won't change.

**Implementation Strategy**:
```rust
impl<'a, A: Allocator + Clone> PersonLazyImpl<'a, A> {
    pub fn name(&self) -> std::cell::Ref<'_, String<A>> {
        // Ensure entire iterator is parsed before returning Ref
        // This guarantees:
        // 1. No RefMut is active (borrow safety)
        // 2. The value is final and confirmed (not intermediate)
        self.ensure_all_fields_parsed();
        self.name.borrow()  // Safe: no RefMut can be active, value is confirmed
    }
    
    fn ensure_all_fields_parsed(&self) {
        // Check if iterator still exists (not yet consumed/parsed)
        let mut field_iter = match self.field_iter.borrow_mut().take() {
            Some(iter) => iter,
            None => return,  // Already parsed completely - values are confirmed
        };
        
        // Parse until iterator is exhausted
        // During this process, fields may be updated multiple times (intermediate values)
        while let Some(result) = field_iter.next() {
            let (field_num, wire_type, value_slice) = result?;
            self.update_field(field_num, wire_type, value_slice)?;
        }
        
        // Iterator is now exhausted - store None to indicate parsing is complete
        // All field values are now final and confirmed
        *self.field_iter.borrow_mut() = None;
    }
    
    fn update_field(&self, field_num: u32, wire_type: u32, value_slice: &'a [u8]) {
        // This may update fields multiple times during parsing
        // Values updated here are intermediate until parsing is complete
        match field_num {
            1 => *self.name.borrow_mut() = parse_string(value_slice)?,
            // ... other fields
        }
    }
}
```

**Benefits**:
- **Automatic safety**: Field getters automatically ensure parsing is complete before returning `Ref`
- **Value correctness**: Users always get the final confirmed value, never intermediate unconfirmed values
- **No user responsibility**: Users don't need to manually ensure parsing is complete or worry about value semantics
- **Runtime safety**: `RefCell` will panic if there's still a conflict, but this design prevents it
- **Lazy by default, eager when needed**: Fields are parsed lazily, but become eager when accessed via getters

**Alternative Approaches (not recommended)**:
- Manual completion: Requiring users to call parsing methods before accessing fields (error-prone, users might access intermediate values)
- Returning intermediate values: Users might get incorrect values if parsing is not complete
- Returning owned values: `clone()` is expensive for large strings
- `try_borrow()` with `Option<Ref>`: Adds API complexity and forces users to handle `None` cases

## Alternative Approaches (Not Recommended)

### Struct-Level RefCell Approach

Wrapping the entire struct in `RefCell` was considered but rejected because:
- **Coarse-grained protection**: Protects entire struct as single unit
- **Borrow conflicts**: Reading from `OnceList` (via `Ref`) conflicts with parsing (via `RefMut`)
- **Unnecessary**: `OnceList` already has interior mutability, so struct-level `RefCell` is redundant

### Enum-Based State Management

Using an enum to distinguish between Builder and Finalized states was considered but rejected because:
- Adds complexity without solving the core problem
- Still requires struct-level `RefCell` for Builder state
- Doesn't address the borrow conflict issue with `OnceList`

## Related Patterns: Rust Builder Pattern

Our design shares similarities with the Builder pattern in Rust, but with important differences:

1. **Lazy Parsing Requirements**: We need to parse incrementally while providing `&self` access to already-parsed fields
2. **Field-Level Interior Mutability**: We use `RefCell`/`Cell` at the field level, not struct level
3. **Built-in Interior Mutability**: We leverage `OnceList`'s built-in interior mutability rather than wrapping it
4. **Runtime State**: We don't need separate builder and finalized types - parsing state is managed through the field iterator

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
