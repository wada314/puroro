# puroro — Implementation Notes

Internal implementation of **generated** protobuf message code: storage, wire I/O, and runtime catalog usage. The stable public API is [DESIGN.md](DESIGN.md).

> **Note:** DESIGN.md accessors stay stable even if internals change (e.g. bitfield layout).

**Canonical examples:** `Task` and `Address` from [DESIGN.md §4](DESIGN.md#reference-schema). Working code: [`sample-generated/`](sample-generated/).

---

## Table of contents

**Part I — Overview**

1. [Project context](#1-project-context)
2. [Architecture overview](#2-architecture-overview)
3. [Implementation status](#3-implementation-status)

**Part II — Runtime catalog (`puroro::fields`)**

4. [Shared infrastructure](#4-shared-infrastructure)
5. [Wire encoding traits](#5-wire-encoding-traits)
6. [Presence policy](#6-presence-policy)
7. [Field wrappers](#7-field-wrappers)
8. [Proto field → catalog mapping](#8-proto-field--catalog-mapping)

**Part III — Generated message**

9. [Struct layout](#9-struct-layout)
10. [Presence bit indices](#10-presence-bit-indices)
11. [Constructors & allocator](#11-constructors--allocator)
12. [Message-level wire I/O](#12-message-level-wire-io)
13. [Derived traits](#13-derived-traits)

**Part IV — Field behaviour**

14. [Singular fields](#14-singular-fields)
15. [Repeated fields](#15-repeated-fields)
16. [Nested messages, oneof, unknown fields](#16-nested-messages-oneof-unknown-fields)

**Part V — Future work**

17. [Planned optimisations & runtime gaps](#17-planned-optimisations--runtime-gaps)

---

## Part I — Overview

## 1. Project context

| Crate | Responsibility |
|---|---|
| **`protobuf-core`** | Wire primitives (`Varint`, `Tag`, `WireType`). Used by `puroro`; generated code does not import it. |
| **`puroro`** | Message runtime: `MessageEncode` / `MessageDecode`, `Optional`, [`fields`](src/fields.rs), encode/decode helpers. |
| **`protoc` plugin** | Emits Rust types and `impl` blocks described here ([DESIGN.md §0](DESIGN.md#0-project-architecture)). |

---

## 2. Architecture overview

Protobuf fields (except **oneof**) are **independent**: each getter/setter/encode/merge arm touches only its own struct member plus shared [`MessageCommon`](#4-shared-infrastructure). The plugin **composes** runtime catalog types — it does not hand-write per-field logic.

### Layer stack

```
protoc plugin
    │  proto field → catalog type + const FIELD / BIT
    ▼
puroro::fields          SingularVarintField<T, P>, SingularLenField<T, P, A>, …
    │  T: VarintProtoType / LenProtoType / …
    │  P: FieldPresence (Implicit / Explicit / LegacyRequired)
    ▼
puroro::encode/decode   Buf adapters, LEN framing, unknown-field helpers
    ▼
protobuf-core           Varint, Tag, WireType
```

### Design goals

| Goal | Approach |
|---|---|
| Minimal generated logic | Message `impl` = thin delegates + `match` dispatch |
| One impl per pattern | Wire trait × presence marker × thin wrapper |
| Monomorphised hot path | `const FIELD` / `BIT` as method type parameters |
| Stable public API | `task.title()` unchanged; internals evolve freely |

### Field independence

| Field kind | Touches other members? | Touches `_common`? |
|---|---|---|
| Singular scalar / string / bytes | No | presence ± alloc |
| Repeated | No | alloc |
| Nested message | No | alloc |
| Closed enum | No | presence + unknown buffer |
| Open enum (explicit) | No | presence |
| **Oneof** | **Yes** (replaces slot) | alloc |
| Unknown tags | No | unknown buffer |

`TaskLazy<A>` layout: [DESIGN.md §8](DESIGN.md#tasklaya--lazy-parse-timing).

---

## 3. Implementation status

| Component | Status |
|---|---|
| `MessageCommon`, `PresenceBits`, `OneofSlot` | **Done** |
| `VarintProtoType` + markers | **Done** |
| `LenProtoType` + `ProtoString` / `ProtoBytes` | **Done** |
| `FieldPresence` (`Implicit` / `Explicit` / `LegacyRequired`) | **Done** |
| `SingularVarintField<T, P>`, `SingularLenField<T, P, A>` | **Done** |
| `merge_closed`, `validate_required` | **Done** |
| `NestedMessageField` | **Done** |
| [`sample-generated`](sample-generated/) (`Task` / `Address`) | **Done** |
| `Fixed32ProtoType` / `Fixed64ProtoType` | **Stub** |
| Repeated catalog wrappers | **Planned** |
| `protoc` plugin | **Planned** |

---

## Part II — Runtime catalog (`puroro::fields`)

## 4. Shared infrastructure

[`MessageCommon<P, A>`](src/fields/common.rs) — one per generated message:

| Member | Role |
|---|---|
| `presence: P` | Bitfield newtype (`TaskPresence`, …) for EXPLICIT / LEGACY_REQUIRED singular fields |
| `unknown_fields: Vec<u8, A>` | Round-trip unknown wire; closed-enum unknown variants |
| `alloc: A` | Cloned by setters, decode, repeated push |

Field catalog methods take `&MessageCommon` / `&mut MessageCommon`, not `&Task`, so wrappers stay decoupled from the parent message type.

[`PresenceBits`](src/fields/presence.rs) — trait implemented on the message-specific presence **newtype** (not on raw `BitArray` — orphan rules). [`MessageCommon::is_present`](src/fields/common.rs) / `set_presence` forward to it.

---

## 5. Wire encoding traits

One marker + trait per protobuf **wire family**. Semantic conversions delegate to **`protobuf-core`** (puroro does not reimplement zigzag/varint).

### Varint ([`varint.rs`](src/fields/varint.rs))

```rust
pub trait VarintProtoType {
    type Value: Copy + PartialEq;
    fn proto_zero() -> Self::Value;
    fn decode_wire(raw: u64) -> Result<Self::Value, DecodeError>;
    fn encode_wire(value: Self::Value) -> u64;
}
// Markers: ProtoInt32, ProtoInt64, ProtoUInt32, ProtoUInt64,
//          ProtoSint32, ProtoSint64, ProtoBool, ProtoEnum
```

### Other families

| Trait | Wire | Markers | Status |
|---|---|---|---|
| `LenProtoType` | LEN | `ProtoString`, `ProtoBytes` | **Done** |
| `Fixed32ProtoType` | I32 | `ProtoFixed32`, `ProtoFloat`, … | Stub |
| `Fixed64ProtoType` | I64 | `ProtoFixed64`, `ProtoDouble`, … | Stub |

Rust storage type alone does **not** identify protobuf encoding (`i32` can be int32, sint32, or enum). The marker type is the source of truth.

---

## 6. Presence policy

[`FieldPresence`](src/fields/field_presence.rs) — ZST markers composed into singular wrappers as type param `P`:

| Method | Role |
|---|---|
| `should_emit(common, bit, payload_empty)` | Encode omit rule |
| `on_set(common, bit)` | After setter / merge |
| `on_clear(common, bit)` | On clear |

| Marker | Encode | Bitfield | Accessors |
|---|---|---|---|
| `Implicit` | Omit when payload empty / type-zero | No-op | `value()` / `borrow()` |
| `Explicit` | Omit when bit unset | Set/clear bit | `optional`, `has`, `clear` |
| `LegacyRequired` | Same as `Explicit` | Same as `Explicit` | Same + `validate_required` |

[`ExplicitFieldPresence`](src/fields/field_presence.rs) gates `optional` / `has` / `clear` (implemented for `Explicit` and `LegacyRequired`). [`RequiredFieldPresence`](src/fields/field_presence.rs) adds `validate_present` for `LegacyRequired`.

IMPLICIT fields still pass `BIT` to catalog methods for a uniform signature; `Implicit` ignores it.

---

## 7. Field wrappers

One generic struct per wire family, parametrised by wire marker `T` and presence `P`. `FIELD: u32` and `BIT: usize` are **method** const parameters.

| Wrapper | Params | Aliases (ergonomics) |
|---|---|---|
| `SingularVarintField<T, P>` | `T: VarintProtoType`, `P: FieldPresence` | `ImplicitVarintField<T>`, `ExplicitVarintField<T>` |
| `SingularLenField<T, P, A>` | `T: LenProtoType`, `P`, `A: Allocator` | `ImplicitString<A>`, `ExplicitString<A>`, … |
| `SingularFixed32Field<T, P>` | (planned) | — |
| `NestedMessageField<M, A>` | `Option<Box<M, A>>` — no bitfield | — |
| `OneofSlot<E>` | mutually exclusive variants | — |

**Closed enum:** `SingularVarintField<ProtoEnum, Explicit>::merge_closed(…, is_known)` — unknown values → `unknown_fields`, bit not set.

**LEGACY_REQUIRED (LEN):** `SingularLenField<…, LegacyRequired, A>::validate_required`.

Adding a wire type = one new `VarintProtoType` impl. Adding a presence mode = one new `FieldPresence` impl.

---

## 8. Proto field → catalog mapping

| Proto field | Generated member type |
|---|---|
| `IMPLICIT int32` | `SingularVarintField<ProtoInt32, Implicit>` |
| `EXPLICIT int32` | `SingularVarintField<ProtoInt32, Explicit>` |
| `IMPLICIT sint32` / `bool` | `SingularVarintField<ProtoSint32 \| ProtoBool, Implicit>` |
| `IMPLICIT open enum` | `SingularVarintField<ProtoEnum, Implicit>` |
| `EXPLICIT closed enum` | `SingularVarintField<ProtoEnum, Explicit>` + `merge_closed` |
| `IMPLICIT string` | `SingularLenField<ProtoString, Implicit, A>` |
| `EXPLICIT string` | `SingularLenField<ProtoString, Explicit, A>` |
| `LEGACY_REQUIRED string` | `SingularLenField<ProtoString, LegacyRequired, A>` |
| `IMPLICIT` / `EXPLICIT bytes` | `SingularLenField<ProtoBytes, P, A>` |
| `repeated int32 PACKED` | `RepeatedPackedVarintField<…>` (planned) |
| nested message | `NestedMessageField<M, A>` |
| `oneof` | `OneofSlot<E>` — not a singular catalog entry |

---

## Part III — Generated message

## 9. Struct layout

```rust
pub struct Task<A: Allocator = Global> {
    _common: MessageCommon<TaskPresence, A>,
    title: SingularLenField<ProtoString, Explicit, A>,
    score: SingularVarintField<ProtoInt32, Implicit>,
    max_retries: SingularVarintField<ProtoInt32, Explicit>,
    owner_id: SingularLenField<ProtoString, LegacyRequired, A>,
    payload: SingularLenField<ProtoBytes, Explicit, A>,
    tag_ids: /* repeated — planned */,
    scores: /* repeated — planned */,
    labels: /* repeated — planned */,
    status: SingularVarintField<ProtoEnum, Implicit>,
    priority: SingularVarintField<ProtoEnum, Explicit>,
    assignee: NestedMessageField<Address<A>, A>,
    notification: OneofSlot<task::Notification<A>>,
}
```

### Storage summary

| Field kind | Inside catalog wrapper | Presence |
|---|---|---|
| IMPLICIT scalar / open enum | `T` or `i32` | — |
| EXPLICIT scalar / enum / string / bytes | same containers | bit in `_common.presence` |
| LEGACY_REQUIRED | same as EXPLICIT + `LegacyRequired` | bit |
| Repeated | `Vec<…, A>` (planned catalog) | empty = absent |
| Nested message | `Option<Box<M, A>>` | `Option`, not bitfield |
| Oneof | `Option<E>` in slot | `Option`, not bitfield |

Unset EXPLICIT slots may hold type-zero / empty heap data; **only the bit** means "set". `Box<str, A>` for strings; `Vec<u8, A>` for bytes.

Public accessors are **one-line delegates** into catalog methods with `&self._common` / `&mut self._common`. `MessageEncode`, `MessageDecode`, `Clone`, `PartialEq` sum the same delegates.

### Codegen emission per message

1. Presence **newtype** + `PresenceBits` impl
2. Struct — `MessageCommon` + catalog members + `OneofSlot` per oneof
3. Const block — `FIELD_*`, `BIT_*`
4. Accessor delegates ([DESIGN.md §4](DESIGN.md#40-generated-per-message-traits))
5. Trait impls — encode/decode/clone/eq as field sums
6. Child modules — enums, oneof enums

IR step: `ProtoField → FieldKind → catalog type + const args`.

---

## 10. Presence bit indices

Tracked singular fields use [`bitvec::BitArray`](https://docs.rs/bitvec) inline in the message (`BitVec` is heap-only and incompatible with custom `A`).

**Rules:** count only EXPLICIT / LEGACY_REQUIRED singular fields; assign bits by **ascending field number** (gaps in field numbers do not create gaps in bit indices); store in `_common.presence` inside a message-specific newtype.

### `Task` — five bits → `BitArray<[u8; 1], Lsb0>`

| Field | # | Presence | `BIT_*` |
|---|---|---|---|
| `title` | 1 | EXPLICIT | `0` |
| `max_retries` | 3 | EXPLICIT | `1` |
| `owner_id` | 4 | LEGACY_REQUIRED | `2` |
| `payload` | 5 | EXPLICIT | `3` |
| `priority` | 10 | EXPLICIT | `4` |

### `Address` — two bits → `BitArray<[u8; 1], Lsb0>`

| Field | # | `BIT_*` |
|---|---|---|
| `street` | 1 | `0` |
| `city` | 2 | `1` |

### Newtype pattern

```rust
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TaskPresence(BitArray<[u8; 1], Lsb0>);

impl TaskPresence {
    pub const ZERO: Self = Self(BitArray::ZERO);
}

impl PresenceBits for TaskPresence {
    fn is_set(&self, bit: usize) -> bool { self.0[bit] }
    fn set(&mut self, bit: usize, present: bool) { self.0.set(bit, present); }
}
```

Generated code indexes bits only inside this `impl`, never elsewhere.

---

## 11. Constructors & allocator

- **`Task::new_in(alloc)`** — default every field; `_common.presence = TaskPresence::ZERO`; heap fields via `*_in(alloc.clone())`.
- **`Task::new()`** — when `A = Global`.
- **`Default`** — `A: Clone + Default` → `new_in(A::default())`.

Runtime **`str_to_box_in(s, alloc)`** — copy bytes into `Box<str, A>` (one `unsafe` in runtime, not generated code).

---

## 12. Message-level wire I/O

### Encode

1. Each field's `encoded_len` / `encode_raw` (catalog applies omit rules — [§14](#14-singular-fields)).
2. Append `_common.unknown_fields` verbatim.
3. `encoded_len` must match bytes written.

**Field order is not guaranteed.** Identical logical content may produce different wire bytes. Compare with `PartialEq`, not wire equality.

Runtime: `encode_varint_field`, `encode_len_field`, `encode_packed_*`, `encoded_len_*`.

```rust
fn encode_raw<B: BufMut>(&self, buf: &mut B) {
    let c = &self._common;
    self.title.encode_raw::<_, _, FIELD_TITLE, BIT_TITLE>(c, buf);
    self.score.encode_raw::<_, _, _, FIELD_SCORE, BIT_UNUSED>(c, buf);
    // …
    buf.put_slice(&c.unknown_fields);
}
```

### Decode

1. Loop: `decode_tag` → `(field_number, wire_type)`.
2. `match field_number` — one catalog `merge` (or `merge_closed`) per arm.
3. Unknown → `skip_field_and_save` into `_common.unknown_fields`.
4. Singular: last wins. Repeated: append. Nested: merge sub-buffer.

```rust
FIELD_PRIORITY => self.priority.merge_closed::<_, _, _, FIELD_PRIORITY, BIT_PRIORITY>(
    &mut self._common, wire_type, buf, |v| Priority::try_from(v).is_ok(),
)?,
```

Nested LEN payloads use `Buf::take(len)` before child `merge_from`.

### Validation

`validate()` — `owner_id.validate_required::<_, BIT_OWNER_ID>(&self._common, FIELD_OWNER_ID)?` (and any other `LegacyRequired` fields). `decode_strict` = decode + validate. `MessageDecode::decode` does **not** auto-validate.

---

## 13. Derived traits

**Messages** (`Task<A>`, …): generated as below. **Enums** (`Status`, `Priority`, oneof): `derive(Clone, Copy, Debug, PartialEq, Eq, Hash)`.

| Trait | Bounds | Notes |
|---|---|---|
| `Default` | `A: Clone + Default` | Clears presence; empty heap fields |
| `Clone` | `A: Clone` | Copy presence newtype; deep-clone catalog fields + `unknown_fields` |
| `Debug` | — | Values + set/unset; no `_alloc` |
| `PartialEq` / `Eq` | — | Semantic: presence + values + `unknown_fields`; ignore stale unset slots and `_alloc` |

**Not generated:** `Copy`, `Ord`, `Hash` (deferred).

**`Global` extras:** `Task::new()`, `impl Default for Task`, `Task::decode(buf)`.

Arena clone copies data into a second tree — use `Arc<Task<A>>` for shared immutable messages.

---

## Part IV — Field behaviour

## 14. Singular fields

### Varint (`SingularVarintField<T, P>`)

| | IMPLICIT | EXPLICIT |
|---|---|---|
| Encode | Omit at type-zero | Omit when bit unset; emit zero if bit set |
| Merge | Set value; `on_set` no-op | `on_set` + set value |
| Getter | `value()` | `optional(&common, DefaultZst)` |
| Setter | `set(&mut common, v)` | same (sets bit) |

Open enum: thin glue — `Status::try_from(field.value())`. Closed enum: use `merge_closed` ([§7](#7-field-wrappers)). `Optional` is a concrete struct (DESIGN.md §3); no `Option<T>` conversion.

### LEN — string & bytes (`SingularLenField<T, P, A>`)

| | IMPLICIT | EXPLICIT / LEGACY_REQUIRED |
|---|---|---|
| Encode | Omit when empty | Omit when bit unset |
| Merge | `decode_string_in` / `decode_bytes_in` | + `on_set` |
| Getter | `borrow()` | `optional(&common, default)` |
| Setter | `set_str` / `set_from_slice` | same (sets bit when applicable) |
| Clear | `clear_value(alloc)` | `clear(&mut common)` |

### LEGACY_REQUIRED

Wire identical to EXPLICIT. Message `validate()` calls `validate_required` on each `LegacyRequired` field ([§12](#12-message-level-wire-io)).

---

## 15. Repeated fields

**Status:** planned catalog types; [`sample-generated`](sample-generated/) uses inline helpers.

| Pattern | Storage | Encode | Decode |
|---|---|---|---|
| Packed scalar | `Vec<T, A>` | One LEN blob | **Both** `WireType::Len` (packed) and `Varint` (single element) — spec requirement |
| Expanded scalar | `Vec<T, A>` | One record per element | Same dual-arm accept |
| Repeated string | `Vec<Box<str, A>, A>` | One LEN per element | `decode_string_in` + push |

---

## 16. Nested messages, oneof, unknown fields

### Nested (`NestedMessageField<M, A>`)

`Option<Box<M, A>>`. Encode: LEN tag + `child.encode_raw`. Decode: create child if absent, `merge_from` on sub-slice (concatenation = merge). Recursion limit: planned ([§17](#17-planned-optimisations--runtime-gaps)).

### Oneof (`OneofSlot<E>`)

Each wire occurrence replaces the whole slot (last wins). Encode active variant only. Decode: one match arm per variant field number; coupling stays in slot + generated helpers — not spread across singular members.

### Unknown

`_common.unknown_fields` — valid partial wire stream via `skip_field_and_save`; re-emitted on encode. `SGroup` / `EGroup` not preserved.

---

## Part V — Future work

## 17. Planned optimisations & runtime gaps

| Area | Current | Target |
|---|---|---|
| UTF-8 validation | Always `decode_string_in` (VERIFY) | Per-field `utf8_validation` feature |
| Recursion limit | Not enforced | Depth counter in nested merge → `RecursionLimitExceeded` |
| Repeated wrappers | Inline in sample | `RepeatedPackedVarintField`, `RepeatedLenField`, … |
| Fixed32/64 catalog | Trait stubs | `SingularFixed32Field<T, P>`, … |
| `protoc` plugin | — | FieldKind → catalog emission |
| Zero-copy views | — | `TaskView<'buf>` (DESIGN.md §8) |
| `TaskLazy` | DESIGN only | Wire buffer + on-demand decode |
| `Hash` / `serde` | Deferred | Opt-in features |
