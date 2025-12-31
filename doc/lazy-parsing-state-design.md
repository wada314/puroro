# Lazy Parsing State Design

This document describes the finalized design for tracking "in-parsing" state in lazy Protocol Buffer message deserialization.

**Note**: For discussion history and design exploration, see `lazy-parsing-state-design-discussion.md`.

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
    /// Returns Rc<Self> - all methods use self: &Rc<Self>
    pub fn new(slice: &'a [u8], alloc: A) -> Rc<Self, A> {
        let alloc_clone = alloc.clone();
        
        // Use Rc::new_cyclic_in to create PersonLazyImpl with MessageParserState that has callback
        Rc::new_cyclic_in(|weak| {
            let message_body_weak = weak.clone();
            
            // Create initial callback that only handles Message Body (when it's alive)
            // This callback will be replaced in Drop::drop with one that handles child messages
            let callback = Box::new(move |fnum: u32, wire_type: u32, value_slice: &'a [u8]| -> Result<(), Error> {
                // Update via Message Body (should always succeed when this callback is active)
                if let Some(message_body) = message_body_weak.upgrade() {
                    let _ = message_body.update_field(fnum, wire_type, value_slice);
                }
                Ok(())
            });
            
            // Create parser state with initial callback
            let parser_state = Rc::new_in(RefCell::new(MessageParserState {
                field_iter: Some(FieldIterator::new(
                    Box::new_in(std::iter::once(slice), alloc.clone())
                )),
                allocator: alloc.clone(),
                field_update_callback: Some(callback),
            }), alloc.clone());
            
            // Create message body
            Self {
                parser_state: parser_state.clone(),
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
        }, alloc)
    }
    
    
    /// Create a new PersonLazyImpl from multiple slices (for scalar message fields)
    /// Returns Rc<Self> - all methods use self: &Rc<Self>
    pub fn new_from_slices(slices: impl Iterator<Item = &'a [u8]> + 'a, alloc: A) -> Rc<Self, A> {
        let alloc_clone = alloc.clone();
        
        // Use Rc::new_cyclic_in to create PersonLazyImpl with MessageParserState that has callback
        Rc::new_cyclic_in(|weak| {
            let message_body_weak = weak.clone();
            
            // Create initial callback that only handles Message Body (when it's alive)
            // This callback will be replaced in Drop::drop with one that handles child messages
            let callback = Box::new(move |fnum: u32, wire_type: u32, value_slice: &'a [u8]| -> Result<(), Error> {
                // Update via Message Body (should always succeed when this callback is active)
                if let Some(message_body) = message_body_weak.upgrade() {
                    let _ = message_body.update_field(fnum, wire_type, value_slice);
                }
                Ok(())
            });
            
            // Create parser state with initial callback
            let parser_state = Rc::new_in(RefCell::new(MessageParserState {
                field_iter: Some(FieldIterator::new(
                    Box::new_in(slices, alloc.clone())
                )),
                allocator: alloc.clone(),
                field_update_callback: Some(callback),
            }), alloc.clone());
            
            // Create message body
            Self {
                parser_state: parser_state.clone(),
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
        }, alloc)
    }
}

// Usage
let person_rc = PersonLazyImpl::new(bytes, alloc);
let name = person_rc.name();  // Returns Ref<'_, String<A>>
let addresses = person_rc.addresses();  // Returns &OnceList<...>
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
    pub fn deserialize_field_1_name(self: &Rc<Self>) -> String<A> {
        // Get parser state via RefCell
        let mut parser_state = self.parser_state.borrow_mut();
        let field_iter = parser_state.field_iter.as_mut().expect("FieldIterator should be initialized");
        
        let mut last_name = String::new_in(parser_state.allocator.clone());
        
        while let Some(result) = field_iter.next() {
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

**Note**: This uses the generic `ensure_field_first_occurrence` method, which works for any field type.

```rust
impl<'a, A: Allocator + Clone> PersonLazyImpl<'a, A> {
    pub fn ensure_scores_parsed(self: &Rc<Self>) -> Result<(), Error> {
        // Check if already have first item
        if self.scores.iter().next().is_some() {
            return Ok(()); // Already parsed first item
        }
        
        // Use generic method to parse until first occurrence of field 10
        // update_field already handles adding the value to scores
        self.ensure_field_first_occurrence(10)
    }
    
    fn update_field(&self, field_num: u32, wire_type: u32, value_slice: &'a [u8]) -> Result<(), Error> {
        match field_num {
            10 => {
                // Use OnceList's built-in interior mutability
                let score = parse_varint(value_slice)?;
                self.scores.push(score);  // push() takes &self
            },
            // ... other fields
        }
        Ok(())
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

**Problem with Previous Approaches**:
- **Raw pointer**: Requires `unsafe` and lifetime parameters
- **Weak reference**: Parent can be dropped, preventing child from completing parsing
- **Strong reference**: Creates cycle (Parent → Child → Parent)

**Solution: Separate Message Body from Parser State**

The key insight is to separate the message into two parts:
1. **Message Body**: Contains field values (name, age, email, etc.)
2. **Parser State**: Contains parsing state (input slices, iterators, etc.)

The Message Body owns the Parser State via `Rc`, and child messages hold a strong reference to the **parent's Parser State** (not the parent's Message Body). This avoids cycles because:
- Parent Message Body → Parent Parser State (Rc)
- Parent Message Body → Child Message Body (Rc)
- Child Message Body → Parent Parser State (Rc)
- **No cycle**: Parent Message Body owns both, so Child → Parent Parser State doesn't create a cycle

**Design**:

```rust
// Parser State - contains parsing state only
// The entire State is wrapped in RefCell (it's a state, so it should be mutable)
// Generic across all message types - does not need to know the specific message type
pub struct MessageParserState<'a, A: Allocator = Global> {
    /// FieldIterator - needs &mut self for Iterator::next()
    field_iter: Option<FieldIterator<'a, Box<dyn Iterator<Item = &'a [u8]> + 'a, A>>>,
    allocator: A,
    /// Field update callback - handles field updates
    /// 
    /// This callback is responsible for updating fields when iterating over field_iter.
    /// 
    /// Initially, the callback captures a Weak reference to Message Body and calls update_field.
    /// When Message Body is dropped, Drop::drop updates this callback to one that captures
    /// child message Weak references and uses static match-case to update them directly.
    /// 
    /// This design allows MessageParserState to be generic across all message types,
    /// as it doesn't need to know the specific message type at compile time.
    field_update_callback: Option<Box<dyn FnMut(u32, u32, &'a [u8]) -> Result<(), Error> + 'a>>,
}

impl<'a, A: Allocator + Clone> MessageParserState<'a, A> {
    /// Continue parsing and update all registered fields via the callback
    /// Called by child when it needs all slices - works even if parent Message Body is dropped
    /// 
    /// **Key Design**: Uses a single callback mechanism that is updated when Message Body is dropped:
    /// - Initially: Callback captures Weak reference to Message Body and calls update_field
    /// - After Drop: Callback is replaced with one that captures child Weak references and uses static match-case
    /// 
    /// **Important**: When Message Body is dropped, the callback updates ALL registered child message fields.
    /// This ensures that multiple scalar message child fields can all receive their slices when
    /// any one of them calls this method.
    pub fn continue_parsing_for_children(&mut self) -> Result<(), Error> {
        let mut field_iter = match self.field_iter.take() {
            Some(iter) => iter,
            None => return Ok(()),  // Already exhausted
        };
        
        // Parse until iterator is exhausted, updating ALL fields via callback
        loop {
            match field_iter.next() {
                Some(Ok((fnum, wire_type, value_slice))) => {
                    // Update field via callback (handles both Message Body and child messages)
                    if let Some(ref mut callback) = self.field_update_callback {
                        let _ = callback(fnum, wire_type, value_slice);
                        // Ignore errors - Message Body or child might be dropped
                    }
                },
                Some(Err(e)) => {
                    // Restore iterator before returning error
                    self.field_iter = Some(field_iter);
                    return Err(e);
                },
                None => {
                    // Iterator exhausted
                    self.field_iter = None;
                    return Ok(());
                }
            }
        }
    }
    
}

// Message Body - contains field values and owns Parser State
pub struct PersonLazyImpl<'a, A: Allocator = Global> {
    // Owns parser state via Rc<RefCell<...>> - State itself is mutable
    // Note: Uses generic MessageParserState, not PersonParserState
    parser_state: Rc<RefCell<MessageParserState<'a, A>>>,
    
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

// Parent message (PersonLazyImpl)
// Key Design Decision: All methods use self: &Rc<Self> instead of &self
// This allows methods to clone Rc<Self> when needed and avoids unsafe raw pointers
impl<'a, A: Allocator + Clone> PersonLazyImpl<'a, A> {
    /// Getter for scalar message field
    /// Returns Some if field is found, None otherwise
    /// Parses until first occurrence
    pub fn address(self: &Rc<Self>) -> Option<Rc<AddressLazyImpl<'a, A>, A>> {
        // Check if already created
        if let Some(ref addr) = *self.address.borrow() {
            return Some(addr.clone());
        }
        
        // Ensure we've parsed until first occurrence of field 6
        // update_field will handle child creation when the field is encountered
        match self.ensure_field_first_occurrence(6) {
            Ok(true) => self.address.borrow().clone(),  // Child was created by update_field
            Ok(false) => None,  // Field not found
            Err(_) => None,  // Error during parsing - treat as field not found
        }
    }
    
    /// Generic method to parse until first occurrence of a field
    /// This works for any field type (scalar, repeated, scalar message)
    /// update_field() handles the field processing, so this method just ensures we've parsed until the first occurrence
    /// Returns Ok(true) if field is found, Ok(false) if field not found, Err if parsing error
    fn ensure_field_first_occurrence(
        self: &Rc<Self>,
        field_num: u32,
    ) -> Result<bool, Error> {
        // Get parser state via RefCell
        let mut parser_state = self.parser_state.borrow_mut();
        
        // Get iterator (may be None if already exhausted)
        let mut field_iter = match parser_state.field_iter.take() {
            Some(iter) => iter,
            None => return Ok(false),  // Already exhausted
        };
        
        // Parse until we find first occurrence of the target field
        loop {
            match field_iter.next() {
                Some(Ok((fnum, wire_type, value_slice))) => {
                    // Update all fields we encounter (including the target field)
                    // Note: We need to drop parser_state borrow before calling update_field
                    // because update_field might need to borrow fields, which could conflict
                    drop(parser_state);
                    self.update_field(fnum, wire_type, value_slice)?;
                    
                    if fnum == field_num {
                        // Found target field! Store iterator back (may have more fields)
                        parser_state = self.parser_state.borrow_mut();
                        parser_state.field_iter = Some(field_iter);
                        return Ok(true);
                    }
                    
                    // Re-borrow parser_state for next iteration
                    parser_state = self.parser_state.borrow_mut();
                },
                Some(Err(e)) => {
                    // Store iterator back before returning error
                    parser_state.field_iter = Some(field_iter);
                    return Err(e);
                },
                None => {
                    // Iterator exhausted, field not found
                    parser_state.field_iter = None;
                    return Ok(false);
                }
            }
        }
    }
    
    
    fn update_field(self: &Rc<Self>, field_num: u32, wire_type: u32, value_slice: &'a [u8]) -> Result<(), Error> {
        match field_num {
            1 => *self.name.borrow_mut() = parse_string(value_slice)?,
            2 => self.age.set(parse_varint(value_slice)?),
            3 => {
                // Optional field with non-Copy type - use RefCell
                let email = parse_string(value_slice)?;
                *self.email.borrow_mut() = Some(email);
            },
            4 => self.status.set(parse_varint(value_slice)?),
            5 => {
                // Optional field with Copy type - use Cell
                let score = parse_varint(value_slice)?;
                self.score.set(Some(score));
            },
            6 => {
                // address - scalar message field
                let mut address = self.address.borrow_mut();
                if let Some(ref mut addr) = *address {
                    // Child already exists - add slice to it
                    addr.add_slice(value_slice)?;
                } else {
                    // First occurrence - create child with first slice
                    let allocator = self.parser_state.borrow().allocator.clone();
                    let parent_parser_state = self.parser_state.clone();
                    let child = AddressLazyImpl::new_from_parent(value_slice, parent_parser_state, allocator);
                    *address = Some(child);
                    // Note: Weak reference will be extracted in Drop::drop to update callback
                }
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
            _ => {}
        }
        Ok(())
    }
}

impl<'a, A: Allocator + Clone> Drop for PersonLazyImpl<'a, A> {
    fn drop(&mut self) {
        // When Message Body is dropped, update callback to handle child messages
        // Extract child Weak references from fields
        let address_weak = self.address.borrow().as_ref().map(|addr| Rc::downgrade(addr));
        
        // Create new callback that captures child Weak references and uses static match-case
        let new_callback: Box<dyn FnMut(u32, u32, &'a [u8]) -> Result<(), Error> + 'a> = 
            Box::new(move |fnum: u32, _wire_type: u32, value_slice: &'a [u8]| -> Result<(), Error> {
                // Static match-case for each child message field
                match fnum {
                    6 => {
                        // address field
                        if let Some(ref addr_weak) = address_weak {
                            if let Some(addr) = addr_weak.upgrade() {
                                let _ = addr.add_slice(value_slice);
                            }
                        }
                    },
                    // Add other child message fields here as needed
                    _ => {
                        // Not a child message field - ignore
                    }
                }
                Ok(())
            });
        
        // Update callback in parser state
        self.parser_state.borrow_mut().field_update_callback = Some(new_callback);
    }
}

// Child message (AddressLazyImpl)
pub struct AddressLazyImpl<'a, A: Allocator = Global> {
    // Owns parser state via Rc<RefCell<...>> - State itself is mutable
    parser_state: Rc<RefCell<AddressParserState<'a, A>>>,
    
    // Parent parser state - strong Rc<RefCell<...>> reference (no cycle!)
    // Child needs parent's parser state to request continued parsing
    parent_parser_state: Rc<RefCell<MessageParserState<'a, A>>>,
    
    // Address fields...
    street: RefCell<String<A>>,
    city: RefCell<String<A>>,
    // ...
}

pub struct AddressParserState<'a, A: Allocator = Global> {
    field_iter: Option<FieldIterator<'a, Box<dyn Iterator<Item = &'a [u8]> + 'a, A>>>,
    // Field slices collected so far
    field_slices: OnceList<&'a [u8], A>,
    allocator: A,
}

impl<'a, A: Allocator + Clone> AddressLazyImpl<'a, A> {
    /// Create from first slice, with parent parser state reference
    /// Note: Child holds strong Rc reference to parent's parser state (not message body)
    /// This avoids cycles: Parent Message Body → Parent Parser State → (Child holds Rc to this)
    /// Returns Rc<Self> - all methods use self: &Rc<Self>
    pub(crate) fn new_from_parent(
        first_slice: &'a [u8],
        parent_parser_state: Rc<RefCell<MessageParserState<'a, A>>>,  // Strong Rc<RefCell<...>> - no cycle!
        alloc: A,
    ) -> Rc<Self, A> {
        let field_slices = OnceList::new_in(alloc.clone());
        field_slices.push(first_slice);
        
        let parser_state = Rc::new_in(RefCell::new(AddressParserState {
            field_iter: Some(FieldIterator::new(
                Box::new_in(field_slices.iter().cloned(), alloc.clone())
            )),
            field_slices,
            allocator: alloc.clone(),
        }), alloc.clone());
        
        Rc::new_in(Self {
            parser_state,
            parent_parser_state,  // Store strong Rc<RefCell<...>> reference
            street: RefCell::new(String::new_in(alloc.clone())),
            city: RefCell::new(String::new_in(alloc.clone())),
            // ...
        }, alloc)
    }
    
    /// Add additional slice from parent
    pub(crate) fn add_slice(self: &Rc<Self>, slice: &'a [u8]) -> Result<(), Error> {
        self.parser_state.borrow_mut().field_slices.push(slice);
        // Note: field_iter needs to be recreated or updated with new slices
        // This is complex because we need to preserve iterator state
        Ok(())
    }
    
    /// Ensure all fields are parsed
    /// This will request parent's parser state to continue parsing if needed
    /// 
    /// **Key Design**: Uses `continue_parsing_for_children` to trigger callbacks that add slices to child messages.
    /// This allows child messages to continue parsing even if parent Message Body is dropped.
    /// The parent's Parser State is kept alive by the child's strong Rc reference.
    fn ensure_all_fields_parsed(self: &Rc<Self>) -> Result<(), Error> {
        // Request parent's parser state to continue parsing
        // This works even if parent Message Body is dropped, because:
        // 1. Child holds strong Rc reference to parent's Parser State
        // 2. continue_parsing_for_children doesn't require parent Message Body to be alive
        // 3. Callbacks (child_update_callbacks) already add slices to child messages via add_slice
        // 4. All registered child message fields receive their slices when any child calls this method
        self.parent_parser_state.borrow_mut().continue_parsing_for_children()?;
        
        // Now parse our own fields from all collected slices
        // Recreate iterator with all slices
        let mut parser_state = self.parser_state.borrow_mut();
        let field_iter = match parser_state.field_iter.take() {
            Some(_) => {
                // Recreate with all slices (previous iterator may be stale)
                Some(FieldIterator::new(
                    Box::new_in(parser_state.field_slices.iter().cloned(), parser_state.allocator.clone())
                ))
            },
            None => return Ok(()),  // Already parsed
        };
        
        parser_state.field_iter = field_iter;
        drop(parser_state);  // Release borrow before parsing
        
        // Parse until exhausted
        loop {
            let mut parser_state = self.parser_state.borrow_mut();
            let field_iter = match parser_state.field_iter.as_mut() {
                Some(iter) => iter,
                None => break,  // Already parsed
            };
            
            match field_iter.next() {
                Some(Ok((field_num, wire_type, value_slice))) => {
                    drop(parser_state);  // Release borrow before calling update_field
                    self.update_field(field_num, wire_type, value_slice)?;
                },
                Some(Err(e)) => {
                    return Err(e);
                },
                None => {
                    // Iterator exhausted
                    parser_state.field_iter = None;
                    break;
                }
            }
        }
        
        Ok(())
    }
}
```

**Note**: For detailed discussion of design alternatives and decision rationale, see `lazy-parsing-state-design-discussion.md`.

## Alternative: Arena Approach with `Rc`

Instead of using raw pointers with lifetime parameters, we can use an Arena approach with `Rc<T, A>`. **Important**: Messages must be created on-demand (lazy), not all at arena creation time.

**Design**: Messages hold strong references to Arena, Arena only owns the allocator.

```rust
struct MessageArena<A: Allocator = Global> {
    // Arena only owns the allocator
    // Messages are NOT stored in arena (no circular dependency)
    allocator: A,
}

struct PersonLazyImpl<A: Allocator = Global> {
    allocator: A,
    // Strong reference to arena (no Weak needed - no cycle!)
    arena: Rc<MessageArena<A>>,
    // Child message - created on-demand, using Rc
    address: RefCell<Option<Rc<AddressLazyImpl<A>, A>>>,
    // Field iterator for lazy parsing
    field_iter: RefCell<Option<FieldIterator<...>>>,
    // ...
}

struct AddressLazyImpl<A: Allocator = Global> {
    allocator: A,
    // Strong reference to arena (no Weak needed - no cycle!)
    arena: Rc<MessageArena<A>>,
    // Parent message - using Weak to avoid cycle (Parent → Child → Parent)
    parent: Weak<PersonLazyImpl<A>, A>,  // Weak to break cycle
    // Field iterator for lazy parsing
    field_iter: RefCell<Option<FieldIterator<...>>>,
    // ...
}
```

Construction - Arena created first, then top-level message:

```rust
impl<A: Allocator + Clone> MessageArena<A> {
    fn new(data: &[u8], alloc: A) -> (Rc<Self, A>, Rc<PersonLazyImpl<A>, A>) {
        // Create arena first (no Rc::new_cyclic_in needed!)
        let arena = Rc::new_in(MessageArena {
            allocator: alloc.clone(),
        }, alloc.clone());
        
        // Create top-level message with arena reference
        let person = Rc::new_in(PersonLazyImpl {
            allocator: alloc.clone(),
            arena: arena.clone(),  // Strong reference - no cycle!
            address: RefCell::new(None),  // Child will be created on-demand
            field_iter: RefCell::new(Some(FieldIterator::new(...))),
            // ...
        }, alloc.clone());
        
        (arena, person)
    }
}
```

Child message created on-demand when parent's getter is called:

**Key Design Decision**: All methods use `self: &Rc<Self>` instead of `&self`. This allows methods to clone `Rc<Self>` when needed to create child messages.

```rust
impl<A: Allocator + Clone> PersonLazyImpl<A> {
    // Note: &self is changed to self: &Rc<Self>
    pub fn address(self: &Rc<Self>) -> Option<Rc<AddressLazyImpl<A>, A>> {
        // Check if already created
        if let Some(ref addr) = *self.address.borrow() {
            return Some(addr.clone());
        }
        
        // Parse until first occurrence of field 6
        self.ensure_field_6_first_occurrence()?;
        
        // Get the created child (should exist now)
        self.address.borrow().clone()
    }
    
    fn ensure_field_6_first_occurrence(self: &Rc<Self>) -> Option<()> {
        // Parse until we find first occurrence of field 6
        let mut field_iter = self.field_iter.borrow_mut().take()?;
        
        loop {
            match field_iter.next() {
                Some(Ok((field_num, wire_type, value_slice))) => {
                    self.update_field(field_num, wire_type, value_slice)?;
                    
                    if field_num == 6 {
                        // Found field 6! Create child on-demand
                        // Use Weak to avoid cycle: Parent → Child → Parent
                        let address = Rc::new_in(AddressLazyImpl {
                            allocator: self.allocator.clone(),
                            arena: self.arena.clone(),  // Clone arena reference
                            parent: Rc::downgrade(self),  // Weak reference to avoid cycle
                            field_iter: RefCell::new(Some(FieldIterator::new(...))),
                            // ...
                        }, self.allocator.clone());
                        
                        // Store in parent
                        *self.address.borrow_mut() = Some(address.clone());
                        
                        // Store iterator back
                        *self.field_iter.borrow_mut() = Some(field_iter);
                        return Some(());
                    }
                },
                None => return None,
            }
        }
    }
    
    fn update_field(self: &Rc<Self>, field_num: u32, wire_type: u32, value_slice: &[u8]) -> Result<(), Error> {
        // ... field update logic ...
    }
}
```

Usage - no lifetime parameters needed, but users must use `Rc`:

```rust
impl<A: Allocator + Clone> AddressLazyImpl<A> {
    fn ensure_all_fields_parsed(self: &Rc<Self>) -> Result<(), Error> {
        // Access parent via Weak - upgrade to Rc when needed
        // Note: parent methods take &Rc<Self>, so we need to upgrade Weak to Rc
        if let Some(parent_rc) = self.parent.upgrade() {
            parent_rc.continue_parsing_for_field(6)?;
        }
        // If parent was dropped, we can't continue parsing - this is expected behavior
        Ok(())
    }
}

// User code:
fn example() {
    let (arena, person_rc) = MessageArena::new(data, alloc);
    
    // All methods require &Rc<Self>
    let address_rc = person_rc.address();  // Returns Option<Rc<AddressLazyImpl>>
    
    // Child can access parent
    address_rc.unwrap().ensure_all_fields_parsed();
    
    // Arena is kept alive by messages (via Rc)
    // If user drops arena, messages still keep it alive
    // Arena is only dropped when all messages are dropped
}
```

**Benefits of using `self: &Rc<Self>`**:
- ✅ Easy to create `Weak` from `Rc` inside methods (`Rc::downgrade(self)`)
- ✅ Simplifies child creation (can pass `Rc::downgrade(self)` as parent to avoid cycle)
- ✅ Consistent API (all methods use same pattern)
- ✅ No need for complex mechanisms to get `Rc<Self>` from `&self`

**Trade-off**:
- ❌ Users must always work with `Rc` (cannot use bare `PersonLazyImpl`)
- ❌ All method calls require dereferencing `Rc` first (but this is automatic with method calls)

**Key Benefits of Arena + Rc Approach**:

1. **No lifetime parameters needed**:
   - `PersonLazyImpl<A>` instead of `PersonLazyImpl<'a, A>`
   - `AddressLazyImpl<A>` instead of `AddressLazyImpl<'a, A>`
   - Simpler type signatures

2. **Type-safe parent-child relationships**:
   - `Rc` provides automatic lifetime management
   - No `unsafe` blocks needed for parent access
   - No circular dependency (Arena doesn't hold messages)
   - Parent → Child: Strong `Rc` (parent owns child)
   - Child → Parent: `Weak` (breaks cycle: Parent → Child → Parent would be a cycle)
   - Strong `Rc` for messages → Arena is fine (no cycle: Messages → Arena, but Arena doesn't hold Messages)

3. **Flexible user references**:
   - User can clone `Rc` and "discard" original arena reference
   - Messages keep arena alive via `Rc` (arena is only dropped when all messages are dropped)
   - More flexible than raw pointer approach

4. **Natural message relationships**:
   - Parent → Child: Strong `Rc` (parent owns child semantically)
   - Child → Parent: `Weak` (child needs parent, but uses `Weak` to avoid cycle)
   - Messages → Arena: Strong `Rc` (messages keep arena alive, no cycle because Arena doesn't hold messages)

5. **Simple construction**:
   - No `Rc::new_cyclic_in` needed (Arena created first, then messages)
   - `Weak` used only for child → parent (breaks parent-child cycle)

**Trade-offs of Arena + Rc Approach**:

✅ Pros:
- No lifetime parameters (simpler API)
- Flexible reference management for users
- Type-safe (no `unsafe` for parent access)
- Simple construction (no `Rc::new_cyclic_in`)
- `Weak` used only for child → parent (breaks parent-child cycle)
- Arena lifetime is clear (kept alive by messages)

❌ Cons:
- Reference counting overhead (`Rc` operations)
- Allocation overhead (`Rc` itself is heap-allocated)
- Arena cannot enumerate messages (but this might not be needed)
- Messages must hold arena reference (but they might need allocator access anyway)

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
    pub fn name(self: &Rc<Self>) -> std::cell::Ref<'_, String<A>> {
        // Ensure entire iterator is parsed before returning Ref
        // This guarantees:
        // 1. No RefMut is active (borrow safety)
        // 2. The value is final and confirmed (not intermediate)
        self.ensure_all_fields_parsed();
        self.name.borrow()  // Safe: no RefMut can be active, value is confirmed
    }
    
    fn ensure_all_fields_parsed(self: &Rc<Self>) {
        // Check if iterator still exists (not yet consumed/parsed)
        let mut parser_state = self.parser_state.borrow_mut();
        let field_iter = match parser_state.field_iter.as_mut() {
            Some(iter) => iter,
            None => return,  // Already parsed completely - values are confirmed
        };
        
        // Parse until iterator is exhausted
        // During this process, fields may be updated multiple times (intermediate values)
        loop {
            match field_iter.next() {
                Some(Ok((field_num, wire_type, value_slice))) => {
                    drop(parser_state);  // Release borrow before calling update_field
                    self.update_field(field_num, wire_type, value_slice)?;
                    parser_state = self.parser_state.borrow_mut();  // Re-borrow for next iteration
                },
                Some(Err(_)) => {
                    // Error handling - iterator state is preserved in parser_state
                    return;
                },
                None => {
                    // Iterator exhausted - store None to indicate parsing is complete
                    // All field values are now final and confirmed
                    parser_state.field_iter = None;
                    return;
                }
            }
        }
    }
    
    fn update_field(self: &Rc<Self>, field_num: u32, wire_type: u32, value_slice: &'a [u8]) -> Result<(), Error> {
        // This may update fields multiple times during parsing
        // Values updated here are intermediate until parsing is complete
        match field_num {
            1 => *self.name.borrow_mut() = parse_string(value_slice)?,
            // ... other fields
        }
        Ok(())
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
