# AI References for Puroro Project

## Project Overview
Puroro is a Rust-idiomatic implementation of Google Protocol Buffers, focusing on:
- Precise Protocol Buffers specification support
- Rust-idiomatic APIs that leverage the type system
- Memory safety without sacrificing performance
- Practical performance (e.g., incremental parsing / lazy parsing where it makes sense)

## Lazy Parser Implementation - Current Snapshot (2026-01)

This section describes the current code behavior at a high level (as of 2026-01), to avoid
confusion and to keep this file small.

Note: Lazy parsing is a current hot topic and an actively developed area, but it is not the only
focus of the project (serialization/deserialization, codegen, and other runtime pieces are also in scope).

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
  - scalar child message caches can be expressed as `OnceCell<Rc<ChildLazyImpl>>` (instead of `RefCell<Option<Rc<_>>>`) when they are set at most once
  - `AddressLazyImpl` string fields are now validated with `str::from_utf8` and stored as `&'slice str` (zero-copy) instead of allocating `String`

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
  - `Repeated::iter_box()` returns an on-demand iterator (does not force full parsing up-front).

## 2026-01-22: why lazy message impls use `Rc`

- Generated lazy message bodies (e.g. `PersonLazyImpl`, `AddressLazyImpl`) are constructed with `Rc::new_cyclic` so the `MessageParserStateRef` callback can capture a `Weak<Self>` and update fields while avoiding cycles.
- Child messages are cached/returned as `Rc<Child>` and repeated message fields store `Rc<Child>` for cheap cloning and stable identity.

## 2026-01-22: split the reference generated `Person` code into submodules

- `sandbox/src/generated/person.rs` is now a thin facade module.
- The reference generated `Person` implementation is split under `sandbox/src/generated/person/` into:
  - `traits.rs` (public traits)
  - `status.rs` (enum)
  - `impl_.rs` (standard impl)
  - `lazy.rs` (lazy impl)
  - `tests.rs` (unit tests)
- This keeps the public surface stable (`generated::person::PersonImpl`, etc.) while making the file sizes manageable as the API grows.
- IMPORTANT: This split is a **design-stage convenience**. The real code generator may choose a different
  output layout (including a simpler "one module = one file" approach). Do not treat the current file
  structure as the final generated format.

## 2026-01-22: lazy parsing - known gaps / TODOs (design stage)

- **`iter_box()` should not force full parse**: `LazyRepeated::iter_box()` should return an on-demand iterator
  (while `len()` may inevitably require full parse). (Implemented in `puroro/src/repeated_lazy.rs`.)
- **Packed repeated**: Support packed encoding for repeated numeric fields (wire type Len) in lazy impls.
- **Unknown fields**: Preserve unknown fields (store + round-trip) instead of discarding them.
- **Zero-copy views**: High-priority future work. Prefer returning borrowed views (`&'slice [u8]` / `&'slice str`)
  where possible; avoid `to_vec()` copies in lazy impls. (Started with `AddressLazyImpl` string fields.)
- **Slice contract**: Multiple slice input is intentional, but each slice is expected to be a self-contained,
  valid protobuf encoding of (part of) the same message (not arbitrary byte-stream fragmentation).
- **Standard impl parse/write/size**: `PersonImpl` and other standard impls still have `todo!()` for parsing/serialization/size.

## 2026-01-24: why we use GATs for fallible lazy traits (design stage)

- We prefer modeling fallible getter return types via **GAT / associated type families** (e.g. `type Addresses<'a>`) instead of
  return-position `impl Trait` in trait methods.
- Rationale:
  - **Encapsulation**: lifetime/allocator captures (e.g. lazy impls carrying `'slice` and `A`) are an implementation detail and should not
    leak into the base trait's signature/capture rules.
  - **Composability**: nested return types like `Repeated<Item = MessageView>` become easier to express by naming components (`AddressItem<'a>`,
    `Addresses<'a>`) without introducing multiple nested opaque types.
  - **Strategy flexibility**: eager and lazy implementations can choose different concrete return types while satisfying the same trait bounds.

## Handy File Pointers

- `puroro/src/lazy_parser.rs`: incremental parser state + parent-chain parsing requests
- `puroro/src/repeated_lazy.rs`: `LazyRepeated` adapter
- `sandbox/src/generated/person.rs` + `sandbox/src/generated/person/`: reference generated-style code for `Person`
- `sandbox/src/generated/address.rs`: reference generated-style code for `Address`

## More Detailed Design Docs

- `doc/lazy-parsing-state-design.md`
- `doc/lazy-parsing-next-steps.md`

## Minimal Reading Order (for new AI agents)

If you are new to this repo and want to avoid reading many docs, start here:

1. `puroro/src/lazy_parser.rs` + `puroro/src/repeated_lazy.rs` (ground truth implementation)
2. `doc/lazy-parsing-state-design.md` (current invariants / mental model)
3. `doc/design-discussion.md` (the top "Current Implementation Status (2026-01-22)" snapshot only; the rest is historical log)