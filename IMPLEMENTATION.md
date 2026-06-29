# puroro — Implementation Notes

This document describes the **internal implementation** of generated code. It covers storage types, encode/decode algorithms, and runtime helper usage. The stable **public interface** is specified in [DESIGN.md](DESIGN.md).

> **Note:** The public API described in DESIGN.md must remain stable even if the internal representations documented here change. For example, presence tracking could move from a per-message bitfield to a different layout without any change to the accessor API.

## Table of contents

0. [Project context](#0-project-context)
1. [Presence bitfield](#1-presence-bitfield)
2. [Storage types per field kind](#2-storage-types-per-field-kind)
3. [Struct layout](#3-struct-layout)
4. [Constructors and the stored allocator](#4-constructors-and-the-stored-allocator)
5. [Encode implementation](#5-encode-implementation)
6. [Decode implementation](#6-decode-implementation)
7. [Per-field encode/decode patterns](#7-per-field-encodedecode-patterns)
   - 7.1 [Scalar: implicit presence](#71-scalar-implicit-presence)
   - 7.2 [Scalar: explicit presence](#72-scalar-explicit-presence)
   - 7.3 [String fields](#73-string-fields)
   - 7.4 [Bytes fields](#74-bytes-fields)
   - 7.5 [Repeated scalar: packed](#75-repeated-scalar-packed)
   - 7.6 [Repeated scalar: non-packed (with packed fallback)](#76-repeated-scalar-non-packed-with-packed-fallback)
   - 7.7 [Repeated string fields](#77-repeated-string-fields)
   - 7.8 [Nested message fields](#78-nested-message-fields)
   - 7.9 [Open enum fields](#79-open-enum-fields)
   - 7.10 [Closed enum fields](#710-closed-enum-fields)
   - 7.11 [Oneof fields](#711-oneof-fields)
   - 7.12 [Unknown fields](#712-unknown-fields)
   - 7.13 [LEGACY_REQUIRED fields](#713-legacy_required-fields)
8. [Derived and utility traits](#8-derived-and-utility-traits)
9. [Other optimization opportunities](#9-other-optimization-opportunities)

---

## 0. Project context

| Crate | Responsibility |
|---|---|
| **`protobuf-core`** | Submodule providing wire-format primitives (varint, tags, field I/O). Used by tooling and potentially by `puroro` internals; **generated code does not import it directly**. |
| **`puroro`** | Message runtime imported by generated code (`MessageEncode`, `MessageDecode`, `Optional`, encode/decode helpers). |
| **`protoc` plugin** | Emits the Rust types and `impl` blocks described here. Primary invocation path; see [DESIGN.md §0](DESIGN.md#0-project-architecture). |

All examples below use the **`Task` / `Address` reference schema** from [DESIGN.md §4](DESIGN.md#reference-schema). That pair is the canonical design example — not a throwaway fixture.

### Runtime stubs (implementation in progress)

The design in DESIGN.md is ahead of the current `puroro` runtime in a few areas:

| Area | Current behaviour | Target behaviour |
|---|---|---|
| UTF-8 validation | `decode_string_in` always validates and returns `DecodeError::InvalidUtf8`. | Generated code calls `decode_string_in` (VERIFY) or `decode_string_unchecked_in` (NONE) based on the field's `utf8_validation` feature. |
| Recursion limit | Not enforced during nested-message merge. | Generated `merge_from` passes a decremented depth counter; depth 0 → `DecodeError::RecursionLimitExceeded`. |
| Group wire types | `skip_field_and_save` / `skip_field` return `DecodeError::InvalidTag` for `SGroup` / `EGroup`. | Unchanged — groups are not preserved; error or ignore is acceptable per DESIGN.md §0. |

---

## 1. Presence bitfield

Explicit presence for **singular** fields (scalar, string, bytes, open/closed enum with `EXPLICIT`, and `LEGACY_REQUIRED`) is tracked in a per-message bitfield, not with per-field `Option<T>` discriminants. This matches the approach used by Google protobuf's C++/Java generators and keeps struct size smaller when many optional fields exist.

### Crate and type: `bitvec::array::BitArray`

Use the [`bitvec`](https://docs.rs/bitvec) crate. Prefer **`BitArray`** (inline, fixed-capacity) over **`BitVec`** (heap-backed):

| Type | Allocation | Custom `A: Allocator` |
|---|---|---|
| **`BitArray<[u8; N], Lsb0>`** (chosen) | Inline in the message struct; no heap | **Not needed** — presence bits are not allocator-aware |
| `BitVec` | Global heap only | **Incompatible** with per-message `A` |

At codegen time the plugin counts every presence-tracked singular field and emits a storage array large enough to hold that many bits (typically `BitArr!(for N, in u8, Lsb0)` or `BitArray<[u64; W], Lsb0>` with `W = ⌈N / 64⌉`). Each field receives a stable bit index (by ascending proto field number among tracked fields). Generated constants name the indices, e.g. `const BIT_TITLE: usize = 0;`.

```rust
// Illustrative sketch for reference `Task`:
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;

type TaskPresence = BitArray<[u8; 2], Lsb0>; // 10 tracked bits → 2 bytes

pub struct Task<A: Allocator = Global> {
    _presence: TaskPresence,
    title: Box<str, A>,           // bit BIT_TITLE
    score: i32,                   // IMPLICIT — no bit
    max_retries: i32,             // bit BIT_MAX_RETRIES
    // ...
    assignee: Option<Box<Address<A>, A>>, // nested — see §2
    _unknown_fields: Vec<u8, A>,
    _alloc: A,
}
```

**Accessor pattern:** `has_title()` → `_presence[BIT_TITLE]`; `title()` → `Optional::new(if has { Some(&*self.title) } else { None }, …)` for strings; `clear_title()` → clear bit and drop/replace heap data; `set_title(v)` → set bit and assign value.

**Fields that stay `Option<…>`:** nested messages and oneofs. Absence there implies no heap allocation for that subtree; a separate presence bit would still require an `Option` (or equivalent) on the `Box`/enum payload.

Runtime helpers (planned): `presence_get`, `presence_set`, `presence_clear` on `&mut BitArray<…>` — thin wrappers so generated code stays readable.

---

## 2. Storage types per field kind

| Field kind | Internal storage type | Presence |
|---|---|---|
| Implicit-presence scalar (`IMPLICIT`) | `T` (e.g. `i32`) | — |
| Explicit-presence scalar (`EXPLICIT`) | `T` | bit in `_presence` |
| Implicit-presence string | `Box<str, A>` | — |
| Explicit-presence string | `Box<str, A>` | bit in `_presence` |
| `bytes` (implicit) | `Vec<u8, A>` | — |
| `bytes` (explicit) | `Vec<u8, A>` | bit in `_presence` |
| Repeated scalar / string / message | `Vec<ElementType, A>` | — (empty vec = absent) |
| Message field | `Option<Box<MessageType<A>, A>>` | `Option` (not bitfield) |
| Open enum (`OPEN`, IMPLICIT) | `i32` | — |
| Open enum (`OPEN`, EXPLICIT) | `i32` | bit in `_presence` |
| Closed enum (`CLOSED`) | `i32` | bit in `_presence` |
| Oneof group | `Option<OurEnum<A>>` | `Option` (not bitfield) |
| Unknown fields | `Vec<u8, A>` | — |
| `LEGACY_REQUIRED` | same as `EXPLICIT` | bit in `_presence` |

When an explicit-presence string/bytes field is **unset**, the bit is `false` and the heap container may be empty (no payload allocation until `set_*`). Scalars and enums store the type zero in the value slot when unset; only the bit distinguishes unset from explicitly set zero.

`Box<str, A>` (2 words) is preferred over `Vec<u8, A>` (3 words) for owned strings. `Vec<u8, A>` is used for `bytes` fields.

---

## 3. Struct layout (`Task<A>`)

Generated eager messages contain one private field per proto field, plus infrastructure fields:

| Field (reference `Task`) | Storage | Notes |
|---|---|---|
| `_presence` | `BitArray<[u8; 2], Lsb0>` | 10 bits for EXPLICIT / LEGACY_REQUIRED singular fields |
| `title` | `Box<str, A>` | field 1, EXPLICIT |
| `score` | `i32` | field 2, IMPLICIT |
| `max_retries` | `i32` | field 3, EXPLICIT, default 3 |
| `owner_id` | `Box<str, A>` | field 4, LEGACY_REQUIRED |
| `payload` | `Vec<u8, A>` | field 5, EXPLICIT bytes |
| `tag_ids` | `Vec<i32, A>` | field 6, repeated packed |
| `scores` | `Vec<i32, A>` | field 7, repeated expanded |
| `labels` | `Vec<Box<str, A>, A>` | field 8, repeated string |
| `status` | `i32` | field 9, IMPLICIT open enum |
| `priority` | `i32` | field 10, EXPLICIT closed enum |
| `assignee` | `Option<Box<Address<A>, A>>` | field 11, nested message |
| `notification` | `Option<task::Notification<A>>` | fields 12–13, oneof |
| `_unknown_fields` | `Vec<u8, A>` | round-trip unknown wire |
| `_alloc` | `A` | cloned by setters / push methods |

The `_alloc` field lets setters allocate without the caller supplying an allocator. It is a ZST when `A = Global`; one pointer when `A = &bumpalo::Bump`.

`TaskLazy<A>` layout is specified in [DESIGN.md §8](DESIGN.md#tasklaya--lazy-parse-timing) — wire in `bytes::Bytes`, per-field caches with interior mutability, decoded values still using `A`.

---

## 4. Constructors and the stored allocator

- **`Task::new_in(alloc)`** — initialises every field to its type default; clears `_presence` to all-false; heap containers use `Vec::new_in(alloc.clone())` / equivalent.
- **`Task::new()`** — available when `A = Global`.
- **`Default`** — requires `A: Clone + Default`; delegates to `new_in(A::default())`.

Runtime helper **`str_to_box_in(s, alloc) -> Box<str, A>`** copies `s` into a `Vec<u8, A>`, boxes it, then reinterprets as `Box<str, A>` (one `unsafe` block inside the runtime, not in generated code).

---

## 5. Encode implementation

Generated `MessageEncode` for eager messages follows these rules:

1. Walk each present field in **implementation-defined order** (typically declaration order in generated code, but **not guaranteed**).
2. Apply the per-field omit rule from [§7](#7-per-field-encodedecode-patterns) before writing.
3. Append `_unknown_fields` verbatim at the end (order relative to known fields is not specified).
4. `encoded_len` must equal the byte count written by `encode_raw`.

**Non-deterministic wire layout.** Two messages with identical field values may encode to **different byte sequences** (field order, spacing inside packed blobs where applicable, ordering inside `_unknown_fields` after merges). Callers must not rely on byte-for-byte equality of `encode_raw` output across encodes or implementations. Semantic equality is via `PartialEq` (see [§8](#8-derived-and-utility-traits)), not wire bytes.

Runtime helpers used: `encode_varint_field`, `encode_len_field`, `encode_packed_*_field`, `encode_i32_field`, `encode_i64_field`, plus `encoded_len_*` counterparts.

---

## 6. Decode implementation

Generated `MessageDecode::merge_from` for eager messages:

1. Loop while the buffer has remaining bytes: read `(field_number, wire_type)` via `decode_tag`.
2. Dispatch on `(field_number, wire_type)` — one arm per known field using patterns in [§7](#7-per-field-encodedecode-patterns).
3. Unknown `(field_number, wire_type)` pairs call `skip_field_and_save` into `_unknown_fields`.
4. Singular scalars: last value wins. Repeated: append. Nested messages: merge into existing sub-message (see §7.8).

Nested sub-messages use `Buf::take(len)` to bound the slice to the declared LEN payload length.

---

## 7. Per-field encode/decode patterns

### 7.1 Scalar: implicit presence

**Storage:** plain `T`. **Encode:** omit when value equals the type zero. **Decode:** assign directly (last wins).

| Proto type | Decode | Encode value cast |
|---|---|---|
| `int32` / `int64` / `uint32` / `uint64` | `decode_varint` + cast | `v as u64` |
| `sint32` / `sint64` | `unzigzag*` after varint | `zigzag*(v)` |
| `bool` | varint `!= 0` | `v as u64` |
| `float` / `double` | `from_bits` on I32 / I64 | `to_bits` |
| `fixed*` / `sfixed*` | `get_u*_le` / `get_i*_le` | `encode_i32_field` / `encode_i64_field` |

No `Optional` wrapper — accessor returns `T` directly.

### 7.2 Scalar: explicit presence

**Storage:** plain `T` + presence bit. **Encode:** omit when bit is `false`; emit even if value is zero when bit is `true`. **Decode:** set bit and assign value (last wins).

**Generated accessor:** `has_*()` reads `_presence[BIT_*]`; `Optional::new(if has { Some(self.field) } else { None }, ThatDefault)` with a private ZST implementing `HasDefault<T>`. On traits, `_raw` and `has_` are default methods calling `.get()` / `.is_set()`. Native `impl` may read the bit and value directly.

**Fallible trait on `Task<A>`:** wrap the infallible `Optional` getter in `Ok(…)` with `Error = Infallible`.

`Optional` is a concrete struct with trivial drop — `task.max_retries().get()` chains without a `let` binding. No conversion to `Option<T>` is provided (see DESIGN.md §3).

### 7.3 String fields

**Storage:** `Box<str, A>` (EXPLICIT or IMPLICIT). EXPLICIT presence from `_presence` bit.

- **Encode EXPLICIT:** omit when bit is `false`; IMPLICIT: omit when empty.
- **Decode:** `decode_string_in(buf, alloc)` — validates UTF-8 when `utf8_validation = VERIFY`. A `NONE` helper is planned; not yet in the runtime. EXPLICIT: set bit and replace `Box<str, A>`.
- **Setter:** set bit, then `str_to_box_in(v, self._alloc.clone())`.
- **Clear:** clear bit; optionally replace with empty `Box<str, A>` to release storage.
- **Accessor:** same `HasDefault` + `Optional` pattern as scalars; lifetime-generic `impl<'a> HasDefault<&'a str>` for string defaults.

### 7.4 Bytes fields

Same EXPLICIT / IMPLICIT rules as strings. Use `decode_bytes_in` / `encode_len_field`. EXPLICIT setter writes into `Vec<u8, A>` and sets the presence bit.

### 7.5 Repeated scalar: packed

**Storage:** `Vec<T, A>`. **Encode:** `encode_packed_varint_field` (or packed I32/I64 helpers). **Decode:** accept **both** `WireType::Varint` (one element) and `WireType::Len` (packed blob) regardless of schema declaration — spec requirement.

### 7.6 Repeated scalar: non-packed (with packed fallback)

**Encode:** one wire record per element. **Decode:** identical dual-arm pattern as §7.5.

### 7.7 Repeated string fields

**Storage:** `Vec<Box<str, A>, A>`. **Encode:** one LEN record per element. **Decode:** `decode_string_in` + push.

### 7.8 Nested message fields

**Storage:** `Option<Box<Child<A>, A>>`. **Encode:** LEN wrapper — tag, length varint, `child.encode_raw`. **Decode:** read length, `Buf::take(len)`, `merge_from` into existing sub-message (create default child if absent) — concatenation equals merge.

**Recursion limit (stub):** nested merge should decrement a depth counter; at zero return `DecodeError::RecursionLimitExceeded`. Error variant exists; enforcement not yet implemented.

### 7.9 Open enum fields

**Storage:** `i32` (IMPLICIT or EXPLICIT). EXPLICIT presence from bit. Wire encoding is VARINT. IMPLICIT encode omits when zero.

### 7.10 Closed enum fields

**Storage:** `i32` + presence bit. On decode, if the numeric value is not a known variant, call `save_unknown_varint_field` instead of setting the bit. Encode includes the field even for the zero variant when the bit is set.

### 7.11 Oneof fields

**Storage:** `Option<NotificationEnum<A>>`. Each variant arm overwrites the previous on decode. Encode writes only the active variant's field number.

### 7.12 Unknown fields

**Storage:** `Vec<u8, A>` — valid partial wire stream. Accumulate via `skip_field_and_save`; re-emit with `put_slice` on encode. Deprecated group wire types are **not** saved — see DESIGN.md §0.

### 7.13 LEGACY_REQUIRED fields

Same storage and encode/decode as EXPLICIT (value + bit). Additionally generate `validate()` (checks presence bit) and `decode_strict()` (`decode` then `validate`). `MessageDecode::decode` does **not** call `validate` automatically.

---

## 8. Derived and utility traits

Generated **message** structs (`Task<A>`, nested messages, etc.) implement the traits below. **Enum types** (`Status`, `Priority`, oneof enums) are separate: they are plain `Copy` types and always get `Clone, Copy, Debug, PartialEq, Eq, Hash` via `derive` (see DESIGN.md §4.6).

### Always generated (any `A: Allocator + Clone`)

| Trait | Bounds on `A` | Behaviour |
|---|---|---|
| **`Default`** | `A: Allocator + Clone + Default` | `Default::default()` → `Task::new_in(A::default())`. Clears `_presence`; heap fields empty. |
| **`Clone`** | `A: Allocator + Clone` | Deep clone: copy `_presence` bitwise; clone heap fields with **`clone_in` / equivalent using `self._alloc.clone()`** so the duplicate uses the same allocator instance as the source; recursively clone nested messages. Does **not** deduplicate arena memory — two clones are independent trees. |
| **`Debug`** | none beyond field types | Prints field values and presence (e.g. `title: … (set)` / `(unset)`). Does not print `_alloc`. |
| **`PartialEq`** | none beyond field types | **Semantic equality:** compares presence bits and field values; unset explicit fields compare equal regardless of stale value slots; `_unknown_fields` compared bytewise; **`_alloc` is ignored**. |
| **`Eq`** | same as `PartialEq` | Markers when all compared components are `Eq`. |

**Not generated:** `Copy` (heap-owned), `PartialOrd` / `Ord` (no total order on messages), `Hash` (see below).

**Auto traits:** `Send` / `Sync` are inferred from field types (e.g. `Task<&'a Bump>` is `Send` if `Bump: Send`, but typically not `Sync`).

### `Global`-only convenience (in addition to generic impls)

When `A = Global` (the default type parameter), the generator also emits:

| Item | Purpose |
|---|---|
| **`impl Default for Task`** | Ergonomic `Task::default()` without naming `Global`. |
| **`Task::new()`** | Same as `Task::default()` / `Task::new_in(Global)`. |
| **`MessageDecode::decode(buf)`** | Usable without `A: Default` on a custom allocator — callers with `Task<Global>` only. |

Custom-allocator users call `Task::new_in(alloc)` and `merge_from` (or a future `decode_in`) instead.

### Optional / deferred

| Trait | Status | Notes |
|---|---|---|
| **`Hash`** | **Deferred** | Could hash semantic content like `PartialEq`, but protobuf messages are rarely map keys; add only if requested. |
| **`serde::Serialize` / `Deserialize`** | **Future feature** | Likely behind `#[puroro(serde)]` or crate feature; would need a defined JSON mapping. Not part of core codegen. |

### Clone and custom allocators (detail)

```rust
impl<A: Allocator + Clone> Clone for Task<A> {
    fn clone(&self) -> Self {
        Self {
            _presence: self._presence.clone(), // BitArray: bitwise copy
            title: self.title.clone(),       // Box<str, A>: Clone uses same allocator
            // … each heap field cloned …
            assignee: self.assignee.as_ref().map(|b| b.clone()),
            _unknown_fields: self._unknown_fields.clone(),
            _alloc: self._alloc.clone(),
        }
    }
}
```

For **`A = Global`**, `Clone` on `Box<str, A>` / `Vec<T, A>` matches `std` behaviour. For **arena `A`**, cloning allocates a **second copy** of all string/bytes/nested data into the same arena (or a cloned bump handle, depending on how the user shares `A`) — it does **not** share `Box` internals between clones. If sharing immutable messages is required, use `Arc<Task<A>>` (user-side), not a special generated impl.

---

## 9. Other optimization opportunities

**Zero-copy strings** — `TaskView<'buf>` (DESIGN.md §8) holds `&'buf str` instead of `Box<str, A>`.

**Arena allocation** — already supported via `new_in(&bump)`; all decoded allocations share one arena.

**Recursion limit** — planned `merge_from_with_depth` runtime helper; default limit TBD (e.g. 100). **Status: stub.**

**`TaskLazy`** — see [DESIGN.md §8](DESIGN.md#tasklaya--lazy-parse-timing) for wire storage, cache states, cursors, and nested `Bytes::slice` — not repeated here.
