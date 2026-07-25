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
    - 15.1 [Map fields](#151-map-fields)
16. [Nested messages, oneof, unknown fields](#16-nested-messages-oneof-unknown-fields)

**Part V — Future work**

17. [Planned optimisations & runtime gaps](#17-planned-optimisations--runtime-gaps)
    - 17.1 [Submessage inline optimisation](#171-submessage-inline-optimisation)
    - 17.2 [String / Bytes inline optimisation](#172-string--bytes-inline-optimisation)

---

## Part I — Overview

## 1. Project context

| Crate | Responsibility |
|---|---|
| **`protobuf-core`** | Wire primitives (`Varint`, `Tag`, `WireType`). Used by `puroro` and `puroro-rt`; generated code does not import it. |
| **`puroro`** | Stable user API: `Message`, `Optional`, `HasDefault`, errors, `WireType`, `UnknownField`. |
| **`puroro-rt`** | Generated-code runtime: [`fields`](puroro-rt/src/fields.rs), wire `encode` / `decode` helpers, `ProtoDefault`. Depends on `puroro` for shared types. |
| **`protoc-gen-puroro`** | `protoc` plugin: decode `CodeGeneratorRequest`, resolve types, emit Rust per [DESIGN.md §0](DESIGN.md#0-project-architecture). |

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
    │  wire/   — ProtoType (singular Slot); RepeatedElement; MapKey;
    │            VarintProtoType (packed / bit-packed helper)
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
| Map | No | alloc (map also owns its own `A`) |
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
| `VarintProtoType` (packed / bit-packed wire helper) | **Done** |
| `RepeatedElement` / `RepeatedElementMerge` / `PackableRepeatedElement` / `RepeatedSlicePush` / `RepeatedVecMut` | **Done** |
| `MapKey` + `MapField` / `MapEntries` (map entry wire encode/merge) | **Done** |
| `FieldPresence` (`Implicit` / `Explicit` / `LegacyRequired` / `Oneof`) | **Done** |
| `ValueSlot`, `SlotInitView` / `SlotInitMut`, `DefaultIn` / `DeallocateIn` / `ProtoEmpty` | **Done** |
| `SingularField<T, P, FIELD>`  | **Done** |
| `ProtoBool` + `BitPacked<VALUE_BIT>` on `SingularField` (singular / oneof `bool`) | **Done** |
| `ValueLayout` / `Inline` / `PayloadAccess` | **Done** |
| Closed-enum unknown → `DecodeError::UnknownClosedEnum` → unknown fields, `validate_required` | **Done** |
| Nested message via `SingularField<ProtoMessage<…>, …>` | **Done** |
| [`sample-generated`](sample-generated/) (`Task` / `Address`) | **Done** — hand-written normative eager output |
| `Fixed32ProtoType` / `Fixed64ProtoType` + markers on `SingularField` / `RepeatedField` | **Done** |
| Repeated catalog (`RepeatedField<T, E, FIELD>`) | **Done** |
| `protoc-gen-puroro` plugin I/O (`CodeGeneratorRequest` / `Response`) | **Done** |
| Descriptor decode (messages / fields / enums / oneofs / features subset) | **Done** (intentional subset; defaults / map_entry / services / extensions not in IR yet) |
| Type resolve (`FileSet`, `TypeRef`, presence / occurrence) | **Done** — `emit` resolves the full request before generating |
| Module forest + `ModuleLayout::SingleFile` | **Done** (`FileTree` deferred) |
| Empty-message emission (no fields / nested types) | **Done** — compile-tested via [`puroro-codegen-tests`](puroro-codegen-tests/) |
| FieldKind → catalog emission (scalars, repeated, enum, oneof, map, …) | **Not started** |

Live plugin output is still a **fake** path: one field-less root message per file. Emission now goes through `resolved::resolve`, but does not yet map fields to the runtime catalog. Full-featured structs in this document and in [`sample-generated/`](sample-generated/) describe the **target** shape the emitter must reach.

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
| [`wire/repeated_element.rs`](puroro-rt/src/fields/wire/repeated_element.rs) | `RepeatedElement` / `RepeatedElementMerge` (`Element` for repeated buffers) |
| [`wire/map_element.rs`](puroro-rt/src/fields/wire/map_element.rs) | `MapKey` (subset of `RepeatedElement`) |
| [`wire/proto_message.rs`](puroro-rt/src/fields/wire/proto_message.rs) | `ProtoMessage` (nested message marker) |
| [`wire/varint.rs`](puroro-rt/src/fields/wire/varint.rs) | `VarintProtoType`, `ProtoInt32`, … |
| [`wire/len.rs`](puroro-rt/src/fields/wire/len.rs) | `ProtoString`, `ProtoBytes` |
| [`wire/fixed.rs`](puroro-rt/src/fields/wire/fixed.rs) | `Fixed32ProtoType` / `Fixed64ProtoType` + `ProtoFixed*` / `ProtoFloat` / `ProtoDouble` |
| [`singular.rs`](puroro-rt/src/fields/singular.rs) | Singular field re-exports |
| [`singular/field.rs`](puroro-rt/src/fields/singular/field.rs) | `SingularField` — `T: ProtoType`, stores `T::Slot` |
| [`repeated.rs`](puroro-rt/src/fields/repeated.rs) | Repeated field re-exports |
| [`repeated/encoding.rs`](puroro-rt/src/fields/repeated/encoding.rs) | `Packed` / `Expanded` (`RepeatedEncoding`) |
| [`repeated/field.rs`](puroro-rt/src/fields/repeated/field.rs) | `RepeatedField` — `T: RepeatedElement`, stores `T::Element` |
| [`map.rs`](puroro-rt/src/fields/map.rs) | Map field re-exports |
| [`map/entries.rs`](puroro-rt/src/fields/map/entries.rs) | `MapEntries` — allocator-owning `HashMap` |
| [`map/entry.rs`](puroro-rt/src/fields/map/entry.rs) | Map-entry wire encode / decode (`key=1`, `value=2`) |
| [`map/field.rs`](puroro-rt/src/fields/map/field.rs) | `MapField` — `K: MapKey`, `V: RepeatedElement` |
| [`oneof.rs`](puroro-rt/src/fields/oneof.rs) | `OneofSlot` |

---

## 4. Shared infrastructure

[`MessageCommon<P, A>`](puroro-rt/src/fields/shared.rs) — one per generated message:

| Member | Role |
|---|---|
| `presence: P` | `BitArray<[u8; N], Lsb0>` for EXPLICIT / LEGACY_REQUIRED presence **and** packed bool value bits |
| `unknown_fields: ManuallyDrop<UnmanagedVec<u8, A>>` | Preserve policy: round-trip unknown wire blob; closed-enum unknowns. Public view via `iter_unknown_fields`. Freed by `MessageCommon::deallocate` |
| `alloc: A` | The single canonical allocator instance; cloned (by value) into every field operation that (de)allocates |

Field catalog methods take `&MessageCommon` / `&mut MessageCommon`, not `&Task`, so wrappers stay decoupled from the parent message type.

[`FieldDeallocate`](puroro-rt/src/fields/shared/field_deallocate.rs) — every catalog field (and [`OneofSlot`](puroro-rt/src/fields/oneof.rs)) implements `deallocate(&mut self, common: &MessageCommon<…>)`. Generated message `Drop` calls this on **each direct child** with the same shape. Copy / bit-packed fields are no-ops. Oneof **variants** are released inside the group's deallocate via [`OneofDeallocate`](puroro-rt/src/fields/oneof.rs) (`deallocate(self, common)`), which forwards to the same field `deallocate(common)`.

[`PresenceBits`](puroro-rt/src/fields/shared.rs) — trait implemented in `puroro-rt` for `BitArray<[u8; N], Lsb0>` (so generated code does not emit a per-message newtype). Bits cover EXPLICIT / LEGACY_REQUIRED **presence** and packed **bool values**. [`MessageCommon::is_bit_set`](puroro-rt/src/fields/shared.rs) / `set_bit` / `bit_mut` forward to it; `bit_mut` returns bitvec's `BitRef<'_, Mut, u8, Lsb0>`.

[`ValueSlot<T>`](puroro-rt/src/fields/shared/value_slot.rs) — singular **slot** storage behind a GAT on [`FieldPresence`](puroro-rt/src/fields/shared/field_presence.rs): always-initialized `T` for `Implicit` / `Oneof`; `MaybeUninit<T>` for `Explicit` / `LegacyRequired`; `Option<T>` for `Message` (pointer presence). Here `T` is [`ProtoType::Slot`](puroro-rt/src/fields/wire/proto_type.rs) (the type marker itself, ZST [`ProtoBool`](puroro-rt/src/fields/wire/varint.rs), or `UnmanagedBox<M, A>` for messages). Construction / teardown thread an allocator via [`DefaultIn<A>`](puroro-rt/src/fields/shared.rs) / [`unmanaged::DeallocateIn<A>`](unmanaged/) (allocator as a **trait parameter**, not an associated type). Reads and mutation go through short-lived views: `slot.with(init, common)` → [`ValueSlotRefAccess`](puroro-rt/src/fields/shared/value_slot.rs) / `slot.with_mut(init, common)` → [`ValueSlotMutAccess`](puroro-rt/src/fields/shared/value_slot.rs) (`get` / `get_mut` / `set` / `clear`). Init markers ([`AlwaysInitialized`](puroro-rt/src/fields/shared/slot_init.rs) / [`BitInit`](puroro-rt/src/fields/shared/slot_init.rs)) are borrow-free; they read/update state through the passed [`MessageCommon`](puroro-rt/src/fields/shared.rs). Slot payloads use [`AddressableSlot`](puroro-rt/src/fields/shared/value_slot.rs); logical bool values are read/written via [`ProtoType`](puroro-rt/src/fields/wire/proto_type.rs) against `_common.presence`. Singular IMPLICIT omit goes through [`ProtoType::is_proto_empty`](puroro-rt/src/fields/wire/proto_type.rs) (slot [`ProtoEmpty`](puroro-rt/src/fields/shared.rs) for addressable types; bit read for `ProtoBool`).

[`SingularField::bind`](puroro-rt/src/fields/singular/field.rs) / [`bind_mut`](puroro-rt/src/fields/singular/field.rs) — inherent MessageCommon binding → [`SingularFieldRef`](puroro-rt/src/fields/singular/field.rs) / [`SingularFieldMut`](puroro-rt/src/fields/singular/field.rs). Repeated fields and [`OneofSlot`](puroro-rt/src/fields/oneof.rs) use the same inherent `bind` / `bind_mut` call shape. Getter / `_mut` payload types are [`ProtoType::Ref`](puroro-rt/src/fields/wire/proto_type.rs) / [`Mut`](puroro-rt/src/fields/wire/proto_type.rs).

---

## 5. Wire encoding traits

One marker + trait per protobuf **wire family**. Semantic conversions delegate to **`protobuf-core`** (`puroro-rt` does not reimplement zigzag/varint). Singular scalar types are thin wrappers over their payload; repeated fields keep the inner `Value` / `Storage`.

### Singular field type markers ([`wire/proto_type.rs`](puroro-rt/src/fields/wire/proto_type.rs))

[`ProtoType`](puroro-rt/src/fields/wire/proto_type.rs) is the trait consumed by [`SingularField`](puroro-rt/src/fields/singular/field.rs) (including nested messages via [`ProtoMessage`](puroro-rt/src/fields/wire/proto_message.rs)). Markers are **allocator-free**; physical storage / views are GATs over `A`. Singular slots use bare wire values (`i32`, `()`, …) / `UnmanagedString` / `UnmanagedBox<M, A>`:

```rust
pub trait ProtoType: Sized {
    type Slot<A: Allocator + Clone>;
    type Ref<'a, A: Allocator + Clone> where Self: 'a, A: 'a;
    type Mut<'a, A: Allocator + Clone>: DerefMut where Self: 'a, A: 'a;
    type Written<A: Allocator + Clone>; // accepted by set / write
    fn encoded_len<'a, A>(value: Self::Ref<'a, A>, field: u32) -> usize;
    fn encode<'a, A, B: BufMut>(value: Self::Ref<'a, A>, field: u32, buf: &mut B);
}
// PayloadAccess: get / with_mut / write / clear / merge (singular wire decode)
// Implemented for ProtoInt32, …, ProtoEnum<E, K>, ProtoString, ProtoBytes, ProtoMessage<M>
// ProtoBool uses BitPacked::merge instead of PayloadAccess
```

Singular wire decode is **merge-into only** (`PayloadAccess::merge` / `BitPacked::merge`). There is no `ProtoType::decode → Written`; nested messages merge into the present child via `Message::merge_from`.

Singular / oneof `bool` uses allocator-free [`ProtoBool`](puroro-rt/src/fields/wire/varint.rs) plus [`BitPacked<VALUE_BIT>`](puroro-rt/src/fields/shared/value_layout.rs) as the field's [`ValueLayout`](puroro-rt/src/fields/shared/value_layout.rs) (orthogonal to presence `P`). Inline payloads use default `L = Inline` via [`PayloadAccess`](puroro-rt/src/fields/wire/proto_type.rs). Allocator `A` lives on [`SingularField`](puroro-rt/src/fields/singular/field.rs) / [`RepeatedField`](puroro-rt/src/fields/repeated/field.rs). Slot construction uses [`DefaultIn<A>`](puroro-rt/src/fields/shared.rs) / [`DeallocateIn<A>`](puroro-rt/src/fields/shared.rs) (allocator as a **trait parameter**, not an associated type), so bare `i32` / `()` work without slot newtypes.
### Repeated elements (`RepeatedElement`)

[`RepeatedElement`](puroro-rt/src/fields/wire/repeated_element.rs) extends [`ProtoType`](puroro-rt/src/fields/wire/proto_type.rs) with GAT `Element<A>`, plus per-element encode / length / deallocate. Decode / merge live on [`RepeatedElementMerge<A>`](puroro-rt/src/fields/wire/repeated_element.rs) so nested messages can require `M: Message<Alloc = A>`. That trait also provides `default_element` / `decode_element` (singular occurrence; packed `Len` rejected) for map-entry interiors. Singular fields store `Slot<A>`; repeated fields store `Element<A>` (not always the same — nested-message repeated uses `Element = M` while singular keeps `Slot = UnmanagedBox<M, A>`).

| Marker | `Element<A>` | Packable | Public mutation |
|---|---|---|---|
| Addressable varint / enum | `VarintProtoType::Value` (`i32`, …) | yes (`PackableRepeatedElement`) | `values_mut` → `RepeatedVecMut` |
| `ProtoString` / `ProtoBytes` | `UnmanagedString<A>` / `UnmanagedVec<u8, A>` | no | `container_mut` → `RepeatedElementsMut` (`push` then fill) |
| `ProtoMessage<M>` | `M` (inline; use site `M::Alloc = A`) | no | `values_mut` → `RepeatedVecMut` |
| `ProtoBool` | `bool` (plain; not `BitPacked`) | yes (`PackableRepeatedElement`) | `values_mut` → `RepeatedVecMut` |

[`MapKey`](puroro-rt/src/fields/wire/map_element.rs) is an empty marker over `RepeatedElement` restricted to valid protobuf map keys (integrals / `bool` / `string`). Map **values** use `RepeatedElement` directly (anything except another map).

### Varint helper

[`VarintProtoType`](puroro-rt/src/fields/wire/varint.rs) is a thin wire helper shared by singular `ProtoType` impls, packed repeated encode/decode, and bit-packed bool. LEN scalars (`ProtoString` / `ProtoBytes`) go through `ProtoType` / `RepeatedElement` directly — there is no parallel `LenProtoType`.

```rust
pub trait VarintProtoType {
    type Value: Copy;
    fn decode_wire(raw: u64) -> Result<Self::Value, DecodeError>;
    fn encode_wire(value: Self::Value) -> u64;
}
```

### Other families

| Trait / family | Wire | Types | Status |
|---|---|---|---|
| `VarintProtoType` | VARINT | numerics, enums, `ProtoBool` ([`wire/varint.rs`](puroro-rt/src/fields/wire/varint.rs)) | **Done** |
| LEN scalars | LEN | `ProtoString`, `ProtoBytes` via `ProtoType` / `RepeatedElement` ([`wire/len.rs`](puroro-rt/src/fields/wire/len.rs)) | **Done** |
| `Fixed32ProtoType` | I32 | `ProtoFixed32`, `ProtoSFixed32`, `ProtoFloat` ([`wire/fixed.rs`](puroro-rt/src/fields/wire/fixed.rs)) | **Done** |
| `Fixed64ProtoType` | I64 | `ProtoFixed64`, `ProtoSFixed64`, `ProtoDouble` ([`wire/fixed.rs`](puroro-rt/src/fields/wire/fixed.rs)) | **Done** |

Float [`ProtoEmpty`](puroro-rt/src/fields/shared.rs) uses Rust `== 0.0` (`-0.0` is empty; `NaN` is non-empty).

Rust payload type alone does **not** identify protobuf encoding (`i32` can be int32 or sint32). Distinct thin wrappers (`ProtoInt32` vs `ProtoSint32`) are the source of truth.

---

## 6. Presence policy

[`FieldPresence`](puroro-rt/src/fields/shared/field_presence.rs) — ZST markers composed into singular wrappers as type param `P`:

| Method / GAT | Role |
|---|---|
| `ValueSlot<T>` | `T` for `Implicit` / `Oneof`; `MaybeUninit<T>` for `Explicit` / `LegacyRequired` |
| `slot_init_mut` / `slot_init_view` | Borrow-free init markers for [`ValueSlot`](puroro-rt/src/fields/shared/value_slot.rs) |
| `should_emit(common, is_payload_empty)` | Encode omit rule |
| `is_set(common, is_payload_empty)` | `Optional::is_set` / presence checks |
| `payload_is_empty(slot)` | Empty / type-zero check via `ProtoEmpty` (`Implicit` only) |

| Marker | Encode | Slot init | Bitfield | Accessors |
|---|---|---|---|---|
| `Implicit` | Omit when payload empty / type-zero | Always initialized | No-op | `value()` |
| `Explicit<BIT>` | Omit when bit unset | Lazy via `with_mut(...).get_mut()` | Set on `set` / `merge` / `value_mut` | `optional`, `clear` |
| `LegacyRequired<BIT>` | Same as `Explicit` | Same as `Explicit` | Same as `Explicit` | Same + `validate_required` |
| `Oneof` | Always emit when variant active | Always initialized | No-op (slot tracks presence) | `value()` / `value_mut()` |

[`RequiredFieldPresence`](puroro-rt/src/fields/shared/field_presence.rs) adds `validate_present` for `LegacyRequired`.

`BIT` is a **const generic on the marker type** (`Explicit<3>`, `LegacyRequired<1>`, …), not a runtime parameter to `bind`.

---

## 7. Field wrappers

**“Singular” means non-repeated** — both presence-tracked (“optional” / `EXPLICIT`) and non-presence-tracked (`IMPLICIT`) fields. It is *not* limited to proto `optional`. Cardinality is singular vs repeated; presence is a separate axis (`FieldPresence`).

Varint and LEN singular scalars share one wrapper, parametrised by [`ProtoType`](puroro-rt/src/fields/wire/proto_type.rs) `T`, presence `P`, and allocator `A`. `FIELD: u32` is a **struct** const generic; `BIT` lives on `Explicit<BIT>` / `LegacyRequired<BIT>`. Markers (`ProtoString`, `ProtoInt32`, …) are allocator-free; `A` sits on the field wrapper. Nested messages use the same wrapper: `SingularField<ProtoMessage<M>, Message|Oneof, FIELD, A>` (typically `M = Address<A>`).

| Wrapper | Module | Params | Aliases (ergonomics) |
|---|---|---|---|
| `SingularField<T, P, FIELD, A, L, D>` | [`singular/field.rs`](puroro-rt/src/fields/singular/field.rs) | `T: ProtoType`, `L: ValueLayout<T, A>` (default `Inline`), stores `P::ValueSlot<T::Slot<A>>` | — |
| `SingularField<ProtoBool, P, FIELD, A, BitPacked<VALUE_BIT>>` | same | `Slot = ()`; value at `VALUE_BIT` via layout | — |
| `SingularField<ProtoMessage<M>, P, FIELD, A>` | same | `Slot = UnmanagedBox<M, A>`; `Message` → `Option`; `Oneof` → always-present | — |
| `RepeatedField<T, E, FIELD, A>` | [`repeated/field.rs`](puroro-rt/src/fields/repeated/field.rs) | `T: RepeatedElement`, `E: RepeatedEncoding<T, A>`, stores `T::Element<A>` | — |
| `MapField<K, V, FIELD, A>` | [`map/field.rs`](puroro-rt/src/fields/map/field.rs) | `K: MapKey`, `V: RepeatedElement`, stores `HashMap<K::Element, V::Element, A>` | — |
| `OneofSlot<E>` | [`oneof.rs`](puroro-rt/src/fields/oneof.rs) | mutually exclusive variants | — |

**Closed enum:** `SingularField<ProtoEnum<E, Closed>, Explicit, FIELD>::bind(…).merge(…)` — `decode` yields `UnknownClosedEnum { raw }`, which `merge` diverts → `unknown_fields`, bit not set.

**LEGACY_REQUIRED:** `SingularField<…, LegacyRequired<BIT>, FIELD>::validate_required`.

Adding a singular wire type = one new `ProtoType` impl (and usually a `VarintProtoType` / fixed helper). Adding a presence mode = one new `FieldPresence` impl.

---

## 8. Proto field → catalog mapping

| Proto field | Generated member type |
|---|---|
| `IMPLICIT int32` | `SingularField<ProtoInt32, Implicit, FIELD, A>` |
| `EXPLICIT int32` | `SingularField<ProtoInt32, Explicit<BIT>, FIELD, A>` |
| `EXPLICIT int32` + `[default = N]` | `SingularField<ProtoInt32, Explicit<BIT>, FIELD, A, Inline, D>` (`D: HasDefault`) |
| `IMPLICIT sint32` | `SingularField<ProtoSint32, Implicit, FIELD, A>` |
| `IMPLICIT bool` | `SingularField<ProtoBool, Implicit, FIELD, A, BitPacked<VALUE_BIT>>` |
| `EXPLICIT bool` | `SingularField<ProtoBool, Explicit<PRESENCE_BIT>, FIELD, A, BitPacked<VALUE_BIT>>` |
| `LEGACY_REQUIRED bool` | `SingularField<ProtoBool, LegacyRequired<PRESENCE_BIT>, FIELD, A, BitPacked<VALUE_BIT>>` |
| oneof `bool` | `SingularField<ProtoBool, Oneof, FIELD, A, BitPacked<VALUE_BIT>>` inside the oneof storage enum |
| `IMPLICIT open enum` | `SingularField<ProtoEnum<E, Open>, Implicit, FIELD, A>` |
| `EXPLICIT closed enum` | `SingularField<ProtoEnum<E, Closed>, Explicit<BIT>, FIELD, A>` (same `.merge`) |
| `IMPLICIT string` | `SingularField<ProtoString, Implicit, FIELD, A>` |
| `EXPLICIT string` | `SingularField<ProtoString, Explicit<BIT>, FIELD, A>` |
| `LEGACY_REQUIRED string` | `SingularField<ProtoString, LegacyRequired<BIT>, FIELD, A>` |
| `IMPLICIT` / `EXPLICIT bytes` | `SingularField<ProtoBytes, P, FIELD, A>` |
| `EXPLICIT fixed32` | `SingularField<ProtoFixed32, Explicit<BIT>, FIELD, A>` |
| `EXPLICIT float` / `double` | `SingularField<ProtoFloat, …, A>` / `SingularField<ProtoDouble, …, A>` |
| `repeated int32 PACKED` | `RepeatedField<ProtoInt32, Packed, FIELD, A>` |
| `repeated fixed32` / `double` PACKED | `RepeatedField<ProtoFixed32, Packed, FIELD, A>` / `RepeatedField<ProtoDouble, Packed, FIELD, A>` |
| `repeated int32 EXPANDED` | `RepeatedField<ProtoInt32, Expanded, FIELD, A>` |
| `repeated string` | `RepeatedField<ProtoString, Expanded, FIELD, A>` |
| `repeated bytes` | `RepeatedField<ProtoBytes, Expanded, FIELD, A>` |
| `map<string, int32>` | `MapField<ProtoString, ProtoInt32, FIELD, A>` |
| `map<int32, Address>` | `MapField<ProtoInt32, ProtoMessage<Address<A>>, FIELD, A>` |
| nested message | `SingularField<ProtoMessage<M>, Message, FIELD, A>` (`M` carries `A`, e.g. `Address<A>`) |
| `oneof` | `OneofSlot<E>` — not a singular catalog entry |

Full singular signature: `SingularField<T, P, FIELD, A, L = Inline, D = ProtoDefault>`.

---

## Part III — Generated message

## 9. Struct layout

```rust
pub struct Task<A: Allocator + Clone = Global> {
    _common: MessageCommon<BitArray<[u8; 2], Lsb0>, A>,
    title: SingularField<ProtoString, Explicit<{ BIT_TITLE }>, { FIELD_TITLE }, A>,
    score: SingularField<ProtoInt32, Implicit, { FIELD_SCORE }, A>,
    max_retries: SingularField<
        ProtoInt32,
        Explicit<{ BIT_MAX_RETRIES }>,
        { FIELD_MAX_RETRIES },
        A,
        Inline,
        MaxRetriesDefault,
    >,
    owner_id: SingularField<ProtoString, LegacyRequired<{ BIT_OWNER_ID }>, { FIELD_OWNER_ID }, A>,
    payload: SingularField<ProtoBytes, Explicit<{ BIT_PAYLOAD }>, { FIELD_PAYLOAD }, A>,
    tag_ids: RepeatedField<ProtoInt32, Packed, { FIELD_TAG_IDS }, A>,
    scores: RepeatedField<ProtoInt32, Expanded, { FIELD_SCORES }, A>,
    labels: RepeatedField<ProtoString, Expanded, { FIELD_LABELS }, A>,
    status: SingularField<ProtoEnum<Status, Open>, Implicit, { FIELD_STATUS }, A>,
    priority: SingularField<ProtoEnum<Priority, Closed>, Explicit<{ BIT_PRIORITY }>, { FIELD_PRIORITY }, A>,
    assignee: SingularField<ProtoMessage<Address<A>>, Message, { FIELD_ASSIGNEE }, A>,
    notification: OneofSlot<NotificationStorage<A>>,
    done: SingularField<ProtoBool, Implicit, { FIELD_DONE }, A, BitPacked<{ BIT_DONE_VALUE }>>,
    flag: SingularField<ProtoBool, Explicit<{ BIT_FLAG }>, { FIELD_FLAG }, A, BitPacked<{ BIT_FLAG_VALUE }>>,
    watchers: RepeatedField<ProtoMessage<Address<A>>, Expanded, { FIELD_WATCHERS }, A>,
    votes: RepeatedField<ProtoBool, Packed, { FIELD_VOTES }, A>,
    attributes: MapField<ProtoString, ProtoInt32, { FIELD_ATTRIBUTES }, A>,
}
```

The `A: Allocator + Clone` struct bound is what lets the generated `Drop` clone the allocator into nested children and free every field from one place. Heap payloads use `unmanaged` types parameterized by `A` plus `PhantomData<A>` (no allocator *instance* in the field, except maps — see below). Protobuf markers are allocator-free; field wrappers carry `A` so slots/`DefaultIn<A>` / `DeallocateIn<A>` associate against `MessageCommon<P, A>`. Singular `bool` uses `SingularField<ProtoBool, …, A, BitPacked<VALUE_BIT>>` with ZST `Slot = ()`; the value lives in `_common.presence`.

**Owned `A` instances:** `_common.alloc`, plus one embedded `A` per `MapField` (`hashbrown::HashMap` owns its allocator).

### Storage summary

| Field kind | Inside catalog wrapper | Presence / value bits |
|---|---|---|
| IMPLICIT varint / open enum / LEN | `ManuallyDrop<T>` (always initialized) | — |
| EXPLICIT / LEGACY_REQUIRED scalar or LEN | `ManuallyDrop<MaybeUninit<T>>` | presence bit in `_common.presence` |
| Singular / oneof `bool` | `SingularField` + `ProtoBool` + `BitPacked<VALUE_BIT>` (`Slot = ()`) | value bit (+ presence bit for EXPLICIT / LEGACY_REQUIRED) in `_common.presence` |
| Repeated | `RepeatedField<T, E, FIELD, A>` (`UnmanagedVec<T::Element<A>>`) | empty = absent |
| Map | `MapField<K, V, FIELD, A>` (`HashMap` of elements, owns `A`) | empty = absent |
| Nested message | `Option<UnmanagedBox<M, A>>` (`Message`) | `Option`, not bitfield |
| Oneof (non-bool) | `Option<E>` in slot | `Option`, not bitfield |

Unset EXPLICIT slots are uninitialized (`MaybeUninit`); **only the bit** means "set". LEN storage is `UnmanagedString` / `UnmanagedVec<u8>`.

Public accessors are **one-line delegates** into catalog methods with `&self._common` / `&mut self._common`. `Message` (encode / merge / unknown / validate) sums the same delegates; the generated `Drop` walks heap fields calling `deallocate(&self._common)` (or oneof `clear`).

### Codegen emission per message

1. Struct — `MessageCommon<BitArray<[u8; N], Lsb0>, A>` + catalog members + `OneofSlot` per oneof
2. **Module-level** `pub const FIELD_*` / `BIT_*` (usable as `match` patterns). `BIT_*` is baked into each `Explicit<BIT>` / `LegacyRequired<BIT>` / `BitPacked<BIT>` field type; `FIELD` is a struct const generic on the wrapper.
3. Inherent accessor delegates ([DESIGN.md §4.0](DESIGN.md#40-inherent-accessors-current))
4. Trait impls — `Message`, `Clone` / `PartialEq` / `Debug` / `Drop` / `DeallocateIn` as field sums
5. Child modules — nested types and oneof submodules named after the oneof (under the message module; package module tree is outer — see below)

IR step: `ProtoField → FieldKind → catalog type + const args`.

**Module tree.** Production layout follows [DESIGN.md §4 — Module layout and naming](DESIGN.md#module-layout-and-naming): `package` → nested Rust modules; each top-level message gets a snake_case submodule; oneofs get snake_case submodules under the parent message. Distinct proto identities that map to the same Rust path are **merged** into one module; item-level clashes are left to `rustc`. Deliberate path changes use a **generate-time rename** (plugin option / config — not a `.proto` option). Cross-forest references use `self::_root::…` ([Path qualification](DESIGN.md#path-qualification)). [`sample-generated/`](sample-generated/) remains flat (no package prefix) as a readable stand-in.

### Path qualification (naming)

Generated code must not rely on ambient `use` imports for the items it references. A `.proto` schema can introduce almost any identifier, so short names risk colliding with user code in the same scope.

**External crates** use leading-`::` absolute paths — `::puroro::Message`, `::puroro_rt::SingularField`, `::core::ops::DerefMut`, `::allocator_api2::alloc::Allocator`, … — and must not be pulled in with `use`.

**Names inside the generated module forest** use `self::_root::…` (e.g. `self::_root::example::v1::address::Address`). They must **not** use leading `::` or `crate::`, because the forest may be embedded as a submodule of an application crate; those prefixes would resolve to the **host** crate root. Layout injects a private `mod _root` into every forest module so `self::_root` means the forest root at any depth:

```rust
// Forest root
mod _root { pub(super) use super::*; }

// Nested modules
mod _root { pub(super) use super::super::_root::*; }
```

Nearby relatives in the same parent (e.g. `pub use empty::Empty`) may stay relative. Generator-reserved `_`-prefixed names (`_root`, `_common`, …) are exempt from proto-derived naming.

Normative wording: [DESIGN.md — Path qualification](DESIGN.md#path-qualification).

**Crate split.** Items from [DESIGN.md §3](DESIGN.md#3-runtime-trait-api) (`Message`, `Optional`, `HasDefault`, `DecodeError`, …) are emitted as `::puroro::…`. Field catalog types, `MessageCommon`, wire helpers, and `ProtoDefault` are emitted as `::puroro_rt::…`. A generated crate's `Cargo.toml` lists both dependencies; end-user application code should not add `puroro-rt` directly.

**Public signatures must not surface `puroro-rt`.** Fully-qualified `::puroro_rt::…` paths are fine in **private** / `pub(crate)` storage and `impl` bodies. They must **not** appear in public function signatures, public type aliases, or other API that forces library users to name `puroro-rt` (use `puroro` traits, RPIT, or concrete user-facing types instead). Normative rule: [DESIGN.md §4 — Public signatures must not surface `puroro-rt`](DESIGN.md#public-signatures-must-not-surface-puroro-rt).

**The checked-in [`sample-generated/`](sample-generated/) deliberately relaxes path qualification for readability.** It pulls names in with `use` and refers to them by short name (`SingularField`, `Allocator`, `MessageCommon`, …) so the reference output stays easy to read and review. Read those short names as stand-ins for the production spellings (`::puroro_rt::…` / `self::_root::…`). The sample still aims to obey the **no public `puroro-rt` in signatures** rule above.

### Generated code comments

Generated Rust is not meant to be hand-edited, but **must be easy to navigate when debugging** (breakpoints, `merge_from` dispatch, diffing encode output). Production plugin output must emit comments from proto metadata; [`sample-generated/`](sample-generated/) demonstrates the convention (the live emitter currently only covers field-less messages and does not yet emit the full comment set).

**File header** — every generated module carries a machine marker and the source message:

```rust
//! @generated from example.proto — do not edit
//! Message `example.Task`
```

The `@generated` marker belongs on **real** plugin output (tooling uses it to collapse/skip generated files). The checked-in [`sample-generated/`](sample-generated/) intentionally **omits** it — those files are a hand-maintained reference, and an `@generated`/`do not edit` banner there would wrongly imply they are tool-generated. Sample headers instead describe what the module illustrates in plain prose.

**Section banners** — major blocks inside the file:

```text
// ---------------------------------------------------------------------------
// Bit indices — …
// ---------------------------------------------------------------------------
// Proto field numbers
// ---------------------------------------------------------------------------

pub const FIELD_TITLE: u32 = 1;   // title
pub const BIT_TITLE: usize = 0;   // title (EXPLICIT)

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
title: SingularField<ProtoString, Explicit<{ BIT_TITLE }>, { FIELD_TITLE }, A>, // proto: string title = 1;
```

**Constants** — **module-level** `pub const` (not associated constants). Module-level consts are valid `match` patterns; associated consts are not. Inside the message module, arms use the bare name (`FIELD_TITLE => …`).

```rust
pub const FIELD_TITLE: u32 = 1;   // title
pub const BIT_TITLE: usize = 0;   // title (EXPLICIT)
```

**Wire I/O** — `merge_from_with_depth` match arms label the proto field:

```rust
FIELD_TITLE => { // title = 1, EXPLICIT string
    self.title
        .bind_mut(&mut self._common)
        .merge(wire_type, buf, depth)?;
}
```

Every field kind merges through the same bound-view shape — `self.<field>.bind_mut(&mut self._common).merge(wire_type, buf, depth)?` (repeated and nested-message fields likewise; oneof uses `OneofSlotMut`) — so the code generator emits one form. Oneof variant arms use the **variant field name** and number. The `_ =>` unknown-field arm gets a short comment (`// unknown field — preserve in _common`).

**What not to comment** — avoid restating obvious one-line delegates (`title().is_set()`). Section + struct + dispatch comments are enough.

**Proto doc comments** — when the `.proto` field has `///` documentation, emit a Rust `///` doc comment on the **public accessor methods** (not on private struct fields unless the proto doc is part of the public API story).

---

## 10. Presence / bool-value bit indices

Tracked bits use [`bitvec::BitArray`](https://docs.rs/bitvec) inline in the message (`BitVec` is heap-only and incompatible with custom `A`). Store them in `_common.presence` as `BitArray<[u8; N], Lsb0>`; `puroro-rt` implements [`PresenceBits`](puroro-rt/src/fields/shared.rs) for that type.

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

### `Address` — four bits → `BitArray<[u8; 1], Lsb0>`

| Field | # | `BIT_*` |
|---|---|---|
| `street` | 1 | `0` |
| `city` | 2 | `1` |
| `postal_code` | 3 | `2` |
| `latitude` | 4 | `3` |

Generated code indexes bits only through `PresenceBits` / `MessageCommon` helpers (`is_bit_set` / `set_bit` / `bit_mut`), not by reaching into raw `BitArray` APIs from accessors. `N` is `ceil(bit_count / 8)` for the message's assigned bits.

---

## 11. Constructors & allocator

- **`Task::new_in(alloc)`** — default every field; `_common.presence = BitArray::ZERO`; heap fields via `*_in(alloc.clone())`, with the last heap field taking the original by move. (Building an empty `unmanaged` container does not allocate, so the clone is only used to decompose an empty `Vec`.)
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
4. Singular: last wins. Repeated: append. Map: insert entry (last-wins on key). Nested: merge sub-buffer.

```rust
FIELD_PRIORITY => self
    .priority
    .bind_mut(&mut self._common)
    .merge(wire_type, buf, depth)?,
```

Nested LEN payloads use `Buf::take(len)` before child `merge_from`.

### Validation

`Message::validate()` — `owner_id.validate_required(&self._common)?` (and any other `LegacyRequired` fields); messages with none return `Ok(())`. `Message::decode` does **not** auto-validate.

---

## 13. Derived traits

**Messages** (`Task<A>`, …): generated as below. **Scalar enums** (`Status`, `Priority`): `#[repr(transparent)]` newtypes over `i32` with associated constants (not Rust enums — proto value aliases may share an integer); `derive(Clone, Copy, Debug, PartialEq, Eq, Hash)`. **Oneof types**: the payload-less `NotificationCase` `derive`s `Clone, Copy, Debug, PartialEq, Eq`. The shared shape `Notification<…>` `derive`s `Clone, Copy, PartialEq` (available when all payload params satisfy the bounds — e.g. the concrete shared Ref shape). Group bound views are rt `OneofView` / `OneofViewMut` (no derives). There are no public Ref/Mut aliases; `OneofGroup::{Ref,Mut}` are inline. The internal `NotificationStorage` alias needs no trait derives (comparison/formatting happen on the safe views). Pattern matching uses `Notification::…` on shared projections.

| Trait | Bounds | Notes |
|---|---|---|
| `Default` | `A: Clone + Default` | Clears presence; empty heap fields |
| `Drop` | `A: Clone` | Calls `deallocate(&_common)` on every direct child ([`FieldDeallocate`](puroro-rt/src/fields/shared/field_deallocate.rs)), then `_common.deallocate()` |
| `Clone` / `CloneIn` | `A: Clone` | Field-wise `field.clone_in(&common, alloc)`; `Clone` clones `MessageCommon.alloc` and delegates |
| `PartialEq` | `A: Clone` | Semantic getter comparison (not wire bytes); float uses Rust `PartialEq` |
| `Debug` | `A: Clone` | Field-name `debug_struct` (oneof shown as `notification` → `case`) |

**Not currently generated:** `Eq`, `Copy`, `Ord`, `Hash`.

**`Global` extras:** `Task::new()`, `impl Default for Task` when `A: Default`, `Message::decode(buf)` when `Self: Default`.

Prefer `Arc<Task<A>>` for shared immutable messages when clone cost matters.

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

**Catalog:** [`RepeatedField<T, E, FIELD>`](puroro-rt/src/fields/repeated/field.rs) where `T: RepeatedElement` and `E: RepeatedEncoding<T>` ([`Packed`](puroro-rt/src/fields/repeated/encoding.rs) / [`Expanded`](puroro-rt/src/fields/repeated/encoding.rs)). Empty vec = absent on encode. Storage is `ManuallyDrop<UnmanagedVec<T::Element, T::Alloc>>`.

| | Packed (`E = Packed`, packable `T` only) | Expanded (`E = Expanded`) |
|---|---|---|
| Encode | One LEN record (varint payload) | One tagged record per element |
| Decode | Both packed + expanded (packable) | Per-element (string / bytes / message / …) |

Mutation uses the bound-view idiom: `field.bind_mut(&mut common)` → [`RepeatedFieldMut`](puroro-rt/src/fields/repeated/field.rs). Markers with [`RepeatedVecMut`](puroro-rt/src/fields/wire/repeated_element.rs) (copy scalars / enums / messages) expose `values_mut()` → `Vec` guard; string / bytes use `push_in` via [`RepeatedSlicePush`](puroro-rt/src/fields/wire/repeated_element.rs) (allocator-less element storage is impractical to build through bare `DerefMut`). `clear` / `deallocate` drain and free heap elements first, then free the buffer. Read: `field.bind(&common)` → [`RepeatedFieldRef`](puroro-rt/src/fields/repeated/field.rs) (`as_slice` / `is_empty`). Merge requires `T: RepeatedElementMerge<A>`.

**`repeated message`:** `RepeatedField<ProtoMessage<M>, Expanded, FIELD, A>` with `Element = M` (no per-element `UnmanagedBox`). Each wire occurrence constructs a new `M` via `Message::new_in` and appends — it does **not** merge into an existing list index. Sample: `Task.watchers` (`repeated Address`).

**`repeated bool`:** `RepeatedField<ProtoBool, Packed|Expanded, FIELD, A>` with plain `bool` elements — **no** [`BitPacked`](puroro-rt/src/fields/shared/value_layout.rs) / MessageCommon bit index (see [Bit-packed bool](#bit-packed-bool-protobool)). Sample: `Task.votes`.

> Note: the bound-view idiom covers every field family — `SingularField`, `RepeatedField`, `MapField`, and `OneofSlot` — on both read and write paths. Terminal `deallocate` stays a direct field method (called from `Drop`).

### 15.1 Map fields

**Catalog:** [`MapField<K, V, FIELD, A>`](puroro-rt/src/fields/map/field.rs) with `K: MapKey`, `V: RepeatedElement`. Storage is [`MapEntries`](puroro-rt/src/fields/map/entries.rs) — a thin `hashbrown::HashMap<K::Element<A>, V::Element<A>, …, A>` that **owns** allocator `A` (unlike `UnmanagedVec` fields). Wire order is unspecified; only the hash map is kept.

**Wire:** each map occurrence is one LEN field `FIELD` whose payload is a synthetic entry message (`key = 1`, `value = 2`). Encode/decode helpers live in [`map/entry.rs`](puroro-rt/src/fields/map/entry.rs). Element tags use `K::encode_element` / `V::encode_element`. Decode uses `RepeatedElementMerge::{decode_element, default_element}` (singular wire types only; packed rejected inside the entry). Missing key/value → type default. Unknown tags inside the entry are skipped via [`skip_field`](puroro-rt/src/decode.rs) (not preserved).

| | Behaviour |
|---|---|
| Encode | One LEN record per map entry (order unspecified) |
| Merge | Decode one entry → `insert` (last-wins; frees replaced value + discarded key) |
| Empty | Absent on the wire |

**Bound views:** `bind` / `bind_mut` → [`MapFieldRef`](puroro-rt/src/fields/map/field.rs) / [`MapFieldMut`](puroro-rt/src/fields/map/field.rs). Mut methods take `&mut self` so a single handle supports multiple ops:

| Method | Role |
|---|---|
| `get` / `get_mut` / `iter` / `len` | Lookup |
| `insert` | Owned key + value elements |
| `insert_in` | `RepeatedSlicePush` keys (`string` / `bytes`) from a slice — like repeated `push_in` |
| `remove` / `clear` | Free key + value via `deallocate_element` |
| `merge` | One wire occurrence |

**Key collision safety:** `HashMap::insert` would drop a colliding incoming key; `MapEntries::insert` keeps the stored key and returns `(incoming_key, previous_value)` for explicit release (required for `UnmanagedString` keys).

Sample: `Task.attributes` — `map<string, int32>` → `MapField<ProtoString, ProtoInt32, { FIELD_ATTRIBUTES }, A>` with accessors `attributes()` / `attributes_mut()` / `clear_attributes()` ([DESIGN.md §4.10](DESIGN.md#410-map-fields)).

---

## 16. Nested messages, oneof, unknown fields

### Nested (`SingularField<ProtoMessage<M>, P, FIELD, A>`)

Same wrapper as other singular fields. Storage is `ManuallyDrop<P::ValueSlot<UnmanagedBox<M, A>>>` with `P: FieldPresence`: [`Message`](puroro-rt/src/fields/shared/field_presence.rs) → `Option<UnmanagedBox<M, A>>`; [`Oneof`](puroro-rt/src/fields/shared/field_presence.rs) → always-present `UnmanagedBox<M, A>` (the slot tracks case presence). Child type bound is `M: Message<Alloc = A>` (empty children via [`Message::new_in`](src/message.rs)). Wire merge / encode / clear live on [`ProtoMessage`](puroro-rt/src/fields/wire/proto_message.rs) / [`ProtoType::merge`](puroro-rt/src/fields/wire/proto_type.rs) (merge-into). Accessors use the usual bound-view idiom (`bind` / `bind_mut` → `get` / `get_mut` / `value` / `merge` / `clear`). Recursion limit: planned ([§17](#17-planned-optimisations--runtime-gaps)).

### Oneof (`OneofSlot<E>`)

Each wire occurrence replaces the whole slot (last wins). Encode active variant only. Decode: one match arm per variant field number.

**Generated types per group.** The owned storage holds allocator-less `unmanaged` values, so it is kept out of the public API. Shape / Storage / Ref / Mut share one generic enum; Storage is a `pub(crate)` alias that implements [`OneofGroup`](puroro-rt/src/fields/oneof.rs). Group bound views are rt [`OneofView`](puroro-rt/src/fields/oneof.rs) / [`OneofViewMut`](puroro-rt/src/fields/oneof.rs) (no per-oneof generated View structs). Message accessors return RPIT `impl OneofGroup<Case = …>` so Storage never appears in signatures.

The sample `oneof notification` is deliberately **heterogeneous** — LEN, VARINT, bool, and message variants — to show the storage kinds:

| Type | Vis | Payloads | Role |
|---|---|---|---|
| `Notification<…>` | `pub` | type params | canonical shape (shared by aliases) |
| `NotificationStorage<A>` | `pub(crate)` | field wrappers | owned storage; `OneofGroup` + `OneofDeallocate`; encode glue |
| `NotificationCase` | `pub` | — | `Copy` discriminant (variants only; unset is `None`) → `notification().case() -> Option<_>` |
| `OneofView` / `OneofViewMut` | rt `pub` | — | group bind (slot + `MessageCommon`) → `notification()` / `notification_mut()` |
| `OneofGroup::Ref` | inline | concrete (`&str`, `i32`, `&Address`, `bool`, …) | projected read → `view.as_ref()` |
| `OneofGroup::Mut` | inline | via [`ProtoType::Mut`](puroro-rt/src/fields/wire/proto_type.rs) | opaque on `notification_mut()` RPIT; typed mut via per-variant `_mut` |

**Ref/Mut payloads are not hard-coded in generated aliases.** They project from each variant's [`ProtoType`](puroro-rt/src/fields/wire/proto_type.rs) marker (`ProtoString`, `ProtoInt32`, `ProtoMessage`, `ProtoBool`, …). Per-variant private field type aliases remain the single source for Storage.

**Variants own field wrappers, not raw storage.** Each variant holds the same field wrapper an ordinary singular field of that kind uses (`SingularField` — including `ProtoBool` + `BitPacked<VALUE_BIT>` for `bool` and `ProtoMessage<M>` for messages), so `value` / `value_mut` / `deallocate` are reused. The wrapper's presence is inert here (`Oneof` / `FieldPresence::Oneof`), so presence-aware omit rules are never consulted; bool still packs its value into `_common.presence`. Markers are allocator-free; unmanaged payloads and field wrappers carry `A` (type only / `PhantomData`); the owned allocator instance stays on the message.

The oneof drives each variant with the **field's own** primitives. Empty construction is [`DefaultIn`](puroro-rt/src/fields/shared.rs) on the variant field wrapper (`SingularField::default_in`). Merging is `slot.bind_mut(common).variant_mut::<FIELD_…>().bind_mut(common).merge(wire, buf)`. Read getters use `slot.bind(common).variant_of::<FIELD_…>().optional()` / `.get()`. [`OneofVariant`](puroro-rt/src/fields/oneof_variant.rs) is keyed by proto field number (no per-variant marker ZSTs).

**The message variant uses `SingularField<ProtoMessage<…>, Oneof, …>` — always-present `UnmanagedBox<M, A>`, not `Option`.**

Parent `_mut` accessors are:
`slot.bind_mut(common).variant_mut::<FIELD_…>().bind_mut(common).value_mut()`.

The storage alias implements [`OneofDeallocate`](puroro-rt/src/fields/oneof.rs) so the previously-active variant is freed through the message allocator before the slot is overwritten. Group accessors use the bound-view idiom: `slot.bind(&common)` / `slot.bind_mut(&mut common)` yield [`OneofSlotRef`](puroro-rt/src/fields/oneof.rs) / [`OneofSlotMut`](puroro-rt/src/fields/oneof.rs). `OneofView` / `OneofViewMut` hold that pair via `OneofGroup`. `OneofSlotMut` consuming methods:

- `variant_mut::<FIELD>() -> &mut OneofVariant::Value` — keeps the active variant if it is already that field number, else frees the previous variant and installs `from_variant(DefaultIn::default_in(alloc.clone()))`. Consumes the slot view so callers can re-borrow `common` and `field.bind_mut(common)` for `.value_mut()` / `.merge(…)`.
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
| Recursion limit | Enforced (`RECURSION_LIMIT = 100`, `merge_from_with_depth`) | — |
| Repeated wrappers | `RepeatedField` + `RepeatedElement` (message / bool / scalar / LEN) | — |
| Map wrappers | `MapField` + `MapKey` / `RepeatedElement` (sample `attributes`) | — |
| `protoc-gen-puroro` field emission | Empty message + module forest; `emit` uses `resolve` | FieldKind → catalog (scalars → nested / enum / repeated / oneof / map) |
| Zero-copy views | — | `TaskView<'buf>` (DESIGN.md §8) |
| `TaskLazy` | DESIGN only | Wire buffer + on-demand decode |
| `Hash` / `serde` | Deferred | Opt-in features |
| Submessage inline | Always `UnmanagedBox<M, A>` for nested messages | Inline small non-repeated messages in the parent struct ([§17.1](#171-submessage-inline-optimisation)) |
| String / Bytes inline | Always heap-allocated LEN payload | Small-string (and maybe bytes) inline storage via union + common-bit tag ([§17.2](#172-string--bytes-inline-optimisation)) |

### 17.1 Submessage inline optimisation

**Idea.** For a **non-repeated** nested message that is small enough (roughly ≤ ~24 bytes of child payload / layout — exact threshold TBD), avoid allocating a separate heap box. Store the child message **inline** as a field of the parent message struct.

**Sharing `MessageCommon`.** The inlined child must share the parent's [`MessageCommon`](#4-shared-infrastructure) (presence bits, allocator, unknown-field buffer) rather than owning its own. Concretely:

1. The message struct type takes the common-field type as a **generic parameter**, bounded by a trait that exposes the bitfield / allocator / unknown buffer the child needs.
2. The current common type gains a method to **scope in** to a particular submessage (e.g. reborrow / view the shared common through the child's bit-index offset or presence layout), so child field accessors keep the same `bind` / `bind_mut(common)` shape.

**Notes / open questions.** Repeated and oneof message variants likely stay boxed (or need a separate design). Codegen must choose heap vs inline per nested type (size / field count). Public accessors (`Option<&M>`, merge-into, etc.) should stay stable — only the storage representation changes.

### 17.2 String / Bytes inline optimisation

**Idea.** Same motivation as §17.1: for a **non-repeated** `string` (and possibly `bytes`) that is short enough, avoid a heap allocation and store the payload **inline** in the field.

**Sketch.** Represent the field storage as a **union** of a heap string/bytes and an inline buffer. Use **one bit in the parent's `MessageCommon` presence bitfield** as a tag indicating which union variant is live (similar in spirit to how [`BitPacked`](puroro-rt/src/fields/shared/value_layout.rs) parks a bool value bit in `_common.presence`).

**Notes / open questions.** Threshold (e.g. SSO-style length that fits in the union without growing the field past a pointer-sized heap handle). Interaction with `utf8_validation`, clear / deallocate, and oneof string variants. Repeated string/bytes stay heap-backed unless a separate design is justified.
