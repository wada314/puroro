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

**Status**: ✅ **Implemented** (Phase 1-4 complete as of 2025-01)

The actual implementation can be found in:
- `sandbox/src/generated/lazy_parser.rs` - Core parsing infrastructure (FieldIterator, MessageParserState)
- `sandbox/src/generated/person.rs` - PersonLazyImpl implementation
- `sandbox/src/generated/address.rs` - AddressLazyImpl implementation

**Key Design Points**:
- Message body owns parser state via `Rc<RefCell<MessageParserState>>`
- Field-level interior mutability: `RefCell`/`Cell` for scalar fields, `OnceList` for repeated fields
- All methods use `self: &Rc<Self>` to allow cloning when needed
- Field getters ensure parsing is complete before returning values (for scalar fields)

For detailed sample code, see `historical/lazy-parsing-implementation-samples.md`.

### Benefits

1. **Simple Structure**: No wrapper struct needed - direct use of the struct
2. **Message-Specific Methods**: Can implement field accessors directly on `PersonLazyImpl`
3. **Trait Implementation**: Can implement traits directly on `PersonLazyImpl`
4. **Fine-Grained Protection**: Each field is protected independently - no struct-level `RefCell`
5. **OnceList's Built-in Interior Mutability**: `OnceList` doesn't need `RefCell` wrapping - it already has `push(&self)`
6. **No Borrow Conflicts**: Reading from `OnceList` and calling `push()` both use `&self`, so no conflicts
7. **Clean API**: Direct access to repeated fields, `Ref`/`Cell::get()` for scalar fields

## Core Data Structure

**Status**: ✅ **Implemented** in `sandbox/src/generated/lazy_parser.rs`

The core data structures are:

1. **FieldIterator**: Iterator over protobuf fields in slices that can be paused and resumed
   - Tracks current slice and position within that slice
   - Accepts any `Iterator<Item = &'a [u8]>`
   - Stored in `RefCell<Option<FieldIterator>>` because `Iterator::next()` requires `&mut self`

2. **MessageParserState**: Parser state that can be shared between message bodies and child messages
   - Contains `FieldIterator` and field update callback
   - Generic across all message types
   - Wrapped in `Rc<RefCell<...>>` for sharing

See the actual implementation in `sandbox/src/generated/lazy_parser.rs` for details.

## Parsing Strategy

### 1. Scalar Non-Message Fields (e.g., `age`)

**Requirement**: Must parse entire message (to get last value, since later fields overwrite)

**Implementation**: Field getters call `ensure_all_fields_parsed()` before returning values. This ensures we get the final confirmed value (last occurrence) rather than intermediate values.

See `PersonLazyImpl::age()` in `sandbox/src/generated/person.rs`.

### 2. Repeated Fields (e.g., `scores`, `addresses`)

**Requirement**: Parse only until first occurrence of target field, but still update other fields

**Note**: This uses the generic `ensure_field_first_occurrence` method, which works for any field type.

**Current Implementation Status (as of Phase 4)**:
- The getters (`scores()`, `addresses()`) currently call `ensure_all_fields_parsed()` before returning `&OnceList`.
- True lazy parsing (parsing elements on-demand when accessed via iterator) is **not yet implemented**.
- This is a **future enhancement** that needs further design discussion.

**Design Challenge**:
The challenge is that `OnceList`'s `iter()` method only returns already-parsed elements. To implement true lazy parsing, we need a way to trigger parsing when elements are accessed via the iterator. However:
- `OnceList` is from an external crate (`once-list2`), so we can't modify its iterator behavior
- The `Repeated` trait implementation for `&OnceList` is also in an external crate (`puroro`)
- We need to parse elements on-demand when `get()`, `len()`, or `iter()` is called

**Possible Approaches for Future Implementation**:
1. **Custom wrapper type**: Create a wrapper around `OnceList` that implements `Repeated` trait and triggers parsing in `get()`, `len()`, and `iter_box()` methods
2. **Custom iterator wrapper**: Wrap `OnceList::iter()` with a custom iterator that parses on-demand during iteration
3. **Modify Repeated trait implementation**: Move the `Repeated` trait implementation from external crate to allow custom parsing logic (significant architectural change)

**Current Implementation**: The getters (`scores()`, `addresses()`) call `ensure_all_fields_parsed()` before returning `&OnceList`. See `PersonLazyImpl::scores()` and `PersonLazyImpl::addresses()` in `sandbox/src/generated/person.rs`.

For future true lazy parsing implementation examples, see `historical/lazy-parsing-implementation-samples.md`.

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

**Implementation Details**:
- `MessageParserState` contains `FieldIterator` and field update callback
- Callback is updated in `Drop::drop` to handle child messages after parent Message Body is dropped
- Child messages collect slices via `add_slice()` and parse them when `ensure_all_fields_parsed()` is called

See the actual implementation in:
- `sandbox/src/generated/person.rs` - `PersonLazyImpl` and `Drop` implementation
- `sandbox/src/generated/address.rs` - `AddressLazyImpl` and `AddressParserState`

For detailed sample code, see `historical/lazy-parsing-implementation-samples.md`.

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
