# AI References for Puroro Project

## Project Overview
Puroro is a Rust-idiomatic implementation of Google Protocol Buffers, focusing on:
- Precise Protocol Buffers specification support
- Rust-idiomatic APIs that leverage the type system
- Memory safety without sacrificing performance
- Lazy parsing support for on-demand deserialization

## Lazy Parser Implementation - Current Snapshot (2026-01)

This section describes the current code behavior at a high level (as of 2026-01), to avoid
confusion and to keep this file small.

- **Core types**: `FieldIterator`, `MessageParserStateRef`
- **Internal core**: `MessageParserStateInner` (the mutable state behind `MessageParserStateRef`, stored as `Rc<RefCell<...>>`)
- **FieldIterator**: stores per-slice iterators (`ProtobufFieldSliceIterator`) inside
  `OnceList<Peekable<_>>`, so new input slices can be appended without recreating a single global iterator.
- **MessageParserStateRef**: wraps `Rc<RefCell<MessageParserStateInner>>` and provides:
  - incremental parsing via `parse_until_with_callback`
  - a parent-chain request mechanism (`next_field()` can request the parent to parse until a new slice is available)
  - a terminating operation `ensure_all_fields_parsed_with_callback()` (after termination, `add_slice()` is rejected)
- **Generated reference implementations** (`sandbox/src/generated/*.rs`):
  - keep a `parser_state: MessageParserStateRef` plus per-field storage
  - `ensure_all_fields_parsed()` delegates to `parser_state.ensure_all_fields_parsed_with_callback()`
  - `LazyRepeated::new()` takes `(parser_state, &OnceList)` (no field-number metadata)

## 2026-01-19: Removed `.count()` / `.nth()` usage from lazy repeated implementation

- Added `MessageParserStateRef::parse_one_field_with_callback()` to advance parsing by exactly one field.
- Reworked `LazyRepeatedIter` to be streaming: when the underlying `OnceList` iterator reaches the end,
  it advances the parent parser one field at a time until a new element appears or the parent is exhausted.
- Updated `Repeated` adapters for `OnceList` to avoid `.count()`/`.nth()` (use manual loops / `enumerate()`).
- Added an integration test asserting that a `OnceList::iter()` created before a `push()` can observe the newly pushed element.

## 2026-01-22: once-list2 0.4.0 tail append integration

- `puroro` now pins `once-list2 = "0.4.0"`.
- `sandbox` also pins `once-list2 = "0.4.0"` to avoid multiple versions (the generated reference code imports tail-caching aliases).
- We use `once_list2::OnceListWithTailLen<T, A>` (via `use ::once_list2::OnceListWithTailLen as OnceList;`) so repeated tail appends are fast and `len()` is O(1).
- `LazyRepeated` was updated to leverage O(1) `len()` for `ensure_at_least()` and to reduce overhead:
  - `LazyRepeatedIter` no longer uses `Box<dyn Iterator>`; it stores the concrete `once_list2::Iter` (cloned).
  - `Repeated::iter_box()` no longer collects into a temporary `Vec<T>`; it boxes `self.list.iter().cloned()` after fully parsing.

## Handy File Pointers

- `puroro/src/lazy_parser.rs`: incremental parser state + parent-chain parsing requests
- `puroro/src/repeated_lazy.rs`: `LazyRepeated` adapter
- `sandbox/src/generated/person.rs`, `sandbox/src/generated/address.rs`: reference generated-style code (traits + impl + lazy impl)

## More Detailed Design Docs

- `doc/lazy-parsing-state-design.md`
- `doc/lazy-parsing-next-steps.md`
