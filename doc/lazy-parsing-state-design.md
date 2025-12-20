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
    // For RefCell fields: ensure parsing is complete before returning Ref
    // This guarantees: 1) no RefMut is active (borrow safety), 2) value is final and confirmed
    pub fn name(&self) -> std::cell::Ref<'_, String<A>> {
        self.ensure_all_fields_parsed();  // Ensure parsing is complete for confirmed values
        self.name.borrow()  // RefCell - returns Ref with final confirmed value
    }
    
    pub fn age(&self) -> i32 {
        self.ensure_all_fields_parsed();  // Ensure parsing is complete for confirmed values
        self.age.get()  // Cell - returns Copy value (final confirmed)
    }
    
    pub fn email(&self) -> Option<std::cell::Ref<'_, String<A>>> {
        self.ensure_all_fields_parsed();  // Ensure parsing is complete for confirmed values
        if self.email.borrow().is_some() {
            Some(std::cell::Ref::map(self.email.borrow(), |opt| opt.as_ref().unwrap()))
        } else {
            None
        }
    }
    
    pub fn score(&self) -> Option<i32> {
        self.ensure_all_fields_parsed();  // Ensure parsing is complete for confirmed values
        self.score.get()  // Cell<Option<i32>> - returns Copy value (final confirmed)
    }
    
    pub fn secondary_status(&self) -> Option<i32> {
        self.ensure_all_fields_parsed();  // Ensure parsing is complete for confirmed values
        self.secondary_status.get()  // Cell<Option<i32>> - returns Copy value (final confirmed)
    }
    
    // Field accessors for repeated fields - direct access, no RefCell!
    pub fn addresses(&self) -> &OnceList<AddressLazyImpl<'a, A>, A> {
        &self.addresses
    }
    
    pub fn scores(&self) -> &OnceList<i32, A> {
        &self.scores
    }
    
    // Parsing methods (if needed for specific use cases)
    // Note: Normal field accessors (e.g., name()) automatically call ensure_all_fields_parsed()
    // These methods are only needed if you want to parse and return a specific field value directly
    pub fn deserialize_field_1_name(&self) -> String<A> {
        // Ensure all fields are parsed first
        self.ensure_all_fields_parsed();
        
        // Now return the final confirmed value
        self.name.borrow().clone()
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

**Requirement**: 
- Must collect all occurrences of the field number at the same nesting level
- Initial access should parse until first occurrence, return `Some(child)` or `None`
- When child needs all slices, it must request parent to continue parsing

**Challenge**: Child messages need to:
1. Hold a reference to the parent (to request continued parsing)
2. Collect additional field slices from parent as they are found
3. Update their own iterator with these slices

**Design**:

```rust
// Parent message (PersonLazyImpl)
impl<'a, A: Allocator + Clone> PersonLazyImpl<'a, A> {
    /// Getter for scalar message field
    /// Returns Some if field is found, None otherwise
    /// Parses until first occurrence
    pub fn address(&self) -> Option<std::cell::Ref<'_, AddressLazyImpl<'a, A>>> {
        // Ensure we've parsed until first occurrence of field 6
        self.ensure_field_6_first_occurrence()?;
        
        // Return reference to child (may not have all slices yet)
        Some(std::cell::Ref::map(self.address.borrow(), |opt| opt.as_ref().unwrap()))
    }
    
    /// Parse until first occurrence of field 6, create child if found
    fn ensure_field_6_first_occurrence(&self) -> Option<()> {
        // Check if already created
        if self.address.borrow().is_some() {
            return Some(());
        }
        
        // Get iterator (may be None if already exhausted)
        let mut field_iter = match self.field_iter.borrow_mut().take() {
            Some(iter) => iter,
            None => return None,  // Already exhausted, field not found
        };
        
        // Parse until we find first occurrence of field 6
        loop {
            match field_iter.next() {
                Some(Ok((field_num, wire_type, value_slice))) => {
                    // Update all fields we encounter
                    self.update_field(field_num, wire_type, value_slice)?;
                    
                    if field_num == 6 {
                        // Found field 6! Create child with first slice and parent reference
                        let mut address = self.address.borrow_mut();
                        if address.is_none() {
                        *address = Some(AddressLazyImpl::new_from_parent(
                            value_slice,
                            self as *const PersonLazyImpl<'a, A>,  // Raw pointer to parent
                            self.allocator.clone(),
                        ));
                        }
                        // Store iterator back (may have more fields)
                        *self.field_iter.borrow_mut() = Some(field_iter);
                        return Some(());
                    }
                },
                Some(Err(e)) => return Err(e),
                None => {
                    // Iterator exhausted, field not found
                    *self.field_iter.borrow_mut() = None;
                    return None;
                }
            }
        }
    }
    
    /// Continue parsing from current position, collecting all occurrences of field_num
    /// Called by child when it needs all slices
    pub(crate) fn continue_parsing_for_field(&self, field_num: u32) -> Result<(), Error> {
        let mut field_iter = match self.field_iter.borrow_mut().take() {
            Some(iter) => iter,
            None => return Ok(()),  // Already exhausted
        };
        
        // Parse until iterator is exhausted, collecting all occurrences of field_num
        loop {
            match field_iter.next() {
                Some(Ok((fnum, wire_type, value_slice))) => {
                    // Update all fields we encounter
                    self.update_field(fnum, wire_type, value_slice)?;
                    
                    // If this is our target field, add slice to child
                    if fnum == field_num {
                        let mut address = self.address.borrow_mut();
                        if let Some(ref mut addr) = *address {
                            addr.add_slice(value_slice)?;
                        }
                    }
                },
                Some(Err(e)) => {
                    *self.field_iter.borrow_mut() = Some(field_iter);
                    return Err(e);
                },
                None => {
                    // Iterator exhausted
                    *self.field_iter.borrow_mut() = None;
                    return Ok(());
                }
            }
        }
    }
    
    fn update_field(&self, field_num: u32, wire_type: u32, value_slice: &'a [u8]) -> Result<(), Error> {
        match field_num {
            6 => {
                // address - scalar message field
                let mut address = self.address.borrow_mut();
                if address.is_none() {
                    // This should have been handled in ensure_field_6_first_occurrence
                    // But handle it here too for safety
                    *address = Some(AddressLazyImpl::new_from_parent(
                        value_slice,
                        self,
                        self.allocator.clone(),
                    ));
                } else {
                    // Additional slice - add to existing child
                    address.as_mut().unwrap().add_slice(value_slice)?;
                }
            }
            // ... other fields
            _ => {}
        }
        Ok(())
    }
}

// Child message (AddressLazyImpl)
pub struct AddressLazyImpl<'a, A: Allocator = Global> {
    allocator: A,
    field_iter: RefCell<Option<FieldIterator<'a, Box<dyn Iterator<Item = &'a [u8]> + 'a, A>>>>,
    
    // Parent reference - needed to request continued parsing
    // Using raw pointer to avoid Clone requirement (safe because parent outlives child via 'a)
    parent: *const PersonLazyImpl<'a, A>,
    
    // Field slices collected so far
    field_slices: OnceList<&'a [u8], A>,
    
    // Address fields...
    street: RefCell<String<A>>,
    city: RefCell<String<A>>,
    // ...
}

impl<'a, A: Allocator + Clone> AddressLazyImpl<'a, A> {
    /// Create from first slice, with parent reference
    /// Note: Parent reference is stored as a raw pointer to avoid Clone requirement
    /// Safety: Parent must outlive child (guaranteed by lifetime 'a)
    pub(crate) fn new_from_parent(
        first_slice: &'a [u8],
        parent: *const PersonLazyImpl<'a, A>,  // Raw pointer to avoid Clone
        alloc: A,
    ) -> Self {
        let field_slices = OnceList::new_in(alloc.clone());
        field_slices.push(first_slice);
        
        Self {
            allocator: alloc.clone(),
            field_iter: RefCell::new(Some(FieldIterator::new(
                Box::new_in(field_slices.iter().cloned(), alloc)
            ))),
            parent,  // Store raw pointer
            field_slices,
            street: RefCell::new(String::new_in(alloc.clone())),
            city: RefCell::new(String::new_in(alloc.clone())),
            // ...
        }
    }
    
    /// Add additional slice from parent
    pub(crate) fn add_slice(&self, slice: &'a [u8]) -> Result<(), Error> {
        self.field_slices.push(slice);
        // Note: field_iter needs to be recreated or updated with new slices
        // This is complex because we need to preserve iterator state
        Ok(())
    }
    
    /// Ensure all fields are parsed
    /// This will request parent to continue parsing if needed
    fn ensure_all_fields_parsed(&self) -> Result<(), Error> {
        // First, request parent to continue parsing and collect all slices
        // Safety: parent pointer is valid because parent outlives child via 'a
        unsafe {
            (&*self.parent).continue_parsing_for_field(6)?;
        }
        
        // Now parse our own fields from all collected slices
        // Recreate iterator with all slices
        let mut field_iter = match self.field_iter.borrow_mut().take() {
            Some(_) => {
                // Recreate with all slices (previous iterator may be stale)
                FieldIterator::new(
                    Box::new_in(self.field_slices.iter().cloned(), self.allocator.clone())
                )
            },
            None => return Ok(()),  // Already parsed
        };
        
        // Parse until exhausted
        while let Some(result) = field_iter.next() {
            let (field_num, wire_type, value_slice) = result?;
            self.update_field(field_num, wire_type, value_slice)?;
        }
        
        *self.field_iter.borrow_mut() = None;
    }
}
```

**Key Design Challenges**:

1. **Parent Reference**: Child needs to hold reference to parent. This is a classic Rust problem. Common solutions include:

   **Option 1: Lifetime-based reference (`&'a Parent`)**
   - **Pros**: Type-safe, no runtime overhead, compiler guarantees validity
   - **Cons**: Requires parent to be stored with lifetime `'a`, which must outlive child. In our case, child is stored in parent's `RefCell<Option<Child>>`, so we'd need parent itself to be in a container that provides the lifetime
   - **Applicability**: Works if parent is stored in an arena or similar structure, but in our case parent is directly owned, making this challenging

   **Option 2: `Rc<RefCell<Parent>>` + `Weak<RefCell<Parent>>`**
   - **Pros**: Avoids circular references (Weak breaks the cycle), type-safe, parent can be dropped while child exists
   - **Cons**: Requires wrapping both parent and child in `Rc<RefCell<>>`, significant runtime overhead (reference counting), allocation overhead
   - **Applicability**: Overkill for our use case since parent always outlives child via lifetime `'a`

   **Option 3: Arena/ID-based approach**
   - **Pros**: All nodes in single arena with same lifetime, no reference cycles, can use indices
   - **Cons**: Requires restructuring to store all messages in an arena, complex to integrate with existing design
   - **Applicability**: Possible but requires major architectural changes

   **Option 4: Raw pointer (`*const Parent`)**
   - **Pros**: No runtime overhead, flexible, can work with existing ownership model
   - **Cons**: Unsafe, requires manual safety guarantees (parent must outlive child, no mutation via pointer)
   - **Applicability**: Works well in our case because `'a` lifetime guarantees parent outlives child, but requires `unsafe` blocks

   **Option 5: No parent reference (callback/closure approach)**
   - **Pros**: No reference issues, type-safe
   - **Cons**: Child can't directly request parent to continue parsing, requires passing parent reference to methods
   - **Applicability**: Could work but less ergonomic API

   **Option 6: Trait-based approach**
   - **Pros**: Abstraction, can use different implementations
   - **Cons**: Dynamic dispatch overhead, still needs to solve the reference problem (trait object still needs a reference to parent)
   - **Applicability**: Doesn't solve the core problem

2. **Iterator State Management**: When parent adds slices to child, child's iterator needs to see them. Options:
   - Recreate iterator from `field_slices` each time (simple but may re-parse)
   - Use a more sophisticated iterator that can accept new slices dynamically

3. **Circular Reference Prevention**: Parent holds `RefCell<Option<AddressLazyImpl>>`, child holds reference to parent. This is safe because:
   - Child's reference to parent is immutable (no mutation through it)
   - Parent's reference to child is interior mutable (`RefCell`)
   - No actual circular ownership (child doesn't own parent, just references it)

**Re-evaluating the Need for Parent Reference**:

The only reason the child needs a parent reference is:
- When `ensure_all_fields_parsed()` is called, it needs to request the parent to continue parsing to collect all field slices

**Why Closure/Callback Doesn't Solve the Problem**:

Using a closure/callback might seem like a solution, but it doesn't actually help:
```rust
// Child holds a closure that can request parent to continue parsing
continue_parsing: Box<dyn Fn() -> Result<(), Error>>
```

However, the closure still needs to capture a reference to the parent:
```rust
let continue_parsing = || {
    parent.continue_parsing_for_field(6)  // Still needs parent reference!
};
```

So we're back to the same problem - the closure needs to hold a reference to the parent, which has the same lifetime/ownership constraints.

**Why We Must Preserve Lazy Parsing**:

**Critical Requirement**: We must preserve true lazy parsing. If the user only needs `address.street` (the first field of the address message), we should NOT parse the entire `address` message. The parent's getter collecting all slices upfront would violate this requirement.

**The Realistic Solution: Raw Pointer with Safety Guarantees**:

Given that:
1. We need true lazy parsing (child decides when to request additional slices)
2. Lifetime `'a` guarantees parent outlives child
3. Closure/callback doesn't solve the reference problem
4. Other approaches (Rc/Weak, Arena) have significant overhead or require major restructuring

The **raw pointer approach is the most practical solution**:
- Minimal overhead (just a pointer)
- Preserves lazy parsing semantics
- Safe because `'a` lifetime guarantees validity
- Well-documented safety invariants
- `unsafe` is limited to the dereference point

**Conclusion**: 
- Closure/callback doesn't solve the problem (still needs parent reference)
- Premature slice collection violates lazy parsing requirement
- Raw pointer with `'a` lifetime guarantee is the most practical solution

**Alternative: Arena Approach with `Rc` and `Rc::new_cyclic_in`**:

Instead of using raw pointers with lifetime parameters, we can use an Arena approach with `Rc<T, A>`:

```rust
struct MessageArena<A: Allocator = Global> {
    // Arena holds all messages as Rc
    messages: Vec<Rc<dyn MessageTrait, A>>,
}

struct PersonLazyImpl<A: Allocator = Global> {
    allocator: A,
    // Reference to arena (Weak to avoid cycle)
    arena: Weak<MessageArena<A>>,
    // Child message - using Rc
    address: RefCell<Option<Rc<AddressLazyImpl<A>, A>>>,
    // ...
}

struct AddressLazyImpl<A: Allocator = Global> {
    allocator: A,
    // Reference to arena (Weak to avoid cycle)
    arena: Weak<MessageArena<A>>,
    // Parent message - using Rc (cloned from parent's Rc)
    parent: Rc<PersonLazyImpl<A>, A>,  // Normal Rc clone
    // ...
}
```

Construction using `Rc::new_cyclic_in`:

```rust
impl<A: Allocator + Clone> MessageArena<A> {
    fn new(data: &[u8], alloc: A) -> Rc<Self, A> {
        // Create arena using Rc::new_cyclic_in
        // This allows messages to reference the arena during construction
        Rc::new_cyclic_in(|arena_weak| {
            // Create Person message
            let person = Rc::new_in(PersonLazyImpl {
                allocator: alloc.clone(),
                arena: arena_weak.clone(),  // Weak reference to arena
                address: RefCell::new(None),
                // ...
            }, alloc.clone());
            
            // Create Address message with reference to parent
            let address = Rc::new_in(AddressLazyImpl {
                allocator: alloc.clone(),
                arena: arena_weak.clone(),
                parent: person.clone(),  // Normal Rc clone (strong reference)
                // ...
            }, alloc.clone());
            
            // Set child in parent
            *person.address.borrow_mut() = Some(address);
            
            MessageArena {
                messages: vec![person, address],
            }
        }, alloc)
    }
}
```

Usage - no lifetime parameters needed:

```rust
impl<A: Allocator + Clone> AddressLazyImpl<A> {
    fn ensure_all_fields_parsed(&self) -> Result<(), Error> {
        // Access parent via Rc - no unsafe needed!
        let parent = &*self.parent;  // Dereference Rc to get &PersonLazyImpl
        parent.continue_parsing_for_field(6)?;
        Ok(())
    }
}
```

**Key Benefits of Arena + Rc Approach**:

1. **No lifetime parameters needed**:
   - `PersonLazyImpl<A>` instead of `PersonLazyImpl<'a, A>`
   - `AddressLazyImpl<A>` instead of `AddressLazyImpl<'a, A>`
   - Simpler type signatures

2. **Type-safe parent-child relationships**:
   - `Rc` provides automatic lifetime management
   - No `unsafe` blocks needed for parent access
   - `Weak` for arena avoids cycles (Arena → Messages → Arena)
   - Strong `Rc` for parent-child is fine (no cycle: Parent → Child → Parent, but not Parent → Child → Arena → Messages → Parent)

3. **Flexible user references**:
   - User can clone `Rc` and "discard" original arena reference
   - Messages can outlive arena reference (until last `Rc` is dropped)
   - More flexible than raw pointer approach

4. **Natural message relationships**:
   - Parent → Child: Strong `Rc` (parent owns child semantically)
   - Child → Parent: Strong `Rc` (child needs parent)
   - Messages → Arena: `Weak` (arena owns messages, avoid cycle)

**Trade-offs of Arena + Rc Approach**:

✅ Pros:
- No lifetime parameters (simpler API)
- Flexible reference management for users
- Type-safe (no `unsafe` for parent access)
- Messages can outlive arena reference

❌ Cons:
- Reference counting overhead (`Rc` operations)
- Allocation overhead (`Rc` itself is heap-allocated)
- More complex construction (need `Rc::new_cyclic_in`)
- `Weak` references add some complexity

**Comparison with Other Rust Patterns**:

| Pattern | Safety | Overhead | Complexity | Our Applicability |
|---------|--------|----------|------------|-------------------|
| `&'a Parent` | ✅ Safe | None | Low | ⚠️ Requires arena/restructuring |
| `Rc<RefCell<Parent>>` + `Weak` | ✅ Safe | Reference counting | Medium | ❌ Overkill, unnecessary |
| Arena/ID | ✅ Safe | Index lookup | High | ⚠️ Major restructuring |
| Raw pointer | ⚠️ Unsafe (but safe with invariants) | None | Low | ✅ Works well |
| **Arena + `Rc` (with `Rc::new_cyclic_in`)** | ✅ **Safe** | **Reference counting** | **Medium** | ✅ **Good alternative** |
| No reference (callbacks) | ✅ Safe | Function pointer | Medium | ⚠️ Less ergonomic |
| Trait object | ✅ Safe | Dynamic dispatch | Medium | ❌ Doesn't solve problem |

**Final Recommendation**:

We have two viable approaches:

1. **Raw pointer approach** (current design):
   - ✅ Zero overhead (just a pointer)
   - ✅ Simple construction
   - ✅ Minimal memory footprint
   - ❌ Requires lifetime parameters (`'a`)
   - ❌ Requires `unsafe` blocks (but safe with `'a` guarantee)
   - ❌ User cannot "discard" parent reference while using child
   - **Best for**: Performance-critical code, minimal memory footprint

2. **Arena + `Rc` approach** (alternative design):
   - ✅ No lifetime parameters (simpler API)
   - ✅ Type-safe (no `unsafe` for parent access)
   - ✅ Flexible user references (can clone/drop independently)
   - ❌ Reference counting overhead
   - ❌ Allocation overhead (each message wrapped in `Rc`)
   - ❌ More complex construction (`Rc::new_cyclic_in`)
   - **Best for**: API simplicity, flexibility, when overhead is acceptable

**Recommendation**: 
- For immediate implementation: Use raw pointer with well-documented safety invariants (parent outlives child via `'a`)
- If API simplicity and user flexibility are more important than performance: Consider Arena + `Rc` approach

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
