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
| **`puroro`** | Stable user API: `Message`, `Optional`, `HasDefault`, errors, `WireType`, `UnknownField`. |
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
puroro_rt::fields       SingularField<T, P, FIELD> (T includes ProtoMessage), …
    │  shared/ — MessageCommon, FieldPresence, ValueSlot,
    │            DefaultIn / DeallocateIn / ProtoEmpty
    │  wire/   — ProtoType (singular Slot); RepeatedItems (repeated Element);
    │            VarintProtoType / LenProtoType (helpers)
    │  singular/, repeated/, oneof/
    │  T: ProtoType thin wrapper (ProtoInt32(i32), ProtoString(…), …)
    │  P: FieldPresence (Implicit / Explicit<BIT> / LegacyRequired<BIT> / Oneof)
    ▼
puroro_rt::encode/decode   Buf adapters, LEN framing, unknown-field helpers
    │  (DecodeError, WireType from puroro)
    ▼
puroro                  Message, Optional, errors
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
| `MessageCommon`, `PresenceBits`, `FieldDeallocate`, `OneofSlot` | **Done** |
| `ProtoType` + thin wrappers (varint / LEN) + `ProtoMessage` | **Done** |
| `VarintProtoType` / `LenProtoType` (scalar / repeated helpers) | **Done** |
| `RepeatedItems` / `PackableRepeatedItems` / `RepeatedSlicePush` | **Done** |
| `FieldPresence` (`Implicit` / `Explicit` / `LegacyRequired` / `Oneof`) | **Done** |
| `ValueSlot`, `SlotInitView` / `SlotInitMut`, `DefaultIn` / `DeallocateIn` / `ProtoEmpty` | **Done** |
| `SingularField<T, P, FIELD>`  | **Done** |
| `ProtoBool` + `BitPacked<VALUE_BIT>` on `SingularField` (singular / oneof `bool`) | **Done** |
| `ValueLayout` / `Inline` / `PayloadAccess` | **Done** |
| Closed-enum unknown → `DecodeError::UnknownClosedEnum` → unknown fields, `validate_required` | **Done** |
| Nested message via `SingularField<ProtoMessage<…>, …>` | **Done** |
| [`sample-generated`](sample-generated/) (`Task` / `Address`) | **Done** |
| `Fixed32ProtoType` / `Fixed64ProtoType` on `SingularField` | **Stub** |
| Repeated catalog (`RepeatedField<T, E, FIELD>`) | **Done** |
| `protoc` plugin | **Planned** |

---

## Part II — Runtime catalog (`puroro_rt::fields`)

### Module layout (`puroro-rt/src/fields/`)

| Path | Contents |
|---|---|
| [`lib.rs`](puroro-rt/src/lib.rs) | Crate-root catalog re-exports (`::puroro_rt::SingularField`, …) |
| [`fields.rs`](puroro-rt/src/fields.rs) | Module root (`pub(crate)`; `pub mod` only) |
| [`shared.rs`](puroro-rt/src/fields/shared.rs) | `MessageCommon`, `PresenceBits`, `DefaultIn`, `DeallocateIn`, `ProtoEmpty` |
| [`shared/field_presence.rs`](puroro-rt/src/fields/shared/field_presence.rs) | `FieldPresence` markers |
| [`shared/field_deallocate.rs`](puroro-rt/src/fields/shared/field_deallocate.rs) | `FieldDeallocate` — uniform `deallocate(&common)` |
| [`shared/slot_init.rs`](puroro-rt/src/fields/shared/slot_init.rs) | `SlotInitView` / `SlotInitMut` init-state handles |
| [`wire.rs`](puroro-rt/src/fields/wire.rs) | Wire-family re-exports |
| [`wire/proto_type.rs`](puroro-rt/src/fields/wire/proto_type.rs) | `ProtoType` (wire) + `PayloadAccess` (inline storage) |
| [`shared/value_layout.rs`](puroro-rt/src/fields/shared/value_layout.rs) | `ValueLayout`, `Inline`, `BitPacked` |
| [`wire/repeated_items.rs`](puroro-rt/src/fields/wire/repeated_items.rs) | `RepeatedItems` (`Element` for repeated buffers) |
| [`wire/proto_message.rs`](puroro-rt/src/fields/wire/proto_message.rs) | `ProtoMessage` (nested message marker) |
| [`wire/varint.rs`](puroro-rt/src/fields/wire/varint.rs) | `VarintProtoType`, `ProtoInt32`, … |
| [`wire/len.rs`](puroro-rt/src/fields/wire/len.rs) | `LenProtoType`, `ProtoString`, … |
| [`wire/fixed.rs`](puroro-rt/src/fields/wire/fixed.rs) | `Fixed32ProtoType` / `Fixed64ProtoType` (stub) |
| [`singular.rs`](puroro-rt/src/fields/singular.rs) | Singular field re-exports |
| [`singular/field.rs`](puroro-rt/src/fields/singular/field.rs) | `SingularField` — `T: ProtoType`, stores `T::Slot` |
| [`repeated.rs`](puroro-rt/src/fields/repeated.rs) | Repeated field re-exports |
| [`repeated/encoding.rs`](puroro-rt/src/fields/repeated/encoding.rs) | `Packed` / `Expanded` (`RepeatedEncoding`) |
| [`repeated/field.rs`](puroro-rt/src/fields/repeated/field.rs) | `RepeatedField` — `T: RepeatedItems`, stores `T::Element` |
| [`oneof.rs`](puroro-rt/src/fields/oneof.rs) | `OneofSlot` |

---

## 4. Shared infrastructure

[`MessageCommon<P, A>`](puroro-rt/src/fields/shared.rs) — one per generated message:

| Member | Role |
|---|---|
| `presence: P` | Bitfield newtype (`TaskPresence`, …) for EXPLICIT / LEGACY_REQUIRED presence **and** packed bool value bits |
| `unknown_fields: ManuallyDrop<UnmanagedVec<u8, A>>` | Preserve policy: round-trip unknown wire blob; closed-enum unknowns. Public view via `iter_unknown_fields`. Freed by `MessageCommon::deallocate` |
| `alloc: A` | The single canonical allocator instance; cloned (by value) into every field operation that (de)allocates |

Field catalog methods take `&MessageCommon` / `&mut MessageCommon`, not `&Task`, so wrappers stay decoupled from the parent message type.

[`FieldDeallocate`](puroro-rt/src/fields/shared/field_deallocate.rs) — every catalog field (and [`OneofSlot`](puroro-rt/src/fields/oneof.rs)) implements `deallocate(&mut self, common: &MessageCommon<…>)`. Generated message `Drop` calls this on **each direct child** with the same shape. Copy / bit-packed fields are no-ops. Oneof **variants** are released inside the group's deallocate via [`OneofDeallocate`](puroro-rt/src/fields/oneof.rs) (`deallocate(self, common)`), which forwards to the same field `deallocate(common)`.

[`PresenceBits`](puroro-rt/src/fields/shared.rs) — trait implemented on the message-specific bitfield **newtype** (not on raw `BitArray` — orphan rules). Bits cover EXPLICIT / LEGACY_REQUIRED **presence** and packed **bool values**. [`MessageCommon::is_bit_set`](puroro-rt/src/fields/shared.rs) / `set_bit` / `bit_mut` forward to it; `bit_mut` returns bitvec's `BitRef<'_, Mut, u8, Lsb0>` (generated presence newtypes wrap `BitArray<[u8; N], Lsb0>`).

[`ValueSlot<T>`](puroro-rt/src/fields/shared/value_slot.rs) — singular **slot** storage behind a GAT on [`FieldPresence`](puroro-rt/src/fields/shared/field_presence.rs): always-initialized `T` for `Implicit` / `Oneof`; `MaybeUninit<T>` for `Explicit` / `LegacyRequired`; `Option<T>` for `NonOneof` (pointer presence). Here `T` is [`ProtoType::Slot`](puroro-rt/src/fields/wire/proto_type.rs) (the type marker itself, ZST [`ProtoBool`](puroro-rt/src/fields/wire/varint.rs), or `UnmanagedBox<M, A>` for messages). Construction / teardown thread an allocator via [`DefaultIn`](puroro-rt/src/fields/shared.rs) / [`DeallocateIn`](puroro-rt/src/fields/shared.rs) (each has associated `type Alloc`; heap payloads set `Alloc = A`). Reads and mutation go through short-lived views: `slot.with(init, common)` → [`ValueSlotRefAccess`](puroro-rt/src/fields/shared/value_slot.rs) / `slot.with_mut(init, common)` → [`ValueSlotMutAccess`](puroro-rt/src/fields/shared/value_slot.rs) (`get` / `get_mut` / `set` / `clear`). Init markers ([`AlwaysInitialized`](puroro-rt/src/fields/shared/slot_init.rs) / [`BitInit`](puroro-rt/src/fields/shared/slot_init.rs)) are borrow-free; they read/update state through the passed [`MessageCommon`](puroro-rt/src/fields/shared.rs). Slot payloads use [`AddressableSlot`](puroro-rt/src/fields/shared/value_slot.rs); logical bool values are read/written via [`ProtoType`](puroro-rt/src/fields/wire/proto_type.rs) against `_common.presence`. Singular IMPLICIT omit goes through [`ProtoType::is_proto_empty`](puroro-rt/src/fields/wire/proto_type.rs) (slot [`ProtoEmpty`](puroro-rt/src/fields/shared.rs) for addressable types; bit read for `ProtoBool`).

[`SingularField::bind`](puroro-rt/src/fields/singular/field.rs) / [`bind_mut`](puroro-rt/src/fields/singular/field.rs) — inherent MessageCommon binding → [`SingularFieldRef`](puroro-rt/src/fields/singular/field.rs) / [`SingularFieldMut`](puroro-rt/src/fields/singular/field.rs). Repeated fields and [`OneofSlot`](puroro-rt/src/fields/oneof.rs) use the same inherent `bind` / `bind_mut` call shape. Getter / `_mut` payload types are [`ProtoType::Ref`](puroro-rt/src/fields/wire/proto_type.rs) / [`Mut`](puroro-rt/src/fields/wire/proto_type.rs).

---

## 5. Wire encoding traits

One marker + trait per protobuf **wire family**. Semantic conversions delegate to **`protobuf-core`** (`puroro-rt` does not reimplement zigzag/varint). Singular scalar types are thin wrappers over their payload; repeated fields keep the inner `Value` / `Storage`.

### Singular field type markers ([`wire/proto_type.rs`](puroro-rt/src/fields/wire/proto_type.rs))

[`ProtoType`](puroro-rt/src/fields/wire/proto_type.rs) is the trait consumed by [`SingularField`](puroro-rt/src/fields/singular/field.rs) (including nested messages via [`ProtoMessage`](puroro-rt/src/fields/wire/proto_message.rs)). It unifies wire/storage semantics for **non-repeated** fields. Each implementor is a **protobuf type marker**; physical field storage is the associated `Slot` (`Self` for addressable markers / ZST [`ProtoBool`](puroro-rt/src/fields/wire/varint.rs); `UnmanagedBox<M, A>` for messages):

```rust
pub trait ProtoType: Sized {
    type Alloc: Allocator + Clone;
    type Slot: AddressableSlot<SlotAlloc = Self::Alloc> + DefaultIn<Alloc = Self::Alloc>;
    type Ref<'a> where Self: 'a;
    type Mut<'a>: DerefMut where Self: 'a;
    type Written;
    const WIRE_TYPE: WireType;
    // IMPLICIT omit: ProtoEmpty on addressable slots; bit read for ProtoBool
    fn is_proto_empty<Pb>(slot: &Self::Slot, common: &MessageCommon<Pb, Self::Alloc>) -> bool;
    fn get<'a, Pb>(slot: &'a Self::Slot, common: &'a MessageCommon<Pb, Self::Alloc>) -> Self::Ref<'a>;
    fn with_mut / write / clear(/* ValueSlot + MessageCommon */);
    // encoded_len / encode stay on the marker: int32 vs sint32 share Ref = i32
    fn encoded_len<'a>(value: Self::Ref<'a>, field: u32) -> usize where Self: 'a;
    fn encode<'a, B: BufMut>(value: Self::Ref<'a>, field: u32, buf: &mut B) where Self: 'a;
    fn decode<B: Buf>(…) -> Result<Self::Written, DecodeError>;
    // default: decode then write (last wins); ProtoMessage overrides for merge-into
    fn merge(/* ValueSlot + MessageCommon + wire */) -> Result<(), DecodeError>;
}
// Implemented for ProtoInt32<A>, …, ProtoBool<A, BIT>, ProtoEnum<E, K, A>, ProtoString<A>, ProtoBytes<A>, ProtoMessage<M, A>
```

Singular / oneof `bool` uses bit-index-free [`ProtoBool<A>`](puroro-rt/src/fields/wire/varint.rs) plus [`BitPacked<VALUE_BIT>`](puroro-rt/src/fields/shared/value_layout.rs) as the field's [`ValueLayout`](puroro-rt/src/fields/shared/value_layout.rs) (orthogonal to presence `P`). Inline payloads use default `L = Inline` via [`PayloadAccess`](puroro-rt/src/fields/wire/proto_type.rs).
### Repeated elements (`RepeatedItems`)

[`RepeatedItems`](puroro-rt/src/fields/wire/repeated_items.rs) extends [`ProtoType`](puroro-rt/src/fields/wire/proto_type.rs) with the physical element type stored in a repeated buffer (`Element`), plus per-element encode / decode / merge / deallocate. Singular fields store `Slot`; repeated fields store `Element` (not always the same — e.g. future nested-message repeated uses `Element = M` while singular keeps `Slot = UnmanagedBox<M, A>`).

| Marker | `Element` | Packable |
|---|---|---|
| Addressable varint / enum | `VarintProtoType::Value` (`i32`, …) | yes (`PackableRepeatedItems`) |
| `ProtoString` / `ProtoBytes` | `LenProtoType::Storage` | no (`RepeatedSlicePush` for `push_*`) |
| `ProtoMessage<M, A>` (future) | `M` | no |
| `ProtoBool` | — (no `RepeatedItems`; use plain `bool` elements later) | — |

### Varint / LEN helpers

[`VarintProtoType`](puroro-rt/src/fields/wire/varint.rs) and [`LenProtoType`](puroro-rt/src/fields/wire/len.rs) are wire/storage helpers shared by singular `ProtoType` impls and `RepeatedItems` impls.

```rust
pub trait VarintProtoType {
    type Value: Copy;
    fn decode_wire(raw: u64) -> Result<Self::Value, DecodeError>;
    fn encode_wire(value: Self::Value) -> u64;
}
```

### Other families

| Trait | Wire | Types | Status |
|---|---|---|---|
| `LenProtoType` | LEN | `ProtoString`, `ProtoBytes` ([`wire/len.rs`](puroro-rt/src/fields/wire/len.rs)) | **Done** |
| `Fixed32ProtoType` | I32 | `ProtoFixed32`, `ProtoFloat`, … ([`wire/fixed.rs`](puroro-rt/src/fields/wire/fixed.rs)) | Stub |
| `Fixed64ProtoType` | I64 | `ProtoFixed64`, `ProtoDouble`, … ([`wire/fixed.rs`](puroro-rt/src/fields/wire/fixed.rs)) | Stub |

Rust payload type alone does **not** identify protobuf encoding (`i32` can be int32 or sint32). Distinct thin wrappers (`ProtoInt32` vs `ProtoSint32`) are the source of truth.

---

## 6. Presence policy

[`FieldPresence`](puroro-rt/src/fields/shared/field_presence.rs) — ZST markers composed into singular wrappers as type param `P`:

| Method / GAT | Role |
|---|---|
| `ValueSlot<T>` | `T` for `Implicit` / `Oneof`; `MaybeUninit<T>` for `Explicit` / `LegacyRequired` |
| `slot_init_mut` / `slot_init_view` | Borrow-free init markers for [`ValueSlot`](puroro-rt/src/fields/shared/value_slot.rs) |
| `should_emit(common, is_payload_empty)` | Encode omit rule |
| `is_set(common, is_payload_empty)` | `has_*` / `Optional` |
| `payload_is_empty(slot)` | Empty / type-zero check via `ProtoEmpty` (`Implicit` only) |

| Marker | Encode | Slot init | Bitfield | Accessors |
|---|---|---|---|---|
| `Implicit` | Omit when payload empty / type-zero | Always initialized | No-op | `value()` |
| `Explicit<BIT>` | Omit when bit unset | Lazy via `with_mut(...).get_mut()` | Set on `set` / `merge` / `value_mut` | `optional`, `clear` |
| `LegacyRequired<BIT>` | Same as `Explicit` | Same as `Explicit` | Same as `Explicit` | Same + `validate_required` |
| `Oneof` | Always emit when variant active | Always initialized | No-op (slot tracks presence) | `value()` / `value_mut(alloc)` |

[`RequiredFieldPresence`](puroro-rt/src/fields/shared/field_presence.rs) adds `validate_present` for `LegacyRequired`.

`BIT` is a **const generic on the marker type** (`Explicit<3>`, `LegacyRequired<1>`, …), not a runtime parameter to `bind`.

---

## 7. Field wrappers

**“Singular” means non-repeated** — both presence-tracked (“optional” / `EXPLICIT`) and non-presence-tracked (`IMPLICIT`) fields. It is *not* limited to proto `optional`. Cardinality is singular vs repeated; presence is a separate axis (`FieldPresence`).

Varint and LEN singular scalars share one wrapper, parametrised by [`ProtoType`](puroro-rt/src/fields/wire/proto_type.rs) `T` and presence `P`. `FIELD: u32` is a **struct** const generic; `BIT` lives on `Explicit<BIT>` / `LegacyRequired<BIT>`. Markers such as `ProtoString<A>` / `ProtoInt32<A>` carry allocator type `A` (and `PhantomData` on unmanaged payloads) so `T::Alloc` matches [`MessageCommon`](puroro-rt/src/fields/shared.rs); fields still do **not** store an allocator instance. Nested messages use the same wrapper: `SingularField<ProtoMessage<M, A>, NonOneof|Oneof, FIELD>`.

| Wrapper | Module | Params | Aliases (ergonomics) |
|---|---|---|---|
| `SingularField<T, P, FIELD, L, D>` | [`singular/field.rs`](puroro-rt/src/fields/singular/field.rs) | `T: ProtoType`, `L: ValueLayout<T>` (default `Inline`), stores `P::ValueSlot<T::Slot>` | — |
| `SingularField<ProtoBool<A>, P, FIELD, BitPacked<VALUE_BIT>>` | same | `Slot = Self` (ZST); value at `VALUE_BIT` via layout | — |
| `SingularField<ProtoMessage<M, A>, P, FIELD>` | same | `Slot = UnmanagedBox<M, A>`; `NonOneof` → `Option`; `Oneof` → always-present | — |
| `RepeatedField<T, E, FIELD>` | [`repeated/field.rs`](puroro-rt/src/fields/repeated/field.rs) | `T: RepeatedItems`, `E: RepeatedEncoding<T>`, stores `T::Element` | `RepeatedPackedVarintField`, … |
| Fixed-width singular | (planned via `ProtoType` + `SingularField`) | — | — |
| `OneofSlot<E>` | [`oneof.rs`](puroro-rt/src/fields/oneof.rs) | mutually exclusive variants | — |

**Closed enum:** `SingularField<ProtoEnum<E, Closed>, Explicit, FIELD>::bind(…).merge(…)` — `decode` yields `UnknownClosedEnum { raw }`, which `merge` diverts → `unknown_fields`, bit not set.

**LEGACY_REQUIRED:** `SingularField<…, LegacyRequired<BIT>, FIELD>::validate_required`.

Adding a singular wire type = one new `ProtoType` impl (and usually a `VarintProtoType` / `LenProtoType` / fixed helper). Adding a presence mode = one new `FieldPresence` impl.

---

## 8. Proto field → catalog mapping

| Proto field | Generated member type |
|---|---|
| `IMPLICIT int32` | `SingularField<ProtoInt32, Implicit, FIELD>`  |
| `EXPLICIT int32` | `SingularField<ProtoInt32, Explicit<BIT>, FIELD>` |
| `IMPLICIT sint32` | `SingularField<ProtoSint32, Implicit, FIELD>` |
| `IMPLICIT bool` | `SingularField<ProtoBool, Implicit, FIELD, BitPacked<VALUE_BIT>>` |
| `EXPLICIT bool` | `SingularField<ProtoBool, Explicit<PRESENCE_BIT>, FIELD, BitPacked<VALUE_BIT>>` |
| `LEGACY_REQUIRED bool` | `SingularField<ProtoBool, LegacyRequired<PRESENCE_BIT>, FIELD, BitPacked<VALUE_BIT>>` |
| oneof `bool` | `SingularField<ProtoBool, Oneof, FIELD, BitPacked<VALUE_BIT>>` inside the oneof storage enum |
| `IMPLICIT open enum` | `SingularField<ProtoEnum<E, Open>, Implicit, FIELD>` |
| `EXPLICIT closed enum` | `SingularField<ProtoEnum<E, Closed>, Explicit<BIT>, FIELD>` (same `.merge`) |
| `IMPLICIT string` | `SingularField<ProtoString, Implicit, FIELD>`  |
| `EXPLICIT string` | `SingularField<ProtoString, Explicit<BIT>, FIELD>` |
| `LEGACY_REQUIRED string` | `SingularField<ProtoString, LegacyRequired<BIT>, FIELD>` |
| `IMPLICIT` / `EXPLICIT bytes` | `SingularField<ProtoBytes, P, FIELD>` |
| `repeated int32 PACKED` | `RepeatedField<ProtoInt32, Packed, FIELD>` |
| `repeated int32 EXPANDED` | `RepeatedField<ProtoInt32, Expanded, FIELD>` |
| `repeated string` | `RepeatedField<ProtoString, Expanded, FIELD>` |
| `repeated bytes` | `RepeatedField<ProtoBytes, Expanded, FIELD>` |
| nested message | `SingularField<ProtoMessage<M, A>, NonOneof, FIELD>` |
| `oneof` | `OneofSlot<E>` — not a singular catalog entry |

---

## Part III — Generated message

## 9. Struct layout

```rust
pub struct Task<A: Allocator + Clone = Global> {
    _common: MessageCommon<TaskPresence, A>,
    title: SingularField<ProtoString<A>, Explicit<{ BIT_TITLE }>, { FIELD_TITLE }>,
    score: SingularField<ProtoInt32<A>, Implicit, { FIELD_SCORE }>,
    max_retries: SingularField<ProtoInt32<A>, Explicit<{ BIT_MAX_RETRIES }>, { FIELD_MAX_RETRIES }>,
    owner_id: SingularField<ProtoString<A>, LegacyRequired<{ BIT_OWNER_ID }>, { FIELD_OWNER_ID }>,
    payload: SingularField<ProtoBytes<A>, Explicit<{ BIT_PAYLOAD }>, { FIELD_PAYLOAD }>,
    tag_ids: RepeatedField<ProtoInt32<A>, Packed, { FIELD_TAG_IDS }>,
    scores: RepeatedField<ProtoInt32<A>, Expanded, { FIELD_SCORES }>,
    labels: RepeatedField<ProtoString<A>, Expanded, { FIELD_LABELS }>,
    status: SingularField<ProtoEnum<Status, Open, A>, Implicit, { FIELD_STATUS }>,
    priority: SingularField<ProtoEnum<Priority, Closed, A>, Explicit<{ BIT_PRIORITY }>, { FIELD_PRIORITY }>,
    assignee: SingularField<ProtoMessage<Address<A>, A>, NonOneof, { FIELD_ASSIGNEE }>,
    notification: OneofSlot<NotificationStorage<A>>,
    done: SingularField<ProtoBool<A>, Implicit, { FIELD_DONE }, BitPacked<{ BIT_DONE_VALUE }>>,
    flag: SingularField<ProtoBool<A>, Explicit<{ BIT_FLAG }>, { FIELD_FLAG }, BitPacked<{ BIT_FLAG_VALUE }>>,
}
```

The `A: Allocator + Clone` struct bound is what lets the generated `Drop` clone the allocator into nested children and free every field from one place. Heap payloads use `unmanaged` types parameterized by `A` plus `PhantomData<A>` (no allocator *instance* in the field). Protobuf markers (`ProtoString<A>`, …) carry the same `A` so `DefaultIn` / `DeallocateIn` associate `Alloc = A` against `MessageCommon<P, A>`. The only owned `A` value remains `_common.alloc`. Singular `bool` uses `SingularField<ProtoBool<A>, …, BitPacked<VALUE_BIT>>` with ZST `Slot = Self`; the value lives in `_common.presence`.

### Storage summary

| Field kind | Inside catalog wrapper | Presence / value bits |
|---|---|---|
| IMPLICIT varint / open enum / LEN | `ManuallyDrop<T>` (always initialized) | — |
| EXPLICIT / LEGACY_REQUIRED scalar or LEN | `ManuallyDrop<MaybeUninit<T>>` | presence bit in `_common.presence` |
| Singular / oneof `bool` | `SingularField` + `ProtoBool` + `BitPacked<VALUE_BIT>` (`Slot = Self`, ZST) | value bit (+ presence bit for EXPLICIT / LEGACY_REQUIRED) in `_common.presence` |
| Repeated | `RepeatedField<T, E, FIELD>` (`UnmanagedVec<T::Element>`) | empty = absent |
| Nested message | `Option<UnmanagedBox<M, A>>` (`NonOneof`) | `Option`, not bitfield |
| Oneof (non-bool) | `Option<E>` in slot | `Option`, not bitfield |

Unset EXPLICIT slots are uninitialized (`MaybeUninit`); **only the bit** means "set". LEN storage is `UnmanagedString` / `UnmanagedVec<u8>`.

Public accessors are **one-line delegates** into catalog methods with `&self._common` / `&mut self._common`. `Message` (encode / merge / unknown / validate) sums the same delegates; the generated `Drop` walks heap fields calling `deallocate(&self._common)` (or oneof `clear`).

### Codegen emission per message

1. Presence **newtype** + `PresenceBits` impl
2. Struct — `MessageCommon` + catalog members + `OneofSlot` per oneof
3. Associated constants on the message type — `impl Foo<A> { pub const FIELD_* …; pub const BIT_* …; }`. `BIT_*` is baked into each `Explicit<BIT>` / `LegacyRequired<BIT>` field type; `FIELD` is a struct const generic on the wrapper.
4. Accessor delegates ([DESIGN.md §4](DESIGN.md#40-generated-per-message-traits))
5. Trait impls — encode/decode/clone/eq as field sums
6. Child modules — enums, oneof enums

IR step: `ProtoField → FieldKind → catalog type + const args`.

### Path qualification (naming)

**Real generated code must fully-qualify every path it emits** — leading-`::` absolute paths such as `::puroro_rt::SingularField`, `::puroro::Message`, `::core::ops::DerefMut`, `::allocator_api2::alloc::Allocator` — and must not depend on `use` imports for the items it references. A `.proto` file can name its packages, messages, and fields with almost any identifier, so any *unqualified* name in the generated output risks colliding with a user-defined type, module, or import that lands in the same scope. Fully-qualified paths are collision-proof. The only names exempt from this are the ones the generator introduces itself and reserves by convention — e.g. the `_common` field and other `_`-prefixed internals — which cannot clash with proto-derived names.

**Crate split.** Items from [DESIGN.md §3](DESIGN.md#3-runtime-trait-api) (`Message`, `Optional`, `HasDefault`, `DecodeError`, …) are emitted as `::puroro::…`. Field catalog types, `MessageCommon`, wire helpers, and `ProtoDefault` are emitted as `::puroro_rt::…`. A generated crate's `Cargo.toml` lists both dependencies; end-user application code should not add `puroro-rt` directly.

**The checked-in [`sample-generated/`](sample-generated/) deliberately breaks this rule for readability.** It pulls names in with `use` and refers to them by short name (`SingularField`, `Allocator`, `MessageCommon`, …) so the reference output stays easy to read and review. Read those short names as stand-ins for the fully-qualified paths the production protoc plugin would actually emit.

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
// Message
// ---------------------------------------------------------------------------
```

**Per-field accessor block** — before each field’s `impl` methods:

```text
// -- title (EXPLICIT string, proto field 1) --
```

Include **presence**, **wire/kind** (string, int32, repeated packed, nested, …), and **proto field number**.

**Struct members** — trailing comment tying storage to proto:

```rust
title: SingularField<ProtoString, Explicit<{ BIT_TITLE }>, { FIELD_TITLE }>, // proto: string title = 1;
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

Every field kind merges through the same bound-view shape — `self.<field>.bind_mut(&mut self._common).merge(wire_type, buf)?` (repeated and nested-message fields likewise take only `common`; oneof uses `OneofSlotMut`) — so the code generator emits one form. Oneof variant arms use the **variant field name** and number. The `_ =>` unknown-field arm gets a short comment (`// unknown field — preserve in _common`).

**What not to comment** — avoid restating obvious one-line delegates (`has_title` → `self.title().is_set()`). Section + struct + dispatch comments are enough.

**Proto doc comments** — when the `.proto` field has `///` documentation, emit a Rust `///` doc comment on the **public accessor methods** (not on private struct fields unless the proto doc is part of the public API story).

---

## 10. Presence / bool-value bit indices

Tracked bits use [`bitvec::BitArray`](https://docs.rs/bitvec) inline in the message (`BitVec` is heap-only and incompatible with custom `A`). Store them in `_common.presence` inside a message-specific newtype.

**Assignment (one pass, ascending field number):**

1. For each `EXPLICIT` / `LEGACY_REQUIRED` singular field (including `bool`), allocate one **presence** bit.
2. For each singular or oneof `bool` field, allocate one **value** bit.

Gaps in field numbers do not create gaps in bit indices. Oneof non-bool variants do not take bits (presence stays on `OneofSlot`).

| Kind | Bits |
|---|---|
| Implicit bool | value 1 |
| Explicit / LegacyRequired bool | presence 1 + value 1 |
| Oneof bool | value 1 |
| Explicit non-bool | presence 1 |

### `Task` — nine bits → `BitArray<[u8; 2], Lsb0>`

| Field | # | Role | `BIT_*` |
|---|---|---|---|
| `title` | 1 | EXPLICIT presence | `0` |
| `max_retries` | 3 | EXPLICIT presence | `1` |
| `owner_id` | 4 | LEGACY_REQUIRED presence | `2` |
| `payload` | 5 | EXPLICIT presence | `3` |
| `priority` | 10 | EXPLICIT presence | `4` |
| `done` | 16 | IMPLICIT bool value | `5` (`BIT_DONE_VALUE`) |
| `flag` | 17 | EXPLICIT presence | `6` |
| `flag` | 17 | EXPLICIT bool value | `7` (`BIT_FLAG_VALUE`) |
| `urgent` | 18 | oneof bool value | `8` (`BIT_URGENT_VALUE`) |

### `Address` — two bits → `BitArray<[u8; 1], Lsb0>`

| Field | # | `BIT_*` |
|---|---|---|
| `street` | 1 | `0` |
| `city` | 2 | `1` |

### Newtype pattern

```rust
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TaskPresence(BitArray<[u8; 2], Lsb0>);

impl TaskPresence {
    pub const ZERO: Self = Self(BitArray::ZERO);
}

impl PresenceBits for TaskPresence {
    fn is_set(&self, bit: usize) -> bool { self.0[bit] }
    fn set(&mut self, bit: usize, value: bool) { self.0.set(bit, value); }
    fn bit_mut(&mut self, bit: usize) -> impl DerefMut<Target = bool> + '_ {
        self.0.get_mut(bit).expect("bit index in range")
    }
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
2. `match field_number` — one catalog `merge` per arm (closed-enum unknowns are diverted inside `merge` via `DecodeError::UnknownClosedEnum`).
3. Unknown → `puroro_rt::decode::skip_field_and_save` into `_common.unknown_fields`.
4. Singular: last wins. Repeated: append. Nested: merge sub-buffer.

```rust
Self::FIELD_PRIORITY => self
    .priority
    .bind_mut(&mut self._common)
    .merge(wire_type, buf)?,
```

Nested LEN payloads use `Buf::take(len)` before child `merge_from`.

### Validation

`Message::validate()` — `owner_id.validate_required(&self._common)?` (and any other `LegacyRequired` fields); messages with none return `Ok(())`. `Message::decode` does **not** auto-validate.

---

## 13. Derived traits

**Messages** (`Task<A>`, …): generated as below. **Scalar enums** (`Status`, `Priority`): `#[repr(transparent)]` newtypes over `i32` with associated constants (not Rust enums — proto value aliases may share an integer); `derive(Clone, Copy, Debug, PartialEq, Eq, Hash)`. **Oneof types**: the payload-less `NotificationCase` `derive`s `Clone, Copy, Debug, PartialEq, Eq`. Group bound views are rt `OneofView` / `OneofViewMut` (no derives). `NotificationRef<'a, A>` is a type alias of the shared `Notification` shape and is `Copy` (all payloads are `Copy`), but its `Copy`/`Clone` are hand-written to drop the spurious `A: Copy` bound the derive would add, and it omits `Debug`/`PartialEq`/`Eq` once a variant borrows a message (`&Address<A>`, which derives neither); a string/scalar-only group could keep the full derives. `NotificationMut<'a, A>` holds guards / `&mut` and derives nothing; the internal `NotificationStorage` alias needs no trait derives (comparison/formatting happen on the safe views). Pattern matching uses `Notification::…` (aliases do not invent variant paths).

| Trait | Bounds | Notes |
|---|---|---|
| `Default` | `A: Clone + Default` | Clears presence; empty heap fields |
| `Drop` | `A: Clone` | Calls `deallocate(&_common)` on every direct child ([`FieldDeallocate`](puroro-rt/src/fields/shared/field_deallocate.rs)), then `_common.deallocate()` |

**Not currently generated:** `Clone`, `PartialEq`, `Eq`, `Debug`, `Copy`, `Ord`, `Hash`. `Clone`/`PartialEq`/`Debug` need `&A` to copy or format the allocator-less fields, so they require a `clone_in(&self, alloc)`-style API (future work) rather than `#[derive]`.

**`Global` extras:** `Task::new()`, `impl Default for Task`, `Task::decode(buf)`.

Compare messages semantically via getters; deep copy (when added) will copy data into a second tree, so prefer `Arc<Task<A>>` for shared immutable messages.

---

## Part IV — Field behaviour

## 14. Singular fields

In this catalog, **singular** means a **non-repeated** field — both `IMPLICIT` (no presence bit; proto3-style / “non-optional”) and `EXPLICIT` / `LEGACY_REQUIRED` (presence-tracked / “optional”). Presence is selected by `P: FieldPresence`; cardinality is selected by using `SingularField` vs a repeated wrapper.

### Unified wrapper (`SingularField<T, P, FIELD>`)

Varint and LEN share [`SingularField`](puroro-rt/src/fields/singular/field.rs), parametrised by type marker `T: ProtoType` and layout `L: ValueLayout<T>`. Addressable scalars use `L = Inline` ([`PayloadAccess`](puroro-rt/src/fields/wire/proto_type.rs)); bit-packed [`ProtoBool`](puroro-rt/src/fields/wire/varint.rs) uses `L = BitPacked<VALUE_BIT>`. There are no wire-family aliases — generated code names `SingularField` directly.

Storage is `ManuallyDrop<P::ValueSlot<T::Slot>>` — `T` / `MaybeUninit<T>` (including ZST `ProtoBool`) depending on presence. Heap LEN payloads need an explicit [`FieldDeallocate::deallocate`](puroro-rt/src/fields/shared/field_deallocate.rs)(`&common`) from message / oneof teardown; copy scalars’ / unit-slot `DeallocateIn` is a no-op.

**Mutation goes through a bound view:** `field.bind_mut(&mut common)` yields [`SingularFieldMut`](puroro-rt/src/fields/singular/field.rs). `value_mut` returns `T::Mut` (e.g. `&mut i32`, `StringGuard`, or bitvec `BitRef` for `ProtoBool`).
**Read accessors also go through a bound view:** `field.bind(&common)` yields [`SingularFieldRef`](puroro-rt/src/fields/singular/field.rs). Generated getters always bind first — even for `IMPLICIT` `value()` which does not consult `common` — so read and write share one shape.

| | IMPLICIT | EXPLICIT / LEGACY_REQUIRED |
|---|---|---|
| Encode | Omit when empty / type-zero | Omit when bit unset |
| Merge | `bind_mut(&mut common).merge(wire, buf)` | same (sets bit via `SlotInitMut`) |
| Getter | `bind(&common).value()` | `bind(&common).optional()` |
| Mutator | `bind_mut(&mut common).value_mut()` → `T::Mut` | same (sets bit) |
| Clear | `bind_mut(&mut common).clear()` | same |
| Release (LEN) | `deallocate(&common)` from message `Drop` | same |

Encode / `deallocate` / `validate_required` stay as plain field methods that take `&common` directly. Enum fields use `ProtoEnum<E, Open|Closed>`; closed-enum unknown values surface as `DecodeError::UnknownClosedEnum` and are diverted by singular `merge`. `Optional` is a concrete struct (DESIGN.md §3); no `Option<T>` conversion.

### Bit-packed `bool` (`ProtoBool` + `BitPacked`)

Singular / oneof `bool` uses [`SingularField`](puroro-rt/src/fields/singular/field.rs) with type marker [`ProtoBool`](puroro-rt/src/fields/wire/varint.rs) (`ProtoType::Slot = Self`, ZST) and layout [`BitPacked<VALUE_BIT>`](puroro-rt/src/fields/shared/value_layout.rs). The logical `bool` lives at `VALUE_BIT` in `_common.presence`; EXPLICIT / LEGACY_REQUIRED also use `P`'s presence bit (orthogonal). Bound views are the same `SingularFieldRef` / `SingularFieldMut`; `value_mut` returns bitvec's `BitRef<'_, Mut, …>` via [`MessageCommon::bit_mut`](puroro-rt/src/fields/shared.rs). Wire encode/decode goes through `ProtoType`; storage access goes through `ValueLayout`. Implicit omit treats a clear value bit as absent; Explicit can encode an explicit `false`.

**`repeated bool` is a different shape** — elements are plain `bool` in the repeated buffer and must not use `BitPacked` / a MessageCommon bit index.

### LEGACY_REQUIRED

Wire identical to EXPLICIT. Message `validate()` calls `validate_required` on each `LegacyRequired` field ([§12](#12-message-level-wire-io)).

---

## 15. Repeated fields

**Catalog:** [`RepeatedField<T, E, FIELD>`](puroro-rt/src/fields/repeated/field.rs) where `T: RepeatedItems` and `E: RepeatedEncoding<T>` ([`Packed`](puroro-rt/src/fields/repeated/encoding.rs) / [`Expanded`](puroro-rt/src/fields/repeated/encoding.rs)). Empty vec = absent on encode. Storage is `ManuallyDrop<UnmanagedVec<T::Element, T::Alloc>>`.

| | Packed (`E = Packed`, packable `T` only) | Expanded (`E = Expanded`) |
|---|---|---|
| Encode | One LEN record (varint payload) | One tagged record per element |
| Decode | Both packed + expanded (packable) | Per-element (string / bytes / …) |

Mutation uses the bound-view idiom: `field.bind_mut(&mut common)` → [`RepeatedFieldMut`](puroro-rt/src/fields/repeated/field.rs). Copy elements expose `values_mut()` → `Vec` guard; string / bytes use `push_in` via [`RepeatedSlicePush`](puroro-rt/src/fields/wire/repeated_items.rs) (allocator-less element storage is impractical to build through bare `DerefMut`). `clear` / `deallocate` drain and free heap elements first, then free the buffer. Read: `field.bind(&common)` → [`RepeatedFieldRef`](puroro-rt/src/fields/repeated/field.rs) (`as_slice` / `is_empty`).

**`repeated bool`:** not wired yet. Elements should be plain `bool` with **no** MessageCommon bit index — do not use [`BitPacked`](puroro-rt/src/fields/shared/value_layout.rs) (see [Bit-packed bool](#bit-packed-bool-protobool)).

**`repeated message` (future):** `RepeatedField<ProtoMessage<M, A>, Expanded, FIELD>` with `Element = M` (no per-element `UnmanagedBox`).

> Note: the bound-view idiom covers every field family — `SingularField`, `RepeatedField`, and `OneofSlot` — on both read and write paths. Terminal `deallocate` stays a direct field method (called from `Drop`).

---

## 16. Nested messages, oneof, unknown fields

### Nested (`SingularField<ProtoMessage<M, A>, P, FIELD>`)

Same wrapper as other singular fields. Storage is `ManuallyDrop<P::ValueSlot<UnmanagedBox<M, A>>>` with `P: FieldPresence`: [`NonOneof`](puroro-rt/src/fields/shared/field_presence.rs) → `Option<UnmanagedBox<M, A>>`; [`Oneof`](puroro-rt/src/fields/shared/field_presence.rs) → always-present `UnmanagedBox<M, A>` (the slot tracks case presence). Child type bound is `M: Message<Alloc = A>` (empty children via [`Message::new_in`](src/message.rs)). Wire merge / encode / clear live on [`ProtoMessage`](puroro-rt/src/fields/wire/proto_message.rs) / [`ProtoType::merge`](puroro-rt/src/fields/wire/proto_type.rs) (merge-into). Accessors use the usual bound-view idiom (`bind` / `bind_mut` → `get` / `get_mut` / `value` / `merge` / `clear`). Recursion limit: planned ([§17](#17-planned-optimisations--runtime-gaps)).

### Oneof (`OneofSlot<E>`)

Each wire occurrence replaces the whole slot (last wins). Encode active variant only. Decode: one match arm per variant field number.

**Generated types per group.** The owned storage holds allocator-less `unmanaged` values, so it is kept out of the public API. Shape / Storage / Ref / Mut share one generic enum; Storage is a `pub(crate)` alias that implements [`OneofGroup`](puroro-rt/src/fields/oneof.rs). Group bound views are rt [`OneofView`](puroro-rt/src/fields/oneof.rs) / [`OneofViewMut`](puroro-rt/src/fields/oneof.rs) (no per-oneof generated View structs). Message accessors return RPIT `impl OneofGroup<Case = …>` so Storage never appears in signatures.

The sample `oneof notification` is deliberately **heterogeneous** — LEN, VARINT, bool, and message variants — to show the storage kinds:

| Type | Vis | Payloads | Role |
|---|---|---|---|
| `Notification<…>` | `pub` | type params | canonical shape (shared by aliases) |
| `NotificationStorage<A>` | `pub(crate)` | field wrappers | owned storage; `OneofGroup` + `OneofDeallocate`; encode glue |
| `NotificationCase` | `pub` | — | `Copy` discriminant (variants only; unset is `None`) → `notification_case() -> Option<_>` |
| `OneofView` / `OneofViewMut` | rt `pub` | — | group bind (slot + `MessageCommon`) → `notification()` / `notification_mut()` |
| `NotificationRef<'a, A>` | `pub` alias | via [`ProtoType::Ref`](puroro-rt/src/fields/wire/proto_type.rs) | projected read → `view.as_ref()` |
| `NotificationMut<'a, A>` | `pub` alias | via [`ProtoType::Mut`](puroro-rt/src/fields/wire/proto_type.rs) | projected mut → `view_mut.as_mut()` |

**Ref/Mut payloads are not hard-coded in generated aliases.** They project from each variant's [`ProtoType`](puroro-rt/src/fields/wire/proto_type.rs) marker (`ProtoString`, `ProtoInt32`, `ProtoMessage`, `ProtoBool`, …). Per-variant private field type aliases remain the single source for Storage.

**Variants own field wrappers, not raw storage.** Each variant holds the same field wrapper an ordinary singular field of that kind uses (`SingularField` / aliases — including `ProtoBool<A, VALUE_BIT>` for `bool` and `ProtoMessage<M, A>` for messages), so `value` / `value_mut` / `deallocate` are reused. The wrapper's presence is inert here (`Oneof` / `FieldPresence::Oneof`), so presence-aware omit rules are never consulted; bool still packs its value into `_common.presence`. Markers and unmanaged payloads are parameterized by `A` (type only / `PhantomData`); the owned allocator instance stays on the message.

The oneof drives each variant with the **field's own** primitives. Empty construction is [`EnumVariant::new_value`](puroro-rt/src/fields/enum_variant.rs) (`new_in` / `with_message_in`). Merging is `slot.bind_mut(common).variant_mut::<V>().bind_mut(common).merge(wire, buf)`. Read getters use `slot.bind(common).variant_of::<V>().optional()` / `.get()`.

**The message variant uses `SingularField<ProtoMessage<…>, Oneof, …>` — always-present `UnmanagedBox<M, A>`, not `Option`.**

Parent `_mut` accessors are:
`slot.bind_mut(common).variant_mut::<V>().bind_mut(common).value_mut()`.

The storage alias implements [`OneofDeallocate`](puroro-rt/src/fields/oneof.rs) so the previously-active variant is freed through the message allocator before the slot is overwritten. Group accessors use the bound-view idiom: `slot.bind(&common)` / `slot.bind_mut(&mut common)` yield [`OneofSlotRef`](puroro-rt/src/fields/oneof.rs) / [`OneofSlotMut`](puroro-rt/src/fields/oneof.rs). `OneofView` / `OneofViewMut` hold that pair via `OneofGroup`. `OneofSlotMut` consuming methods:

- `variant_mut::<V>() -> &mut EnumVariant::Value` — keeps the active variant if it is already `V`, else frees the previous variant and installs `from_variant(EnumVariant::new_value(alloc.clone()))`. Consumes the slot view so callers can re-borrow `common` and `field.bind_mut(common)` for `.value_mut()` / `.merge(…)`.
- `set(value)` — replaces the whole group (frees the old variant).
- `clear()` — frees the active variant; backs `OneofViewMut::clear`, `clear_notification`, and the message `Drop`.

The `set_*` per-variant setters are removed, matching the other field families.

### Unknown

**Storage (default Preserve):** `_common.unknown_fields` — contiguous partial wire stream via `puroro_rt::decode::skip_field_and_save` / `save_unknown_varint_field`; re-emitted on encode. `SGroup` / `EGroup` not preserved.

**Public accessor:** `Message::unknown_fields()` returns `impl Iterator<Item = ::puroro::UnknownField<'_>>` by parsing that blob with [`iter_unknown_fields`](puroro-rt/src/decode.rs) (also `MessageCommon::iter_unknown_fields`). Encode paths read the blob directly and do not go through the iterator.

---

## Part V — Future work

## 17. Planned optimisations & runtime gaps

| Area | Current | Target |
|---|---|---|
| UTF-8 validation | Always `decode_string_in` (VERIFY) | Per-field `utf8_validation` feature |
| Recursion limit | Not enforced | Depth counter in nested merge → `RecursionLimitExceeded` |
| Repeated wrappers | `RepeatedField` + `RepeatedItems` | `repeated bool` / `repeated message` |
| Fixed32/64 catalog | Trait stubs | `ProtoType` impls + `SingularField` |
| `protoc` plugin | — | FieldKind → catalog emission |
| Zero-copy views | — | `TaskView<'buf>` (DESIGN.md §8) |
| `TaskLazy` | DESIGN only | Wire buffer + on-demand decode |
| `Hash` / `serde` | Deferred | Opt-in features |
