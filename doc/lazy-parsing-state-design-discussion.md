# Lazy Parsing State Design - Discussion History

This document contains the discussion history and design exploration for the lazy parsing state design. For the current finalized design decisions, see `lazy-parsing-state-design.md`.

## Key Design Challenges

### 1. Parent Reference: Child needs to hold reference to parent

This is a classic Rust problem. Common solutions include:

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

### 2. Iterator State Management

When parent adds slices to child, child's iterator needs to see them. Options:
- Recreate iterator from `field_slices` each time (simple but may re-parse)
- Use a more sophisticated iterator that can accept new slices dynamically

### 3. Circular Reference Prevention

Parent holds `RefCell<Option<AddressLazyImpl>>`, child holds reference to parent. This is safe because:
- Child's reference to parent is immutable (no mutation through it)
- Parent's reference to child is interior mutable (`RefCell`)
- No actual circular ownership (child doesn't own parent, just references it)

## Re-evaluating the Need for Parent Reference

The only reason the child needs a parent reference is:
- When `ensure_all_fields_parsed()` is called, it needs to request the parent to continue parsing to collect all field slices

### Why Closure/Callback Doesn't Solve the Problem

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

### Why We Must Preserve Lazy Parsing

**Critical Requirement**: We must preserve true lazy parsing. If the user only needs `address.street` (the first field of the address message), we should NOT parse the entire `address` message. The parent's getter collecting all slices upfront would violate this requirement.

### The Realistic Solution: Raw Pointer with Safety Guarantees

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

## Alternative: Arena Approach with `Rc` - Discussion

Instead of using raw pointers with lifetime parameters, we can use an Arena approach with `Rc<T, A>`. **Important**: Messages must be created on-demand (lazy), not all at arena creation time.

### Design Question: Should Arena hold messages, or should messages hold Arena?

**Option A: Arena holds messages (initial design)**
- Arena holds all messages as `Rc`
- Messages are added on-demand as they are parsed
- Messages hold `Weak<MessageArena>` to avoid cycles
- Requires `Rc::new_cyclic_in` for construction

**Option B: Messages hold Arena (final design)**
- Arena only owns the allocator
- Messages are NOT stored in arena (no circular dependency)
- Messages hold strong `Rc<MessageArena>`
- No `Rc::new_cyclic_in` needed

**Why Option B was chosen**:
- ✅ No need for `Weak` references (messages hold `Rc<MessageArena>`)
- ✅ No circular dependency (Arena doesn't hold messages)
- ✅ Simpler construction (no `Rc::new_cyclic_in` needed)
- ✅ Arena lifetime is clear (messages keep arena alive via `Rc`)
- ❌ Arena cannot enumerate messages (but this might not be needed)
- ❌ Messages must hold arena reference (but they might need allocator access anyway)

### Parent-Child Reference Discussion

**Initial approach**: Child held strong `Rc` reference to parent
- **Problem**: This creates a cycle (Parent → Child → Parent)
- **Solution**: Child holds `Weak` reference to parent instead

**Final design**:
- Parent → Child: Strong `Rc` (parent owns child semantically)
- Child → Parent: `Weak` (breaks cycle: Parent → Child → Parent would be a cycle)
- Messages → Arena: Strong `Rc` (messages keep arena alive, no cycle because Arena doesn't hold messages)

### Detailed Arena Approach Implementation

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

## Comparison with Other Rust Patterns

| Pattern | Safety | Overhead | Complexity | Our Applicability |
|---------|--------|----------|------------|-------------------|
| `&'a Parent` | ✅ Safe | None | Low | ⚠️ Requires arena/restructuring |
| `Rc<RefCell<Parent>>` + `Weak` | ✅ Safe | Reference counting | Medium | ❌ Overkill, unnecessary |
| Arena/ID | ✅ Safe | Index lookup | High | ⚠️ Major restructuring |
| Raw pointer | ⚠️ Unsafe (but safe with invariants) | None | Low | ✅ Works well |
| **Arena + `Rc` (messages hold Arena)** | ✅ **Safe** | **Reference counting** | **Medium** | ✅ **Good alternative** |
| No reference (callbacks) | ✅ Safe | Function pointer | Medium | ⚠️ Less ergonomic |
| Trait object | ✅ Safe | Dynamic dispatch | Medium | ❌ Doesn't solve problem |

## Final Recommendation Discussion

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
   - ✅ Simple construction (no `Rc::new_cyclic_in` needed)
   - ✅ `Weak` used only for child → parent (breaks parent-child cycle)
   - ❌ Reference counting overhead
   - ❌ Allocation overhead (each message wrapped in `Rc`)
   - ❌ Arena cannot enumerate messages (but this might not be needed)
   - **Best for**: API simplicity, flexibility, when overhead is acceptable

**Recommendation**: 
- For immediate implementation: Use raw pointer with well-documented safety invariants (parent outlives child via `'a`)
- If API simplicity and user flexibility are more important than performance: Consider Arena + `Rc` approach

