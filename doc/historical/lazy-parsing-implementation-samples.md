# Lazy Parsing Implementation Samples (Archived)

This file contains detailed sample code examples from the lazy parsing design that have been implemented.
The actual implementation can be found in:
- `sandbox/src/generated/lazy_parser.rs` - Core parsing infrastructure
- `sandbox/src/generated/person.rs` - PersonLazyImpl implementation
- `sandbox/src/generated/address.rs` - AddressLazyImpl implementation

**Status**: ✅ Implemented (Phase 1-4 complete as of 2025-01)

**Archived Date**: 2025-01

---

## Implementation Sample Code

### Core Data Structure

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
```

### FieldIterator Implementation

```rust
/// Iterator over protobuf fields in slices.
///
/// Can be paused and resumed, making it easy to parse incrementally.
/// Tracks the current slice and position within that slice.
pub struct FieldIterator<'a> {
    /// Iterator over the slices
    /// This iterator maintains its own state
    slice_iter: std::boxed::Box<dyn Iterator<Item = &'a [u8]> + 'a>,
    /// Current slice being parsed
    current_slice: Option<&'a [u8]>,
    /// Current position within the current slice
    position: usize,
}

impl<'a> FieldIterator<'a> {
    /// Create a new FieldIterator from any iterator over slices
    /// The iterator is created once and maintains its state - no need to recreate it
    pub fn new(slice_iter: std::boxed::Box<dyn Iterator<Item = &'a [u8]> + 'a>) -> Self {
        Self {
            slice_iter,
            current_slice: None,
            position: 0,
        }
    }
}

impl<'a> Iterator for FieldIterator<'a> {
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

### Parsing Strategy Examples

#### 1. Scalar Non-Message Fields

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

#### 2. Repeated Fields

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

#### 3. Scalar Message Fields

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
```

See the actual implementation in:
- `sandbox/src/generated/lazy_parser.rs` for FieldIterator and MessageParserState
- `sandbox/src/generated/person.rs` for PersonLazyImpl
- `sandbox/src/generated/address.rs` for AddressLazyImpl

