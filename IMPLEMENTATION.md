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
| Repeated catalog (`RepeatedVarintField`, `RepeatedLenField`) | **Done** |
| `protoc` plugin | **Planned** |

---

## Part II — Runtime catalog (`puroro::fields`)

## 4. Shared infrastructure

[`MessageCommon<P, A>`](src/fields/common.rs) — one per generated message:

| Member | Role |
|---|---|
| `presence: P` | Bitfield newtype (`TaskPresence`, …) for EXPLICIT / LEGACY_REQUIRED singular fields |
| `unknown_fields: ManuallyDrop<UnmanagedVec<u8>>` | Round-trip unknown wire; closed-enum unknown variants. Allocator-less; freed by `MessageCommon::deallocate` |
| `alloc: A` | The single canonical allocator copy; borrowed (`&A`) by every field operation that (de)allocates |

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
| `Implicit` | Omit when payload empty / type-zero | No-op | `value()` |
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
| `repeated int32 PACKED` | `RepeatedPackedVarintField<ProtoInt32, A>` |
| `repeated int32 EXPANDED` | `RepeatedExpandedVarintField<ProtoInt32, A>` |
| `repeated string` | `RepeatedLenField<ProtoString, A>` |
| `repeated bytes` | `RepeatedLenField<ProtoBytes, A>` |
| nested message | `NestedMessageField<M, A>` |
| `oneof` | `OneofSlot<E>` — not a singular catalog entry |

---

## Part III — Generated message

## 9. Struct layout

```rust
pub struct Task<A: Allocator + Clone = Global> {
    _common: MessageCommon<TaskPresence, A>,
    title: SingularLenField<ProtoString, Explicit, A>,
    score: SingularVarintField<ProtoInt32, Implicit>,
    max_retries: SingularVarintField<ProtoInt32, Explicit>,
    owner_id: SingularLenField<ProtoString, LegacyRequired, A>,
    payload: SingularLenField<ProtoBytes, Explicit, A>,
    tag_ids: RepeatedPackedVarintField<ProtoInt32, A>,
    scores: RepeatedExpandedVarintField<ProtoInt32, A>,
    labels: RepeatedLenField<ProtoString, A>,
    status: SingularVarintField<ProtoEnum, Implicit>,
    priority: SingularVarintField<ProtoEnum, Explicit>,
    assignee: NestedMessageField<Address<A>, A>,
    notification: OneofSlot<task::Notification>,
}
```

The `A: Allocator + Clone` struct bound is what lets the generated `Drop` clone the allocator into nested children and free every field from one place. Field wrappers keep their payloads in allocator-less `unmanaged` containers behind `ManuallyDrop`, so `A` appears inline only once (in `_common.alloc`); the per-field `A` in the wrapper types is a `PhantomData` marker, not a stored allocator.

### Storage summary

| Field kind | Inside catalog wrapper | Presence |
|---|---|---|
| IMPLICIT scalar / open enum | `T` or `i32` | — |
| EXPLICIT scalar / enum / string / bytes | same containers | bit in `_common.presence` |
| LEGACY_REQUIRED | same as EXPLICIT + `LegacyRequired` | bit |
| Repeated | `RepeatedVarintField` / `RepeatedLenField` | empty = absent |
| Nested message | `Option<UnmanagedBox<M>>` | `Option`, not bitfield |
| Oneof | `Option<E>` in slot | `Option`, not bitfield |

Unset EXPLICIT slots may hold type-zero / empty heap data; **only the bit** means "set". `UnmanagedString` for strings; `UnmanagedVec<u8>` for bytes (each wrapped in `ManuallyDrop`).

Public accessors are **one-line delegates** into catalog methods with `&self._common` / `&mut self._common`. `MessageEncode` and `MessageDecode` sum the same delegates; the generated `Drop` walks the same fields calling `deallocate(&self._common.alloc)`.

### Codegen emission per message

1. Presence **newtype** + `PresenceBits` impl
2. Struct — `MessageCommon` + catalog members + `OneofSlot` per oneof
3. Associated constants on the message type — `impl Foo<A> { pub const FIELD_* …; pub const BIT_* …; }`. Catalog methods take `field: u32` / `bit: usize` as normal parameters.
4. Accessor delegates ([DESIGN.md §4](DESIGN.md#40-generated-per-message-traits))
5. Trait impls — encode/decode/clone/eq as field sums
6. Child modules — enums, oneof enums

IR step: `ProtoField → FieldKind → catalog type + const args`.

### Generated code comments

Generated Rust is not meant to be hand-edited, but **must be easy to navigate when debugging** (breakpoints, `merge_from` dispatch, diffing encode output). The protoc plugin emits comments from proto metadata; [`sample-generated/`](sample-generated/) demonstrates the convention.

**File header** — every generated module:

```rust
//! @generated from example.proto — do not edit
//! Message `example.Task`
```

**Section banners** — major blocks inside the file:

```text
// ---------------------------------------------------------------------------
// Presence bitfield (N tracked singular fields)
// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------
// Field constants (associated with `Task`)
// ---------------------------------------------------------------------------

impl<A: Allocator> Task<A> {
    pub const FIELD_TITLE: u32 = 1;   // title
    pub const BIT_TITLE: usize = 0;   // title (EXPLICIT)
}
// ---------------------------------------------------------------------------
// Message struct
// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------
// MessageEncode / MessageDecode
// ---------------------------------------------------------------------------
```

**Per-field accessor block** — before each field’s `impl` methods:

```text
// -- title (EXPLICIT string, proto field 1) --
```

Include **presence**, **wire/kind** (string, int32, repeated packed, nested, …), and **proto field number**.

**Struct members** — trailing comment tying storage to proto:

```rust
title: SingularLenField<ProtoString, Explicit, A>, // proto: string title = 1;
```

**Constants** — associated constants on `impl Task<A>`. Use `Self::FIELD_*` / `Self::BIT_*` inside message impls; `Task::<A>::FIELD_*` in free helpers outside the type.

```rust
impl<A: Allocator> Task<A> {
    pub const FIELD_TITLE: u32 = 1;   // title
    pub const BIT_TITLE: usize = 0;   // title (EXPLICIT)
}
```

**Wire I/O** — `merge_from` match arms label the proto field:

```rust
Self::FIELD_TITLE => { // title = 1, EXPLICIT string
    self.title.merge(&mut self._common, Self::BIT_TITLE, wire_type, buf)?;
}
```

Oneof variant arms use the **variant field name** and number. The `_ =>` unknown-field arm gets a short comment (`// unknown field — preserve in _common`).

**What not to comment** — avoid restating obvious one-line delegates (`has_title` → `self.title.has(...)`). Section + struct + dispatch comments are enough.

**Proto doc comments** — when the `.proto` field has `///` documentation, emit a Rust `///` doc comment on the **public accessor methods** (not on private struct fields unless the proto doc is part of the public API story).

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

- **`Task::new_in(alloc)`** — default every field; `_common.presence = TaskPresence::ZERO`; heap fields via `*_in(alloc.clone())`, with the last heap field taking the original by move. (Building an empty `unmanaged` container does not allocate, so the clone is only used to decompose an empty `Vec`.)
- **`Task::new()`** — when `A = Global`.
- **`Default`** — `A: Clone + Default` → `new_in(A::default())`.

Runtime **`str_to_unmanaged_in(s, alloc)`** — copy bytes into an `UnmanagedString`; **`bytes_to_unmanaged_in(v, alloc)`** for `UnmanagedVec<u8>`. The `unsafe` (raw-parts / `deallocate`) is confined to the `puroro` runtime and the generated `Drop`, not to generated accessors.

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
    self.title.encode_raw(c, Self::FIELD_TITLE, Self::BIT_TITLE, buf);
    self.score.encode_raw(c, Self::FIELD_SCORE, Self::BIT_UNUSED, buf);
    // …
    let unknown: &[u8] = &c.unknown_fields;
    buf.put_slice(unknown);
}
```

### Decode

1. Loop: `decode_tag` → `(field_number, wire_type)`.
2. `match field_number` — one catalog `merge` (or `merge_closed`) per arm.
3. Unknown → `skip_field_and_save` into `_common.unknown_fields`.
4. Singular: last wins. Repeated: append. Nested: merge sub-buffer.

```rust
Self::FIELD_PRIORITY => self.priority.merge_closed(
    &mut self._common,
    Self::FIELD_PRIORITY,
    Self::BIT_PRIORITY,
    wire_type,
    buf,
    |v| Priority::try_from(v).is_ok(),
)?,
```

Nested LEN payloads use `Buf::take(len)` before child `merge_from`.

### Validation

`validate()` — `owner_id.validate_required(&self._common, Self::BIT_OWNER_ID, Self::FIELD_OWNER_ID)?` (and any other `LegacyRequired` fields). `decode_strict` = decode + validate. `MessageDecode::decode` does **not** auto-validate.

---

## 13. Derived traits

**Messages** (`Task<A>`, …): generated as below. **Scalar enums** (`Status`, `Priority`): `derive(Clone, Copy, Debug, PartialEq, Eq, Hash)`. **Oneof enums** (`Notification`): hand-written `Debug` / `PartialEq` / `Eq` that compare through `Deref` to `str` (the variants hold allocator-less `UnmanagedString`, so `#[derive]` is not possible).

| Trait | Bounds | Notes |
|---|---|---|
| `Default` | `A: Clone + Default` | Clears presence; empty heap fields |
| `Drop` | `A: Clone` | Frees every field through `_common.alloc`, then `_common.deallocate()` |

**Not currently generated:** `Clone`, `PartialEq`, `Eq`, `Debug`, `Copy`, `Ord`, `Hash`. `Clone`/`PartialEq`/`Debug` need `&A` to copy or format the allocator-less fields, so they require a `clone_in(&self, alloc)`-style API (future work) rather than `#[derive]`.

**`Global` extras:** `Task::new()`, `impl Default for Task`, `Task::decode(buf)`.

Compare messages semantically via getters; deep copy (when added) will copy data into a second tree, so prefer `Arc<Task<A>>` for shared immutable messages.

---

## Part IV — Field behaviour

## 14. Singular fields

### Varint (`SingularVarintField<T, P>`)

| | IMPLICIT | EXPLICIT |
|---|---|---|
| Encode | Omit at type-zero | Omit when bit unset; emit zero if bit set |
| Merge | Set value; `on_set` no-op | `on_set` + set value |
| Getter | `value()` | `optional(&common, DefaultZst)` |
| Mutator | `value_mut(&mut common, bit)` → `&mut T::Value` | same (sets bit) |

Open enum: thin glue — `Status::try_from(field.value())`. Closed enum: use `merge_closed` ([§7](#7-field-wrappers)). `Optional` is a concrete struct (DESIGN.md §3); no `Option<T>` conversion. Enum `_mut` accessors expose the raw `i32` storage.

### LEN — string & bytes (`SingularLenField<T, P, A>`)

The payload is `ManuallyDrop<T::Storage>` (`UnmanagedString` / `UnmanagedVec<u8>`).

| | IMPLICIT | EXPLICIT / LEGACY_REQUIRED |
|---|---|---|
| Encode | Omit when empty | Omit when bit unset |
| Merge | decode new, then `deallocate` old, then store | + `on_set` |
| Getter | `value()` | `optional(&common, bit, default)` |
| Mutator | `value_mut(&self.alloc)` → guard (`impl DerefMut`) | same; generated `_mut` sets the bit first |
| Clear | `clear(&mut common, bit)` (frees old, resets empty) | same |
| Release | `deallocate(&mut self, &A)` (called from message `Drop`) | same |

### LEGACY_REQUIRED

Wire identical to EXPLICIT. Message `validate()` calls `validate_required` on each `LegacyRequired` field ([§12](#12-message-level-wire-io)).

---

## 15. Repeated fields

**Catalog:** [`RepeatedVarintField<T, E, A>`](src/fields/repeated_varint.rs) and [`RepeatedLenField<T, A>`](src/fields/repeated_len.rs). Empty vec = absent on encode.

### Varint (`RepeatedVarintField<T, E, A>`)

`E` is [`Packed`](src/fields/repeated_encoding.rs) or [`Expanded`](src/fields/repeated_encoding.rs) — affects **encode only**. [`merge`](src/fields/repeated_varint.rs) always accepts both LEN (packed blob) and VARINT (single element).

| | Packed (`E = Packed`) | Expanded (`E = Expanded`) |
|---|---|---|
| Encode | One LEN record | One VARINT per element |
| Decode | Both forms | Both forms |

Elements live in `ManuallyDrop<UnmanagedVec<T::Value>>`. Accessors: `as_slice()`, `values_mut(&A)` → guard (`impl DerefMut<Target = Vec<_, &A>>`), `clear(&A)`, `merge(&A, …)`, `deallocate(&A)`.

### LEN (`RepeatedLenField<T, A>`)

One LEN record per element (`repeated string` / `repeated bytes`), stored as `ManuallyDrop<UnmanagedVec<T::Storage>>`. Because each element is itself allocator-less, `deallocate`/`clear` **drain and free every element first**, then free the buffer. The typed `push_in(&A, impl AsRef<[u8]>)` helper is kept (the bare `DerefMut` would expose allocator-less element storage that is impractical to construct).

---

## 16. Nested messages, oneof, unknown fields

### Nested (`NestedMessageField<M, A>`)

`Option<UnmanagedBox<M>>`. Encode: LEN tag + `child.encode_raw`. Decode: create child (via `M::new_in(common.alloc.clone())` boxed with `&common.alloc`) if absent, `merge_from` on sub-slice (concatenation = merge). `deallocate`/`clear` take the box and release it; the child's own `Drop` frees its fields recursively. Recursion limit: planned ([§17](#17-planned-optimisations--runtime-gaps)).

### Oneof (`OneofSlot<E>`)

Each wire occurrence replaces the whole slot (last wins). Encode active variant only. Decode: one match arm per variant field number; coupling stays in slot + generated helpers — not spread across singular members. Because variants hold allocator-less `UnmanagedString`, setters/decode `take()` and `deallocate` the previous variant before storing a new one, and the message `Drop` `take()`s and frees the active variant. (Oneof is a tentative implementation prioritising correctness of compile/round-trip.)

### Unknown

`_common.unknown_fields` — valid partial wire stream via `skip_field_and_save`; re-emitted on encode. `SGroup` / `EGroup` not preserved.

---

## Part V — Future work

## 17. Planned optimisations & runtime gaps

| Area | Current | Target |
|---|---|---|
| UTF-8 validation | Always `decode_string_in` (VERIFY) | Per-field `utf8_validation` feature |
| Recursion limit | Not enforced | Depth counter in nested merge → `RecursionLimitExceeded` |
| Repeated wrappers | `RepeatedVarintField`, `RepeatedLenField` |
| Fixed32/64 catalog | Trait stubs | `SingularFixed32Field<T, P>`, … |
| `protoc` plugin | — | FieldKind → catalog emission |
| Zero-copy views | — | `TaskView<'buf>` (DESIGN.md §8) |
| `TaskLazy` | DESIGN only | Wire buffer + on-demand decode |
| `Hash` / `serde` | Deferred | Opt-in features |
