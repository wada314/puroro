# Lazy Parsing State Design

This document describes the finalized design for tracking "in-parsing" state in lazy Protocol Buffer message deserialization.

## Design Overview

Each message struct has its **own cursor** that tracks parsing progress through its own `field_slices`. When parsing for one field, we update ALL fields we encounter along the way. This allows us to pause parsing after finding the first occurrence of a field and resume later when needed.

**Critical Insight**: For scalar message fields (e.g., Person → Address → Location):
- Each message has its own cursor for parsing its own `field_slices`
- Scalar message fields hold a reference to the parent message
- When the child needs all slices, it asks the parent to continue parsing from the parent's cursor
- The parent continues parsing and collects all occurrences of the child's field number

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

impl<'a, A: Allocator> FieldIterator<'a, A> {
    pub fn new(field_slices: &'a OnceList<&'a [u8], A>) -> Self {
        Self {
            field_slices,
            slice_index: 0,
            position: 0,
            current_slice: None,
        }
    }
    
    /// Resume from a saved position
    pub fn resume_from(slice_index: usize, position: usize, field_slices: &'a OnceList<&'a [u8], A>) -> Self {
        Self {
            field_slices,
            slice_index,
            position,
            current_slice: None,
        }
    }
    
    /// Get current position (for saving/resuming)
    pub fn position(&self) -> (usize, usize) {
        (self.slice_index, self.position)
    }
}

impl<'a, A: Allocator> Iterator for FieldIterator<'a, A> {
    type Item = Result<(u32, u32, &'a [u8]), Error>; // field_num, wire_type, value_slice
    
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Get current slice if we don't have one cached
            if self.current_slice.is_none() {
                // Find the slice at slice_index
                let mut iter = self.field_slices.iter();
                for _ in 0..self.slice_index {
                    iter.next()?;
                }
                self.current_slice = iter.next();
            }
            
            let slice = self.current_slice?;
            
            // Check if we've exhausted this slice
            if self.position >= slice.len() {
                // Move to next slice
                self.slice_index += 1;
                self.position = 0;
                self.current_slice = None;
                continue;
            }
            
            // Parse field tag
            let remaining = &slice[self.position..];
            let (field_num, wire_type, field_len) = match parse_field_tag(remaining) {
                Ok(tag) => tag,
                Err(e) => return Some(Err(e)),
            };
            
            let tag_len = /* calculate tag length */;
            self.position += tag_len;
            
            // Parse field value
            if self.position + field_len > slice.len() {
                return Some(Err(Error::InvalidWireFormat("Field extends beyond slice".into())));
            }
            
            let value_slice = &slice[self.position..self.position + field_len];
            self.position += field_len;
            
            return Some(Ok((field_num, wire_type, value_slice)));
        }
    }
}

pub struct PersonLazyImpl<'a, A: Allocator = Global> {
    /// Raw input buffer slices (for this message level)
    field_slices: OnceList<&'a [u8], A>,
    allocator: A,
    
    /// Field iterator - can be paused and resumed
    /// Uses RefCell because Iterator requires &mut self
    field_iter: RefCell<Option<FieldIterator<'a, A>>>,
    
    // Cached parsed values (updated incrementally as we parse)
    // Scalar fields: Use RefCell<Option<T>> for non-Copy types, Cell<T> for Copy types
    // This allows multiple updates during partial parsing
    name: RefCell<Option<String>>,
    age: Cell<i32>,  // Copy type, can use Cell
    email: RefCell<Option<String>>,
    status: Cell<i32>,  // Copy type (enum stored as i32)
    score: RefCell<Option<i32>>,  // Optional field, use RefCell<Option<T>>
    address: OnceCell<AddressLazyImpl<'a, A>>,  // Message field, set once on first occurrence
    secondary_status: RefCell<Option<i32>>,  // Optional field
    scores: OnceList<i32, A>,
    addresses: OnceList<AddressLazyImpl<'a, A>, A>,
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
    // Get or initialize iterator
    let mut iter = self.field_iter.borrow_mut().take().unwrap_or_else(|| {
        FieldIterator::new(&self.field_slices)
    });
    
    // Parse entire message, updating all fields as we go
    let mut last_name = String::new();
    
    // Simply iterate through fields - much cleaner!
    while let Some(result) = iter.next() {
        let (field_num, wire_type, value_slice) = result?;
        
        // Update ALL fields we encounter
        match field_num {
            1 => { // name
                last_name = parse_string(value_slice)?;
                self.name.set(last_name.clone()).ok();
            }
            2 => { // age
                let age = parse_varint(value_slice)?;
                self.age.set(age).ok();
            }
            10 => { // scores
                let score = parse_varint(value_slice)?;
                self.scores.push(score);
            }
            // ... update other fields
            _ => {} // unknown field, skip
        }
    }
    
    // Save iterator state (exhausted, but save position for consistency)
    *self.field_iter.borrow_mut() = Some(iter);
    
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
    
    // Get or initialize iterator
    let mut iter = self.field_iter.borrow_mut().take().unwrap_or_else(|| {
        FieldIterator::new(&self.field_slices)
    });
    
    // Parse until we find first occurrence of field 10 (scores)
    while let Some(result) = iter.next() {
        let (field_num, wire_type, value_slice) = result?;
        
        // Update ALL fields we encounter (not just target)
        match field_num {
            1 => { // name
                let name = parse_string(value_slice)?;
                // Can be updated multiple times during partial parsing
                *self.name.borrow_mut() = Some(name);
            }
            2 => { // age
                let age = parse_varint(value_slice)?;
                // Copy type, can use Cell::set()
                self.age.set(age);
            }
            10 => { // scores - TARGET FIELD
                let score = parse_varint(value_slice)?;
                self.scores.push(score);
                // Found first occurrence! Stop here and save iterator state
                *self.field_iter.borrow_mut() = Some(iter);
                return;
            }
            // ... update other fields
            _ => {}
        }
    }
    
    // Iterator exhausted, save it anyway
    *self.field_iter.borrow_mut() = Some(iter);
}
```

### 3. Scalar Message Fields (e.g., `address`)

**Critical Issue**: Scalar message fields need all occurrences, but we don't know all slices upfront. When we first encounter field 6 (address), we only see the first occurrence. We need to collect all occurrences to create the child message.

**Two Design Options**:

#### Option 1: Parse Entire Parent Message (Simpler)

When accessing a scalar message field, parse the entire parent message first:

```rust
fn deserialize_field_6_address(&self) -> AddressLazyImpl<'a, A> {
    // Get or initialize iterator
    let mut iter = self.field_iter.borrow_mut().take().unwrap_or_else(|| {
        FieldIterator::new(&self.field_slices)
    });
    
    // Collect all slices for field 6 (address)
    let mut address_slices = OnceList::new_in(self.allocator.clone());
    
    // Parse entire message, updating all fields and collecting field 6 slices
    while let Some(result) = iter.next() {
        let (field_num, wire_type, value_slice) = result?;
        
        // Update ALL fields we encounter
        match field_num {
            1 => { /* update name */ }
            2 => { /* update age */ }
            6 => { // address - TARGET FIELD
                // Collect this slice for AddressLazyImpl
                address_slices.push(value_slice);
            }
            10 => { /* update scores */ }
            // ... update other fields
            _ => {}
        }
    }
    
    // Save iterator state (exhausted)
    *self.field_iter.borrow_mut() = Some(iter);
    
    // Create AddressLazyImpl with all collected slices
    AddressLazyImpl::new_from_slices(address_slices, self.allocator.clone())
}
```

**Pros**: Simple, all slices available when creating child message
**Cons**: Must parse entire parent message even if we only need first occurrence

#### Option 2: Child Message Holds Reference to Parent Parser (More Lazy)

The child message holds a reference to the parent's parsing state and fetches slices on-demand:

```rust
pub struct AddressLazyImpl<'a, A: Allocator = Global> {
    /// Reference to parent message
    /// Needed to collect all slices when field_slices is incomplete
    parent: &'a PersonLazyImpl<'a, A>,
    /// Field number in parent (6 for address)
    parent_field_num: u32,
    /// Slices collected so far
    /// OnceList already supports push() via &self, so no RefCell needed
    field_slices: OnceList<&'a [u8], A>,
    allocator: A,
    
    /// Field iterator - can be paused and resumed
    /// Uses RefCell because Iterator requires &mut self
    field_iter: RefCell<Option<FieldIterator<'a, A>>>,
    
    // Cache fields
    // Scalar fields: Use RefCell<Option<T>> for non-Copy types, Cell<T> for Copy types
    street: RefCell<Option<String>>,
    city: RefCell<Option<String>>,
    zip_code: Cell<i32>,  // Copy type
}

impl<'a, A> AddressLazyImpl<'a, A> {
    /// Create from first occurrence, with reference to parent
    pub fn new_from_first_slice(
        first_slice: &'a [u8],
        parent: &'a PersonLazyImpl<'a, A>,
        parent_field_num: u32,
        alloc: A,
    ) -> Self {
        let field_slices = OnceList::new_in(alloc.clone());
        field_slices.push(first_slice);
        Self {
            parent,
            parent_field_num,
            field_slices, // OnceList already supports push() via &self
            allocator: alloc,
            field_iter: RefCell::new(None),
            street: OnceCell::new(),
            city: OnceCell::new(),
            zip_code: OnceCell::new(),
        }
    }
    
    /// Push a slice to field_slices (called from parent's update_field)
    /// This is needed because the iterator has already passed the slice
    /// OnceList supports push() via &self, so no RefCell needed
    fn push_slice(&self, slice: &'a [u8]) {
        self.field_slices.push(slice);
    }
}
```

**Pros**: Can be lazy - only collect all slices when needed
**Cons**: More complex, child needs reference to parent (lifetime complexity), parent needs interior mutability for field updates

**Current Recommendation**: Option 2 (more lazy, but requires careful design of parent access)

## Example Flow

```
Initial state: cursor = None, all fields empty

User requests scores (repeated field):
  cursor = None → initialize to (slice: 0, pos: 0)
  Parse Field 1 (name) → update name field
  Parse Field 2 (age) → update age field  
  Parse Field 10 (scores = 10) → update scores, STOP HERE
  cursor = (slice: 0, pos: after field 10)

User requests addresses (repeated field):
  cursor = (slice: 0, pos: after field 10)
  Parse Field 6 (address) → update address field (collect first slice, create AddressLazyImpl with parent ref)
  Parse Field 10 (scores = 20) → update scores field (append)
  Parse Field 9 (addresses = {...}) → update addresses, STOP HERE
  cursor = (slice: 0, pos: after field 9)

User requests address.street (nested field access):
  AddressLazyImpl needs to parse its fields
  deserialize_field_6_address() called
    → Parent parses entire message via parse_entire_message()
    → update_field automatically collects all field 6 slices to AddressLazyImpl's field_slices
    → Parent updates its own fields as we go (via interior mutability)
  Then parse AddressLazyImpl's field_slices for street field
    → Use AddressLazyImpl's own iterator to parse through its field_slices
  
  If Location is nested in Address:
    LocationLazyImpl has reference to Address (its parent)
    When Location is accessed, Address parses entire message
    update_field automatically collects all Location slices to LocationLazyImpl's field_slices

User requests name (scalar non-message):
  cursor = (slice: 0, pos: after field 9)
  Must parse entire remaining message
  Parse all remaining fields, updating all as we go
  cursor = (slice: 0, pos: end of message)
```

## Benefits of Iterator-Based Approach

1. **Simpler Code**: No manual `slice_index`/`position` tracking - iterator handles it
2. **Cleaner Parsing Logic**: Just iterate and match on field numbers - much more readable
3. **Memory Efficient**: One iterator per message instead of one per field
4. **Incremental Updates**: Fields get updated as we parse, even if not the target
5. **Cache-Friendly**: Sequential parsing through buffer
6. **Natural Progression**: Iterator moves forward, never backward
7. **Easy to Pause/Resume**: Store iterator state, recreate from position when needed

## Internal Methods for Message Parsing

Based on the four behaviors needed, we can extract common operations into internal methods:

### 1. Iterator Management

```rust
impl<'a, A: Allocator + Clone> PersonLazyImpl<'a, A> {
    /// Get or initialize the field iterator
    fn get_or_init_iterator(&self) -> FieldIterator<'a, A> {
        self.field_iter.borrow_mut().take().unwrap_or_else(|| {
            FieldIterator::new(&self.field_slices)
        })
    }
    
    /// Save the iterator state back
    fn save_iterator(&self, iter: FieldIterator<'a, A>) {
        *self.field_iter.borrow_mut() = Some(iter);
    }
}
```

### 2. Field Update Callback

```rust
impl<'a, A: Allocator + Clone> PersonLazyImpl<'a, A> {
    /// Update a field based on its field number
    /// This is called for every field encountered during parsing
    fn update_field(&self, field_num: u32, wire_type: u32, value_slice: &'a [u8]) -> Result<(), Error> {
        match field_num {
            1 => { // name
                let name = parse_string(value_slice)?;
                // Can be updated multiple times during partial parsing
                *self.name.borrow_mut() = Some(name);
            }
            2 => { // age
                let age = parse_varint(value_slice)?;
                // Copy type, can use Cell::set()
                self.age.set(age);
            }
            3 => { // email
                let email = parse_string(value_slice)?;
                // Can be updated multiple times during partial parsing
                *self.email.borrow_mut() = Some(email);
            }
            4 => { // status
                let status = parse_varint(value_slice)?;
                // Copy type, can use Cell::set()
                self.status.set(status);
            }
            5 => { // score
                let score = parse_varint(value_slice)?;
                // Optional field, can be updated multiple times
                *self.score.borrow_mut() = Some(score);
            }
            6 => { // address - scalar message field
                // CRITICAL: We must push this slice NOW because the iterator has already passed it.
                if let Some(addr) = self.address.get() {
                    // Additional occurrence: push to existing child's field_slices
                    addr.push_slice(value_slice);
                } else {
                    // First occurrence: create child message and push the slice
                    let addr = AddressLazyImpl::new_from_first_slice(
                        value_slice,
                        self,
                        6,
                        self.allocator.clone(),
                    );
                    // new_from_first_slice already pushes first_slice to addr.field_slices
                    self.address.set(addr).ok();
                }
            }
            8 => { // secondary_status
                let status = parse_varint(value_slice)?;
                // Optional field, can be updated multiple times
                *self.secondary_status.borrow_mut() = Some(status);
            }
            9 => { // addresses - repeated message field
                // First occurrence: create child message and add to list
                let addr = AddressLazyImpl::new_from_first_slice(
                    value_slice,
                    self,
                    9,
                    self.allocator.clone(),
                );
                self.addresses.push(addr);
            }
            10 => { // scores - repeated scalar field
                let score = parse_varint(value_slice)?;
                self.scores.push(score);
            }
            _ => {} // unknown field, skip
        }
        Ok(())
    }
}
```

### 3. Parse Until Target Field (for Repeated Fields)

```rust
impl<'a, A: Allocator + Clone> PersonLazyImpl<'a, A> {
    /// Parse until we find the first occurrence of target_field_num
    /// Updates all fields encountered along the way
    /// Returns true if target field was found, false if iterator exhausted
    fn parse_until_field(&self, target_field_num: u32) -> Result<bool, Error> {
        let mut iter = self.get_or_init_iterator();
        
        while let Some(result) = iter.next() {
            let (field_num, wire_type, value_slice) = result?;
            
            // Update ALL fields we encounter
            self.update_field(field_num, wire_type, value_slice)?;
            
            if field_num == target_field_num {
                // Found target field! Stop here
                self.save_iterator(iter);
                return Ok(true);
            }
        }
        
        // Iterator exhausted
        self.save_iterator(iter);
        Ok(false)
    }
}
```

### 4. Parse Entire Message (with Optional Slice Collection)

```rust
impl<'a, A: Allocator + Clone> PersonLazyImpl<'a, A> {
    /// Parse entire message, updating all fields
    /// Used for scalar non-message fields that need the last value
    /// 
    /// **Note**: For scalar message fields, all occurrences are automatically collected
    /// in `update_field` when they are encountered, so no separate collection step is needed.
    fn parse_entire_message(&self) -> Result<(), Error> {
        let mut iter = self.get_or_init_iterator();
        
        while let Some(result) = iter.next() {
            let (field_num, wire_type, value_slice) = result?;
            
            // Update ALL fields we encounter
            // For scalar message fields, update_field automatically pushes slices to child's field_slices
            self.update_field(field_num, wire_type, value_slice)?;
        }
        
        self.save_iterator(iter);
        Ok(())
    }
}
```

### Usage in Field Accessors

Now the field accessors become much simpler:

```rust
impl<'a, A: Allocator + Clone> PersonLazyImpl<'a, A> {
    // Scalar non-message field
    fn deserialize_field_1_name(&self) -> String {
        self.parse_entire_message().unwrap();
        // After parsing entire message, value is final
        self.name.borrow().as_ref().unwrap().clone()
    }
    
    // Repeated field
    fn ensure_scores_parsed(&self) {
        if self.scores.iter().next().is_some() {
            return; // Already parsed first item
        }
        self.parse_until_field(10).unwrap(); // Parse until field 10 (scores)
    }
    
    // Scalar message field
    fn deserialize_field_6_address(&self) -> AddressLazyImpl<'a, A> {
        // Parse entire message - update_field will automatically collect all field 6 slices
        // to the child's field_slices as it encounters them
        self.parse_entire_message().unwrap();
        
        // Return the child (now with all slices collected)
        self.address.get().unwrap().clone() // or return reference, depending on API design
    }
}
```

### Summary of Internal Methods

1. **`get_or_init_iterator()`**: Get or create the field iterator
2. **`save_iterator(iter)`**: Save iterator state back
3. **`update_field(field_num, wire_type, value_slice)`**: Update a field based on field number
   - For scalar message fields, automatically pushes slices to child's `field_slices` as they are encountered
4. **`parse_until_field(target_field_num)`**: Parse until first occurrence of target field
5. **`parse_entire_message()`**: Parse entire message, updating all fields
   - For scalar message fields, all occurrences are automatically collected via `update_field`

**Key Insight**: `update_field` handles slice collection for scalar message fields automatically.
When `parse_entire_message` is called, all scalar message field occurrences are collected
as the iterator encounters them - no separate collection step is needed.

These methods provide a clean separation of concerns:
- Iterator management is centralized
- Field updates are handled in one place
- Parsing strategies (parse until vs. parse entire) are clearly separated
- Child message slice collection is handled separately

## Implementation Notes

### Field Tag Parsing

To skip fields we're not interested in, we need to parse the field tag to get:
- Field number (to identify the field)
- Wire type (to know how to skip the value)
- Value length (for length-delimited fields)

```rust
fn parse_field_tag(bytes: &[u8]) -> Result<(u32, u32, usize), Error> {
    // Parse varint for field number and wire type
    // Wire type determines how to skip the value
    // For length-delimited (wire type 2), read length prefix
}
```

### Interior Mutability Requirements

Since we're updating fields during parsing (which happens through `&self`), we need interior mutability. Here's a breakdown of what mutability is needed for each operation:

#### Field Types and Mutability

| Field Type | Storage Type | Mutability Needed | Why |
|------------|--------------|-------------------|-----|
| **Top-level `field_slices`** | `OnceList<&'a [u8], A>` | Immutable after construction | Set once during `new()`/`new_from_slices()` |
| **Child `field_slices`** (scalar message) | `OnceList<&'a [u8], A>` | Built-in interior mutability | `OnceList::push()` works via `&self` |
| **Field iterator** | `RefCell<Option<FieldIterator>>` | Mutable via `RefCell` | `Iterator::next()` requires `&mut self` |
| **Scalar fields** (name, age, etc.) | `RefCell<Option<T>>` or `Cell<T>` (if `Copy`) | Update multiple times | Can be updated multiple times during partial parsing, but final after message end |
| **Repeated scalar fields** (scores) | `OnceList<T, A>` | Push multiple times | Can add items incrementally |
| **Repeated message fields** (addresses) | `OnceList<T, A>` | Push multiple times | Can add items incrementally |
| **Scalar message fields** (address) | `OnceCell<AddressLazyImpl>` | Set once | Created on first occurrence |

#### Operations and Their Mutability Requirements

**1. Iterator Management**
```rust
fn get_or_init_iterator(&self) -> FieldIterator<'a, A> {
    // Needs: RefCell::borrow_mut() on field_iter
    // Reason: Iterator::next() requires &mut self, so we need &mut FieldIterator
    self.field_iter.borrow_mut().take().unwrap_or_else(|| {
        FieldIterator::new(&self.field_slices)
    })
}

fn save_iterator(&self, iter: FieldIterator<'a, A>) {
    // Needs: RefCell::borrow_mut() on field_iter
    // Reason: Storing iterator state back
    *self.field_iter.borrow_mut() = Some(iter);
}
```

**2. Scalar Field Updates**
```rust
fn update_field(&self, field_num: u32, ...) {
    match field_num {
        1 => { // name
            // Needs: RefCell::borrow_mut() or Cell::set() (if Copy)
            // Reason: Can be updated multiple times during partial parsing
            // For String (non-Copy): use RefCell<Option<String>>
            *self.name.borrow_mut() = Some(name);
            // For i32 (Copy): use Cell<i32> (simpler, no Option needed)
            self.age.set(age);
        }
    }
}
```

**3. Repeated Field Updates**
```rust
fn update_field(&self, field_num: u32, ...) {
    match field_num {
        10 => { // scores
            // Needs: OnceList::push() (interior mutability built-in)
            // Reason: Can push multiple times incrementally
            self.scores.push(score);
        }
    }
}
```

**4. Scalar Message Field Updates (Complex!)**
```rust
fn update_field(&self, field_num: u32, ...) {
    match field_num {
        6 => { // address
            if let Some(addr) = self.address.get() {
                // Needs: OnceList::push() (built-in interior mutability)
                // Reason: Parent (via &self) pushes to child's field_slices
                // OnceList supports push() via &self, so no RefCell needed
                addr.push_slice(value_slice);
            } else {
                // Needs: OnceCell::set() on self.address
                // Reason: Create child on first occurrence
                let addr = AddressLazyImpl::new_from_first_slice(...);
                self.address.set(addr).ok();
            }
        }
    }
}
```

**5. Child Message Slice Collection**
```rust
impl AddressLazyImpl {
    fn push_slice(&self, slice: &'a [u8]) {
        // Needs: OnceList::push() (built-in interior mutability)
        // Reason: Called from parent's update_field (via &self)
        // OnceList supports push() via &self, so no RefCell needed
        // This allows cross-struct mutation: parent pushes to child's field_slices
        self.field_slices.push(slice);
    }
}
```

#### Mutability Complexity Summary

**Simple Cases (Built-in Interior Mutability):**
- ✅ `OnceList::push()` - Thread-safe, multiple pushes
- ✅ `Cell<T>::set()` - For `Copy` types (e.g., `i32`), simple and efficient

**Complex Cases (Require RefCell):**
- ⚠️ `RefCell<Option<FieldIterator>>` - Needed because `Iterator::next()` requires `&mut self`
- ⚠️ `RefCell<Option<T>>` - For non-`Copy` types (e.g., `String`), allows multiple updates

**Cross-Struct Mutation (Simplified!):**
The cross-struct mutation where parent's `update_field` (via `&self`) pushes to child's `field_slices` is actually simpler than it looks:
- Parent holds `address: OnceCell<AddressLazyImpl>`
- Child holds `field_slices: OnceList<&'a [u8], A>` (no RefCell needed!)
- Parent's `update_field` (via `&self`) calls `child.push_slice()` which does `self.field_slices.push()`
- `OnceList::push()` works via `&self` thanks to built-in interior mutability
- This allows parent to push to child's state through immutable references, but no `RefCell` is needed

**Scalar Field Mutability Requirements:**
- **Problem**: Scalar fields can be updated multiple times during partial parsing
  - Example: `parse_until_field(10)` is called multiple times, each time updating `name` field
  - `OnceCell` only allows one-time set, so subsequent updates fail silently (`.ok()` ignores the error)
- **Solution**: Use `RefCell<Option<T>>` for non-`Copy` types, `Cell<T>` for `Copy` types
  - `RefCell<Option<String>>` for `name`, `email` (String fields)
  - `Cell<i32>` for `age`, `status` (Copy types)
  - `RefCell<Option<i32>>` for optional fields like `score`, `secondary_status`
- **Optimization Opportunity**: Once message parsing reaches the end, the value is final and won't be updated anymore
  - This can be used for optimization (e.g., convert `RefCell<Option<T>>` to `OnceCell<T>` after parsing completes)
  - But for simplicity, we can keep `RefCell<Option<T>>` throughout
  - The `Option` wrapper allows us to distinguish "not yet parsed" (`None`) from "parsed but empty" (`Some("")`)

**Why RefCell instead of Cell?**
- `Iterator::next()` requires `&mut self`, so we can't use `Cell` (which only provides `get()`/`set()` for `Copy` types)
- `RefCell` allows us to get `&mut FieldIterator` through `borrow_mut()`
- For non-`Copy` types like `String`, we need `RefCell` to allow multiple updates

### Parent Parser Access (Option 2)

If using Option 2, the child message needs to update parent fields. This requires:
- Parent fields to use interior mutability (`Cell`, `OnceCell`, `OnceList`)
- Or a way to get mutable access to parent (complex with lifetimes)
- Or a trait-based approach where parent provides update methods

## Implementation Notes

### Creating Messages

```rust
impl<'a, A: Allocator + Clone> PersonLazyImpl<'a, A> {
    pub fn new(field_slices: OnceList<&'a [u8], A>, alloc: A) -> Self {
        Self {
            field_slices,
            allocator: alloc,
            field_iter: RefCell::new(None),  // Iterator created lazily on first use
            name: RefCell::new(None),
            age: Cell::new(0),  // Default value for Copy type
            email: RefCell::new(None),
            status: Cell::new(0),
            score: RefCell::new(None),
            address: OnceCell::new(),
            secondary_status: RefCell::new(None),
            // ... initialize all fields
        }
    }
}

impl<'a, A: Allocator + Clone> AddressLazyImpl<'a, A> {
    pub fn new_from_first_slice(
        first_slice: &'a [u8],
        parent: &'a PersonLazyImpl<'a, A>,
        parent_field_num: u32,
        alloc: A,
    ) -> Self {
        let field_slices = OnceList::new_in(alloc.clone());
        field_slices.push(first_slice);
        Self {
            parent,
            parent_field_num,
            field_slices, // OnceList already supports push() via &self
            allocator: alloc,
            field_iter: RefCell::new(None),  // Iterator created lazily on first use
            street: RefCell::new(None),
            city: RefCell::new(None),
            zip_code: Cell::new(0),  // Copy type
        }
    }
    
    /// Push a slice to field_slices (called from parent's update_field)
    /// OnceList supports push() via &self, so no RefCell needed
    fn push_slice(&self, slice: &'a [u8]) {
        self.field_slices.push(slice);
    }
}
```

### Parent Field Updates

When parsing for a child message field, we encounter parent fields. To update them:
- Parent fields use interior mutability (`OnceCell`, `OnceList`) - already required
- Parent's `parse_entire_message` method updates parent's fields as it parses
- For scalar message fields, `update_field` automatically pushes slices to child's `field_slices`
- Child doesn't need mutable access to parent - parent updates itself via interior mutability

### Nested Message Example

```rust
// Wire format: Person { address: Address { location: Location { ... } } }
let person = PersonLazyImpl::new(field_slices, alloc);
// person has its own cursor and field_slices

// Access address (scalar message field)
let address = person.address(); 
// Triggers: parent.parse_entire_message()
// Parent parses entire message, update_field automatically collects all field 6 slices
// to address.field_slices as they are encountered
// address has its own iterator and field_slices (now complete)

// Access location (nested scalar message field in Address)
let location = address.location();
// Triggers: address.parse_entire_message()
// Address parses entire message, update_field automatically collects all field 1 slices
// to location.field_slices as they are encountered
// location has its own iterator and field_slices (now complete)

// Each level has its own iterator, tracking progress through its own slices
```

## Slice Ownership Design: Child vs Parent

There are two possible designs for where scalar message field slices are stored:

### Option 1: Child Owns Slices (Current Design)

```rust
pub struct AddressLazyImpl<'a, A: Allocator = Global> {
    parent: &'a PersonLazyImpl<'a, A>,
    parent_field_num: u32,
    /// Child owns its own slices
    /// OnceList supports push() via &self, so no RefCell needed
    field_slices: OnceList<&'a [u8], A>,
    // ... other fields
}

impl PersonLazyImpl {
    fn update_field(&self, field_num: u32, wire_type: u32, value_slice: &'a [u8]) {
        if field_num == 6 { // address
            if let Some(addr) = self.address.get() {
                addr.push_slice(value_slice); // Push to child's field_slices
            } else {
                let addr = AddressLazyImpl::new_from_first_slice(...);
                self.address.set(addr).ok();
            }
        }
    }
}
```

**Pros:**
- ✅ **Self-contained**: Child message is independent - owns its own data
- ✅ **Cleaner parent**: Parent doesn't need to track slices for each child field
- ✅ **Direct access**: Child can directly access `field_slices` without going through parent
- ✅ **Simpler iterator**: Child's `FieldIterator` can directly reference its own `field_slices`
- ✅ **Better encapsulation**: Each message manages its own parsing state

**Cons:**
- ❌ **Cross-struct mutation**: Parent's `update_field` pushes to child's `field_slices`
  - Note: `OnceList::push()` works via `&self`, so no `RefCell` needed
  - Child needs parent reference anyway for lazy parsing, so this isn't a unique disadvantage

### Option 2: Parent Owns Slices

```rust
pub struct PersonLazyImpl<'a, A: Allocator = Global> {
    field_slices: OnceList<&'a [u8], A>,
    // ... other fields
    /// Parent owns slices for scalar message fields
    /// OnceList supports push() via &self, so no RefCell needed
    address_slices: OnceList<&'a [u8], A>,
    address: OnceCell<AddressLazyImpl<'a, A>>,
}

pub struct AddressLazyImpl<'a, A: Allocator = Global> {
    parent: &'a PersonLazyImpl<'a, A>,
    parent_field_num: u32,
    /// Child holds reference to parent's slices
    // No field_slices field - accesses parent.address_slices instead
    // ... other fields
}

impl PersonLazyImpl {
    fn update_field(&self, field_num: u32, wire_type: u32, value_slice: &'a [u8]) {
        if field_num == 6 { // address
            // OnceList supports push() via &self, so no RefCell needed
            self.address_slices.push(value_slice);
            if self.address.get().is_none() {
                let addr = AddressLazyImpl::new(self, 6, ...);
                self.address.set(addr).ok();
            }
        }
    }
}

impl AddressLazyImpl {
    fn get_field_slices(&self) -> &OnceList<&'a [u8], A> {
        &self.parent.address_slices // Access parent's slices directly
    }
}
```

**Pros:**
- ✅ **Centralized ownership**: Parent owns all data - clearer ownership model
- ✅ **No RefCell needed**: `OnceList` supports `push()` via `&self`
- ✅ **Parent controls collection**: Parent directly manages slice collection

**Cons:**
- ❌ **Parent complexity**: Parent must have a `OnceList<...>` field for each scalar message field
- ❌ **Indirect access**: Child must go through parent to access slices (more indirection)
- ❌ **Iterator complexity**: Child's `FieldIterator` needs to reference parent's slices
- ❌ **Less encapsulation**: Child depends on parent's internal structure
- ❌ **Code generation complexity**: Must generate slice storage fields in parent for each scalar message field

### Recommendation: Option 1 (Child Owns Slices) ✓

**Reasoning:**
1. **Better Encapsulation**: Each message is self-contained and manages its own parsing state
2. **Simpler Code Generation**: No need to generate slice storage fields in parent for each scalar message field
3. **Cleaner API**: Child can directly access its `field_slices` without indirection
4. **More Flexible**: Child's iterator can directly reference its own `field_slices`
5. **Consistent Pattern**: All messages (top-level and nested) follow the same pattern - they all own their `field_slices`

**Note**: Both designs require the child to hold a parent reference for lazy parsing, so that's not a differentiating factor. The key difference is where the slices are stored and accessed.

## Open Questions

1. **Parent Field Updates**: How should parent update its fields when parsing for child?
   - Interior mutability in parent (already needed) - simplest ✓
   - Parent's `parse_entire_message` method updates parent fields directly
   - `update_field` automatically handles slice collection for scalar message fields
   - No need for child to have mutable access to parent

2. **Parent Reference Lifetime**: How to manage the lifetime of parent reference?
   - Child holds `&'a Parent` - lifetime tied to parent (current design)
   - Parent must outlive child (natural with Rust lifetimes)
   - Works correctly for arbitrary nesting depth

3. **Slice Ownership**: Should child or parent own scalar message field slices?
   - **Option 1 (Child owns)**: Better encapsulation, simpler code generation, cleaner API ✓ (adopted)
   - **Option 2 (Parent owns)**: Centralized ownership, but more complex parent structure

## Similarity to Builder Pattern

The current lazy parsing implementation shares interesting similarities with the **Builder Pattern**:

### Builder Pattern Analogy

**Traditional Builder Pattern:**
```rust
let mut builder = PersonBuilder::new();
builder.set_name("Alice");
builder.set_age(30);
let person = builder.build(); // Finalize - builder is consumed, person is immutable
```

**Lazy Parsing Pattern:**
```rust
let person = PersonLazyImpl::new(field_slices, alloc);
// Parsing in progress - fields can be updated multiple times
person.parse_until_field(10); // Updates name, age, etc. as it encounters them
person.parse_until_field(9);  // Updates name, age again if encountered
// ...
person.parse_entire_message(); // Finalize - all fields are now final
// After this point, fields won't be updated anymore
```

### Key Similarities

1. **Mutable During Construction**: 
   - Builder: Fields can be set multiple times before `build()`
   - Lazy Parser: Fields can be updated multiple times during partial parsing

2. **Finalization Point**:
   - Builder: `build()` consumes the builder and returns an immutable object
   - Lazy Parser: `parse_entire_message()` marks the end of parsing - fields are now final

3. **State Transition**:
   - Builder: `Builder` → `build()` → `Person` (immutable)
   - Lazy Parser: `Parsing in progress` → `parse_entire_message()` → `Parsing complete` (fields final)

### Design Implications

This similarity suggests potential optimizations:

1. **Type-Level Finalization**: After `parse_entire_message()` completes, we could convert `RefCell<Option<T>>` to `OnceCell<T>` to:
   - Reduce runtime overhead (no more `RefCell` borrows)
   - Make immutability explicit at the type level
   - Prevent accidental updates after finalization

2. **Explicit Finalization State**: We could track whether parsing is complete:
   ```rust
   pub struct PersonLazyImpl<'a, A: Allocator = Global> {
       // ... fields ...
       is_finalized: Cell<bool>, // Track if parsing is complete
   }
   ```

3. **Builder-Like API**: We could provide a method that finalizes and returns a more efficient representation:
   ```rust
   impl PersonLazyImpl {
       /// Finalize parsing and return an optimized immutable representation
       fn finalize(self) -> PersonLazyFinal<'a, A> {
           // Convert RefCell<Option<T>> to OnceCell<T>
           // Convert Cell<T> to T (for Copy types)
           // This consumes self, ensuring no further updates
       }
   }
   ```

### Current Design Choice

For simplicity, the current design keeps `RefCell<Option<T>>` and `Cell<T>` throughout the lifetime of the message, even after parsing is complete. This:
- ✅ Simplifies the implementation
- ✅ Avoids complex state transitions
- ✅ Allows lazy parsing to remain truly lazy (no forced finalization)

However, the Builder pattern analogy suggests that **explicit finalization** could be a valuable optimization for cases where:
- The entire message will be parsed anyway
- Performance is critical
- Type-level immutability guarantees are desired

### Future Consideration

If we want to optimize for the "parse entire message" case, we could introduce a separate type:
- `PersonLazyImpl`: Current design - allows incremental parsing, fields can be updated
- `PersonLazyFinal`: After `parse_entire_message()`, convert to this type with `OnceCell<T>` fields

This would be similar to how some builders have both a mutable builder and an immutable final product.
