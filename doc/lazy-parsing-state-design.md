# Lazy Parsing State Design

This document describes the finalized design for tracking "in-parsing" state in lazy Protocol Buffer message deserialization.

**Note**: For discussion history and design exploration, see `lazy-parsing-state-design-discussion.md`.

## Design Overview

Each lazy message owns a **parser state** (`MessageParserStateRef`) that tracks parsing progress through the currently available input slices. While parsing for one field, we update ALL fields we encounter along the way (protobuf semantics). This allows us to pause parsing and resume later when needed.

**Key Design Principles**:
- **Unified interface**: All message types (top-level and child) use the same structure and methods
- **Flexible message hierarchy**: Any message can be used as both top-level and child message

**Critical Insight**: For scalar message fields (e.g., Person → Address → Location):
- Message fields hold a reference to the parent's parser state (not message body)

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

**Status**: ✅ Implemented

The actual implementation can be found in:
- `puroro/src/lazy_parser.rs` - Core parsing infrastructure (`FieldIterator`, `MessageParserStateRef`, `MessageParserStateInner`)
- `puroro/src/repeated_lazy.rs` - `LazyRepeated` adapter
- `sandbox/src/generated/person.rs` - reference generated-style `Person*` implementations
- `sandbox/src/generated/address.rs` - reference generated-style `Address*` implementations

**Key Design Points**:
- Message body owns parser state via `MessageParserStateRef` (internally `Rc<RefCell<MessageParserStateInner>>`)
- Field-level interior mutability: `RefCell`/`Cell` for scalar fields, `OnceList` for repeated fields
- Field getters ensure parsing is complete before returning values (for scalar fields)
- **Unified interface**: All message types (`PersonLazyImpl`, `AddressLazyImpl`, etc.) have the same structure and interface
  - All messages have `parent_parser_state: Option<MessageParserStateRef<...>>` (None for top-level, Some(...) for child messages)
  - All messages use the same `new(slice, alloc, parent_parser_state)` constructor pattern
  - All messages can be used as both top-level and child messages

### Benefits

1. **Simple Structure**: No wrapper struct needed - direct use of the struct
2. **Message-Specific Methods**: Can implement field accessors directly on `PersonLazyImpl`
3. **Trait Implementation**: Can implement traits directly on `PersonLazyImpl`
4. **Fine-Grained Protection**: Each field is protected independently - no struct-level `RefCell`
5. **OnceList's Built-in Interior Mutability**: `OnceList` doesn't need `RefCell` wrapping - it already has `push(&self)`
6. **No Borrow Conflicts**: Reading from `OnceList` and calling `push()` both use `&self`, so no conflicts
7. **Clean API**: Direct access to repeated fields, `Ref`/`Cell::get()` for scalar fields

## Core Data Structure

**Status**: ✅ Implemented in `puroro/src/lazy_parser.rs`

The core data structures are:

1. **FieldIterator**: Iterator over protobuf fields in slices that can be paused and resumed
   - Tracks current slice and position within that slice
   - Accepts any `Iterator<Item = &'a [u8]>`
   - Internally stores per-slice iterators so additional slices can be appended

2. **MessageParserStateInner / MessageParserStateRef**: parser state shared between message bodies and child messages
   - Contains a `FieldIterator` and a field update callback
   - Stored as `Rc<RefCell<MessageParserStateInner>>` and accessed via `MessageParserStateRef`

See the actual implementation in `puroro/src/lazy_parser.rs` for details.

## Parsing Strategy

### 1. Scalar Non-Message Fields (e.g., `age`)

**Requirement**: Must parse entire message (to get last value, since later fields overwrite)

**Implementation**: Field getters call `ensure_all_fields_parsed()` before returning values. This ensures we get the final confirmed value (last occurrence) rather than intermediate values.

See `PersonLazyImpl::age()` in `sandbox/src/generated/person.rs`.

### 2. Repeated Fields (e.g., `scores`, `addresses`)

**Requirement**: Parse only until first occurrence of target field, but still update other fields

**Note**: This uses the generic `ensure_field_first_occurrence` method, which works for any field type.

**Status**: ✅ **Implemented** (2025-01)

The getters (`scores()`, `addresses()`) return `LazyRepeated` wrapper that implements true lazy parsing:
- Elements are parsed on-demand when accessed via `iter()`, `get()`, `len()`, etc.
- Parsing stops when the required number of elements is found (incremental parsing)
- All fields encountered during parsing are still updated (protobuf semantics)

See the actual implementation in:
- `puroro/src/repeated_lazy.rs` - `LazyRepeated` and `LazyRepeatedIter` implementation
- `sandbox/src/generated/person.rs` - `PersonLazyImpl::scores()` and `addresses()` getters

For detailed implementation details and known limitations, see the "Implementation: True Lazy Parsing for Repeated Fields" section below.

### 3. Scalar Message Fields (e.g., `address`)

**Requirement**: 
- Must collect all occurrences of the field number at the same nesting level (when needed)
- Initial access should parse until first occurrence, return `Some(child)` or `None`
- When child's field getter is called (e.g., `address().street()`), the child may request parent to continue parsing to collect slices based on the field access pattern (e.g., scalar fields that need the last value require all occurrences, but there are many other patterns)

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

**Implementation Details**:
- `MessageParserStateInner` contains `FieldIterator` and field update callback
- Callback is updated in `Drop::drop` to handle child messages after parent Message Body is dropped
- Child messages collect slices via `add_slice()` and parse them when `ensure_all_fields_parsed()` is called
- **Unified constructor**: All message types use `new(slice, alloc, parent_parser_state)` where:
  - `parent_parser_state: None` for top-level messages
  - `parent_parser_state: Some(parent_parser_state)` for child messages
- **Parser state sharing**: Child messages hold `Option<MessageParserStateRef<...>>` to request continued parsing from parent

See the actual implementation in:
- `sandbox/src/generated/person.rs` - `PersonLazyImpl` implementation
- `sandbox/src/generated/address.rs` - `AddressLazyImpl` implementation (same pattern as PersonLazyImpl)

**Note**: For detailed discussion of design alternatives and decision rationale, including the Arena approach, see `lazy-parsing-state-design-discussion.md`.

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
| **Scalar message fields** | `RefCell<Option<MessageLazyImpl>>` | Set/replace and route additional slices | `Option<non-Copy>` requires `RefCell` |

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
- Field getters call `ensure_all_fields_parsed()` before returning values
- `ensure_all_fields_parsed()` parses the entire iterator and updates all fields
- `update_field()` handles field-specific parsing logic

See `PersonLazyImpl::age()`, `PersonLazyImpl::ensure_all_fields_parsed()`, and `PersonLazyImpl::update_field()` in `sandbox/src/generated/person.rs` for the actual implementation.

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

---

## Implementation: True Lazy Parsing for Repeated Fields

### Status: ✅ Implemented

**Implementation Complete (2025-01)**: Repeated fields (`scores`, `addresses`) now support true lazy parsing via the `LazyRepeated` wrapper type.

**Current Implementation**:
- `PersonLazyImpl::scores()` and `addresses()` return `LazyRepeated` wrapper (not `&OnceList`)
- `LazyRepeated` implements the `Repeated` trait, maintaining API compatibility
- Elements are parsed on-demand when accessed via `iter()`, `get()`, `len()`, etc.
- Parsing stops when the required number of elements is found (incremental parsing)

**Code Example** (from `sandbox/src/generated/person.rs`):
```rust
pub fn scores(&self) -> LazyRepeated<'slice, '_, i32, A> {
    LazyRepeated::new(self.parser_state.clone(), &self.scores)
}
```

### Implementation Details

**LazyRepeated Wrapper**:
- Holds a reference to the parent's `MessageParserStateRef`
- Holds a reference to the underlying `OnceList`
- Implements the `Repeated` trait by delegating to `OnceList` but triggering parsing when needed

**Key Methods**:
1. **`ensure_at_least(needed: usize)`**: Parses until at least `needed` elements are available
2. **`ensure_fully_parsed()`**: Parses all remaining fields (used for `len()` and `iter_box()`)
3. **`iter()`**: Returns `LazyRepeatedIter` that triggers parsing on-demand in `next()`

**Repeated Trait Implementation**:
- `get(index)`: Calls `ensure_at_least(index + 1)` then delegates to `OnceList::get()`
- `len()`: Calls `ensure_fully_parsed()` then delegates to `OnceList::len()`
- `is_empty()`: Calls `ensure_at_least(1)` then checks if any elements exist
- `iter()`: Returns `LazyRepeatedIter` wrapper
- `iter_box()`: Calls `ensure_fully_parsed()` then collects all elements into a boxed iterator

**LazyRepeatedIter Iterator**:
- Wraps the underlying `OnceList::iter()` iterator
- When iterator is exhausted, checks if parent parser has more fields
- If yes, triggers parsing via `ensure_at_least()` and recreates iterator with newly parsed elements
- Provides seamless on-demand parsing during iteration

### How It Works

**Pattern**: Same as scalar child message fields - object triggers parent's parse until condition is met.

1. **Getter Call**: `person.scores()` returns `LazyRepeated` immediately (no parsing)
2. **Element Access**: When `scores().get(5)` is called:
   - `LazyRepeated::get()` calls `ensure_at_least(6)` (need 6 elements for index 5)
   - `ensure_at_least()` calls `parent_parser_state.parse_until_with_callback(...)`
   - Parent parser continues parsing, updating all fields via callback
   - Target field (`scores`) gets new elements added to `OnceList` via `push()`
   - Loop continues until 6 elements are available or parser is exhausted
3. **Iterator Access**: When `scores().iter().next()` is called:
   - `LazyRepeatedIter::next()` checks if inner iterator has more elements
   - If exhausted, calls `ensure_at_least(current_count + 1)` to get one more element
   - Recreates iterator and skips already-seen elements
   - Returns the next newly-parsed element

### Known Limitations and Future Optimizations

1. **Efficiency**: the current implementation is correct, but has avoidable overhead (e.g., repeated `.iter().count()` checks and iterator recreation in `LazyRepeatedIter`).

2. **Unused Field**: (resolved) `LazyRepeated` no longer stores `_field_number` since it was unused.

See `doc/lazy-parsing-next-steps.md` for detailed next steps and optimization opportunities.

### Key Insight: Parallel with Scalar Child Message Fields

**Important Observation**: The repeated field case is **similar to the scalar child message field case** (`address` field).

**Scalar Child Message Field Pattern**:
- The child message object (`AddressLazyImpl`) holds a reference to the parent's parser state
- When the child's field getter is called (e.g., `address().street()`), the child calls `ensure_all_fields_parsed()`
- `ensure_all_fields_parsed()` calls `parent_parser_state.ensure_all_fields_parsed_with_callback()` to collect all slices
- The parent's parser state continues parsing and collects all occurrences of the child's field number
- The child's `Drop::drop` updates the parent's callback to route new slices to the child via `add_slice()`
- **Key**: Fields are parsed ON-DEMAND when accessed, not when the child object is created
- **Important**: The child message requests parent to continue parsing based on the child's field access pattern. There are many patterns (e.g., scalar fields that need the last value require all occurrences, but other patterns may require different amounts). The current implementation collects all occurrences for simplicity, but in principle it could be optimized based on the specific field access pattern.

**Repeated Field Pattern (Implemented)**:
- The repeated field has its own "object" (`LazyRepeated` wrapper) that holds a reference to the parent's parser state
- When the repeated field object needs more elements (e.g., `iter().next()`, `get(5)`, `len()`), it triggers the parent to continue parsing
- The parent's parser state continues parsing and collects occurrences of the repeated field's field number
- Each occurrence is added to the underlying `OnceList` via `push()`
- **Key**: Elements are parsed ON-DEMAND when accessed, not when the getter is called
- **Implementation**: See `puroro/src/repeated_lazy.rs` for the actual implementation

**Key Similarity**:
- Both cases need an "object" that can **trigger the owner message's parse until a condition is met**
- Both parse fields/elements ON-DEMAND when accessed, not upfront
- The child/scalar message object triggers parsing when its fields are accessed
- The repeated field object should trigger parsing when its elements are accessed

**Key Insight - They Are The Same Pattern**:
Both patterns are fundamentally the same: **trigger the parent's parse until a condition is met**.
- Scalar child message: When a field getter is accessed (e.g., `address().street()` calls `ensure_all_fields_parsed()` which collects all slices from parent), the child requests parent to continue parsing until all occurrences are collected. The condition depends on the field type and access pattern (scalar fields need all occurrences to get the last value, but there are many other patterns).
- Repeated field: When an element accessor is called (e.g., `scores().get(5)`), the `LazyRepeated` wrapper requests parent to continue parsing until the required number of elements is found (implemented)

The difference is only in **what condition triggers the collection**:
- Scalar child message: Condition depends on the child message's field access pattern (e.g., "all occurrences needed" for scalar fields that need the last value, but there are many other patterns depending on which fields are accessed)
- Repeated field: Condition is "N elements needed" (where N depends on the access pattern: `get(5)` → need 6 elements, `len()` → need all elements, `iter().next()` → need 1 element)

This insight led to the implementation of the **`LazyRepeated` wrapper type** for repeated fields:
1. Holds a reference to the parent's parser state (similar to `AddressLazyImpl` holding `parent_parser_state`)
2. Holds a reference to the underlying `OnceList`
3. Implements the `Repeated` trait by delegating to `OnceList` but triggering parsing when needed

**Implementation**: See `puroro/src/repeated_lazy.rs` for the complete implementation.

### Implementation Summary

The implementation follows the same pattern as scalar child message fields:
- Both use an object (wrapper) that holds a reference to the parent's parser state
- Both trigger parent's parsing when needed (field access or element access)
- The key difference is the condition: scalar child messages need all occurrences, repeated fields need N elements

The `LazyRepeated` wrapper successfully implements true lazy parsing for repeated fields, maintaining API compatibility through the `Repeated` trait implementation.
