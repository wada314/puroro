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
10. [Field-centric codegen architecture](#10-field-centric-codegen-architecture)

---

## 0. Project context

| Crate | Responsibility |
|---|---|
| **`protobuf-core`** | Wire-format primitives: `Varint`, `Tag`, `WireType`, field read/write traits. **`puroro` depends on it** for all varint/tag logic; generated code does not import it directly. |
| **`puroro`** | Message runtime imported by generated code (`MessageEncode`, `MessageDecode`, `Optional`, **`fields`**, encode/decode helpers). **Uses `protobuf-core`** internally for varints, tags, and wire types. |
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

Generated eager messages are a **product of composable field types** (see [§10](#10-field-centric-codegen-architecture)) plus one shared [`MessageCommon`](src/fields/common.rs) and any oneof slots.

```rust
// Illustrative generated layout for reference `Task`:
pub struct Task<A: Allocator = Global> {
    _common: MessageCommon<TaskPresence, A>,
    title: puroro::fields::ExplicitString<1, BIT_TITLE, A>,
    score: puroro::fields::ImplicitVarintField<puroro::fields::ProtoInt32>,
    max_retries: puroro::fields::ExplicitVarintField<puroro::fields::ProtoInt32>,
    owner_id: puroro::fields::ExplicitString<4, BIT_OWNER_ID, A>,
    payload: puroro::fields::ExplicitBytes<5, BIT_PAYLOAD, A>,
    tag_ids: puroro::fields::RepeatedPackedI32<6, A>,
    scores: puroro::fields::RepeatedExpandedI32<7, A>,
    labels: puroro::fields::RepeatedString<8, A>,
    status: puroro::fields::ImplicitVarintField<puroro::fields::ProtoEnum>,
    priority: puroro::fields::ExplicitVarintField<puroro::fields::ProtoEnum>,
    assignee: puroro::fields::NestedMessage<11, Address<A>, A>,
    notification: OneofSlot<task::Notification<A>>,
}
```

| Member | Role |
|---|---|
| `_common.presence` | `BitArray<[u8; 2], Lsb0>` — 10 tracked bits |
| `_common.unknown_fields` | Round-trip unknown wire |
| `_common.alloc` | Cloned by field setters / push |
| `title` … `assignee` | Independent field types; each knows its field number and (if applicable) presence bit |
| `notification` | [`OneofSlot`](src/fields/oneof.rs) — **not** a catalog field type; variants are mutually exclusive |

Public accessors on `Task` are **one-line delegates** into the field type, passing `_common.parts()` or `_common.parts_mut()` as needed. `MessageEncode`, `MessageDecode`, `Clone`, and `PartialEq` on the message are the **sum of the same delegates** — no field-specific logic lives in the message `impl` body beyond dispatch tables.

`TaskLazy<A>` layout is specified in [DESIGN.md §8](DESIGN.md#tasklaya--lazy-parse-timing).

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

---

## 10. Field-centric codegen architecture

Protobuf message fields (except **`oneof`**) are **independent**: each field's getter, setter, encode arm, decode arm, clone, and equality check only reads/writes **its own member** plus **shared message infrastructure** (presence bitfield, allocator, unknown-field buffer). The code generator should not emit bespoke logic per message — it **composes** a fixed catalog of runtime field types from `puroro::fields`.

### Design goals (hobby-project aggressive)

| Goal | Approach |
|---|---|
| Minimal generated logic | Message `impl` = thin delegates + dispatch tables |
| Single implementation per proto pattern | **Two-layer** catalog: wire-encoding trait + presence wrapper (see below) |
| Monomorphised hot path | `const` field number / presence bit / wire encoding as type parameters — no trait objects |
| Stable public API | User-facing `task.title()` unchanged; only generated internals differ |

### Three layers

```
┌─────────────────────────────────────────────────────────────┐
│  protoc plugin                                              │
│  Maps each proto field → catalog type + const parameters    │
│  Emits: struct members, delegate accessors, match arms      │
└──────────────────────────┬──────────────────────────────────┘
                           │ uses
┌──────────────────────────▼──────────────────────────────────┐
│  puroro::fields  (runtime catalog)                          │
│  VarintProtoType + ImplicitVarintField / ExplicitVarintField│
│  Fixed32/64ProtoType + …  String/LEN …  MessageCommon       │
└──────────────────────────┬──────────────────────────────────┘
                           │ calls
┌──────────────────────────▼──────────────────────────────────┐
│  puroro::encode / puroro::decode / puroro::fields::varint    │
│  Buf/BufMut adapters + message-level helpers (LEN, unknown)   │
└──────────────────────────┬──────────────────────────────────┘
                           │ delegates varint/tag/wire types
┌──────────────────────────▼──────────────────────────────────┐
│  protobuf-core (Varint, Tag, WireType, Field, …)            │
└─────────────────────────────────────────────────────────────┘
```

### Shared infrastructure: `MessageCommon`

All singular/repeated fields share one struct (see [`src/fields/common.rs`](src/fields/common.rs)):

| Member | Used by |
|---|---|
| `presence: P` | EXPLICIT / LEGACY_REQUIRED singular fields |
| `unknown_fields: Vec<u8, A>` | Message-level unknown tags; **closed enum** diversion |
| `alloc: A` | String/bytes/repeated/nested setters and decode |

Field methods take a **`MessageParts`** (read) or **`MessagePartsMut`** (write/merge) view — not `&Task` — so field types stay decoupled from the parent message type.

```rust
// Generated accessor (always this shape):
pub fn title(&self) -> Optional<&str, impl HasDefault<&str>> {
    self.title.get(self._common.parts())
}
pub fn set_title(&mut self, v: &str)
where
    A: Clone,
{
    self.title.set(self._common.parts_mut(), v);
}
```

### Two-layer field catalog

Protobuf field behaviour splits into two **orthogonal** axes:

| Axis | What varies | Where it lives |
|---|---|---|
| **Wire encoding** | int32 vs sint32 vs bool vs float vs string … | Zero-sized marker + trait (`VarintProtoType`, `Fixed32ProtoType`, …) |
| **Presence** | IMPLICIT vs EXPLICIT vs LEGACY_REQUIRED | Field wrapper (`ImplicitVarintField<T>`, `ExplicitVarintField<T>`, …) |

The plugin never emits `ImplicitI32` vs `ImplicitSint32` as separate catalog entries — it emits **`ImplicitVarintField<ProtoInt32>`** vs **`ImplicitVarintField<ProtoSint32>`**. All varint wire logic is written **once** in the wrappers; all zigzag/bool/enum semantics live in **`VarintProtoType`** impls.

```
                    ┌─────────────────────────────────────┐
  ProtoField        │  presence layer (1 impl per mode)   │
  ─────────►        │  ImplicitVarintField<T>             │
  int32 IMPLICIT    │  ExplicitVarintField<T>             │
  int32 EXPLICIT    │  ImplicitFixed32Field<T>  (planned) │
                    └──────────────┬──────────────────────┘
                                   │ T: VarintProtoType / …
                    ┌──────────────▼──────────────────────┐
  wire layer        │  ProtoInt32, ProtoSint32, ProtoBool,  │
  (marker types)    │  ProtoUInt64, ProtoEnum, …            │
                    └──────────────┬──────────────────────┘
                                   │ calls
                    ┌──────────────▼──────────────────────┐
                    │  puroro::encode / puroro::decode    │
                    └─────────────────────────────────────┘
```

#### Layer 1 — wire encoding traits ([`src/fields/varint.rs`](src/fields/varint.rs))

Each protobuf **type** that shares a wire representation gets a marker struct and a trait impl.
**Semantic conversions use [`protobuf_core::Varint`](../../protobuf-core/src/varint.rs)** (`from_int32`, `to_sint32`, …) — puroro does not reimplement zigzag or varint parsing in the field layer.

```rust
pub trait VarintProtoType {
    type Value: Copy + PartialEq;
    fn proto_zero() -> Self::Value;
    fn decode_wire(raw: u64) -> Result<Self::Value, DecodeError>;
    fn encode_wire(value: Self::Value) -> u64;
}

// Markers (all in varint.rs):
// ProtoInt32, ProtoInt64, ProtoUInt32, ProtoUInt64,
// ProtoSint32 (zigzag), ProtoSint64 (zigzag),
// ProtoBool, ProtoEnum (same wire as int32)
```

Parallel traits for other wire families ([`fixed32.rs`](src/fields/fixed32.rs), [`fixed64.rs`](src/fields/fixed64.rs), future `len.rs` for string/bytes/message):

| Trait | Wire type | Planned markers |
|---|---|---|
| `VarintProtoType` | VARINT | above |
| `Fixed32ProtoType` | I32 | `ProtoFixed32`, `ProtoSfixed32`, `ProtoFloat` |
| `Fixed64ProtoType` | I64 | `ProtoFixed64`, `ProtoSfixed64`, `ProtoDouble` |
| `LenProtoType` | LEN | `ProtoString`, `ProtoBytes`, `ProtoMessage<M>` |

**Important:** Rust storage type alone does **not** identify protobuf encoding (`i32` can be int32, sint32, sfixed32, or enum). The marker type is the source of truth — matching the naming rule in `protobuf-core` docs.

#### Layer 2 — presence wrappers ([`src/fields/scalar.rs`](src/fields/scalar.rs))

One generic struct per `(wire family × presence mode)` pair:

| Wrapper | Param `T` | Const params on methods |
|---|---|---|
| `ImplicitVarintField<T>` | `T: VarintProtoType` | `FIELD: u32` |
| `ExplicitVarintField<T>` | `T: VarintProtoType` | `FIELD: u32`, `BIT: usize` |
| `ImplicitFixed32Field<T>` | `T: Fixed32ProtoType` | `FIELD` (planned) |
| `RepeatedPackedVarintField<T, A>` | `T: VarintProtoType` | `FIELD` (planned) |

Generated struct members and accessors:

```rust
pub struct Task<A: Allocator = Global> {
    _common: MessageCommon<TaskPresence, A>,
    score: ImplicitVarintField<ProtoInt32>,
    max_retries: ExplicitVarintField<ProtoInt32>,
    status: ImplicitVarintField<ProtoEnum>,
    // …
}

pub fn score(&self) -> i32 {
    self.score.get()
}
pub fn set_score(&mut self, v: i32) {
    self.score.set(v);
}
pub fn max_retries(&self) -> Optional<i32, impl HasDefault<i32>> {
    self.max_retries.get(self._common.parts(), MaxRetriesDefault)
}
// encode (field number + bit index are const args):
self.score.encode_raw::<2, _>(buf);
self.max_retries.encode_raw::<_, _, 3, BIT_MAX_RETRIES>(self._common.parts(), buf);
```

Open enum accessors (`status() -> Result<Status, i32>`) are **thin generated glue** on top of `ImplicitVarintField<ProtoEnum>::get()` + `Status::try_from`. Closed enum merge adds a **policy** hook (unknown variant → `unknown_fields`) — same `ExplicitVarintField<ProtoEnum>` storage, specialised `merge` wrapper.

#### Plugin mapping (replaces flat catalog table)

| Proto field | Generated member type |
|---|---|
| `IMPLICIT int32` | `ImplicitVarintField<ProtoInt32>` |
| `EXPLICIT int32 [default=3]` | `ExplicitVarintField<ProtoInt32>` |
| `IMPLICIT sint32` | `ImplicitVarintField<ProtoSint32>` |
| `IMPLICIT bool` | `ImplicitVarintField<ProtoBool>` |
| `IMPLICIT open enum` | `ImplicitVarintField<ProtoEnum>` |
| `EXPLICIT closed enum` | `ExplicitVarintField<ProtoEnum>` + closed merge policy |
| `IMPLICIT float` | `ImplicitFixed32Field<ProtoFloat>` (planned) |
| `IMPLICIT string` | `ImplicitLenField<ProtoString, A>` (planned) |
| `EXPLICIT string` | `ExplicitLenField<ProtoString, A>` (planned) |
| `repeated int32 PACKED` | `RepeatedPackedVarintField<ProtoInt32, A>` (planned) |
| nested message | `NestedMessageField<M, A>` (planned) |
| `oneof` | [`OneofSlot<E>`](src/fields/oneof.rs) |

Adding a new varint protobuf type (e.g. a future edition type) = **one new `VarintProtoType` impl** — zero changes to `ImplicitVarintField` / `ExplicitVarintField`.

**Not in the catalog:** `oneof` — use [`OneofSlot<E>`](src/fields/oneof.rs). Decode emits **one match arm per variant field number**, each calling `notification.merge_variant_12(…)` / `merge_variant_13(…)` on the slot. Coupling stays inside `OneofSlot` + generated merge helpers, not spread across independent field members.

### Message-level glue (generated, but trivial)

**Encode / `encoded_len`** — sum field contributions; no field order guarantee (§5):

```rust
fn encoded_len(&self) -> usize {
    let p = self._common.parts();
    0
        .adding(self.title.encoded_len(&p))
        .adding(self.score.encoded_len())
        // … every field …
        + self._common.unknown_fields.len()
}
fn encode_raw<B: BufMut>(&self, buf: &mut B) {
    let p = self._common.parts();
    self.title.encode_raw(&p, buf);
    self.score.encode_raw(buf);
    // …
    buf.put_slice(&self._common.unknown_fields);
}
```

**Decode** — only the message owns the tag loop; each arm is one call:

```rust
match field_number {
    1 => self.title.merge(wire_type, self._common.parts_mut(), buf)?,
    2 => self.score.merge(wire_type, buf)?,
    12 => self.notification.merge_email(wire_type, self._common.parts_mut(), buf)?,
    13 => self.notification.merge_phone(wire_type, self._common.parts_mut(), buf)?,
    _ => skip_field_and_save(field_number, wire_type, &mut self._common.unknown_fields, buf)?,
}
```

**Clone / PartialEq / Debug / Default** — same delegation pattern; each catalog type implements the same-named inherent methods.

**`validate()`** — iterate LEGACY_REQUIRED fields only: `owner_id.validate_bit(BIT_OWNER_ID, &self._common.presence)?`.

### Codegen emission per message

For each proto message the plugin emits:

1. **`PresenceBits` impl** on the message's `BitArray` alias (forwards const bit indices).
2. **Struct** — `MessageCommon<P, A>` + one catalog-typed member per field + `OneofSlot` per oneof.
3. **Const block** — `FIELD_*`, `BIT_*` for each field.
4. **Public accessors** — one-line delegates (§4.0 API in DESIGN.md unchanged).
5. **Trait impls** — `MessageEncode`, `MessageDecode`, `Clone`, … as sums of field delegates.
6. **Child modules** — only for nested enums / oneof enums (unchanged).

The plugin's internal IR step is: **`ProtoField → FieldKind enum → pick catalog type → render const args`**. Adding a new proto feature (e.g. map) means **one new catalog type**, not changes to every message template.

### `PresenceBits` on the message bitfield

Generated code implements [`PresenceBits`](src/fields/presence.rs) on the message-specific `BitArray` alias so field types depend on the trait, not on `bitvec` in generated code:

```rust
type TaskPresence = BitArray<[u8; 2], Lsb0>;

impl PresenceBits for TaskPresence {
    fn is_set(&self, bit: usize) -> bool { self[bit] }
    fn set(&mut self, bit: usize, present: bool) { self.set(bit, present); }
}
```

### Independence summary

| Field kind | Touches other field members? | Touches `_common`? |
|---|---|---|
| Singular scalar/string/bytes | No | presence ± alloc |
| Repeated | No | alloc |
| Nested message | No (only own `Option<Box<Child>>`) | alloc |
| Closed enum | No | presence + unknown |
| Open enum | No | presence (if explicit) |
| **Oneof** | **Yes — replaces whole slot** | alloc on variant payload |
| Unknown tags | No | unknown buffer only |

### Implementation status

| Component | Status |
|---|---|
| `MessageCommon`, `MessageParts*`, `PresenceBits`, `OneofSlot` | **Done** |
| `VarintProtoType` + markers (`ProtoInt32` … `ProtoEnum`) | **Done** |
| `ImplicitVarintField` / `ExplicitVarintField` | **Done** |
| `Fixed32ProtoType` / `Fixed64ProtoType` traits | **Stub** |
| LEN / repeated / nested / closed-enum policy wrappers | **Planned** |
| `protoc` plugin field-kind → type mapping | **Planned** |
