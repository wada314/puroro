# puroro — Implementation Notes

This document describes the **internal implementation** of generated code. It covers storage types, encode/decode algorithms, and runtime helper usage. The stable **public interface** is specified in [DESIGN.md](DESIGN.md).

> **Note:** The public API described in DESIGN.md must remain stable even if the internal representations documented here change. For example, presence tracking for optional scalars currently uses `Option<T>` fields, but could be replaced by a per-message bitfield without any change to the accessor API.

## Table of contents

0. [Project context](#0-project-context)
1. [Storage types per field kind](#1-storage-types-per-field-kind)
2. [Struct layout](#2-struct-layout)
3. [Constructors and the stored allocator](#3-constructors-and-the-stored-allocator)
4. [Encode implementation](#4-encode-implementation)
5. [Decode implementation](#5-decode-implementation)
6. [Per-field encode/decode patterns](#6-per-field-encodedecode-patterns)
   - 6.1 [Scalar: implicit presence](#61-scalar-implicit-presence)
   - 6.2 [Scalar: explicit presence](#62-scalar-explicit-presence)
   - 6.3 [String fields](#63-string-fields)
   - 6.4 [Bytes fields](#64-bytes-fields)
   - 6.5 [Repeated scalar: packed](#65-repeated-scalar-packed)
   - 6.6 [Repeated scalar: non-packed (with packed fallback)](#66-repeated-scalar-non-packed-with-packed-fallback)
   - 6.7 [Repeated string fields](#67-repeated-string-fields)
   - 6.8 [Nested message fields](#68-nested-message-fields)
   - 6.9 [Open enum fields](#69-open-enum-fields)
   - 6.10 [Closed enum fields](#610-closed-enum-fields)
   - 6.11 [Oneof fields](#611-oneof-fields)
   - 6.12 [Unknown fields](#612-unknown-fields)
   - 6.13 [LEGACY_REQUIRED fields](#613-legacy_required-fields)
7. [Optimization opportunities](#7-optimization-opportunities)

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

## 1. Storage types per field kind

| Field kind | Internal storage type |
|---|---|
| Implicit-presence scalar (`IMPLICIT`) | `T` (e.g. `i32`) — the Rust primitive directly |
| Explicit-presence scalar (`EXPLICIT`) | `Option<T>` |
| Implicit-presence string | `allocator_api2::boxed::Box<str, A>` |
| Explicit-presence string | `Option<allocator_api2::boxed::Box<str, A>>` |
| `bytes` field (implicit presence) | `allocator_api2::vec::Vec<u8, A>` |
| `bytes` field (explicit presence) | `Option<allocator_api2::vec::Vec<u8, A>>` |
| Repeated scalar / string / message | `allocator_api2::vec::Vec<ElementType, A>` |
| Message field (always optional in struct) | `Option<allocator_api2::boxed::Box<MessageType<A>, A>>` |
| Open enum (`OPEN`) | `i32` (IMPLICIT) or `Option<i32>` (EXPLICIT) |
| Closed enum (`CLOSED`) | `Option<i32>` |
| Oneof group | `Option<OurEnum<A>>` |
| Unknown fields | `allocator_api2::vec::Vec<u8, A>` |
| `LEGACY_REQUIRED` field | Same as `EXPLICIT` (`Option<T>`) |

`Box<str, A>` (2 words: ptr + byte length) is preferred over `Vec<u8, A>` (3 words) for owned strings since strings are generally written once and then read. `Vec<u8, A>` is used for `bytes` fields where incremental appending is more natural.

> **Optimization note:** explicit-presence scalars currently use `Option<T>`. A future optimisation could use `T` storage with a per-message presence bitfield, reducing struct size at the cost of slightly more complex accessor code. The public API (`has_X()`, `set_X()`, `clear_X()`) would be identical. This is the approach used by Google protobuf's generated C++ code.

---

## 2. Struct layout (`Task<A>`)

Generated eager messages contain one private field per proto field, plus two infrastructure fields:

| Field (reference `Task`) | Storage | Notes |
|---|---|---|
| `title` | `Option<Box<str, A>>` | field 1, EXPLICIT |
| `score` | `i32` | field 2, IMPLICIT |
| `max_retries` | `Option<i32>` | field 3, EXPLICIT, default 3 |
| `owner_id` | `Option<Box<str, A>>` | field 4, LEGACY_REQUIRED |
| `payload` | `Option<Vec<u8, A>>` | field 5, EXPLICIT bytes |
| `tag_ids` | `Vec<i32, A>` | field 6, repeated packed |
| `scores` | `Vec<i32, A>` | field 7, repeated expanded |
| `labels` | `Vec<Box<str, A>, A>` | field 8, repeated string |
| `status` | `i32` | field 9, IMPLICIT open enum |
| `priority` | `Option<i32>` | field 10, EXPLICIT closed enum |
| `assignee` | `Option<Box<Address<A>, A>>` | field 11, nested message |
| `notification` | `Option<task::Notification<A>>` | fields 12–13, oneof |
| `_unknown_fields` | `Vec<u8, A>` | round-trip unknown wire |
| `_alloc` | `A` | cloned by setters / push methods |

The `_alloc` field lets setters allocate without the caller supplying an allocator. It is a ZST when `A = Global`; one pointer when `A = &bumpalo::Bump`.

`TaskLazy<A>` layout is specified in [DESIGN.md §8](DESIGN.md#tasklaya--lazy-parse-timing) — wire in `bytes::Bytes`, per-field caches with interior mutability, decoded values still using `A`.

---

## 3. Constructors and the stored allocator

- **`Task::new_in(alloc)`** — initialises every field to its type default; heap containers use `Vec::new_in(alloc.clone())` / equivalent.
- **`Task::new()`** — available when `A = Global`.
- **`Default`** — requires `A: Clone + Default`; delegates to `new_in(A::default())`.

Runtime helper **`str_to_box_in(s, alloc) -> Box<str, A>`** copies `s` into a `Vec<u8, A>`, boxes it, then reinterprets as `Box<str, A>` (one `unsafe` block inside the runtime, not in generated code).

---

## 4. Encode implementation

Generated `MessageEncode` for eager messages follows these rules:

1. Walk each field in field-number order (or any order — wire order among fields is not semantically significant).
2. Apply the per-field omit rule from [§6](#6-per-field-encodedecode-patterns) before writing.
3. Append `_unknown_fields` verbatim at the end.
4. `encoded_len` must equal the byte count written by `encode_raw`.

Runtime helpers used: `encode_varint_field`, `encode_len_field`, `encode_packed_*_field`, `encode_i32_field`, `encode_i64_field`, plus `encoded_len_*` counterparts.

---

## 5. Decode implementation

Generated `MessageDecode::merge_from` for eager messages:

1. Loop while the buffer has remaining bytes: read `(field_number, wire_type)` via `decode_tag`.
2. Dispatch on `(field_number, wire_type)` — one arm per known field using patterns in [§6](#6-per-field-encodedecode-patterns).
3. Unknown `(field_number, wire_type)` pairs call `skip_field_and_save` into `_unknown_fields`.
4. Singular scalars: last value wins. Repeated: append. Nested messages: merge into existing sub-message (see §6.8).

Nested sub-messages use `Buf::take(len)` to bound the slice to the declared LEN payload length.

---

## 6. Per-field encode/decode patterns

### 6.1 Scalar: implicit presence

**Storage:** plain `T`. **Encode:** omit when value equals the type zero. **Decode:** assign directly (last wins).

| Proto type | Decode | Encode value cast |
|---|---|---|
| `int32` / `int64` / `uint32` / `uint64` | `decode_varint` + cast | `v as u64` |
| `sint32` / `sint64` | `unzigzag*` after varint | `zigzag*(v)` |
| `bool` | varint `!= 0` | `v as u64` |
| `float` / `double` | `from_bits` on I32 / I64 | `to_bits` |
| `fixed*` / `sfixed*` | `get_u*_le` / `get_i*_le` | `encode_i32_field` / `encode_i64_field` |

No `Optional` wrapper — accessor returns `T` directly.

### 6.2 Scalar: explicit presence

**Storage:** `Option<T>`. **Encode:** omit when `None`; emit even if inner value is zero. **Decode:** always `Some(decode …)`.

**Generated accessor:** define a private ZST implementing `HasDefault<T>` with the proto `[default = …]` constant; return `Optional::new(self.field, ThatDefault)`. On traits, `_raw` and `has_` are default methods calling `.get()` / `.is_set()`. Native `impl` may read the internal `Option` directly.

**Fallible trait on `Task<A>`:** wrap the infallible `Optional` getter in `Ok(…)` with `Error = Infallible`.

`Optional` is a concrete struct with trivial drop — `task.max_retries().get()` chains without a `let` binding. No conversion to `Option<T>` is provided (see DESIGN.md §3).

### 6.3 String fields

**Storage:** `Option<Box<str, A>>` (EXPLICIT) or `Box<str, A>` (IMPLICIT).

- **Encode EXPLICIT:** omit when `None`; IMPLICIT: omit when empty.
- **Decode:** `decode_string_in(buf, alloc)` — validates UTF-8 when `utf8_validation = VERIFY`. A `NONE` helper is planned; not yet in the runtime.
- **Setter:** `str_to_box_in(v, self._alloc.clone())`.
- **Accessor:** same `HasDefault` + `Optional` pattern as scalars; lifetime-generic `impl<'a> HasDefault<&'a str>` for string defaults.

### 6.4 Bytes fields

Same EXPLICIT / IMPLICIT rules as strings. Use `decode_bytes_in` / `encode_len_field`. EXPLICIT setter reuses or creates `Vec<u8, A>` via `get_or_insert_with`.

### 6.5 Repeated scalar: packed

**Storage:** `Vec<T, A>`. **Encode:** `encode_packed_varint_field` (or packed I32/I64 helpers). **Decode:** accept **both** `WireType::Varint` (one element) and `WireType::Len` (packed blob) regardless of schema declaration — spec requirement.

### 6.6 Repeated scalar: non-packed (with packed fallback)

**Encode:** one wire record per element. **Decode:** identical dual-arm pattern as §6.5.

### 6.7 Repeated string fields

**Storage:** `Vec<Box<str, A>, A>`. **Encode:** one LEN record per element. **Decode:** `decode_string_in` + push.

### 6.8 Nested message fields

**Storage:** `Option<Box<Child<A>, A>>`. **Encode:** LEN wrapper — tag, length varint, `child.encode_raw`. **Decode:** read length, `Buf::take(len)`, `merge_from` into existing sub-message (create default child if absent) — concatenation equals merge.

**Recursion limit (stub):** nested merge should decrement a depth counter; at zero return `DecodeError::RecursionLimitExceeded`. Error variant exists; enforcement not yet implemented.

### 6.9 Open enum fields

**Storage:** `i32` (IMPLICIT) or `Option<i32>` (EXPLICIT). Wire encoding is VARINT. IMPLICIT encode omits when zero.

### 6.10 Closed enum fields

**Storage:** `Option<i32>`. On decode, if the numeric value is not a known variant, call `save_unknown_varint_field` instead of storing in the typed field. Encode includes the field even for the zero variant when set.

### 6.11 Oneof fields

**Storage:** `Option<NotificationEnum<A>>`. Each variant arm overwrites the previous on decode. Encode writes only the active variant's field number.

### 6.12 Unknown fields

**Storage:** `Vec<u8, A>` — valid partial wire stream. Accumulate via `skip_field_and_save`; re-emit with `put_slice` on encode. Deprecated group wire types are **not** saved — see DESIGN.md §0.

### 6.13 LEGACY_REQUIRED fields

Same storage and encode/decode as EXPLICIT. Additionally generate `validate()` (checks `Option` is `Some`) and `decode_strict()` (`decode` then `validate`). `MessageDecode::decode` does **not** call `validate` automatically.

---

## 7. Optimization opportunities

**Presence bitfield** — replace per-field `Option<T>` discriminants with a `_presence: u64` bitset; accessor API unchanged. Used by Google protobuf C++/Java generators.

**Zero-copy strings** — `TaskView<'buf>` (DESIGN.md §8) holds `&'buf str` instead of `Box<str, A>`.

**Arena allocation** — already supported via `new_in(&bump)`; all decoded allocations share one arena.

**Recursion limit** — planned `merge_from_with_depth` runtime helper; default limit TBD (e.g. 100). **Status: stub.**

**`TaskLazy`** — see [DESIGN.md §8](DESIGN.md#tasklaya--lazy-parse-timing) for wire storage, cache states, cursors, and nested `Bytes::slice` — not repeated here.
