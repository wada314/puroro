# Lazy Parsing State Design - Discussion History (Condensed)

This document is a **condensed** record of design exploration around lazy parsing state.
It intentionally keeps only the key trade-offs that can still help when editing the current code.

For the current intended behavior and invariants, see `lazy-parsing-state-design.md`.

## Problem Statement

We want true lazy parsing for nested messages and repeated fields:

- Do not parse the whole message eagerly if the user only touches a small subset of fields.
- Preserve protobuf semantics (later occurrences overwrite scalar fields; repeated fields append).
- Allow nested (child) messages to request additional parsing from their parent when needed.

## Approaches Considered (Historical)

### 1. Parent reference by lifetime (`&'a Parent`)

- **Pros**: no refcount overhead, very explicit ownership.
- **Cons**: hard to express with on-demand child creation while the parent owns the child; quickly runs into self-referential ergonomics.

### 2. `Rc<...>` / `Weak<...>` parent pointers

- **Pros**: safe, ergonomic for on-demand creation.
- **Cons**: easy to create ownership cycles (Parent → Child → Parent) unless carefully broken.
- **Extra**: if the parent message body is dropped, child still needs a way to keep parsing.

### 3. Raw pointer (`*const Parent`)

- **Pros**: minimal overhead.
- **Cons**: requires `unsafe` and carefully maintained invariants; easy to become unsound during refactors.

### 4. Arena / ID-based approaches

- **Pros**: unified lifetime and no cycles.
- **Cons**: significant architectural complexity; not needed for the current implementation.

## Chosen Direction (Current Implementation)

The current code avoids keeping a strong reference to the parent **message body** by separating:

- **Message body**: owns field values.
- **Parser state**: owns parsing cursor + callback routing.

Child messages keep a handle to the **parent parser state** (not the parent message body), so they can request additional parsing without creating an ownership cycle.

See:
- `puroro/src/lazy_parser.rs` (`MessageParserStateRef`, `MessageParserStateInner`, parent-chain parsing requests)
- `puroro/src/repeated_lazy.rs` (`LazyRepeated` adapter)
- `sandbox/src/generated/*.rs` (reference generated-style implementations)

