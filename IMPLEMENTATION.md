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
    │  shared/ — MessageCommon, FieldPresence, ValueSlot
    │  wire/   — VarintProtoType, LenProtoType
    │  singular/, repeated/, oneof/
    │  T: VarintProtoType / LenProtoType / …
    │  P: FieldPresence (Implicit / Explicit<BIT> / LegacyRequired<BIT>)
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
| Monomorphised hot path | `const FIELD` on wrappers; `BIT` on `Explicit<BIT>` / `LegacyRequired<BIT>` |
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
| `ValueSlot`, `SlotInitView` / `SlotInitMut`, `ProtoZero` | **Done** |
| `SingularVarintField<T, P>`, `SingularLenField<T, P, A>` | **Done** |
| `merge_closed`, `validate_required` | **Done** |
| `NestedMessageField` | **Done** |
| [`sample-generated`](sample-generated/) (`Task` / `Address`) | **Done** |
| `Fixed32ProtoType` / `Fixed64ProtoType` | **Stub** |
| Repeated catalog (`RepeatedVarintField`, `RepeatedLenField`) | **Done** |
| `protoc` plugin | **Planned** |

---

## Part II — Runtime catalog (`puroro::fields`)

### Module layout (`src/fields/`)

| Path | Contents |
|---|---|
| [`fields.rs`](src/fields.rs) | Public re-exports |
| [`shared.rs`](src/fields/shared.rs) | `MessageCommon`, `PresenceBits`, `ProtoZero` |
| [`shared/field_presence.rs`](src/fields/shared/field_presence.rs) | `FieldPresence` markers |
| [`shared/value_slot.rs`](src/fields/shared/value_slot.rs) | `ValueSlot<T>` / `MaybeUninit<T>` storage |
| [`shared/slot_init.rs`](src/fields/shared/slot_init.rs) | `SlotInitView` / `SlotInitMut` init-state handles |
| [`wire.rs`](src/fields/wire.rs) | Wire-family re-exports |
| [`wire/varint.rs`](src/fields/wire/varint.rs) | `VarintProtoType`, `ProtoInt32`, … |
| [`wire/len.rs`](src/fields/wire/len.rs) | `LenProtoType`, `ProtoString`, … |
| [`wire/fixed.rs`](src/fields/wire/fixed.rs) | `Fixed32ProtoType` / `Fixed64ProtoType` (stub) |
| [`singular.rs`](src/fields/singular.rs) | Singular field re-exports |
| [`singular/varint.rs`](src/fields/singular/varint.rs) | `SingularVarintField` |
| [`singular/len.rs`](src/fields/singular/len.rs) | `SingularLenField` |
| [`singular/message.rs`](src/fields/singular/message.rs) | `NestedMessageField` |
| [`repeated.rs`](src/fields/repeated.rs) | Repeated field re-exports |
| [`repeated/encoding.rs`](src/fields/repeated/encoding.rs) | `Packed` / `Expanded` |
| [`repeated/varint.rs`](src/fields/repeated/varint.rs) | `RepeatedVarintField` |
| [`repeated/len.rs`](src/fields/repeated/len.rs) | `RepeatedLenField` |
| [`oneof.rs`](src/fields/oneof.rs) | `OneofSlot` |

---

## 4. Shared infrastructure

[`MessageCommon<P, A>`](src/fields/shared.rs) — one per generated message:

| Member | Role |
|---|---|
| `presence: P` | Bitfield newtype (`TaskPresence`, …) for EXPLICIT / LEGACY_REQUIRED singular fields |
| `unknown_fields: ManuallyDrop<UnmanagedVec<u8>>` | Round-trip unknown wire; closed-enum unknown variants. Allocator-less; freed by `MessageCommon::deallocate` |
| `alloc: A` | The single canonical allocator copy; borrowed (`&A`) by every field operation that (de)allocates |

Field catalog methods take `&MessageCommon` / `&mut MessageCommon`, not `&Task`, so wrappers stay decoupled from the parent message type.

[`PresenceBits`](src/fields/shared.rs) — trait implemented on the message-specific presence **newtype** (not on raw `BitArray` — orphan rules). [`MessageCommon::is_present`](src/fields/shared.rs) / `set_presence` forward to it.

[`ValueSlot<T>`](src/fields/shared/value_slot.rs) — singular varint storage behind a GAT on [`FieldPresence`](src/fields/shared/field_presence.rs): always-initialized `T` for `Implicit` / `Oneof`; `MaybeUninit<T>` for `Explicit` / `LegacyRequired`. Mutation passes a [`SlotInitMut`](src/fields/shared/slot_init.rs) handle (`slot_init_mut(common)`); reads pass [`SlotInitView`](src/fields/shared/slot_init.rs) (`slot_init_view(common)`). [`ProtoZero`](src/fields/shared.rs) supplies protobuf type-zero for lazy init (`ValueSlot::as_mut` writes `T::proto_zero()` when the slot is still uninitialized).

---

## 5. Wire encoding traits

One marker + trait per protobuf **wire family**. Semantic conversions delegate to **`protobuf-core`** (puroro does not reimplement zigzag/varint).

### Varint ([`wire/varint.rs`](src/fields/wire/varint.rs))

```rust
pub trait VarintProtoType {
    type Value: Copy + PartialEq;
    fn proto_zero() -> Self::Value;
    fn decode_wire(raw: u64) -> Result<Self::Value, DecodeError>;
    fn encode_wire(value: Self::Value) -> u64;
}
// Markers: ProtoInt32, ProtoInt64, ProtoUInt32, ProtoUInt64,
//          ProtoSint32, ProtoSint64, ProtoBool, ProtoEnum<E>
```

### Other families

| Trait | Wire | Markers | Status |
|---|---|---|---|
| `LenProtoType` | LEN | `ProtoString`, `ProtoBytes` ([`wire/len.rs`](src/fields/wire/len.rs)) | **Done** |
| `Fixed32ProtoType` | I32 | `ProtoFixed32`, `ProtoFloat`, … ([`wire/fixed.rs`](src/fields/wire/fixed.rs)) | Stub |
| `Fixed64ProtoType` | I64 | `ProtoFixed64`, `ProtoDouble`, … ([`wire/fixed.rs`](src/fields/wire/fixed.rs)) | Stub |

Rust storage type alone does **not** identify protobuf encoding (`i32` can be int32, sint32, or enum). The marker type is the source of truth.

---

## 6. Presence policy

[`FieldPresence`](src/fields/shared/field_presence.rs) — ZST markers composed into singular wrappers as type param `P`:

| Method / GAT | Role |
|---|---|
| `ValueSlot<T>` | `T` for `Implicit` / `Oneof`; `MaybeUninit<T>` for `Explicit` / `LegacyRequired` |
| `slot_init_mut` / `slot_init_view` | Init-state handles for [`ValueSlot`](src/fields/shared/value_slot.rs) |
| `should_emit(common, is_payload_empty)` | Encode omit rule |
| `is_set(common, is_payload_empty)` | `has_*` / `Optional` |
| `payload_is_empty(slot)` | Type-zero check (`Implicit` only) |

| Marker | Encode | Slot init | Bitfield | Accessors |
|---|---|---|---|---|
| `Implicit` | Omit when payload empty / type-zero | Always initialized | No-op | `value()` |
| `Explicit<BIT>` | Omit when bit unset | Lazy via `ValueSlot::as_mut` | Set on `set` / `merge` / `value_mut` | `optional`, `has`, `clear` |
| `LegacyRequired<BIT>` | Same as `Explicit` | Same as `Explicit` | Same as `Explicit` | Same + `validate_required` |

[`ExplicitFieldPresence`](src/fields/shared/field_presence.rs) gates `optional` / `has` / `clear` (implemented for `Explicit` and `LegacyRequired`). [`RequiredFieldPresence`](src/fields/shared/field_presence.rs) adds `validate_present` for `LegacyRequired`.

`BIT` is a **const generic on the marker type** (`Explicit<3>`, `LegacyRequired<1>`, …), not a runtime parameter to `bind`.

---

## 7. Field wrappers

One generic struct per wire family, parametrised by wire marker `T` and presence `P`. `FIELD: u32` is a **struct** const generic; `BIT` lives on `Explicit<BIT>` / `LegacyRequired<BIT>` in the field member type.

| Wrapper | Module | Params | Aliases (ergonomics) |
|---|---|---|---|
| `SingularVarintField<T, P>` | [`singular/varint.rs`](src/fields/singular/varint.rs) | `T: VarintProtoType`, `P: FieldPresence` | `ImplicitVarintField<T>`, `ExplicitVarintField<T>` |
| `SingularLenField<T, P, A>` | [`singular/len.rs`](src/fields/singular/len.rs) | `T: LenProtoType`, `P`, `A: Allocator` | `ImplicitString<A>`, `ExplicitString<A>`, … |
| `SingularFixed32Field<T, P>` | (planned, [`wire/fixed.rs`](src/fields/wire/fixed.rs)) | — | — |
| `NestedMessageField<M, A, P = Optional>` | [`singular/message.rs`](src/fields/singular/message.rs) | `Optional`: `Option<Box<M>>`; `Present`: bare `Box<M>` — no bitfield | — |
| `OneofSlot<E>` | [`oneof.rs`](src/fields/oneof.rs) | mutually exclusive variants | — |

**Closed enum:** `SingularVarintField<ProtoEnum<E>, Explicit>::merge_closed(…, |wire: i32| …)` — unknown values → `unknown_fields`, bit not set.

**LEGACY_REQUIRED (LEN):** `SingularLenField<…, LegacyRequired, A>::validate_required`.

Adding a wire type = one new `VarintProtoType` impl. Adding a presence mode = one new `FieldPresence` impl.

---

## 8. Proto field → catalog mapping

| Proto field | Generated member type |
|---|---|
| `IMPLICIT int32` | `SingularVarintField<ProtoInt32, Implicit>` |
| `EXPLICIT int32` | `SingularVarintField<ProtoInt32, Explicit>` |
| `IMPLICIT sint32` / `bool` | `SingularVarintField<ProtoSint32 \| ProtoBool, Implicit>` |
| `IMPLICIT open enum` | `SingularVarintField<ProtoEnum<E>, Implicit>` |
| `EXPLICIT closed enum` | `SingularVarintField<ProtoEnum<E>, Explicit>` + `merge_closed` |
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
    title: SingularLenField<ProtoString, Explicit<{ BIT_TITLE }>, { FIELD_TITLE }, A>,
    score: SingularVarintField<ProtoInt32, Implicit, { FIELD_SCORE }>,
    max_retries: SingularVarintField<ProtoInt32, Explicit<{ BIT_MAX_RETRIES }>, { FIELD_MAX_RETRIES }>,
    owner_id: SingularLenField<ProtoString, LegacyRequired<{ BIT_OWNER_ID }>, { FIELD_OWNER_ID }, A>,
    payload: SingularLenField<ProtoBytes, Explicit<{ BIT_PAYLOAD }>, { FIELD_PAYLOAD }, A>,
    tag_ids: RepeatedPackedVarintField<ProtoInt32, A>,
    scores: RepeatedExpandedVarintField<ProtoInt32, A>,
    labels: RepeatedLenField<ProtoString, A>,
    status: SingularVarintField<ProtoEnum<Status>, Implicit, { FIELD_STATUS }>,
    priority: SingularVarintField<ProtoEnum<Priority>, Explicit<{ BIT_PRIORITY }>, { FIELD_PRIORITY }>,
    assignee: NestedMessageField<Address<A>, A>,
    notification: OneofSlot<task::Notification>,
}
```

The `A: Allocator + Clone` struct bound is what lets the generated `Drop` clone the allocator into nested children and free every field from one place. Field wrappers keep their payloads in allocator-less `unmanaged` containers behind `ManuallyDrop`, so `A` appears inline only once (in `_common.alloc`); the per-field `A` in the wrapper types is a `PhantomData` marker, not a stored allocator.

### Storage summary

| Field kind | Inside catalog wrapper | Presence |
|---|---|---|
| IMPLICIT varint / open enum | `T` (always initialized) | — |
| EXPLICIT varint / enum | `MaybeUninit<T>` | bit in `_common.presence` |
| IMPLICIT / EXPLICIT string / bytes | `ManuallyDrop<UnmanagedString>` / `UnmanagedVec<u8>` | bit for EXPLICIT / LEGACY_REQUIRED |
| LEGACY_REQUIRED | same as EXPLICIT + `LegacyRequired` | bit |
| Repeated | `RepeatedVarintField` / `RepeatedLenField` | empty = absent |
| Nested message | `Option<UnmanagedBox<M>>` | `Option`, not bitfield |
| Oneof | `Option<E>` in slot | `Option`, not bitfield |

Unset EXPLICIT slots may hold type-zero / empty heap data; **only the bit** means "set". `UnmanagedString` for strings; `UnmanagedVec<u8>` for bytes (each wrapped in `ManuallyDrop`).

Public accessors are **one-line delegates** into catalog methods with `&self._common` / `&mut self._common`. `MessageEncode` and `MessageDecode` sum the same delegates; the generated `Drop` walks the same fields calling `deallocate(self._common.alloc.clone())`.

### Codegen emission per message

1. Presence **newtype** + `PresenceBits` impl
2. Struct — `MessageCommon` + catalog members + `OneofSlot` per oneof
3. Associated constants on the message type — `impl Foo<A> { pub const FIELD_* …; pub const BIT_* …; }`. `BIT_*` is baked into each `Explicit<BIT>` / `LegacyRequired<BIT>` field type; `FIELD` is a struct const generic on the wrapper.
4. Accessor delegates ([DESIGN.md §4](DESIGN.md#40-generated-per-message-traits))
5. Trait impls — encode/decode/clone/eq as field sums
6. Child modules — enums, oneof enums

IR step: `ProtoField → FieldKind → catalog type + const args`.

### Path qualification (naming)

**Real generated code must fully-qualify every path it emits** — leading-`::` absolute paths such as `::puroro::SingularLenField`, `::core::ops::DerefMut`, `::allocator_api2::alloc::Allocator` — and must not depend on `use` imports for the items it references. A `.proto` file can name its packages, messages, and fields with almost any identifier, so any *unqualified* name in the generated output risks colliding with a user-defined type, module, or import that lands in the same scope. Fully-qualified paths are collision-proof. The only names exempt from this are the ones the generator introduces itself and reserves by convention — e.g. the `_common` field and other `_`-prefixed internals — which cannot clash with proto-derived names.

**The checked-in [`sample-generated/`](sample-generated/) deliberately breaks this rule for readability.** It pulls names in with `use` and refers to them by short name (`SingularLenField`, `Allocator`, `MessageCommon`, …) so the reference output stays easy to read and review. Read those short names as stand-ins for the fully-qualified paths the production protoc plugin would actually emit.

### Generated code comments

Generated Rust is not meant to be hand-edited, but **must be easy to navigate when debugging** (breakpoints, `merge_from` dispatch, diffing encode output). The protoc plugin emits comments from proto metadata; [`sample-generated/`](sample-generated/) demonstrates the convention.

**File header** — every generated module carries a machine marker and the source message:

```rust
//! @generated from example.proto — do not edit
//! Message `example.Task`
```

The `@generated` marker belongs on **real** plugin output (tooling uses it to collapse/skip generated files). The checked-in [`sample-generated/`](sample-generated/) intentionally **omits** it — those files are a hand-maintained reference, and an `@generated`/`do not edit` banner there would wrongly imply they are tool-generated. Sample headers instead describe what the module illustrates in plain prose.

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
title: SingularLenField<ProtoString, Explicit<{ BIT_TITLE }>, { FIELD_TITLE }, A>, // proto: string title = 1;
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
    self.title
        .bind(&mut self._common)
        .merge(wire_type, buf)?;
}
```

Every field kind merges through the same bound-view shape — `self.<field>.bind(&mut self._common).merge(wire_type, buf)?` (repeated and nested-message fields likewise take only `common`; oneof uses `OneofSlotMut`) — so the code generator emits one form. Oneof variant arms use the **variant field name** and number. The `_ =>` unknown-field arm gets a short comment (`// unknown field — preserve in _common`).

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
    self.title.encode_raw(c, buf);
    self.score.encode_raw(c, buf);
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
Self::FIELD_PRIORITY => self
    .priority
    .bind(&mut self._common)
    .merge_closed(wire_type, buf, |v| Priority::try_from(v).is_ok())?,
```

Nested LEN payloads use `Buf::take(len)` before child `merge_from`.

### Validation

`validate()` — `owner_id.validate_required(&self._common)?` (and any other `LegacyRequired` fields). `decode_strict` = decode + validate. `MessageDecode::decode` does **not** auto-validate.

---

## 13. Derived traits

**Messages** (`Task<A>`, …): generated as below. **Scalar enums** (`Status`, `Priority`): `#[repr(transparent)]` newtypes over `i32` with associated constants (not Rust enums — proto value aliases may share an integer); `derive(Clone, Copy, Debug, PartialEq, Eq, Hash)`. **Oneof types**: the payload-less `NotificationCase` `derive`s `Clone, Copy, Debug, PartialEq, Eq`. `NotificationRef<'a, A>` is `Copy` (all payloads are `Copy`), but its `Copy`/`Clone` are hand-written to drop the spurious `A: Copy` bound the derive would add, and it omits `Debug`/`PartialEq`/`Eq` once a variant borrows a message (`&Address<A>`, which derives neither); a string/scalar-only group could keep the full derives. `NotificationMut<'a, A>` holds guards / `&mut` and derives nothing; the internal `NotificationStorage` needs no trait derives (comparison/formatting happen on the safe views).

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

**Mutation goes through a bound view**, exactly like the LEN family: callers first `field.bind(&mut common)` to get a short-lived [`SingularVarintFieldMut`](src/fields/singular/varint.rs) view carrying `(field, common)`, then call one consuming method. Scalars store no allocator inline (the value is `Copy`), so the view exists purely to fold presence — and, for closed enums, `unknown_fields` capture — into one call. Two lifetimes keep the `&mut T::Value` returned by `value_mut` tied to the field slot (`'f`) only; the `common` borrow (`'c`) is released as the method returns.

| | IMPLICIT | EXPLICIT |
|---|---|---|
| Encode | Omit at type-zero | Omit when bit unset; emit zero if bit set |
| Merge | `bind(&mut common).merge(wire, buf)` — `ValueSlot::set` | same (sets bit via `SlotInitMut`) |
| Getter | `value()` | `optional(&common)` |
| Mutator | `bind(&mut common).value_mut()` → `&mut T::Value` | same (sets bit) |
| Clear | `bind(&mut common).clear()` (resets type-zero) | same |

Getters and encode (`value` / `optional` / `has` / `encoded_len` / `encode_raw`) need only a shared `&common` and stay as plain field methods. Enum fields use `ProtoEnum<E>` where `E: ProtoEnumStorage`; open vs closed decode rules live in the generated `decode_from_wire` impl. Closed enum unknown values use `merge_closed(…, |wire: i32| …)` on decode. `Optional` is a concrete struct (DESIGN.md §3); no `Option<T>` conversion.

### LEN — string & bytes (`SingularLenField<T, P, A>`)

The payload is `ManuallyDrop<T::Storage>` (`UnmanagedString` / `UnmanagedVec<u8>`).

**Mutation goes through a bound view.** Rather than threading `common` through every method, callers first `field.bind(&mut common)` to get a short-lived [`SingularLenFieldMut`](src/fields/singular/len.rs) view that carries the whole mutation context, then call one of its consuming methods. This keeps the field struct a pure storage holder and collapses each generated accessor to a single call (no separate `set_presence`). The view uses **two lifetimes** so the guard returned by `value_mut` borrows only the field storage (`'f`) — the `common` borrow (`'c`) is released as the method returns.

| | IMPLICIT | EXPLICIT / LEGACY_REQUIRED |
|---|---|---|
| Encode | Omit when empty | Omit when bit unset |
| Merge | `bind(&mut common).merge(wire, buf)` — decode new, free old, store | same |
| Getter | `value()` | `optional(&common)` |
| Mutator | `bind(&mut common).value_mut()` → guard (`impl DerefMut`), marks presence | same |
| Clear | `bind(&mut common).clear()` (frees old, resets empty) | same |
| Release | `deallocate(&mut self, A)` (owned clone; called from message `Drop`) | same |

Getters and encode (`value` / `optional` / `has` / `encoded_len` / `encode_raw`) stay on the field itself: they only need a shared `&common`, so they are already clean one-line delegates and are left outside the view. Every operation that actually (de)allocates takes the allocator **by value** — an `alloc.clone()`, obtained inside the view from `common.alloc`. The allocator is thus consistently type `A` for both the growing allocation and the eventual free (never `&A`); interchangeability of clones is guaranteed by the `Allocator + Clone` contract.

### LEGACY_REQUIRED

Wire identical to EXPLICIT. Message `validate()` calls `validate_required` on each `LegacyRequired` field ([§12](#12-message-level-wire-io)).

---

## 15. Repeated fields

**Catalog:** [`RepeatedVarintField<T, E, A>`](src/fields/repeated/varint.rs) and [`RepeatedLenField<T, A>`](src/fields/repeated/len.rs). Empty vec = absent on encode.

### Varint (`RepeatedVarintField<T, E, A>`)

`E` is [`Packed`](src/fields/repeated/encoding.rs) or [`Expanded`](src/fields/repeated/encoding.rs) — affects **encode only**. [`merge`](src/fields/repeated/varint.rs) always accepts both LEN (packed blob) and VARINT (single element).

| | Packed (`E = Packed`) | Expanded (`E = Expanded`) |
|---|---|---|
| Encode | One LEN record | One VARINT per element |
| Decode | Both forms | Both forms |

Elements live in `ManuallyDrop<UnmanagedVec<T::Value>>`. Mutation uses the same bound-view idiom as the LEN family: `field.bind(&mut common)` yields a [`RepeatedVarintFieldMut`](src/fields/repeated/varint.rs) (no presence bit — repeated fields have none), whose consuming methods are `values_mut()` → guard (`impl DerefMut<Target = Vec<_, A>>`), `merge(wire, buf)`, and `clear()`; each obtains its own owned `alloc.clone()` from `common`. Read-only paths (`as_slice` / `is_empty` / `encoded_len` / `encode_raw`) stay on the field. `deallocate(A)` also stays on the field (called once from message `Drop`).

### LEN (`RepeatedLenField<T, A>`)

One LEN record per element (`repeated string` / `repeated bytes`), stored as `ManuallyDrop<UnmanagedVec<T::Storage>>`. Mutation uses the same bound-view idiom: `field.bind(&mut common)` yields a [`RepeatedLenFieldMut`](src/fields/repeated/len.rs) (no presence bit — repeated fields have none), whose consuming methods are `push_in(impl AsRef<[u8]>)`, `merge(wire, buf)`, and `clear()`. The typed `push_in` helper is kept instead of a bare `DerefMut` (which would expose allocator-less element storage that is impractical to construct). Because each element is itself allocator-less, `clear`/`deallocate` **drain and free every element first** (each via its own owned `alloc.clone()`), then free the buffer. `deallocate(A)` stays on the field (called from `Drop`).

> Note: the bound-view idiom (`field.bind(&mut common).op()`) now covers every mutable field family — `SingularLenField`, `RepeatedLenField`, `SingularVarintField`, and `RepeatedVarintField`. Only the terminal `deallocate(A)` stays a direct field method (called from `Drop`).

---

## 16. Nested messages, oneof, unknown fields

### Nested (`NestedMessageField<M, A>`)

Storage is chosen by a `MessagePresence` marker (a GAT): `Optional` (default) = `Option<UnmanagedBox<M>>` for ordinary fields; `Present` = a bare `UnmanagedBox<M>` for oneof variants (the slot tracks presence, so the box is always there). See [`singular/message.rs`](src/fields/singular/message.rs). Mutation uses the **same bound-view idiom** as the other families (`field.bind(&mut common).merge(wire, buf)` / `.get_mut()` / `.clear()`; oneof variants use `bind_oneof`), so a nested message reads and merges exactly like a varint or repeated field in generated code — the code generator emits one shape for every field kind. A nested message has no presence bit, so — like a repeated field — `bind` takes only `common` and the view carries no bit index. `Optional` exposes `get()` / `get_mut()` (insert-if-absent); `Present` exposes varint-like `value()` / `value_mut()` (`&M` / `&mut M`, no `unwrap`). Encode: LEN tag + `child.encode_raw`. Decode: `Optional` creates the child (via `M::new_in(common.alloc.clone())`) if absent then `merge_from`s the sub-slice (concatenation = merge); `Present` merges into the always-present child. Release: `Optional::deallocate` (terminal, from the owning `Drop`, `&mut self`) and the view's `clear` free the box; `Present::deallocate` consumes the field **by value** (no `Option` to null out), called via `OneofDeallocate`. The child's own `Drop` frees its fields recursively. Recursion limit: planned ([§17](#17-planned-optimisations--runtime-gaps)).

### Oneof (`OneofSlot<E>`)

Each wire occurrence replaces the whole slot (last wins). Encode active variant only. Decode: one match arm per variant field number.

**Four generated types per group.** The owned storage holds allocator-less `unmanaged` values, so it is kept out of the public API and split from the safe views:

The sample `oneof notification` is deliberately **heterogeneous** — LEN, VARINT, and message variants — to show all three storage kinds:

| Type | Vis | Payloads | Role |
|---|---|---|---|
| `NotificationStorage<A>` | `pub(crate)` | `SingularLenField` / `SingularVarintField` / `NestedMessageField` | owned storage (field wrappers); `OneofDeallocate<A>`; owns encode glue |
| `NotificationCase` | `pub` | — | `Copy` discriminant (variants only; unset is `None`) → `notification_case() -> Option<_>` |
| `NotificationRef<'a, A>` | `pub` | `&'a str` / `i32` / `&'a Address<A>` | borrowed read view → `notification()` |
| `NotificationMut<'a, A>` | `pub` | `StringGuard<'a, A>` / `&'a mut i32` / `&'a mut Address<A>` | borrowed mutable view → `notification_mut()` |

**Variants own field wrappers, not raw storage.** Each variant holds the same field wrapper an ordinary singular field of that kind uses (`SingularLenField` for `string`/`bytes`, `SingularVarintField` for scalars, `NestedMessageField` for messages), so `value` / `value_mut` / `deallocate` are reused rather than reimplemented. The wrapper's presence is inert here (the `OneofSlot` tracks presence and the storage enum frames encode/merge itself), so a presence-agnostic policy is picked (`Implicit` for LEN/VARINT; `Present` storage for the message wrapper — see below) and the wrapper's presence-aware methods are never called. Because the LEN/message wrappers pin their allocator type, the storage enum is generic over `A` (the scalar wrapper is allocator-free).

The oneof drives each variant with the **field's own** primitives — no bespoke helpers on the enum. Build-empty (for `variant_mut`'s `make`) is `new_in(alloc)` / `new()` / `with_message_in(alloc)`. Merging one wire occurrence is uniformly `field.bind_oneof(common).merge(wire, buf)` for **every** kind — `bind_oneof` is the presence-free bind for oneof variants (a oneof has no bit). For LEN/VARINT it binds against a dummy index (`Implicit` presence ignores it) and yields the same `…FieldMut` view as `bind`; for the message wrapper it is the bind of its `Present` storage. Read/read-mut are `value()`/`value_mut(…)` for **all** kinds — including the message variant, thanks to `Present` (below). The VARINT variant owns no heap, so its `OneofDeallocate` arm is a no-op and its constructor ignores the allocator.

**The message variant uses `NestedMessageField<M, A, Present>` — a bare box, not `Option`.** `NestedMessageField` carries a `MessagePresence` marker (a GAT choosing the stored container): the default `Optional` stores `Option<UnmanagedBox<M>>` (ordinary fields, inline presence), while `Present` stores a bare `UnmanagedBox<M>` that is always there (oneof variants, presence in the slot). `Present` gives the message variant scalar-like accessors — `value() -> &M` / `value_mut() -> &mut M`, no `Option`, no `unwrap()` — so `to_ref`/`to_mut`/`postal_mut` read uniformly with the other kinds. Since there is no `Option` to null out, `Present::deallocate` consumes the field **by value**, which is exactly how `OneofDeallocate` frees a variant.

`NotificationRef` is `Copy` (all payloads are `Copy`, including the `&Address<A>` reference), but its `Copy`/`Clone` are hand-written to avoid a spurious `A: Copy` bound from the derive, and it omits `Debug`/`PartialEq`/`Eq` because the message payload `Address<A>` implements neither.

The storage enum is deliberately **not** named `Notification`: exposing an `unmanaged`-holding value by the canonical name would let a caller own one and hit the panic-on-implicit-drop footgun, and would leak the `unmanaged` type into the API. `NotificationStorage::{case, to_ref, to_mut}` map storage → the safe views. (A single generic enum parametrised over a payload "mode" was rejected: the case enum is payload-less, ref/mut need GAT-style lifetime/allocator threading, and per-mode impls diverge — concrete enums emit and read better.)

**Per-variant `bind_<variant>_mut` helpers on the storage enum** hide the `variant_mut(is_match, make)` + `let Self::Variant(f) = … else unreachable!()` boilerplate: `bind_email_address_mut(slot, common) -> &mut SingularLenField<…>`, `bind_webhook_id_mut(…) -> &mut SingularVarintField<…>`, `bind_postal_mut(…) -> &mut NestedMessageField<…, Present>`, etc. Each forces the group to its variant (installing a default, freeing any other) and returns the inner field wrapper, borrowing only the slot so `common` is free on return. The single `unreachable!()` per variant lives here instead of at every call site. The parent's `_mut` accessors are then one-liners: `bind_email_address_mut(…).value_mut(alloc)`, `bind_webhook_id_mut(…).value_mut()`, `bind_postal_mut(…).value_mut()` — all `value_mut`, no `unwrap`.

**The encode glue lives on the storage enum, not on the parent message** — `encoded_len(&slot)` and `encode(&slot, buf)` as associated functions — keeping the group decoupled from any specific message. **Merge, however, is not a `merge_<variant>` helper on the enum.** Because each variant *is* a field wrapper, the parent's `merge_from` selects the variant with `NotificationStorage::bind_<variant>_mut(&mut slot, &mut common)` (freeing any other variant) and merges the occurrence into the returned field via `field.bind_oneof(common).merge(…)` — uniformly for every kind. This reuses each field's merge machinery rather than re-deriving it on the enum, and lets the message variant merge successive occurrences into the current child rather than replacing it. The variant field-number constants sit at **module scope** (`notification::FIELD_EMAIL_ADDRESS`, …) rather than as associated `const`s, so they stay usable as `match` patterns in the parent's `merge_from` even though the storage enum is generic over `A`.

The storage enum implements [`OneofDeallocate<A>`](src/fields/oneof.rs) (`unsafe fn deallocate(self, alloc: A)` — `A` is a **trait** parameter, since the field wrappers pin the allocator type) so the previously-active variant is freed explicitly through the message allocator before the slot is overwritten. Mutation uses the same bound-view idiom as the other families: `slot.bind(&mut common)` yields an [`OneofSlotMut`](src/fields/oneof.rs) whose consuming methods are:

- `variant_mut(is_match, make) -> &mut E` — keeps the active variant if `is_match`, else frees it and installs `make(alloc.clone())`. Backs the per-variant `bind_<variant>_mut` helpers, which pattern-match out the inner field wrapper (the lone `unreachable!()`) and hand it back; the parent's `_mut` accessors and decode arms then apply the kind-appropriate step (`value_mut(alloc)` for LEN, `value_mut()` for VARINT, `get_present_mut().unwrap()` for message) or merge into it (`bind_oneof(common).merge(…)` / `merge(common, …)`).
- `set(value)` — replaces the whole group with an already-built value (frees the old variant).
- `clear()` — frees the active variant; backs `clear_*` and the message `Drop`.

The inherent `OneofSlot::{set, take, clear, get_mut}` remain as low-level primitives used by the view; they do not free on their own. The `set_*` per-variant setters are removed, matching the `set_*`-abolition across the other families. (Oneof is a tentative implementation prioritising correctness of compile/round-trip.)

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
