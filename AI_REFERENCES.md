# AI References for Puroro Project

## Project Overview
Puroro is a Rust-idiomatic implementation of Google Protocol Buffers, focusing on:
- Precise Protocol Buffers specification support
- Rust-idiomatic APIs that leverage the type system
- Memory safety without sacrificing performance
- Lazy parsing support for on-demand deserialization

## Lazy Parser Implementation - Session Summary (2025-01)

### Current Implementation Status

**Status**: ✅ **Phase 1-4 Complete** (as of 2025-01)

**Key Achievements**:
1. **Unified Interface Pattern**: All message types (`PersonLazyImpl`, `AddressLazyImpl`, etc.) now use the same structure and interface
2. **Multiple Parse Support**: Input slices are stored in `field_slices`, enabling getters to be called multiple times
3. **Flexible Message Hierarchy**: Any message can be used as both top-level and child message

### Unified Message Structure

All lazy message implementations follow the same pattern:

```rust
pub struct MessageLazyImpl<'a, A: Allocator + Clone + 'a = Global> {
    /// Owns parser state via Rc<RefCell<...>>
    parser_state: Rc<RefCell<MessageParserState<'a, A>>>,
    
    /// Parent parser state - None for top-level, Some(...) for child messages
    parent_parser_state: Option<Rc<RefCell<MessageParserState<'a, A>>>>,
    
    /// Input slices stored for multiple parse support
    field_slices: OnceList<&'a [u8], A>,
    
    // ... field values ...
}
```

### Unified Constructor Pattern

All messages use a single `new()` method:

```rust
pub fn new(
    slice: &'a [u8],
    alloc: A,
    parent_parser_state: Option<Rc<RefCell<MessageParserState<'a, A>>>>,
) -> Rc<Self>
```

- **Top-level messages**: Pass `parent_parser_state: None`
- **Child messages**: Pass `parent_parser_state: Some(parent_parser_state)`

### Key Design Decisions

1. **Field-Level Interior Mutability**: 
   - No struct-level `RefCell` wrapping
   - `RefCell`/`Cell` for scalar fields, `OnceList` for repeated fields
   - Fine-grained protection, no borrow conflicts

2. **Input Slice Storage**:
   - All messages store input slices in `field_slices: OnceList<&'a [u8], A>`
   - Enables multiple parse passes by recreating iterators from stored slices
   - Supports both slice-based and iterator-based inputs (by collecting slices)

3. **Parent Parser State Reference**:
   - Child messages hold `Option<Rc<RefCell<MessageParserState>>>` to parent's parser state
   - Avoids cycles: Parent Message Body → Parent Parser State → (Child holds Rc to this)
   - Allows child to request continued parsing even after parent Message Body is dropped

4. **Unified Interface**:
   - All message types have the same methods: `new()`, `add_slice()`, `ensure_all_fields_parsed()`, field getters
   - No distinction between "top-level" and "child" message implementations
   - Any message can be used in either role

### Implementation Files

- `puroro/src/lazy_parser.rs` - Core parsing infrastructure (`FieldIterator`, `MessageParserState`)
- `puroro/src/repeated_lazy.rs` - `LazyRepeated` wrapper for on-demand repeated field parsing
- `sandbox/src/generated/person.rs` - `PersonLazyImpl` implementation
- `sandbox/src/generated/address.rs` - `AddressLazyImpl` implementation

### Documentation

- `doc/lazy-parsing-state-design.md` - Finalized design document
- `doc/lazy-parsing-next-steps.md` - Next steps and optimizations
- `doc/lazy-parsing-state-design-discussion.md` - Design exploration history

### Recent Changes (2025-01)

1. **Unified Interface**: `PersonLazyImpl` and `AddressLazyImpl` now have identical structure
2. **Unified Constructor**: `new()` and `new_from_parent()` merged into single `new()` method
3. **Multiple Parse Support**: Added `field_slices` to all messages for multiple parse passes
4. **Code Simplification**: Removed unnecessary allocator cloning, unified `Rc::new_cyclic` pattern

### Open Questions for Next Discussion

- Performance optimizations for `continue_parsing_for_children()`
- `_field_number` field usage in `LazyRepeated`
- Additional testing and edge cases
