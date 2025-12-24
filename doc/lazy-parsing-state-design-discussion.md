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

