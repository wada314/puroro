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

At codegen time the plugin counts every presence-tracked singular field and emits a storage array large enough to hold that many bits (typically `BitArr!(for N, in u8, Lsb0)` or `BitArray<[u64; W], Lsb0>` with `W = ⌈N / 64⌉`). Each field receives a stable bit index assigned **by ascending proto field number among tracked fields only** (gaps in field numbers do not create gaps in bit indices). Generated constants name the indices, e.g. `const BIT_TITLE: usize = 0;`.

**Reference `Task` — five tracked bits** (IMPLICIT / repeated / nested / oneof fields are omitted from the count):

| Proto field | Field # | Presence | Bit index |
|---|---|---|---|
| `title` | 1 | EXPLICIT | `BIT_TITLE = 0` |
| `max_retries` | 3 | EXPLICIT | `BIT_MAX_RETRIES = 1` |
| `owner_id` | 4 | LEGACY_REQUIRED | `BIT_OWNER_ID = 2` |
| `payload` | 5 | EXPLICIT | `BIT_PAYLOAD = 3` |
| `priority` | 10 | EXPLICIT (closed enum) | `BIT_PRIORITY = 4` |

→ `BitArray<[u8; 1], Lsb0>` (one byte, five bits used). **Not** ten bits — older drafts over-counted.

The bitfield lives in [`MessageCommon::presence`](src/fields/common.rs), not as a loose struct member. Generated code wraps `BitArray` in a **message-specific newtype** so it can implement [`PresenceBits`](src/fields/presence.rs) without orphan-rule issues:

```rust
// Illustrative — see sample-generated/src/task.rs
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TaskPresence(BitArray<[u8; 1], Lsb0>);

impl TaskPresence {
    pub const ZERO: Self = Self(BitArray::ZERO);
}

impl PresenceBits for TaskPresence {
    fn is_set(&self, bit: usize) -> bool { self.0[bit] }
    fn set(&mut self, bit: usize, present: bool) { self.0.set(bit, present); }
}

pub struct Task<A: Allocator = Global> {
    _common: MessageCommon<TaskPresence, A>,
    title: SingularLenField<ProtoString, Explicit, A>,       // BIT_TITLE
    score: SingularVarintField<ProtoInt32, Implicit>,        // no bit
    max_retries: SingularVarintField<ProtoInt32, Explicit>,   // BIT_MAX_RETRIES
    owner_id: SingularLenField<ProtoString, LegacyRequired, A>, // BIT_OWNER_ID
    // …
    assignee: NestedMessageField<Address<A>, A>,            // Option — no bit
    notification: OneofSlot<task::Notification<A>>,         // Option — no bit
}
```

**Accessor pattern (EXPLICIT):** `has_title()` → `self.title.has(&self._common)` (delegates to `common.is_present(BIT_TITLE)`); `title()` → `self.title.optional(&self._common, TitleDefault)`; `set_title` / `clear_title` → field catalog `set_str` / `clear` with `&mut self._common` (catalog calls `FieldPresence::on_set` / `on_clear`).

**IMPLICIT fields** still pass a `BIT` const to catalog methods for a uniform signature; `Implicit` ignores it (`on_set` / `on_clear` are no-ops).

**Fields that stay `Option<…>`:** nested messages and oneofs. Absence there implies no heap allocation for that subtree; a separate presence bit would still require an `Option` (or equivalent) on the `Box`/enum payload.

Runtime access: field types call [`MessageCommon::is_present`](src/fields/common.rs) / `set_presence`, which forward to [`PresenceBits`](src/fields/presence.rs). Generated code does **not** index `BitArray` directly outside the `PresenceBits` impl.

---

## 2. Storage types per field kind

| Field kind | Catalog storage (inside wrapper) | Presence |
|---|---|---|
| Implicit-presence scalar (`IMPLICIT`) | `T` in `SingularVarintField<T, Implicit>` | — |
| Explicit-presence scalar (`EXPLICIT`) | `T` in `SingularVarintField<T, Explicit>` | bit in `_common.presence` |
| `LEGACY_REQUIRED` scalar | same as EXPLICIT + `LegacyRequired` marker | bit in `_common.presence` |
| Implicit-presence string / bytes | `Box<str, A>` / `Vec<u8, A>` in `SingularLenField<…, Implicit, A>` | — |
| Explicit-presence string / bytes | same containers in `SingularLenField<…, Explicit, A>` | bit in `_common.presence` |
| Repeated scalar / string / message | `Vec<ElementType, A>` | — (empty vec = absent) |
| Message field | `Option<Box<MessageType<A>, A>>` in `NestedMessageField` | `Option` (not bitfield) |
| Open enum (`OPEN`, IMPLICIT) | `i32` in `SingularVarintField<ProtoEnum, Implicit>` | — |
| Open enum (`OPEN`, EXPLICIT) | `i32` in `SingularVarintField<ProtoEnum, Explicit>` | bit in `_common.presence` |
| Closed enum (`CLOSED`, EXPLICIT) | `i32` in `SingularVarintField<ProtoEnum, Explicit>` | bit in `_common.presence` |
| Oneof group | `Option<OurEnum<A>>` in `OneofSlot` | `Option` (not bitfield) |
| Unknown fields | `Vec<u8, A>` in `_common.unknown_fields` | — |

When an explicit-presence string/bytes field is **unset**, the bit is `false` and the heap container may be empty (no payload allocation until `set_*`). Scalars and enums store the type zero in the value slot when unset; only the bit distinguishes unset from explicitly set zero.

`Box<str, A>` (2 words) is preferred over `Vec<u8, A>` (3 words) for owned strings. `Vec<u8, A>` is used for `bytes` fields.

---

## 3. Struct layout (`Task<A>`)

Generated eager messages are a **product of composable field types** (see [§10](#10-field-centric-codegen-architecture)) plus one shared [`MessageCommon`](src/fields/common.rs) and any oneof slots.

```rust
// Illustrative generated layout for reference `Task`:
pub struct Task<A: Allocator = Global> {
    _common: MessageCommon<TaskPresence, A>,
    title: SingularLenField<ProtoString, Explicit, A>,
    score: SingularVarintField<ProtoInt32, Implicit>,
    max_retries: SingularVarintField<ProtoInt32, Explicit>,
    owner_id: SingularLenField<ProtoString, LegacyRequired, A>,
    payload: SingularLenField<ProtoBytes, Explicit, A>,
    tag_ids: /* RepeatedPacked… — planned */,
    scores: /* RepeatedExpanded… — planned */,
    labels: /* RepeatedString… — planned */,
    status: SingularVarintField<ProtoEnum, Implicit>,
    priority: SingularVarintField<ProtoEnum, Explicit>,
    assignee: NestedMessageField<Address<A>, A>,
    notification: OneofSlot<task::Notification<A>>,
}
```

| Member | Role |
|---|---|
| `_common.presence` | `TaskPresence` newtype over `BitArray<[u8; 1], Lsb0>` — **5** tracked bits (see §1 table) |
| `_common.unknown_fields` | Round-trip unknown wire |
| `_common.alloc` | Cloned by field setters / push |
| `title` … `assignee` | Independent field types; each knows its field number and (if applicable) presence bit |
| `notification` | [`OneofSlot`](src/fields/oneof.rs) — **not** a catalog field type; variants are mutually exclusive |

Working reference implementation: [`sample-generated/`](sample-generated/).

Public accessors on `Task` are **one-line delegates** into the field type, passing `&self._common` or `&mut self._common` as needed. `MessageEncode`, `MessageDecode`, `Clone`, and `PartialEq` on the message are the **sum of the same delegates** — no field-specific logic lives in the message `impl` body beyond dispatch tables.

`TaskLazy<A>` layout is specified in [DESIGN.md §8](DESIGN.md#tasklaya--lazy-parse-timing).

---

## 4. Constructors and the stored allocator

- **`Task::new_in(alloc)`** — initialises every field to its type default; `_common.presence` cleared to all-false (`TaskPresence::ZERO`); heap containers use `Vec::new_in(alloc.clone())` / equivalent.
- **`Task::new()`** — available when `A = Global`.
- **`Default`** — requires `A: Clone + Default`; delegates to `new_in(A::default())`.

Runtime helper **`str_to_box_in(s, alloc) -> Box<str, A>`** copies `s` into a `Vec<u8, A>`, boxes it, then reinterprets as `Box<str, A>` (one `unsafe` block inside the runtime, not in generated code).

---

## 5. Encode implementation

Generated `MessageEncode` for eager messages follows these rules:

1. Walk each present field in **implementation-defined order** (typically declaration order in generated code, but **not guaranteed**).
2. Apply the per-field omit rule from [§7](#7-per-field-encodedecode-patterns) before writing.
3. Append `_common.unknown_fields` verbatim at the end (order relative to known fields is not specified).
4. `encoded_len` must equal the byte count written by `encode_raw`.

**Non-deterministic wire layout.** Two messages with identical field values may encode to **different byte sequences** (field order, spacing inside packed blobs where applicable, ordering inside `_unknown_fields` after merges). Callers must not rely on byte-for-byte equality of `encode_raw` output across encodes or implementations. Semantic equality is via `PartialEq` (see [§8](#8-derived-and-utility-traits)), not wire bytes.

Runtime helpers used: `encode_varint_field`, `encode_len_field`, `encode_packed_*_field`, `encode_i32_field`, `encode_i64_field`, plus `encoded_len_*` counterparts.

---

## 6. Decode implementation

Generated `MessageDecode::merge_from` for eager messages:

1. Loop while the buffer has remaining bytes: read `(field_number, wire_type)` via `decode_tag`.
2. Dispatch on `(field_number, wire_type)` — one arm per known field using patterns in [§7](#7-per-field-encodedecode-patterns).
3. Unknown `(field_number, wire_type)` pairs call `skip_field_and_save` into `_common.unknown_fields`.
4. Singular scalars: last value wins. Repeated: append. Nested messages: merge into existing sub-message (see §7.8).

Nested sub-messages use `Buf::take(len)` to bound the slice to the declared LEN payload length.

---

## 7. Per-field encode/decode patterns

### 7.1 Scalar: implicit presence

**Catalog:** `SingularVarintField<T, Implicit>` (or fixed-width equivalent when implemented). **Encode:** `FieldPresence::should_emit` → omit when value equals type-zero. **Decode:** `merge` assigns value directly (last wins); `Implicit::on_set` is a no-op.

Accessor returns `field.value()` directly — no `Optional` wrapper.

### 7.2 Scalar: explicit presence

**Catalog:** `SingularVarintField<T, Explicit>`. **Encode:** omit when `common.is_present(BIT)` is false; emit even if value is zero when the bit is set. **Decode:** `Explicit::on_set` then assign value (last wins).

**Generated accessor:** `has_*()` → `field.has(&self._common)`; `*_()` → `field.optional(&self._common, ThatDefault)` with a private ZST implementing `HasDefault<T>`. On traits, `_raw` and `has_` are default methods calling `.get()` / `.is_set()`.

**Fallible trait on `Task<A>`:** wrap the infallible `Optional` getter in `Ok(…)` with `Error = Infallible`.

`Optional` is a concrete struct with trivial drop — `task.max_retries().get()` chains without a `let` binding. No conversion to `Option<T>` is provided (see DESIGN.md §3).

### 7.3 String fields

**Catalog:** `SingularLenField<ProtoString, P, A>` where `P` is `Implicit` or `Explicit` / `LegacyRequired`. Payload is `Box<str, A>` inside the wrapper.

- **Encode:** `P::should_emit(common, BIT, is_empty)` — EXPLICIT omits when bit unset; IMPLICIT omits when empty.
- **Decode:** `merge` → `decode_string_in` via `LenProtoType`; `P::on_set` for EXPLICIT / LEGACY_REQUIRED.
- **Setter:** `set_str(&mut common, v)` — sets bit (if applicable) then `str_to_box_in`.
- **Clear:** `clear(&mut common)` on EXPLICIT types.
- **Accessor:** IMPLICIT → `borrow()`; EXPLICIT → `optional(&common, default)`.

### 7.4 Bytes fields

**Catalog:** `SingularLenField<ProtoBytes, P, A>`. Same presence rules as strings; `set_from_slice` for setters.

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

**Catalog:** `SingularVarintField<ProtoEnum, P>`. Wire encoding is VARINT. IMPLICIT encode omits when zero; EXPLICIT uses the presence bit like any other scalar.

### 7.10 Closed enum fields

**Catalog:** `SingularVarintField<ProtoEnum, Explicit>`. On decode, call [`merge_closed`](src/fields/scalar.rs) with an `is_known` predicate; unknown numeric values go to `_common.unknown_fields` **without** setting the presence bit. Encode includes the field even for the zero variant when the bit is set.

```rust
self.priority.merge_closed::<_, _, _, FIELD_PRIORITY, BIT_PRIORITY>(
    &mut self._common,
    wire_type,
    buf,
    |v| Priority::try_from(v).is_ok(),
)?;
```

### 7.11 Oneof fields

**Storage:** `Option<NotificationEnum<A>>`. Each variant arm overwrites the previous on decode. Encode writes only the active variant's field number.

### 7.12 Unknown fields

**Storage:** `_common.unknown_fields: Vec<u8, A>` — valid partial wire stream. Accumulate via `skip_field_and_save`; re-emit with `put_slice` on encode. Deprecated group wire types are **not** saved — see DESIGN.md §0.

### 7.13 LEGACY_REQUIRED fields

**Catalog:** `Singular*Field<T, LegacyRequired>` — wire/encode/merge identical to `Explicit` ([`FieldPresence`](src/fields/field_presence.rs) impl delegates to the same bit rules). Additionally generate:

- **`validate()`** — per required field: `field.validate_required::<_, BIT>(&self._common, FIELD_NO)?` (LEN fields) or equivalent via [`RequiredFieldPresence::validate_present`](src/fields/field_presence.rs).
- **`decode_strict()`** — `decode` then `validate`.

`MessageDecode::decode` does **not** call `validate` automatically.

---

## 8. Derived and utility traits

Generated **message** structs (`Task<A>`, nested messages, etc.) implement the traits below. **Enum types** (`Status`, `Priority`, oneof enums) are separate: they are plain `Copy` types and always get `Clone, Copy, Debug, PartialEq, Eq, Hash` via `derive` (see DESIGN.md §4.6).

### Always generated (any `A: Allocator + Clone`)

| Trait | Bounds on `A` | Behaviour |
|---|---|---|
| **`Default`** | `A: Allocator + Clone + Default` | `Default::default()` → `Task::new_in(A::default())`. Clears `_common.presence`; heap fields empty. |
| **`Clone`** | `A: Allocator + Clone` | Deep clone: copy `_common.presence` bitwise; clone each catalog field; clone `_common.unknown_fields`; `_common.alloc.clone()`. |
| **`Debug`** | none beyond field types | Prints field values and presence (e.g. `title: … (set)` / `(unset)`). Does not print `_alloc`. |
| **`PartialEq`** | none beyond field types | **Semantic equality:** compares `_common.presence`, field values, `_common.unknown_fields`; unset explicit fields compare equal regardless of stale value slots; **`_common.alloc` is ignored**. |
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
            _common: MessageCommon {
                presence: self._common.presence, // Copy newtype
                unknown_fields: self._common.unknown_fields.clone(),
                alloc: self._common.alloc.clone(),
            },
            title: self.title.clone(),
            // … each catalog field …
            assignee: self.assignee.clone(),
            notification: self.notification.clone(),
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
│  VarintProtoType + SingularVarintField<T, P>                 │
│  LenProtoType + SingularLenField<T, P, A>  FieldPresence    │
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

Field methods take **`&MessageCommon`** or **`&mut MessageCommon`** — not `&Task` — so field types stay decoupled from the parent message type.

```rust
// Generated accessor (always this shape):
pub fn title(&self) -> Optional<&str, impl HasDefault<&str>> {
    self.title.get(&self._common)
}
pub fn set_title(&mut self, v: &str)
where
    A: Clone,
{
    self.title.set(&mut self._common, v);
}
```

### Two-layer field catalog

Protobuf field behaviour splits into two **orthogonal** axes:

| Axis | What varies | Where it lives |
|---|---|---|
| **Wire encoding** | int32 vs sint32 vs bool vs float vs string … | Zero-sized marker + trait (`VarintProtoType`, `Fixed32ProtoType`, …) |
| **Presence** | IMPLICIT vs EXPLICIT vs LEGACY_REQUIRED | [`FieldPresence`](src/fields/field_presence.rs) marker (`Implicit` / `Explicit` / `LegacyRequired`) composed into `Singular*Field<T, P>` |

The plugin never emits `ImplicitI32` vs `ImplicitSint32` as separate catalog entries — it emits **`SingularVarintField<ProtoInt32, Implicit>`** (alias `ImplicitVarintField<ProtoInt32>`) vs **`SingularVarintField<ProtoSint32, Implicit>`**. All varint wire logic is written **once** in `SingularVarintField`; presence policy lives in **`FieldPresence`** impls; zigzag/bool/enum semantics live in **`VarintProtoType`** impls.

```
                    ┌─────────────────────────────────────┐
  ProtoField        │  SingularVarintField<T, P>          │
  ─────────►        │  SingularLenField<T, P, A>          │
  int32 IMPLICIT    │  SingularFixed32Field<T, P> (plan)  │
  int32 EXPLICIT    └──────────────┬──────────────────────┘
                                   │ P: FieldPresence (Implicit / Explicit / LegacyRequired)
                    ┌──────────────▼──────────────────────┐
                    │  Implicit  — omit when payload empty│
                    │  Explicit  — bitfield via common      │
                    │  LegacyRequired — same as Explicit  │
                    │    + validate_required on message   │
                    └──────────────┬──────────────────────┘
                                   │ T: VarintProtoType / LenProtoType / …
                    ┌──────────────▼──────────────────────┐
  wire layer        │  ProtoInt32, ProtoSint32, ProtoBool,  │
  (marker types)    │  ProtoUInt64, ProtoEnum, ProtoString… │
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

Parallel traits for other wire families ([`fixed32.rs`](src/fields/fixed32.rs), [`fixed64.rs`](src/fields/fixed64.rs), [`len.rs`](src/fields/len.rs)):

| Trait | Wire type | Markers |
|---|---|---|
| `VarintProtoType` | VARINT | above |
| `Fixed32ProtoType` | I32 | `ProtoFixed32`, `ProtoSfixed32`, `ProtoFloat` (planned) |
| `Fixed64ProtoType` | I64 | `ProtoFixed64`, `ProtoSfixed64`, `ProtoDouble` (planned) |
| `LenProtoType` | LEN | `ProtoString`, `ProtoBytes` (**done**); `ProtoMessage<M>` (planned) |

**Important:** Rust storage type alone does **not** identify protobuf encoding (`i32` can be int32, sint32, sfixed32, or enum). The marker type is the source of truth — matching the naming rule in `protobuf-core` docs.

#### Layer 2 — presence policy ([`src/fields/field_presence.rs`](src/fields/field_presence.rs))

[`FieldPresence`](src/fields/field_presence.rs) is a zero-sized marker trait with three hooks used by all singular wrappers:

| Method | Role |
|---|---|
| `should_emit(common, bit, payload_empty)` | Whether to write on encode |
| `on_set(common, bit)` | After setter / merge (sets bit for EXPLICIT) |
| `on_clear(common, bit)` | When EXPLICIT field is cleared |

| Marker | Behaviour |
|---|---|
| `Implicit` | Emit when payload non-empty; no bitfield updates |
| `Explicit` | Emit when `common.is_present(bit)`; bitfield on set/clear |
| `LegacyRequired` | Same wire behaviour as `Explicit`; adds [`RequiredFieldPresence::validate_present`](src/fields/field_presence.rs) |

[`ExplicitFieldPresence`](src/fields/field_presence.rs) is a sub-trait gating EXPLICIT-only accessors (`optional`, `has`, `clear`). Implemented for `Explicit` and `LegacyRequired`.

#### Layer 3 — singular wrappers ([`src/fields/scalar.rs`](src/fields/scalar.rs), [`len_field.rs`](src/fields/len_field.rs))

One generic struct per wire family, parametrised by wire marker `T` and presence `P`:

| Wrapper | Param `T` | Param `P` | Type aliases |
|---|---|---|---|
| `SingularVarintField<T, P>` | `T: VarintProtoType` | `P: FieldPresence` | `ImplicitVarintField<T>`, `ExplicitVarintField<T>` |
| `SingularLenField<T, P, A>` | `T: LenProtoType` | `P: FieldPresence` | `ImplicitString<A>`, `ExplicitString<A>`, … |
| `SingularFixed32Field<T, P>` | `T: Fixed32ProtoType` | `P: FieldPresence` | (planned) |

Const params (`FIELD: u32`, `BIT: usize`) are method type parameters, not struct generics.

Generated struct members and accessors:

```rust
pub struct Task<A: Allocator = Global> {
    _common: MessageCommon<TaskPresence, A>,
    score: ImplicitVarintField<ProtoInt32>,           // = SingularVarintField<ProtoInt32, Implicit>
    max_retries: ExplicitVarintField<ProtoInt32>,     // = SingularVarintField<ProtoInt32, Explicit>
    status: ImplicitVarintField<ProtoEnum>,
    // …
}

pub fn score(&self) -> i32 {
    self.score.value()
}
pub fn set_score(&mut self, v: i32) {
    self.score.set::<_, _, BIT_SCORE>(&mut self._common, v);  // IMPLICIT: on_set is a no-op
}
pub fn max_retries(&self) -> Optional<i32, impl HasDefault<i32>> {
    self.max_retries.optional::<_, _, MaxRetriesDefault, BIT_MAX_RETRIES>(&self._common, MaxRetriesDefault)
}
// encode (field number + bit index are const method params):
self.score.encode_raw::<_, _, _, FIELD_SCORE, BIT_UNUSED>(&self._common, buf);
self.max_retries.encode_raw::<_, _, _, FIELD_MAX_RETRIES, BIT_MAX_RETRIES>(&self._common, buf);
```

Open enum accessors (`status() -> Result<Status, i32>`) are **thin generated glue** on top of `SingularVarintField<ProtoEnum, Implicit>::value()` + `Status::try_from`. Closed enum decode uses [`merge_closed`](src/fields/scalar.rs) (see §7.10).

#### Plugin mapping (replaces flat catalog table)

| Proto field | Generated member type |
|---|---|
| `IMPLICIT int32` | `ImplicitVarintField<ProtoInt32>` |
| `EXPLICIT int32 [default=3]` | `ExplicitVarintField<ProtoInt32>` |
| `IMPLICIT sint32` | `ImplicitVarintField<ProtoSint32>` |
| `IMPLICIT bool` | `ImplicitVarintField<ProtoBool>` |
| `IMPLICIT open enum` | `ImplicitVarintField<ProtoEnum>` |
| `EXPLICIT closed enum` | `SingularVarintField<ProtoEnum, Explicit>` + `merge_closed` |
| `LEGACY_REQUIRED string` | `SingularLenField<ProtoString, LegacyRequired, A>` |
| `IMPLICIT float` | `ImplicitFixed32Field<ProtoFloat>` (planned) |
| `IMPLICIT string` | `ImplicitString<A>` (= `SingularLenField<ProtoString, Implicit, A>`) |
| `EXPLICIT string` | `ExplicitString<A>` (= `SingularLenField<ProtoString, Explicit, A>`) |
| `IMPLICIT bytes` | `ImplicitBytes<A>` |
| `EXPLICIT bytes` | `ExplicitBytes<A>` |
| `repeated int32 PACKED` | `RepeatedPackedVarintField<ProtoInt32, A>` (planned) |
| nested message | `NestedMessageField<M, A>` |
| `oneof` | [`OneofSlot<E>`](src/fields/oneof.rs) |

Adding a new varint protobuf type (e.g. a future edition type) = **one new `VarintProtoType` impl** — zero changes to `SingularVarintField`. Adding a new presence mode = **one new `FieldPresence` impl** — zero changes to wire encoding.

**Not in the catalog:** `oneof` — use [`OneofSlot<E>`](src/fields/oneof.rs). Decode emits **one match arm per variant field number**, each calling `notification.merge_variant_12(…)` / `merge_variant_13(…)` on the slot. Coupling stays inside `OneofSlot` + generated merge helpers, not spread across independent field members.

### Message-level glue (generated, but trivial)

**Encode / `encoded_len`** — sum field contributions; no field order guarantee (§5):

```rust
fn encoded_len(&self) -> usize {
    let c = &self._common;
    self.title.encoded_len::<_, FIELD_TITLE, BIT_TITLE>(c)
        + self.score.encoded_len::<_, _, FIELD_SCORE, BIT_UNUSED>(c)
        // … every field …
        + c.unknown_fields.len()
}
fn encode_raw<B: BufMut>(&self, buf: &mut B) {
    let c = &self._common;
    self.title.encode_raw::<_, _, FIELD_TITLE, BIT_TITLE>(c, buf);
    self.score.encode_raw::<_, _, _, FIELD_SCORE, BIT_UNUSED>(c, buf);
    // …
    buf.put_slice(&c.unknown_fields);
}
```

**Decode** — only the message owns the tag loop; each arm is one call:

```rust
match field_number {
    FIELD_TITLE => self.title.merge::<_, _, BIT_TITLE>(&mut self._common, wire_type, buf)?,
    FIELD_SCORE => self.score.merge::<_, _, _, BIT_UNUSED>(&mut self._common, wire_type, buf)?,
    FIELD_PRIORITY => self.priority.merge_closed::<_, _, _, FIELD_PRIORITY, BIT_PRIORITY>(
        &mut self._common, wire_type, buf, |v| Priority::try_from(v).is_ok(),
    )?,
    // oneof arms …
    _ => skip_field_and_save(field_number, wire_type, buf, &mut self._common.unknown_fields)?,
}
```

**Clone / PartialEq / Debug / Default** — same delegation pattern; each catalog type implements the same-named inherent methods.

**`validate()`** — per LEGACY_REQUIRED field, e.g. `self.owner_id.validate_required::<_, BIT_OWNER_ID>(&self._common, FIELD_OWNER_ID)?`.

### Codegen emission per message

For each proto message the plugin emits:

1. **`PresenceBits` impl** on the message-specific presence **newtype** (wraps `BitArray`, forwards bit indices).
2. **Struct** — `MessageCommon<P, A>` + one catalog-typed member per field + `OneofSlot` per oneof.
3. **Const block** — `FIELD_*`, `BIT_*` for each field.
4. **Public accessors** — one-line delegates (§4.0 API in DESIGN.md unchanged).
5. **Trait impls** — `MessageEncode`, `MessageDecode`, `Clone`, … as sums of field delegates.
6. **Child modules** — only for nested enums / oneof enums (unchanged).

The plugin's internal IR step is: **`ProtoField → FieldKind enum → pick catalog type → render const args`**. Adding a new proto feature (e.g. map) means **one new catalog type**, not changes to every message template.

### `PresenceBits` on the message bitfield

Generated code implements [`PresenceBits`](src/fields/presence.rs) on a message-specific newtype wrapping `BitArray` (required for orphan rules — see §1):

```rust
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TaskPresence(BitArray<[u8; 1], Lsb0>);

impl PresenceBits for TaskPresence {
    fn is_set(&self, bit: usize) -> bool { self.0[bit] }
    fn set(&mut self, bit: usize, present: bool) { self.0.set(bit, present); }
}
```

`Address` uses the same pattern with **two** tracked bits (`street`, `city`) in `BitArray<[u8; 1], Lsb0>`.

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
| `MessageCommon`, `PresenceBits`, `OneofSlot` | **Done** |
| `VarintProtoType` + markers | **Done** |
| `FieldPresence` + `Implicit` / `Explicit` / `LegacyRequired` | **Done** |
| `merge_closed` on `SingularVarintField` | **Done** |
| `validate_required` on `SingularLenField<_, LegacyRequired, _>` | **Done** |
| `sample-generated` reference (`Task` / `Address`) | **Done** |
| `SingularVarintField<T, P>` (+ `ImplicitVarintField` / `ExplicitVarintField` aliases) | **Done** |
| `LenProtoType` + `ProtoString` / `ProtoBytes` | **Done** |
| `SingularLenField<T, P, A>` (+ string/bytes aliases) | **Done** |
| `NestedMessageField` | **Done** |
| `Fixed32ProtoType` / `Fixed64ProtoType` traits | **Stub** |
| Repeated LEN / packed scalar wrappers | **Planned** |
| `protoc` plugin field-kind → type mapping | **Planned** |
