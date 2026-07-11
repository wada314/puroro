# puroro — Implementation Notes

Internal implementation of **generated** protobuf message code: storage, wire I/O, and the `puroro-rt` field catalog. The stable public API is [DESIGN.md](DESIGN.md).

> **Note:** DESIGN.md accessors stay stable even if internals change (e.g. bitfield layout).

**Canonical examples:** `Task` and `Address` from [DESIGN.md §4](DESIGN.md#reference-schema). Working code: [`sample-generated/`](sample-generated/).

---

## Table of contents

**Part I — Overview**

1. [Project context](#1-project-context)
2. [Architecture overview](#2-architecture-overview)
3. [Implementation status](#3-implementation-status)

**Part II — Runtime catalog (`puroro_rt::fields`)**

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
| **`protobuf-core`** | Wire primitives (`Varint`, `Tag`, `WireType`). Used by `puroro` and `puroro-rt`; generated code does not import it. |
| **`puroro`** | Stable user API: `MessageEncode` / `MessageDecode`, `Optional`, `HasDefault`, errors, `WireType`. |
| **`puroro-rt`** | Generated-code runtime: [`fields`](puroro-rt/src/fields.rs), wire `encode` / `decode` helpers, `ProtoDefault`. Depends on `puroro` for shared types. |
| **`protoc` plugin** | Emits Rust types and `impl` blocks described here ([DESIGN.md §0](DESIGN.md#0-project-architecture)). |

---

## 2. Architecture overview

Protobuf fields (except **oneof**) are **independent**: each getter/setter/encode/merge arm touches only its own struct member plus shared [`MessageCommon`](#4-shared-infrastructure). The plugin **composes** runtime catalog types — it does not hand-write per-field logic.

### Layer stack

```
protoc plugin
    │  proto field → catalog type + const FIELD / BIT
    │  emits ::puroro::… (traits, Optional, errors)
    │       ::puroro_rt::… (catalog, wire helpers)
    ▼
puroro_rt::fields       SingularField<T, P, FIELD>, NestedMessageField, …
    │  shared/ — MessageCommon, FieldPresence, ValueSlot,
    │            DefaultIn / DeallocateIn / ProtoEmpty
    │  wire/   — ScalarProtoType (singular); VarintProtoType / LenProtoType
    │            (also used by repeated)
    │  singular/, repeated/, oneof/
    │  T: ScalarProtoType (ProtoInt32, ProtoString, …)
    │  P: FieldPresence (Implicit / Explicit<BIT> / LegacyRequired<BIT> / Oneof)
    ▼
puroro_rt::encode/decode   Buf adapters, LEN framing, unknown-field helpers
    │  (DecodeError, WireType from puroro)
    ▼
puroro                  MessageEncode, MessageDecode, Optional, errors
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
| `ScalarProtoType` + varint / LEN markers | **Done** |
| `VarintProtoType` / `LenProtoType` (repeated + scalar helpers) | **Done** |
| `FieldPresence` (`Implicit` / `Explicit` / `LegacyRequired` / `Oneof`) | **Done** |
| `ValueSlot`, `SlotInitView` / `SlotInitMut`, `DefaultIn` / `DeallocateIn` / `ProtoEmpty` | **Done** |
| `SingularField<T, P, FIELD>` (+ `SingularVarintField` / `SingularLenField` aliases) | **Done** |
| `merge_closed`, `validate_required` | **Done** |
| `NestedMessageField` | **Done** |
| [`sample-generated`](sample-generated/) (`Task` / `Address`) | **Done** |
| `Fixed32ProtoType` / `Fixed64ProtoType` on `SingularField` | **Stub** |
| Repeated catalog (`RepeatedVarintField`, `RepeatedLenField`) | **Done** |
| `protoc` plugin | **Planned** |

---

## Part II — Runtime catalog (`puroro_rt::fields`)

### Module layout (`puroro-rt/src/fields/`)

| Path | Contents |
|---|---|
| [`fields.rs`](puroro-rt/src/fields.rs) | Public re-exports |
| [`shared.rs`](puroro-rt/src/fields/shared.rs) | `MessageCommon`, `PresenceBits`, `ProtoZero`, `DefaultIn`, `DeallocateIn`, `ProtoEmpty` |
| [`shared/field_presence.rs`](puroro-rt/src/fields/shared/field_presence.rs) | `FieldPresence` markers |
| [`shared/value_slot.rs`](puroro-rt/src/fields/shared/value_slot.rs) | `ValueSlot<T>` / `MaybeUninit<T>` storage |
| [`shared/slot_init.rs`](puroro-rt/src/fields/shared/slot_init.rs) | `SlotInitView` / `SlotInitMut` init-state handles |
| [`wire.rs`](puroro-rt/src/fields/wire.rs) | Wire-family re-exports |
| [`wire/scalar.rs`](puroro-rt/src/fields/wire/scalar.rs) | `ScalarProtoType` (singular varint + LEN) |
| [`wire/varint.rs`](puroro-rt/src/fields/wire/varint.rs) | `VarintProtoType`, `ProtoInt32`, … |
| [`wire/len.rs`](puroro-rt/src/fields/wire/len.rs) | `LenProtoType`, `ProtoString`, … |
| [`wire/fixed.rs`](puroro-rt/src/fields/wire/fixed.rs) | `Fixed32ProtoType` / `Fixed64ProtoType` (stub) |
| [`singular.rs`](puroro-rt/src/fields/singular.rs) | Singular field re-exports |
| [`singular/field.rs`](puroro-rt/src/fields/singular/field.rs) | `SingularField` / `SingularFieldMut` |
| [`singular/varint.rs`](puroro-rt/src/fields/singular/varint.rs) | `SingularVarintField` aliases |
| [`singular/len.rs`](puroro-rt/src/fields/singular/len.rs) | `SingularLenField` aliases |
| [`singular/message.rs`](puroro-rt/src/fields/singular/message.rs) | `NestedMessageField` |
| [`repeated.rs`](puroro-rt/src/fields/repeated.rs) | Repeated field re-exports |
| [`repeated/encoding.rs`](puroro-rt/src/fields/repeated/encoding.rs) | `Packed` / `Expanded` |
| [`repeated/varint.rs`](puroro-rt/src/fields/repeated/varint.rs) | `RepeatedVarintField` |
| [`repeated/len.rs`](puroro-rt/src/fields/repeated/len.rs) | `RepeatedLenField` |
| [`oneof.rs`](puroro-rt/src/fields/oneof.rs) | `OneofSlot` |

---

## 4. Shared infrastructure

[`MessageCommon<P, A>`](puroro-rt/src/fields/shared.rs) — one per generated message:

| Member | Role |
|---|---|
| `presence: P` | Bitfield newtype (`TaskPresence`, …) for EXPLICIT / LEGACY_REQUIRED singular fields |
| `unknown_fields: ManuallyDrop<UnmanagedVec<u8>>` | Round-trip unknown wire; closed-enum unknown variants. Allocator-less; freed by `MessageCommon::deallocate` |
| `alloc: A` | The single canonical allocator copy; borrowed (`&A`) by every field operation that (de)allocates |

Field catalog methods take `&MessageCommon` / `&mut MessageCommon`, not `&Task`, so wrappers stay decoupled from the parent message type.

[`PresenceBits`](puroro-rt/src/fields/shared.rs) — trait implemented on the message-specific presence **newtype** (not on raw `BitArray` — orphan rules). [`MessageCommon::is_present`](puroro-rt/src/fields/shared.rs) / `set_presence` forward to it.

[`ValueSlot<T>`](puroro-rt/src/fields/shared/value_slot.rs) — singular scalar storage behind a GAT on [`FieldPresence`](puroro-rt/src/fields/shared/field_presence.rs): always-initialized `T` for `Implicit` / `Oneof`; `MaybeUninit<T>` for `Explicit` / `LegacyRequired`. Construction / replace / clear thread an allocator via [`DefaultIn`](puroro-rt/src/fields/shared.rs) / [`DeallocateIn`](puroro-rt/src/fields/shared.rs) so heap payloads (`UnmanagedString`, `UnmanagedVec`) and copy scalars share one slot API. Mutation passes a [`SlotInitMut`](puroro-rt/src/fields/shared/slot_init.rs) handle (`slot_init_mut(common)`); reads pass [`SlotInitView`](puroro-rt/src/fields/shared/slot_init.rs) (`slot_init_view(common)`). [`ProtoEmpty`](puroro-rt/src/fields/shared.rs) drives IMPLICIT omit-on-encode (`is_proto_zero` for scalars, `is_empty` for LEN).

---

## 5. Wire encoding traits

One marker + trait per protobuf **wire family**. Semantic conversions delegate to **`protobuf-core`** (`puroro-rt` does not reimplement zigzag/varint).

### Singular scalars ([`wire/scalar.rs`](puroro-rt/src/fields/wire/scalar.rs))

[`ScalarProtoType`](puroro-rt/src/fields/wire/scalar.rs) is the trait consumed by [`SingularField`](puroro-rt/src/fields/singular/field.rs). It unifies varint and LEN for **non-repeated** fields:

```rust
pub trait ScalarProtoType {
    type Storage: DefaultIn + DeallocateIn + ProtoEmpty;
    type Ref<'a> where Self: 'a;
    type Mut<'a, A: Allocator + 'a>: DerefMut where Self: 'a;
    const WIRE_TYPE: WireType;
    fn get<'a>(storage: &'a Self::Storage) -> Self::Ref<'a>;
    fn with_mut<'a, A: Allocator + 'a>(storage: &'a mut Self::Storage, alloc: A) -> Self::Mut<'a, A>;
    fn encoded_len(field: u32, storage: &Self::Storage) -> usize;
    fn encode<B: BufMut>(field: u32, storage: &Self::Storage, buf: &mut B);
    fn decode<B: Buf, A: Allocator>(wire_type: WireType, buf: &mut B, alloc: A) -> Result<Self::Storage, DecodeError>;
}
// Implemented for ProtoInt32, …, ProtoBool, ProtoEnum<E>, ProtoString, ProtoBytes
```

### Varint / LEN helpers (also used by repeated)

[`VarintProtoType`](puroro-rt/src/fields/wire/varint.rs) and [`LenProtoType`](puroro-rt/src/fields/wire/len.rs) remain for repeated fields and for the `ScalarProtoType` impls that delegate to them.

```rust
pub trait VarintProtoType {
    type Value: Copy + ProtoZero;
    fn decode_wire(raw: u64) -> Result<Self::Value, DecodeError>;
    fn encode_wire(value: Self::Value) -> u64;
}
```

### Other families

| Trait | Wire | Markers | Status |
|---|---|---|---|
| `LenProtoType` | LEN | `ProtoString`, `ProtoBytes` ([`wire/len.rs`](puroro-rt/src/fields/wire/len.rs)) | **Done** |
| `Fixed32ProtoType` | I32 | `ProtoFixed32`, `ProtoFloat`, … ([`wire/fixed.rs`](puroro-rt/src/fields/wire/fixed.rs)) | Stub |
| `Fixed64ProtoType` | I64 | `ProtoFixed64`, `ProtoDouble`, … ([`wire/fixed.rs`](puroro-rt/src/fields/wire/fixed.rs)) | Stub |

Rust storage type alone does **not** identify protobuf encoding (`i32` can be int32, sint32, or enum). The marker type is the source of truth.

---

## 6. Presence policy

[`FieldPresence`](puroro-rt/src/fields/shared/field_presence.rs) — ZST markers composed into singular wrappers as type param `P`:

| Method / GAT | Role |
|---|---|
| `ValueSlot<T>` | `T` for `Implicit` / `Oneof`; `MaybeUninit<T>` for `Explicit` / `LegacyRequired` |
| `slot_init_mut` / `slot_init_view` | Init-state handles for [`ValueSlot`](puroro-rt/src/fields/shared/value_slot.rs) |
| `should_emit(common, is_payload_empty)` | Encode omit rule |
| `is_set(common, is_payload_empty)` | `has_*` / `Optional` |
| `payload_is_empty(slot)` | Empty / type-zero check via `ProtoEmpty` (`Implicit` only) |

| Marker | Encode | Slot init | Bitfield | Accessors |
|---|---|---|---|---|
| `Implicit` | Omit when payload empty / type-zero | Always initialized | No-op | `value()` |
| `Explicit<BIT>` | Omit when bit unset | Lazy via `ValueSlot::ensure_init` / `as_mut` | Set on `set` / `merge` / `value_mut` | `optional`, `clear` |
| `LegacyRequired<BIT>` | Same as `Explicit` | Same as `Explicit` | Same as `Explicit` | Same + `validate_required` |
| `Oneof` | Always emit when variant active | Always initialized | No-op (slot tracks presence) | `value()` / `value_mut(alloc)` |

[`RequiredFieldPresence`](puroro-rt/src/fields/shared/field_presence.rs) adds `validate_present` for `LegacyRequired`.

`BIT` is a **const generic on the marker type** (`Explicit<3>`, `LegacyRequired<1>`, …), not a runtime parameter to `bind`.

---

## 7. Field wrappers

**“Singular” means non-repeated** — both presence-tracked (“optional” / `EXPLICIT`) and non-presence-tracked (`IMPLICIT`) fields. It is *not* limited to proto `optional`. Cardinality is singular vs repeated; presence is a separate axis (`FieldPresence`).

Varint and LEN singular scalars share one wrapper, parametrised by [`ScalarProtoType`](puroro-rt/src/fields/wire/scalar.rs) `T` and presence `P`. `FIELD: u32` is a **struct** const generic; `BIT` lives on `Explicit<BIT>` / `LegacyRequired<BIT>`. The allocator is **not** a type parameter on the field — storage is allocator-less and `A` comes from [`MessageCommon`](puroro-rt/src/fields/shared.rs) at call sites. Nested messages stay a separate type (merge-into semantics + pointer presence).

| Wrapper | Module | Params | Aliases (ergonomics) |
|---|---|---|---|
| `SingularField<T, P, FIELD>` | [`singular/field.rs`](puroro-rt/src/fields/singular/field.rs) | `T: ScalarProtoType`, `P: FieldPresence` | `SingularVarintField`, `SingularLenField`, `ImplicitInt32`, `ExplicitString`, … |
| Fixed-width singular | (planned via `ScalarProtoType` + `SingularField`) | — | — |
| `NestedMessageField<M, P, FIELD, A>` | [`singular/message.rs`](puroro-rt/src/fields/singular/message.rs) | `P: MessagePresence` — `Singular`: `Option<UnmanagedBox<M>>`; `Oneof`: bare `UnmanagedBox<M>` — no bitfield | — |
| `OneofSlot<E>` | [`oneof.rs`](puroro-rt/src/fields/oneof.rs) | mutually exclusive variants | — |

**Closed enum:** `SingularField<ProtoEnum<E>, Explicit, FIELD>::bind_mut(…).merge_closed(…, |wire: i32| …)` — unknown values → `unknown_fields`, bit not set.

**LEGACY_REQUIRED:** `SingularField<…, LegacyRequired<BIT>, FIELD>::validate_required`.

Adding a singular wire type = one new `ScalarProtoType` impl (and usually a `VarintProtoType` / `LenProtoType` / fixed helper). Adding a presence mode = one new `FieldPresence` impl.

---

## 8. Proto field → catalog mapping

| Proto field | Generated member type |
|---|---|
| `IMPLICIT int32` | `SingularField<ProtoInt32, Implicit, FIELD>` (= `SingularVarintField<…>`) |
| `EXPLICIT int32` | `SingularField<ProtoInt32, Explicit<BIT>, FIELD>` |
| `IMPLICIT sint32` / `bool` | `SingularField<ProtoSint32 \| ProtoBool, Implicit, FIELD>` |
| `IMPLICIT open enum` | `SingularField<ProtoEnum<E>, Implicit, FIELD>` |
| `EXPLICIT closed enum` | `SingularField<ProtoEnum<E>, Explicit<BIT>, FIELD>` + `merge_closed` |
| `IMPLICIT string` | `SingularField<ProtoString, Implicit, FIELD>` (= `SingularLenField<…>`) |
| `EXPLICIT string` | `SingularField<ProtoString, Explicit<BIT>, FIELD>` |
| `LEGACY_REQUIRED string` | `SingularField<ProtoString, LegacyRequired<BIT>, FIELD>` |
| `IMPLICIT` / `EXPLICIT bytes` | `SingularField<ProtoBytes, P, FIELD>` |
| `repeated int32 PACKED` | `RepeatedPackedVarintField<ProtoInt32, FIELD, A>` |
| `repeated int32 EXPANDED` | `RepeatedExpandedVarintField<ProtoInt32, FIELD, A>` |
| `repeated string` | `RepeatedLenField<ProtoString, FIELD, A>` |
| `repeated bytes` | `RepeatedLenField<ProtoBytes, FIELD, A>` |
| nested message | `NestedMessageField<M, Singular, FIELD, A>` |
| `oneof` | `OneofSlot<E>` — not a singular catalog entry |

---

## Part III — Generated message

## 9. Struct layout

```rust
pub struct Task<A: Allocator + Clone = Global> {
    _common: MessageCommon<TaskPresence, A>,
    title: SingularLenField<ProtoString, Explicit<{ BIT_TITLE }>, { FIELD_TITLE }>,
    score: SingularVarintField<ProtoInt32, Implicit, { FIELD_SCORE }>,
    max_retries: SingularVarintField<ProtoInt32, Explicit<{ BIT_MAX_RETRIES }>, { FIELD_MAX_RETRIES }>,
    owner_id: SingularLenField<ProtoString, LegacyRequired<{ BIT_OWNER_ID }>, { FIELD_OWNER_ID }>,
    payload: SingularLenField<ProtoBytes, Explicit<{ BIT_PAYLOAD }>, { FIELD_PAYLOAD }>,
    tag_ids: RepeatedPackedVarintField<ProtoInt32, { FIELD_TAG_IDS }, A>,
    scores: RepeatedExpandedVarintField<ProtoInt32, { FIELD_SCORES }, A>,
    labels: RepeatedLenField<ProtoString, { FIELD_LABELS }, A>,
    status: SingularVarintField<ProtoEnum<Status>, Implicit, { FIELD_STATUS }>,
    priority: SingularVarintField<ProtoEnum<Priority>, Explicit<{ BIT_PRIORITY }>, { FIELD_PRIORITY }>,
    assignee: NestedMessageField<Address<A>, Singular, { FIELD_ASSIGNEE }, A>,
    notification: OneofSlot<NotificationStorage<A>>,
}
```

The `A: Allocator + Clone` struct bound is what lets the generated `Drop` clone the allocator into nested children and free every field from one place. Singular scalar / string / bytes wrappers keep allocator-less `unmanaged` (or `Copy`) payloads behind `ManuallyDrop` and do **not** take `A` as a type parameter — `A` appears inline only in `_common.alloc`, repeated fields, nested messages, and oneof storage that embeds those.

### Storage summary

| Field kind | Inside catalog wrapper | Presence |
|---|---|---|
| IMPLICIT varint / open enum / LEN | `ManuallyDrop<T>` (always initialized) | — |
| EXPLICIT / LEGACY_REQUIRED scalar or LEN | `ManuallyDrop<MaybeUninit<T>>` | bit in `_common.presence` |
| Repeated | `RepeatedVarintField` / `RepeatedLenField` | empty = absent |
| Nested message | `Option<UnmanagedBox<M>>` (`Singular`) | `Option`, not bitfield |
| Oneof | `Option<E>` in slot | `Option`, not bitfield |

Unset EXPLICIT slots are uninitialized (`MaybeUninit`); **only the bit** means "set". LEN storage is `UnmanagedString` / `UnmanagedVec<u8>`.

Public accessors are **one-line delegates** into catalog methods with `&self._common` / `&mut self._common`. `MessageEncode` and `MessageDecode` sum the same delegates; the generated `Drop` walks heap fields calling `deallocate(&self._common)` (or oneof `clear`).

### Codegen emission per message

1. Presence **newtype** + `PresenceBits` impl
2. Struct — `MessageCommon` + catalog members + `OneofSlot` per oneof
3. Associated constants on the message type — `impl Foo<A> { pub const FIELD_* …; pub const BIT_* …; }`. `BIT_*` is baked into each `Explicit<BIT>` / `LegacyRequired<BIT>` field type; `FIELD` is a struct const generic on the wrapper.
4. Accessor delegates ([DESIGN.md §4](DESIGN.md#40-generated-per-message-traits))
5. Trait impls — encode/decode/clone/eq as field sums
6. Child modules — enums, oneof enums

IR step: `ProtoField → FieldKind → catalog type + const args`.

### Path qualification (naming)

**Real generated code must fully-qualify every path it emits** — leading-`::` absolute paths such as `::puroro_rt::SingularLenField`, `::puroro::MessageDecode`, `::core::ops::DerefMut`, `::allocator_api2::alloc::Allocator` — and must not depend on `use` imports for the items it references. A `.proto` file can name its packages, messages, and fields with almost any identifier, so any *unqualified* name in the generated output risks colliding with a user-defined type, module, or import that lands in the same scope. Fully-qualified paths are collision-proof. The only names exempt from this are the ones the generator introduces itself and reserves by convention — e.g. the `_common` field and other `_`-prefixed internals — which cannot clash with proto-derived names.

**Crate split.** Items from [DESIGN.md §3](DESIGN.md#3-runtime-trait-api) (`MessageEncode`, `MessageDecode`, `Optional`, `HasDefault`, `DecodeError`, …) are emitted as `::puroro::…`. Field catalog types, `MessageCommon`, wire helpers, and `ProtoDefault` are emitted as `::puroro_rt::…`. A generated crate's `Cargo.toml` lists both dependencies; end-user application code should not add `puroro-rt` directly.

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
title: SingularLenField<ProtoString, Explicit<{ BIT_TITLE }>, { FIELD_TITLE }>, // proto: string title = 1;
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

**What not to comment** — avoid restating obvious one-line delegates (`has_title` → `self.title().is_set()`). Section + struct + dispatch comments are enough.

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

Runtime **`str_to_unmanaged_in(s, alloc)`** — copy bytes into an `UnmanagedString`; **`bytes_to_unmanaged_in(v, alloc)`** for `UnmanagedVec<u8>` (`puroro_rt::decode`). The `unsafe` (raw-parts / `deallocate`) is confined to the `puroro-rt` runtime and the generated `Drop`, not to generated accessors.

---

## 12. Message-level wire I/O

### Encode

1. Each field's `encoded_len` / `encode_raw` (catalog applies omit rules — [§14](#14-singular-fields)).
2. Append `_common.unknown_fields` verbatim.
3. `encoded_len` must match bytes written.

**Field order is not guaranteed.** Identical logical content may produce different wire bytes. Compare with `PartialEq`, not wire equality.

Runtime (`puroro_rt::encode`): `encode_varint_field`, `encode_len_field`, `encode_packed_*`, `encoded_len_*`.

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

1. Loop: `puroro_rt::decode::decode_tag` → `(field_number, wire_type)`.
2. `match field_number` — one catalog `merge` (or `merge_closed`) per arm.
3. Unknown → `puroro_rt::decode::skip_field_and_save` into `_common.unknown_fields`.
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

**Messages** (`Task<A>`, …): generated as below. **Scalar enums** (`Status`, `Priority`): `#[repr(transparent)]` newtypes over `i32` with associated constants (not Rust enums — proto value aliases may share an integer); `derive(Clone, Copy, Debug, PartialEq, Eq, Hash)`. **Oneof types**: the payload-less `NotificationCase` `derive`s `Clone, Copy, Debug, PartialEq, Eq`. `NotificationView` / `NotificationViewMut` are bound-view structs (no derives). `NotificationRef<'a, A>` is `Copy` (all payloads are `Copy`), but its `Copy`/`Clone` are hand-written to drop the spurious `A: Copy` bound the derive would add, and it omits `Debug`/`PartialEq`/`Eq` once a variant borrows a message (`&Address<A>`, which derives neither); a string/scalar-only group could keep the full derives. `NotificationMut<'a, A>` holds guards / `&mut` and derives nothing; the internal `NotificationStorage` needs no trait derives (comparison/formatting happen on the safe views).

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

In this catalog, **singular** means a **non-repeated** field — both `IMPLICIT` (no presence bit; proto3-style / “non-optional”) and `EXPLICIT` / `LEGACY_REQUIRED` (presence-tracked / “optional”). Presence is selected by `P: FieldPresence`; cardinality is selected by using `SingularField` vs a repeated wrapper.

### Unified wrapper (`SingularField<T, P, FIELD>`)

Varint and LEN share [`SingularField`](puroro-rt/src/fields/singular/field.rs), driven by [`ScalarProtoType`](puroro-rt/src/fields/wire/scalar.rs). Ergonomic aliases (`SingularVarintField`, `SingularLenField`, `ImplicitInt32`, `ExplicitString`, …) are type aliases of the same struct.

Storage is `ManuallyDrop<P::ValueSlot<T::Storage>>` (`T` or `MaybeUninit<T>`). Heap LEN payloads need an explicit `deallocate(&common)` / `deallocate_in(alloc)` from message / oneof `Drop`; copy scalars’ `DeallocateIn` is a no-op.

**Mutation goes through a bound view:** `field.bind_mut(&mut common)` yields [`SingularFieldMut`](puroro-rt/src/fields/singular/field.rs). `value_mut` ensures the slot is initialized, then returns `T::Mut` (e.g. `&mut i32` or `StringGuard`) after releasing the `common` borrow so the handle only ties up the field.

**Read accessors also go through a bound view:** `field.bind(&common)` yields [`SingularFieldRef`](puroro-rt/src/fields/singular/field.rs). Generated getters always bind first — even for `IMPLICIT` `value()` which does not consult `common` — so read and write share one shape.

| | IMPLICIT | EXPLICIT / LEGACY_REQUIRED |
|---|---|---|
| Encode | Omit when empty / type-zero | Omit when bit unset |
| Merge | `bind_mut(&mut common).merge(wire, buf)` | same (sets bit via `SlotInitMut`) |
| Getter | `bind(&common).value()` | `bind(&common).optional()` |
| Mutator | `bind_mut(&mut common).value_mut()` → `T::Mut` | same (sets bit) |
| Clear | `bind_mut(&mut common).clear()` | same |
| Release (LEN) | `deallocate(&common)` from message `Drop` | same |

Encode / `deallocate` / `validate_required` stay as plain field methods that take `&common` directly. Enum fields use `ProtoEnum<E>`; closed-enum unknown values use `merge_closed(…, |wire: i32| …)` on decode. `Optional` is a concrete struct (DESIGN.md §3); no `Option<T>` conversion.

### LEGACY_REQUIRED

Wire identical to EXPLICIT. Message `validate()` calls `validate_required` on each `LegacyRequired` field ([§12](#12-message-level-wire-io)).

---

## 15. Repeated fields

**Catalog:** [`RepeatedVarintField<T, E, A>`](puroro-rt/src/fields/repeated/varint.rs) and [`RepeatedLenField<T, A>`](puroro-rt/src/fields/repeated/len.rs). Empty vec = absent on encode.

### Varint (`RepeatedVarintField<T, E, A>`)

`E` is [`Packed`](puroro-rt/src/fields/repeated/encoding.rs) or [`Expanded`](puroro-rt/src/fields/repeated/encoding.rs) — affects **encode only**. [`merge`](puroro-rt/src/fields/repeated/varint.rs) always accepts both LEN (packed blob) and VARINT (single element).

| | Packed (`E = Packed`) | Expanded (`E = Expanded`) |
|---|---|---|
| Encode | One LEN record | One VARINT per element |
| Decode | Both forms | Both forms |

Elements live in `ManuallyDrop<UnmanagedVec<T::Value>>`. Mutation uses the bound-view idiom: `field.bind_mut(&mut common)` yields a [`RepeatedVarintFieldMut`](puroro-rt/src/fields/repeated/varint.rs) (no presence bit — repeated fields have none), whose consuming methods are `values_mut()` → guard (`impl DerefMut<Target = Vec<_, A>>`), `merge(wire, buf)`, and `clear()`; each obtains its own owned `alloc.clone()` from `common`. Read accessors use the same idiom: `field.bind(&common)` yields [`RepeatedVarintFieldRef`](puroro-rt/src/fields/repeated/varint.rs) with `as_slice` / `is_empty` (they ignore `common`, but generated getters still bind for uniformity). Encode / `deallocate(A)` stay on the field (called once from message `Drop`).

### LEN (`RepeatedLenField<T, A>`)

One LEN record per element (`repeated string` / `repeated bytes`), stored as `ManuallyDrop<UnmanagedVec<T::Storage>>`. Mutation uses the same bound-view idiom: `field.bind_mut(&mut common)` yields a [`RepeatedLenFieldMut`](puroro-rt/src/fields/repeated/len.rs) (no presence bit — repeated fields have none), whose consuming methods are `push_in(impl AsRef<[u8]>)`, `merge(wire, buf)`, and `clear()`. The typed `push_in` helper is kept instead of a bare `DerefMut` (which would expose allocator-less element storage that is impractical to construct). Because each element is itself allocator-less, `clear`/`deallocate` **drain and free every element first** (each via its own owned `alloc.clone()`), then free the buffer. Read accessors use `field.bind(&common)` → [`RepeatedLenFieldRef`](puroro-rt/src/fields/repeated/len.rs) (`as_slice` / `is_empty`). `deallocate(A)` stays on the field (called from `Drop`).

> Note: the bound-view idiom (`field.bind(&common).op()` / `field.bind_mut(&mut common).op()`) covers every field family — `SingularField`, `RepeatedLenField`, `RepeatedVarintField`, `NestedMessageField`, and `OneofSlot` — on both read and write paths. Terminal `deallocate` stays a direct field method (called from `Drop`).

---

## 16. Nested messages, oneof, unknown fields

### Nested (`NestedMessageField<M, P, FIELD, A>`)

Storage is chosen by a `MessagePresence` marker (a GAT): [`Singular`](puroro-rt/src/fields/singular/message.rs) = `Option<UnmanagedBox<M>>` for ordinary fields; [`Oneof`](puroro-rt/src/fields/shared/field_presence.rs) = a bare `UnmanagedBox<M>` for oneof variants (the slot tracks presence, so the box is always there). Mutation uses the **same bound-view idiom** as scalar fields (`field.bind_mut(&mut common).merge(wire, buf)` / `.get_mut()` / `.clear()`). Read accessors use `field.bind(&common).get()` / `.value()`. A nested message has no presence bit. `Singular` exposes `get()` / `get_mut()` (insert-if-absent); `Oneof` exposes `value()` / `value_mut()` (`&M` / `&mut M`). Encode: LEN tag + `child.encode_raw`. Decode merges into the child (creating it on first merge for `Singular`). Recursion limit: planned ([§17](#17-planned-optimisations--runtime-gaps)).

### Oneof (`OneofSlot<E>`)

Each wire occurrence replaces the whole slot (last wins). Encode active variant only. Decode: one match arm per variant field number.

**Generated types per group.** The owned storage holds allocator-less `unmanaged` values, so it is kept out of the public API and split from the safe views:

The sample `oneof notification` is deliberately **heterogeneous** — LEN, VARINT, and message variants — to show all three storage kinds:

| Type | Vis | Payloads | Role |
|---|---|---|---|
| `NotificationStorage<A>` | `pub(crate)` | `SingularLenField` / `SingularVarintField` / `NestedMessageField` | owned storage (field wrappers); `OneofDeallocate<A>`; owns encode glue |
| `NotificationCase` | `pub` | — | `Copy` discriminant (variants only; unset is `None`) → `notification_case() -> Option<_>` |
| `NotificationView<'a, A>` | `pub` | — | shared group bind (slot + `MessageCommon`) → `notification()` |
| `NotificationViewMut<'a, A>` | `pub` | — | mut group bind → `notification_mut()`; `as_view` / `as_mut` / `clear` |
| `NotificationRef<'a, A>` | `pub` | `&'a str` / `i32` / `&'a Address<A>` | projected read enum → `view.as_ref()` |
| `NotificationMut<'a, A>` | `pub` | `StringGuard<'a, A>` / `&'a mut i32` / `&'a mut Address<A>` | projected mut enum → `view_mut.as_mut()` |

**Variants own field wrappers, not raw storage.** Each variant holds the same field wrapper an ordinary singular field of that kind uses (`SingularField` / aliases for scalars and LEN; `NestedMessageField` for messages), so `value` / `value_mut` / `deallocate` are reused. The wrapper's presence is inert here (`Oneof` / `FieldPresence::Oneof`), so presence-aware methods are never needed for omit rules. Singular scalar / LEN wrappers no longer take `A`; the storage enum stays generic over `A` for nested message variants and for `value_mut(alloc)` call sites.

The oneof drives each variant with the **field's own** primitives. Build-empty is `new_in(alloc)` / `with_message_in(alloc)`. Merging is `bind_<variant>_mut(…).merge(wire, buf)`. Read/read-mut are `value()` / `value_mut(alloc)` for scalar and LEN (alloc ignored for copy scalars).

**The message variant uses `NestedMessageField<…, Oneof, …>` — a bare box, not `Option`.**

**Per-variant `bind_<variant>_mut` helpers** return the field's bound mutation view. Parent `_mut` accessors are one-liners: `bind_email_address_mut(…).value_mut()`, `bind_webhook_id_mut(…).value_mut()`, `bind_postal_mut(…).value_mut()`.

The storage enum implements [`OneofDeallocate<A>`](puroro-rt/src/fields/oneof.rs) (`unsafe fn deallocate(self, alloc: A)`) so the previously-active variant is freed through the message allocator before the slot is overwritten. Group accessors use the bound-view idiom: `slot.bind(&common)` / `slot.bind_mut(&mut common)` yield [`OneofSlotRef`](puroro-rt/src/fields/oneof.rs) / [`OneofSlotMut`](puroro-rt/src/fields/oneof.rs). Generated `NotificationView` / `NotificationViewMut` hold that pair. `OneofSlotMut` consuming methods:

- `variant_mut::<V>(make) -> &mut Value` — keeps the active variant if it is already `V`, else frees the previous variant and installs `from_variant(make(alloc.clone()))`. Backs the per-variant `bind_<variant>_mut` helpers.
- `set(value)` — replaces the whole group (frees the old variant).
- `clear()` — frees the active variant; backs `NotificationViewMut::clear`, `clear_notification`, and the message `Drop`.

The `set_*` per-variant setters are removed, matching the other field families.

### Unknown

`_common.unknown_fields` — valid partial wire stream via `puroro_rt::decode::skip_field_and_save`; re-emitted on encode. `SGroup` / `EGroup` not preserved.

---

## Part V — Future work

## 17. Planned optimisations & runtime gaps

| Area | Current | Target |
|---|---|---|
| UTF-8 validation | Always `decode_string_in` (VERIFY) | Per-field `utf8_validation` feature |
| Recursion limit | Not enforced | Depth counter in nested merge → `RecursionLimitExceeded` |
| Repeated wrappers | `RepeatedVarintField`, `RepeatedLenField` | — |
| Fixed32/64 catalog | Trait stubs | `ScalarProtoType` impls + `SingularField` |
| `protoc` plugin | — | FieldKind → catalog emission |
| Zero-copy views | — | `TaskView<'buf>` (DESIGN.md §8) |
| `TaskLazy` | DESIGN only | Wire buffer + on-demand decode |
| `Hash` / `serde` | Deferred | Opt-in features |
