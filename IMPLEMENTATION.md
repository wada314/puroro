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
10. [Common bit indices](#10-common-bit-indices)
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
| **`protoc-gen-puroro`** | `protoc` plugin: decode `CodeGeneratorRequest`, resolve types, generate Rust per [DESIGN.md §0](DESIGN.md#0-project-architecture). |

---

## 2. Architecture overview

Protobuf fields (except **oneof**) are **independent**: each getter/setter/encode/merge arm touches only its own struct member plus shared [`MessageCommon`](#4-shared-infrastructure). The plugin **composes** runtime catalog types — it does not hand-write per-field logic.

### Generator layer stack

`generate()` orchestrates (including plugin `parameter` knobs such as `proto2_utf8`); `emit` only prepares `syn::Item` / `Prepared*` and installs them into the forest.

```
plugin_io          CodeGeneratorRequest / Response (wire)
  │
descriptor         protoc subset (FQN strings, sparse features)
  │
resolved           type graph, occurrence, edition features
  │
plan               per-message catalog shape + bits + member order
  │
prepare (emit)     plan + names → syn::Item + Prepared*
  │
forest + layout    ModuleForest → prettyprinted files
```

### Runtime layer stack

```
protoc plugin
    │  proto field → catalog type + const FIELD / BIT
    │  emits ::puroro::… (traits, Optional, errors)
    │       ::puroro_rt::… (catalog, wire helpers)
    ▼
puroro_rt::fields       SingularField<T, P, FIELD> (T includes ProtoMessage), …
    │  shared/ — MessageCommon, FieldPresence, ValueSlot,
    │            DefaultIn / DeallocateIn / ProtoEmpty
    │  wire/   — WirePayload (wire shape); EncodeType + encode_field;
    │            SingularType; RepeatedElement; MapKey;
    │            Numerical<C> / LenScalar<C> (+ NumericalType / LenCodec)
    │  singular/, repeated/, oneof/
    │  T: SingularType marker (ProtoInt32, ProtoString, … aliases)
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
| One impl per pattern | Wire trait × presence marker × type marker |
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
| `MessageCommon`, `MessageCommonBits`, `MessageCommonAlloc`, `FieldDeallocate`, `OneofSlot` | **Done** |
| `SingularType` + `Numerical` / `LenScalar` markers + `ProtoMessage` | **Done** |
| `NumericalType` / `LenCodec` (codec traits under Method 1 blankets) | **Done** |
| `RepeatedElement` / `RepeatedElementMerge` / `PackableRepeatedElement` / `RepeatedVecMut` | **Done** |
| `MapKey` + `MapField` (map entry wire encode/merge) | **Done** |
| `FieldPresence` (`Implicit` / `Explicit` / `LegacyRequired` / `Oneof`) | **Done** |
| `ValueSlot`, `SlotInitView` / `SlotInitMut`, `DefaultIn` / `DeallocateIn` / `ProtoEmpty` | **Done** |
| `SingularField<T, P, FIELD>`  | **Done** |
| `ProtoBool` + `BitPacked<VALUE_BIT>` (generated default) or `Inline` on `SingularField` | **Done** |
| `ValueLayout` / `Inline` / `Boxed` / `PayloadAccess` | **Done** |
| Closed-enum unknown → `DecodeError::UnknownClosedEnum` → unknown fields, `validate_required` | **Done** |
| Nested message via `SingularField<ProtoMessage<…>, …>` | **Done** |
| [`sample-generated`](sample-generated/) (`Task` / `Address`) | **Done** — hand-written normative eager output |
| `Fixed*` / float / double markers on `SingularField` / `RepeatedField` (via `NumericalType`) | **Done** |
| Repeated catalog (`RepeatedField<T, E, FIELD>`) | **Done** |
| `protoc-gen-puroro` plugin I/O (`CodeGeneratorRequest` / `Response`) | **Done** |
| Descriptor decode (messages / fields / enums / oneofs / features subset) | **Done** (intentional subset; `default_value` and `map_entry` decoded; services / extensions not in IR yet) |
| Type resolve (`FileSet`, `TypeRef`, presence / occurrence) | **Done** — [`generate()`](protoc-gen-puroro/src/generate.rs) resolves the full request, then prepare / forest / layout |
| Editions features in resolve / FieldKind | **Partial** — `field_presence`, `enum_type`, `repeated_field_encoding`, `utf8_validation` resolved and emitted (`ProtoString` / `ProtoStringUnchecked`); `message_encoding=DELIMITED` rejected; JSON / naming / visibility still traps |
| Module forest + `ModuleLayout::SingleFile` | **Done** (`FileTree` deferred) |
| Empty-message emission (no fields / nested types) | **Done** — compile-tested via [`puroro-codegen-tests`](puroro-codegen-tests/) (`protoc` + plugin) |
| FieldKind IR (`plan_fields`, bit assignment, catalog kind) | **Done** — scalars / repeated / enum / oneof / map / custom defaults; synthetic `map_entry` messages planned as `FieldKind::Map` (not emitted as structs) |
| FieldKind → catalog emission (struct members, accessors, visitors) | **Done** — singular + repeated scalar / string / bytes / bool / enum / message; real oneof groups; maps; `[default = …]` markers (`mod defaults` + `SingularField` `D`); nested message/enum decls; zero-less enums (`Type`/`Label`); official `descriptor.proto`+`plugin.proto` compile-tested. Typed extensions / services not yet |

Live plugin emits the eager-path field families shown by [`sample-generated/`](sample-generated/) (`Task` / `Address`) via `resolved::resolve` + [`field_kind::plan_fields`](protoc-gen-puroro/src/field_kind.rs): singular / repeated / enum / message / real oneof / map / custom defaults. Coverage is split across [`puroro-codegen-tests`](puroro-codegen-tests/) fixtures (`scalars`, `oneof_basic`, `map_basic`, `custom_defaults`, `utf8_validation`, …; official `descriptor.proto` / `plugin.proto` by `official_plugin`). Deliberate differences from the hand-written sample — flat module layout, short-name `use`s, omitted `@generated` headers — are documented in [§9](#9-struct-layout); they are not missing field features. Remaining generator gaps outside the sample surface: typed extensions, services, `FileTree` layout.

---

## Part II — Runtime catalog (`puroro_rt::fields`)

### Module layout (`puroro-rt/src/fields/`)

| Path | Contents |
|---|---|
| [`lib.rs`](puroro-rt/src/lib.rs) | Crate-root re-exports: symbols the generator names as `::puroro_rt::…` |
| [`fields.rs`](puroro-rt/src/fields.rs) | Module root (`pub(crate)`; `pub mod` only) |
| [`shared.rs`](puroro-rt/src/fields/shared.rs) | `MessageCommon`, `MessageCommonBits`, `MessageCommonAlloc`, `DefaultIn`, `DeallocateIn`, `ProtoEmpty` |
| [`shared/field_presence.rs`](puroro-rt/src/fields/shared/field_presence.rs) | `FieldPresence` markers |
| [`shared/field_deallocate.rs`](puroro-rt/src/fields/shared/field_deallocate.rs) | `FieldDeallocate` — uniform `deallocate(&common)` |
| [`shared/slot_init.rs`](puroro-rt/src/fields/shared/slot_init.rs) | `SlotInitView` / `SlotInitMut` init-state handles |
| [`wire.rs`](puroro-rt/src/fields/wire.rs) | Wire-family re-exports |
| [`wire/encode_type.rs`](puroro-rt/src/fields/wire/encode_type.rs) | `EncodeType` + `encode_field` / `encoded_len_field` (tagged framing) |
| [`wire/wire_payload.rs`](puroro-rt/src/fields/wire/wire_payload.rs) | `WirePayload` / `CopyWirePayload` (Varint / Fixed32 / Fixed64 / Len bodies) |
| [`wire/singular_type.rs`](puroro-rt/src/fields/wire/singular_type.rs) | `SingularType` marker + `PayloadAccess` (`Slot` / `Mut` / `Written` for inline) |
| [`shared/value_layout.rs`](puroro-rt/src/fields/shared/value_layout.rs) | `ValueLayout` (`Slot` / `Mut`), `Inline`, `Boxed`, `BitPacked`, `InlineOrHeap` |
| [`wire/repeated_element.rs`](puroro-rt/src/fields/wire/repeated_element.rs) | `RepeatedElement` / `RepeatedElementMerge` (`Element` for repeated buffers) |
| [`wire/map_element.rs`](puroro-rt/src/fields/wire/map_element.rs) | `MapKey` (subset of `RepeatedElement`) |
| [`wire/proto_message.rs`](puroro-rt/src/fields/wire/proto_message.rs) | `ProtoMessage` (nested message marker) |
| [`wire/numerical.rs`](puroro-rt/src/fields/wire/numerical.rs) | `Numerical<C>`, `NumericalType` codecs, `ProtoInt32` / … / fixed / `ProtoEnum` + enum kind traits |
| [`wire/len.rs`](puroro-rt/src/fields/wire/len.rs) | `LenScalar<C>`, `LenCodec`, `ProtoString`, `ProtoBytes` |
| [`wire/sso_buf.rs`](puroro-rt/src/fields/wire/sso_buf.rs) | Shared 3-word untagged SSO union (`SsoBuf`) |
| [`wire/sso_string.rs`](puroro-rt/src/fields/wire/sso_string.rs) | `SsoString` / `SsoStringMut` |
| [`wire/sso_bytes.rs`](puroro-rt/src/fields/wire/sso_bytes.rs) | `SsoBytes` / `SsoBytesMut` |
| [`singular.rs`](puroro-rt/src/fields/singular.rs) | Singular field re-exports |
| [`singular/field.rs`](puroro-rt/src/fields/singular/field.rs) | `SingularField` — `T: SingularType`, stores `L::Slot` |
| [`repeated.rs`](puroro-rt/src/fields/repeated.rs) | Repeated field re-exports |
| [`repeated/encoding.rs`](puroro-rt/src/fields/repeated/encoding.rs) | `Packed` / `Expanded` (`RepeatedEncoding`) |
| [`repeated/field.rs`](puroro-rt/src/fields/repeated/field.rs) | `RepeatedField` — `T: RepeatedElement`, stores `T::Element` |
| [`map.rs`](puroro-rt/src/fields/map.rs) | Map field re-exports |
| [`map/entry.rs`](puroro-rt/src/fields/map/entry.rs) | Map-entry wire encode / decode (`key=1`, `value=2`) |
| [`map/field.rs`](puroro-rt/src/fields/map/field.rs) | `MapField` — `K: MapKey`, `V: RepeatedElement`; owns `HashMap` of elements |
| [`oneof.rs`](puroro-rt/src/fields/oneof.rs) | `OneofSlot` |

---

## 4. Shared infrastructure

[`MessageCommon<B, A>`](puroro-rt/src/fields/shared.rs) — one per generated message:

| Member | Role |
|---|---|
| `bits: B` | `BitArray<[u8; N], Lsb0>` **common bits**: EXPLICIT / LEGACY_REQUIRED presence, packed bool values, and string / bytes SSO heap-arm bits |
| `unknown_fields: ManuallyDrop<UnmanagedVec<u8, A>>` | Preserve policy: round-trip unknown wire blob; closed-enum unknowns. Public view via `iter_unknown_fields`. Freed by `MessageCommon::deallocate` |
| `alloc: A` | The single canonical allocator instance; cloned (by value) into every field operation that (de)allocates |

Field catalog methods take `&MessageCommon` / `&mut MessageCommon`, not `&Task`, so wrappers stay decoupled from the parent message type.

[`FieldDeallocate`](puroro-rt/src/fields/shared/field_deallocate.rs) — every catalog field (and [`OneofSlot`](puroro-rt/src/fields/oneof.rs)) implements `deallocate(&mut self, common: &MessageCommon<…>)`. Generated message `Drop` calls this on **each direct child** with the same shape. Copy / bit-packed fields are no-ops. Oneof **variants** are released inside the group's deallocate via [`OneofDeallocate`](puroro-rt/src/fields/oneof.rs) (`deallocate(self, common)`), which forwards to the same field `deallocate(common)`.

[`MessageCommonBits`](puroro-rt/src/fields/shared.rs) / [`MessageCommonAlloc`](puroro-rt/src/fields/shared.rs) — catalog bounds on the common context (not on the bit-storage type). Common bits cover EXPLICIT / LEGACY_REQUIRED **presence**, packed **bool values**, and string / bytes **SSO heap-arm** bits. `MessageCommon` implements both; inherent `is_bit_set` / `set_bit` / `bit_mut` forward to `MessageCommonBits` (`bit_mut` returns bitvec's `BitRef<'_, Mut, u8, Lsb0>`). Generated messages store `BitArray<[u8; N], Lsb0>` in `_common.bits` with no per-message newtype.

[`ValueSlot<T>`](puroro-rt/src/fields/shared/value_slot.rs) — singular **slot** storage behind a GAT on [`FieldPresence`](puroro-rt/src/fields/shared/field_presence.rs): always-initialized `T` for `Implicit` / `Oneof`; `MaybeUninit<T>` for `Explicit` / `LegacyRequired`; `Option<T>` for `Message` (pointer presence). Here `T` is [`ValueLayout::Slot`](puroro-rt/src/fields/shared/value_layout.rs) (bare `i32` / `()`, [`SsoString`](puroro-rt/src/fields/wire/sso_string.rs) / [`SsoBytes`](puroro-rt/src/fields/wire/sso_bytes.rs), heap `UnmanagedString` / `UnmanagedVec`, nested `M` for inlined messages, or `UnmanagedBox<M, A>` for boxed messages). Construction uses [`DefaultIn<A>`](puroro-rt/src/fields/shared.rs). Drop / clone extract a live payload via `take_value` / `get_value` / `from_optional`; [`ValueLayout`](puroro-rt/src/fields/shared/value_layout.rs) frees it (`DeallocateIn` for inline payloads, heap bit + SSO `deallocate` for SSO) and [`ValueLayoutClone`](puroro-rt/src/fields/shared/value_layout.rs) deep-copies it. Reads and mutation go through short-lived views: `slot.with(init, common)` → [`ValueSlotRefAccess`](puroro-rt/src/fields/shared/value_slot.rs) / `slot.with_mut(init, common)` → [`ValueSlotMutAccess`](puroro-rt/src/fields/shared/value_slot.rs) (`get` / `get_mut` / `replace` / `take_clear`). Init markers ([`AlwaysInitialized`](puroro-rt/src/fields/shared/slot_init.rs) / [`BitInit`](puroro-rt/src/fields/shared/slot_init.rs)) are borrow-free; they read/update state through the passed [`MessageCommon`](puroro-rt/src/fields/shared.rs). Slot payloads use [`AddressableSlot`](puroro-rt/src/fields/shared/value_slot.rs); logical bool values are read/written via [`SingularType`](puroro-rt/src/fields/wire/singular_type.rs) against `_common.bits`. Singular IMPLICIT omit goes through [`ValueLayout::is_proto_empty`](puroro-rt/src/fields/shared/value_layout.rs).

[`SingularField::bind`](puroro-rt/src/fields/singular/field.rs) / [`bind_mut`](puroro-rt/src/fields/singular/field.rs) — inherent MessageCommon binding → [`SingularFieldRef`](puroro-rt/src/fields/singular/field.rs) / [`SingularFieldMut`](puroro-rt/src/fields/singular/field.rs). Repeated fields and [`OneofSlot`](puroro-rt/src/fields/oneof.rs) use the same inherent `bind` / `bind_mut` call shape. Getter / `_mut` payload types are [`EncodeType::View`](puroro-rt/src/fields/wire/encode_type.rs) / [`ValueLayout::Mut`](puroro-rt/src/fields/shared/value_layout.rs).

---

## 5. Wire encoding traits

Encode responsibilities are layered (omit stays at the catalog):

```text
Message encode
  → FieldEncode / OneofEncodable   (omit / empty / oneof match)
       → encode_field / encoded_len_field   (tag + untagged wire body)
            → EncodeType                   (proto type → complete wire body)
                 → WirePayload             (wire shape: Varint / Fixed32 / Fixed64 / Len)
```

Packed repeated fields concatenate scalar [`NumericalType`](puroro-rt/src/fields/wire/numerical.rs) /
[`EncodeType`](puroro-rt/src/fields/wire/encode_type.rs) bodies (via
[`PackableRepeatedElement`](puroro-rt/src/fields/wire/repeated_element.rs)), then wrap once with
Len framing in the repeated encoder (not via per-element `Len` `WirePayload`).

One marker + trait per protobuf **type** (e.g. `int32`, `string`). Wire-shape read/write lives on
[`WirePayload`](puroro-rt/src/fields/wire/wire_payload.rs). Semantic conversions delegate to
**`protobuf-core`** (`puroro-rt` does not reimplement zigzag/varint).

### Wire shape ([`wire/wire_payload.rs`](puroro-rt/src/fields/wire/wire_payload.rs))

[`WirePayload`](puroro-rt/src/fields/wire/wire_payload.rs) is the complete **tag-free** body for a
**wire type** (`Varint` / `Int32` / `Int64` / `Len`). `Len` encode (length varint + content) lives on
[`EncodeType`](puroro-rt/src/fields/wire/encode_type.rs) for string / bytes / message; this module
keeps owned [`LenPayload`](puroro-rt/src/fields/wire/wire_payload.rs) for decode. Copy numericals also
implement [`CopyWirePayload`](puroro-rt/src/fields/wire/wire_payload.rs) (`decode` without an
allocator).

### Proto-type encode + tagged framing ([`wire/encode_type.rs`](puroro-rt/src/fields/wire/encode_type.rs))

[`EncodeType`](puroro-rt/src/fields/wire/encode_type.rs) is implemented by proto **type** markers
(`ProtoInt32`, `ProtoString`, `ProtoMessage<M>`, …). `View` is both the singular getter view and the
tagged-encode input, and is always `Copy`. `payload_len` / `encode_payload` emit the complete
untagged wire body (for `Len`: length + content; numericals via [`WirePayload`](puroro-rt/src/fields/wire/wire_payload.rs)).
Free helpers [`encode_field`](puroro-rt/src/fields/wire/encode_type.rs) /
[`encoded_len_field`](puroro-rt/src/fields/wire/encode_type.rs) add only the tag — they do **not**
apply presence omit. [`SingularType`](puroro-rt/src/fields/wire/singular_type.rs) extends
`EncodeType` as a singular-type marker (no storage GATs). Slot / mutator types
live on [`ValueLayout`](puroro-rt/src/fields/shared/value_layout.rs);
[`PayloadAccess`](puroro-rt/src/fields/wire/singular_type.rs) supplies the inline
`Slot` / `Mut` / `Written` that `Inline` aliases.

### Singular field type markers ([`wire/singular_type.rs`](puroro-rt/src/fields/wire/singular_type.rs))

[`SingularType`](puroro-rt/src/fields/wire/singular_type.rs) is the trait consumed by [`SingularField`](puroro-rt/src/fields/singular/field.rs) (including nested messages via [`ProtoMessage`](puroro-rt/src/fields/wire/proto_message.rs)). Markers are **allocator-free**; physical storage / views are GATs over `A`. Singular slots use bare wire values (`i32`, `()`, …) / `SsoString` / `SsoBytes` (singular string / bytes) / `UnmanagedString` (repeated string) / nested `M` ([`Inline`](puroro-rt/src/fields/shared/value_layout.rs)) or `UnmanagedBox<M, A>` ([`Boxed`](puroro-rt/src/fields/shared/value_layout.rs)):

```rust
pub trait SingularType: EncodeType {}
// Getter view = EncodeType::View
// PayloadAccess: Slot / Mut / Written + get / with_mut / write / clear / merge
// ValueLayout<T, A>: Slot / Mut (Inline aliases PayloadAccess; Boxed / BitPacked / InlineOrHeap choose their own)
// SingularField::FieldEncode calls encode_field after FieldPresence::should_emit
// Repeated / map use RepeatedElement: EncodeType (not SingularType)
```

Singular wire decode is **merge-into only** (`PayloadAccess::merge` / `BitPacked::merge`). There is no `SingularType::decode → Written`; nested messages merge into the present child via `Message::merge_from`.

Singular / oneof `bool` uses allocator-free [`ProtoBool`](puroro-rt/src/fields/wire/numerical.rs) plus [`BitPacked<VALUE_BIT>`](puroro-rt/src/fields/shared/value_layout.rs) as the field's [`ValueLayout`](puroro-rt/src/fields/shared/value_layout.rs) (orthogonal to presence `P`). Inline payloads use default `L = Inline` via [`PayloadAccess`](puroro-rt/src/fields/wire/singular_type.rs). Allocator `A` lives on [`SingularField`](puroro-rt/src/fields/singular/field.rs) / [`RepeatedField`](puroro-rt/src/fields/repeated/field.rs). Slot construction uses [`DefaultIn<A>`](puroro-rt/src/fields/shared.rs) (allocator as a **trait parameter**, not an associated type), so bare `i32` / `()` work without slot newtypes. Inline payload teardown uses [`DeallocateIn<A>`](puroro-rt/src/fields/shared.rs); SSO teardown uses the heap bit.

### Repeated elements (`RepeatedElement`)

[`RepeatedElement`](puroro-rt/src/fields/wire/repeated_element.rs) extends [`EncodeType`](puroro-rt/src/fields/wire/encode_type.rs) (not [`SingularType`](puroro-rt/src/fields/wire/singular_type.rs)) with GAT `Element<A>`, [`wire_view`](puroro-rt/src/fields/wire/repeated_element.rs) (`Element` → `View`), and deallocate. Dual-use markers implement both `SingularType` and `RepeatedElement`. Expanded / map-entry tagged encode call `encode_field(T::wire_view(elem), …)`. Decode / merge live on [`RepeatedElementMerge<A>`](puroro-rt/src/fields/wire/repeated_element.rs) so nested messages can require `M: Message<Alloc = A>`. That trait also provides `default_element` / `decode_element` (singular occurrence; packed `Len` rejected) for map-entry interiors. Singular fields store `Slot<A>`; repeated fields store `Element<A>` (not always the same — nested-message repeated uses `Element = M`; singular boxed uses `Slot = UnmanagedBox<M, A>`, inlined singular uses `Slot = M`).

| Marker | `Element<A>` | Packable | Public mutation |
|---|---|---|---|
| Addressable varint / enum / fixed | `NumericalType::NativeType` (`i32`, …) | yes (`PackableRepeatedElement`) | `values_mut` → `RepeatedVecMut` |
| `ProtoString` / `ProtoBytes` | `UnmanagedString<A>` / `UnmanagedVec<u8, A>` | no | `container_mut` → `RepeatedElementsMut` (`push` then fill) |
| `ProtoMessage<M>` | `M` (inline; use site `M::Alloc = A`) | no | `values_mut` → `RepeatedVecMut` |
| `ProtoBool` | `bool` (plain; not `BitPacked`) | yes (`PackableRepeatedElement`) | `values_mut` → `RepeatedVecMut` |

[`MapKey`](puroro-rt/src/fields/wire/map_element.rs) is an empty marker over `RepeatedElement` restricted to valid protobuf map keys (integrals / `bool` / `string`). Map **values** use `RepeatedElement` directly (anything except another map).

### Numerical / LEN Method 1 markers

Public codegen uses aliases such as [`ProtoInt32`](puroro-rt/src/fields/wire/numerical.rs) /
[`ProtoString`](puroro-rt/src/fields/wire/len.rs). Internally these are
[`Numerical`](puroro-rt/src/fields/wire/numerical.rs)`<C>` /
[`LenScalar`](puroro-rt/src/fields/wire/len.rs)`<C>` over codec traits
[`NumericalType`](puroro-rt/src/fields/wire/numerical.rs) /
[`LenCodec`](puroro-rt/src/fields/wire/len.rs) (blankets on the wrappers, not on each alias).
`Numerical` / `LenScalar` / codecs are `pub` for coherence but **not** crate-root re-exported.

[`NumericalType`](puroro-rt/src/fields/wire/numerical.rs) maps `NativeType` ↔
[`CopyWirePayload`](puroro-rt/src/fields/wire/wire_payload.rs) via `to_wire_body` /
`from_wire_body`. `NativeType` is the **logical** copy value for encode/decode and field
get/set — not a storage contract. Inline singular storage (`AddressableSlot`,
`Slot = NativeType` for addressable markers) lives on
[`PayloadAccess`](puroro-rt/src/fields/wire/singular_type.rs); singular
[`ProtoBool`](puroro-rt/src/fields/wire/numerical.rs) uses that path with
[`Inline`](puroro-rt/src/fields/shared/value_layout.rs) (`Slot = bool`) or
[`BitPacked`](puroro-rt/src/fields/shared/value_layout.rs) (`Slot = ()`; generated default).
Tagged encode / packed repeated merge live on blankets over `Numerical<C>` /
`LenScalar<C>` (`EncodeType`, `RepeatedElementMerge`, `PackableRepeatedElement`).

| Helper | Wire | Markers | Status |
|---|---|---|---|
| `WirePayload` | VARINT / I32 / I64 / LEN | [`wire/wire_payload.rs`](puroro-rt/src/fields/wire/wire_payload.rs) | **Done** |
| `Numerical` + `NumericalType` | VARINT / I32 / I64 | `ProtoInt32`, …, bool, enums, fixed ([`wire/numerical.rs`](puroro-rt/src/fields/wire/numerical.rs)) | **Done** |
| `EncodeType` (blanket) | same | via `Numerical` / `LenScalar` | **Done** |
| `LenScalar` + `LenCodec` | LEN | `ProtoString`, `ProtoBytes` ([`wire/len.rs`](puroro-rt/src/fields/wire/len.rs)) | **Done** |
| Fixed (via `Numerical`) | I32 / I64 | `ProtoFixed*` / `ProtoFloat` / `ProtoDouble` | **Done** |

Float [`ProtoEmpty`](puroro-rt/src/fields/shared.rs) uses Rust `== 0.0` (`-0.0` is empty; `NaN` is non-empty).

Rust payload type alone does **not** identify protobuf encoding (`i32` can be int32 or sint32). Distinct type aliases (`ProtoInt32` vs `ProtoSInt32`) are the source of truth.

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

Varint and LEN singular scalars share one wrapper, parametrised by [`SingularType`](puroro-rt/src/fields/wire/singular_type.rs) `T`, presence `P`, and allocator `A`. `FIELD: u32` is a **struct** const generic; `BIT` lives on `Explicit<BIT>` / `LegacyRequired<BIT>`. Markers (`ProtoString`, `ProtoInt32`, …) are allocator-free; `A` sits on the field wrapper. Nested messages use the same wrapper: boxed `SingularField<ProtoMessage<M>, Message|Oneof, FIELD, A, Boxed>` (typically `M = Address<A>`); inlined singular `SingularField<ProtoMessage<M>, Explicit<BIT>, FIELD, A>` (`Slot = M`).

| Wrapper | Module | Params | Aliases (ergonomics) |
|---|---|---|---|
| `SingularField<T, P, FIELD, A, L, D>` | [`singular/field.rs`](puroro-rt/src/fields/singular/field.rs) | `T: SingularType`, `L: ValueLayout<T, A>` (default `Inline`), stores `P::ValueSlot<L::Slot>` | — |
| `SingularField<ProtoBool, P, FIELD, A, BitPacked<VALUE_BIT>>` | same | `Slot = ()`; value at `VALUE_BIT` via layout | — |
| `SingularField<ProtoMessage<M>, Message, FIELD, A, Boxed>` | same | `Slot = UnmanagedBox<M, A>`; `Message` → `Option` | boxed singular |
| `SingularField<ProtoMessage<M>, Explicit<BIT>, FIELD, A>` | same | `Slot = M`; presence bit + `MaybeUninit<M>` | inlined singular |
| `SingularField<ProtoMessage<M>, Oneof, FIELD, A, Boxed>` | same | always-present `UnmanagedBox<M, A>` | oneof message variant |
| `RepeatedField<T, E, FIELD, A>` | [`repeated/field.rs`](puroro-rt/src/fields/repeated/field.rs) | `T: RepeatedElement`, `E: RepeatedEncoding<T, A>`, stores `T::Element<A>` | — |
| `MapField<K, V, FIELD, A>` | [`map/field.rs`](puroro-rt/src/fields/map/field.rs) | `K: MapKey`, `V: RepeatedElement`, stores `HashMap<K::Element, V::Element, A>` | — |
| `OneofSlot<E>` | [`oneof.rs`](puroro-rt/src/fields/oneof.rs) | mutually exclusive variants | — |

**Closed enum:** `SingularField<ProtoEnum<E, Closed>, Explicit, FIELD>::bind(…).merge(…)` — `decode` yields `UnknownClosedEnum { raw }`, which `merge` diverts → `unknown_fields`, bit not set.

**LEGACY_REQUIRED:** `SingularField<…, LegacyRequired<BIT>, FIELD>::validate_required`.

Adding a singular wire type = one new codec + `Numerical` / `LenScalar` alias (blankets already cover `SingularType` / `EncodeType`). Adding a presence mode = one new `FieldPresence` impl.

---

## 8. Proto field → catalog mapping

| Proto field | Generated member type |
|---|---|
| `IMPLICIT int32` | `SingularField<ProtoInt32, Implicit, FIELD, A>` |
| `EXPLICIT int32` | `SingularField<ProtoInt32, Explicit<BIT>, FIELD, A>` |
| `EXPLICIT int32` + `[default = N]` | `SingularField<ProtoInt32, Explicit<BIT>, FIELD, A, Inline, D>` (`D: HasDefault`) |
| `IMPLICIT sint32` | `SingularField<ProtoSInt32, Implicit, FIELD, A>` |
| `IMPLICIT bool` | `SingularField<ProtoBool, Implicit, FIELD, A, BitPacked<VALUE_BIT>>` |
| `EXPLICIT bool` | `SingularField<ProtoBool, Explicit<PRESENCE_BIT>, FIELD, A, BitPacked<VALUE_BIT>>` |
| `LEGACY_REQUIRED bool` | `SingularField<ProtoBool, LegacyRequired<PRESENCE_BIT>, FIELD, A, BitPacked<VALUE_BIT>>` |
| oneof `bool` | `SingularField<ProtoBool, Oneof, FIELD, A, BitPacked<VALUE_BIT>>` inside the oneof storage enum |
| `IMPLICIT open enum` | `SingularField<ProtoEnum<E, Open>, Implicit, FIELD, A>` |
| `EXPLICIT closed enum` | `SingularField<ProtoEnum<E, Closed>, Explicit<BIT>, FIELD, A>` (same `.merge`) |
| `IMPLICIT string` | `SingularField<ProtoString, Implicit, FIELD, A, InlineOrHeap<SSO_BIT>>` (`VERIFY`) |
| `IMPLICIT string` + `utf8_validation=NONE` | `SingularField<ProtoStringUnchecked, Implicit, FIELD, A, InlineOrHeap<SSO_BIT>>` (bytes views / `BytesMut`) |
| `EXPLICIT string` | `SingularField<ProtoString, Explicit<BIT>, FIELD, A, InlineOrHeap<SSO_BIT>>` |
| `LEGACY_REQUIRED string` | `SingularField<ProtoString, LegacyRequired<BIT>, FIELD, A, InlineOrHeap<SSO_BIT>>` |
| `IMPLICIT` / `EXPLICIT bytes` | `SingularField<ProtoBytes, P, FIELD, A, InlineOrHeap<SSO_BIT>>` |
| `EXPLICIT fixed32` | `SingularField<ProtoFixed32, Explicit<BIT>, FIELD, A>` |
| `EXPLICIT float` / `double` | `SingularField<ProtoFloat, …, A>` / `SingularField<ProtoDouble, …, A>` |
| `repeated int32 PACKED` | `RepeatedField<ProtoInt32, Packed, FIELD, A>` |
| `repeated fixed32` / `double` PACKED | `RepeatedField<ProtoFixed32, Packed, FIELD, A>` / `RepeatedField<ProtoDouble, Packed, FIELD, A>` |
| `repeated int32 EXPANDED` | `RepeatedField<ProtoInt32, Expanded, FIELD, A>` |
| `repeated string` | `RepeatedField<ProtoString, Expanded, FIELD, A>` (`VERIFY`; `NONE` uses `ProtoStringUnchecked` + `RepeatedBytesMut`) |
| `repeated bytes` | `RepeatedField<ProtoBytes, Expanded, FIELD, A>` |
| `map<string, int32>` | `MapField<ProtoString, ProtoInt32, FIELD, A>` |
| `map<int32, Address>` | `MapField<ProtoInt32, ProtoMessage<Address<A>>, FIELD, A>` |
| nested message (boxed) | `SingularField<ProtoMessage<M>, Message, FIELD, A, Boxed>` (`M` carries `A`, e.g. `Address<A>`) |
| nested message (inlined) | `SingularField<ProtoMessage<M>, Explicit<BIT>, FIELD, A>` (`Slot = M`; sample `Task.origin`) |
| `oneof` | `OneofSlot<E>` — not a singular catalog entry |

Full singular signature: `SingularField<T, P, FIELD, A, L = Inline, D = ProtoDefault>`.

---

## Part III — Generated message

## 9. Struct layout

```rust
pub struct Task<A: Allocator = Global> {
    _common: MessageCommon<BitArray<[u8; 2], Lsb0>, A>,
    title: SingularField<ProtoString, Explicit<{ BIT_TITLE }>, { FIELD_TITLE }, A, InlineOrHeap<{ BIT_TITLE_SSO }>>,
    score: SingularField<ProtoInt32, Implicit, { FIELD_SCORE }, A>,
    max_retries: SingularField<
        ProtoInt32,
        Explicit<{ BIT_MAX_RETRIES }>,
        { FIELD_MAX_RETRIES },
        A,
        Inline,
        MaxRetriesDefault,
    >,
    owner_id: SingularField<ProtoString, LegacyRequired<{ BIT_OWNER_ID }>, { FIELD_OWNER_ID }, A, InlineOrHeap<{ BIT_OWNER_ID_SSO }>>,
    payload: SingularField<ProtoBytes, Explicit<{ BIT_PAYLOAD }>, { FIELD_PAYLOAD }, A, InlineOrHeap<{ BIT_PAYLOAD_SSO }>>,
    tag_ids: RepeatedField<ProtoInt32, Packed, { FIELD_TAG_IDS }, A>,
    scores: RepeatedField<ProtoInt32, Expanded, { FIELD_SCORES }, A>,
    labels: RepeatedField<ProtoString, Expanded, { FIELD_LABELS }, A>,
    status: SingularField<ProtoEnum<Status, Open>, Implicit, { FIELD_STATUS }, A>,
    priority: SingularField<ProtoEnum<Priority, Closed>, Explicit<{ BIT_PRIORITY }>, { FIELD_PRIORITY }, A>,
    assignee: SingularField<ProtoMessage<Address<A>>, Message, { FIELD_ASSIGNEE }, A, Boxed>,
    origin: SingularField<ProtoMessage<Point<A>>, Explicit<{ BIT_ORIGIN }>, { FIELD_ORIGIN }, A>,
    notification: OneofSlot<NotificationStorage<A>>,
    done: SingularField<ProtoBool, Implicit, { FIELD_DONE }, A, BitPacked<{ BIT_DONE_VALUE }>>,
    flag: SingularField<ProtoBool, Explicit<{ BIT_FLAG }>, { FIELD_FLAG }, A, BitPacked<{ BIT_FLAG_VALUE }>>,
    watchers: RepeatedField<ProtoMessage<Address<A>>, Expanded, { FIELD_WATCHERS }, A>,
    votes: RepeatedField<ProtoBool, Packed, { FIELD_VOTES }, A>,
    attributes: MapField<ProtoString, ProtoInt32, { FIELD_ATTRIBUTES }, A>,
}
```

The `A: Allocator + Clone` struct bound is what lets the generated `Drop` clone the allocator into nested children and free every field from one place. Heap payloads use `unmanaged` types parameterized by `A` plus `PhantomData<A>` (no allocator *instance* in the field, except maps — see below). Protobuf markers are allocator-free; field wrappers carry `A` so slots/`DefaultIn<A>` / `DeallocateIn<A>` associate against `MessageCommon<B, A>`. Singular `bool` uses `SingularField<ProtoBool, …, A, BitPacked<VALUE_BIT>>` with ZST `Slot = ()`; the value lives in `_common.bits`.

**Owned `A` instances:** `_common.alloc`, plus one embedded `A` per `MapField` (`hashbrown::HashMap` owns its allocator).

### Storage summary

| Field kind | Inside catalog wrapper | Presence / value bits |
|---|---|---|
| IMPLICIT varint / open enum / LEN | `ManuallyDrop<T>` (always initialized) | — |
| EXPLICIT / LEGACY_REQUIRED scalar or LEN | `ManuallyDrop<MaybeUninit<T>>` | presence bit in `_common.bits` |
| Singular / oneof `bool` | `SingularField` + `ProtoBool` + `BitPacked<VALUE_BIT>` (`Slot = ()`) | value bit (+ presence bit for EXPLICIT / LEGACY_REQUIRED) in `_common.bits` |
| Repeated | `RepeatedField<T, E, FIELD, A>` (`UnmanagedVec<T::Element<A>>`) | empty = absent |
| Map | `MapField<K, V, FIELD, A>` (`HashMap` of elements, owns `A`) | empty = absent |
| Nested message (boxed) | `Option<UnmanagedBox<M, A>>` (`Message` + `Boxed`) | `Option`, not bitfield |
| Nested message (inlined) | `MaybeUninit<M>` (`Explicit` + `Inline`) | presence bit in `_common.bits` |
| Oneof (non-bool) | `Option<E>` in slot | `Option`, not bitfield |

Unset EXPLICIT slots are uninitialized (`MaybeUninit`); **only the bit** means "set". Singular string / bytes storage is `SsoString` / `SsoBytes` (+ `InlineOrHeap` tag bit); repeated string / bytes use `UnmanagedString` / `UnmanagedVec<u8>`.

Public accessors are **one-line delegates** into catalog methods with `&self._common` / `&mut self._common`. `Message` (encode / merge / unknown / validate) sums the same delegates; the generated `Drop` walks heap fields calling `deallocate(&self._common)` (or oneof `clear`).

### Codegen emission per message

1. Struct — `MessageCommon<BitArray<[u8; N], Lsb0>, A>` + catalog members + `OneofSlot` per oneof
2. **Module-level** `pub const FIELD_*` / `BIT_*` (usable as `match` patterns). `BIT_*` is baked into each `Explicit<BIT>` / `LegacyRequired<BIT>` / `BitPacked<BIT>` field type; `FIELD` is a struct const generic on the wrapper.
3. Inherent accessor delegates ([DESIGN.md §4.0](DESIGN.md#40-inherent-accessors-current))
4. Trait impls — `Message`, `Clone` / `PartialEq` / `Debug` / `Drop` / `DeallocateIn` as field sums
5. Child modules — nested types and oneof submodules named after the oneof (under the message **companion** module; the struct itself lives in the parent package / enclosing companion — see below)

IR step: `ProtoField → FieldKind → catalog type + const args` — implemented as [`protoc-gen-puroro::field_kind`](protoc-gen-puroro/src/field_kind.rs) (`plan_fields` assigns `FIELD_*` / `BIT_*` and builds [`MessageFieldPlan`](protoc-gen-puroro/src/field_kind/plan.rs)).

**Module tree.** Production layout follows [DESIGN.md §4 — Module layout and naming](DESIGN.md#module-layout-and-naming): `package` → nested Rust modules; each message struct is defined in that parent module; a snake_case companion holds `FIELD_*` / `BIT_*`, nested types, and oneofs (omitted when empty). Distinct proto identities that map to the same companion path are **merged**; item-level clashes are left to `rustc`. Deliberate path changes use a **generate-time rename** (plugin option / config — not a `.proto` option). Cross-forest references use `self::_root::…` ([Path qualification](DESIGN.md#path-qualification)). [`sample-generated/`](sample-generated/) remains flat (no package prefix) as a readable stand-in.

### Path qualification (naming)

Generated code must not rely on ambient `use` imports for the items it references. A `.proto` schema can introduce almost any identifier, so short names risk colliding with user code in the same scope.

**External crates** use leading-`::` absolute paths — `::puroro::Message`, `::puroro_rt::SingularField`, `::core::ops::DerefMut`, `::allocator_api2::alloc::Allocator`, … — and must not be pulled in with `use`.

**Names inside the generated module forest** use `self::_root::…` (e.g. `self::_root::example::v1::Address`). They must **not** use leading `::` or `crate::`, because the forest may be embedded as a submodule of an application crate; those prefixes would resolve to the **host** crate root. Layout injects a private `mod _root` into every forest module so `self::_root` means the forest root at any depth:

```rust
// Forest root
mod _root { pub(super) use super::*; }

// Nested modules
mod _root { pub(super) use super::super::_root::*; }
```

Nearby relatives in the same parent (sibling structs, child `mod` names) may stay relative. Generator-reserved `_`-prefixed names (`_root`, `_common`, …) are exempt from proto-derived naming.

Normative wording: [DESIGN.md — Path qualification](DESIGN.md#path-qualification).

**Crate split.** Items from [DESIGN.md §3](DESIGN.md#3-runtime-trait-api) (`Message`, `Optional`, `HasDefault`, `DecodeError`, …) are emitted as `::puroro::…`. Field catalog types, `MessageCommon`, wire helpers, and `ProtoDefault` are emitted as `::puroro_rt::…`. A generated crate's `Cargo.toml` lists both dependencies; end-user application code should not add `puroro-rt` directly.

**Public signatures must not surface `puroro-rt`.** Fully-qualified `::puroro_rt::…` paths are fine in **private** / `pub(crate)` storage and `impl` bodies. They must **not** appear in public function signatures, public type aliases, or other API that forces library users to name `puroro-rt` (use `puroro` traits, RPIT, or concrete user-facing types instead). Normative rule: [DESIGN.md §4 — Public signatures must not surface `puroro-rt`](DESIGN.md#public-signatures-must-not-surface-puroro-rt).

**The checked-in [`sample-generated/`](sample-generated/) deliberately relaxes path qualification for readability.** It pulls names in with `use` and refers to them by short name (`SingularField`, `Allocator`, `MessageCommon`, …) so the reference output stays easy to read and review. Read those short names as stand-ins for the production spellings (`::puroro_rt::…` / `self::_root::…`). The sample still aims to obey the **no public `puroro-rt` in signatures** rule above.

### Generated headers

Navigational comments in generated Rust (section banners, per-field `// proto: …` labels, merge-arm notes, synthesized accessor docs) are **not required**. Semantics live in `puroro` / `puroro-rt` rustdoc and in DESIGN / this document. The hand-written [`sample-generated/`](sample-generated/) may keep informal comments for human review of the reference shape; that is not a codegen contract.

**File header** — every **real** plugin module carries a machine marker and the source message:

```rust
/*!
@generated — do not edit
- `example.proto`
*/
```

Tooling uses `@generated` to collapse/skip generated files. The checked-in [`sample-generated/`](sample-generated/) intentionally **omits** it — those files are a hand-maintained reference, and an `@generated`/`do not edit` banner there would wrongly imply they are tool-generated.

**Constants** — `FIELD_*` / `BIT_*` are **module-level** `pub const` on the message companion (not associated constants). Module-level consts are valid `match` patterns; associated consts are not. Inside the message `impl` (in the parent module), arms qualify them (`task::FIELD_TITLE => …`) or `use` the companion.

Every field kind merges through the same bound-view shape — `self.<field>.bind_mut(&mut self._common).merge(wire_type, buf, depth)?` (repeated and nested-message fields likewise; oneof uses `OneofSlotMut`) — so the code generator emits one form. Oneof variant arms use the **variant field name** and number.

---

## 10. Common bit indices

Tracked bits use [`bitvec::BitArray`](https://docs.rs/bitvec) inline in the message (`BitVec` is heap-only and incompatible with custom `A`). Store them in `_common.bits` as `BitArray<[u8; N], Lsb0>`; catalog code indexes them through [`MessageCommonBits`](puroro-rt/src/fields/shared.rs) on [`MessageCommon`](puroro-rt/src/fields/shared.rs).

**Assignment (one pass, ascending field number):**

1. For each `EXPLICIT` / `LEGACY_REQUIRED` singular field (including `bool`), allocate one **presence** bit.
2. For each singular or oneof `string` / `bytes` field that uses SSO (the default; not `(puroro.*_layout) = HEAP`), allocate one **heap-arm** bit (`BIT_*_SSO`).
3. For each singular or oneof `bool` field, allocate one **value** bit.

Gaps in field numbers do not create gaps in bit indices. Oneof non-bool variants do not take a presence bit (presence stays on `OneofSlot`) but string / bytes variants still take an SSO heap bit.

| Kind | Bits |
|---|---|
| Implicit bool | value 1 |
| Explicit / LegacyRequired bool | presence 1 + value 1 |
| Oneof bool | value 1 |
| Explicit non-bool | presence 1 |
| Singular / oneof SSO `string` / `bytes` | heap-arm 1 |

### `Task` — fourteen bits → `BitArray<[u8; 2], Lsb0>`

| Field | # | Role | `BIT_*` |
|---|---|---|---|
| `title` | 1 | EXPLICIT presence | `0` |
| `title` | 1 | SSO heap | `1` (`BIT_TITLE_SSO`) |
| `max_retries` | 3 | EXPLICIT presence | `2` |
| `owner_id` | 4 | LEGACY_REQUIRED presence | `3` |
| `owner_id` | 4 | SSO heap | `4` (`BIT_OWNER_ID_SSO`) |
| `payload` | 5 | EXPLICIT presence | `5` |
| `payload` | 5 | SSO heap | `6` (`BIT_PAYLOAD_SSO`) |
| `priority` | 10 | EXPLICIT presence | `7` |
| `email_address` | 12 | SSO heap | `8` (`BIT_EMAIL_ADDRESS_SSO`) |
| `phone_number` | 13 | SSO heap | `9` (`BIT_PHONE_NUMBER_SSO`) |
| `done` | 16 | IMPLICIT bool value | `10` (`BIT_DONE_VALUE`) |
| `flag` | 17 | EXPLICIT presence | `11` |
| `flag` | 17 | EXPLICIT bool value | `12` (`BIT_FLAG_VALUE`) |
| `urgent` | 18 | oneof bool value | `13` (`BIT_URGENT_VALUE`) |

### `Address` — six bits → `BitArray<[u8; 1], Lsb0>`

| Field | # | Role | `BIT_*` |
|---|---|---|---|
| `street` | 1 | EXPLICIT presence | `0` |
| `street` | 1 | SSO heap | `1` (`BIT_STREET_SSO`) |
| `city` | 2 | EXPLICIT presence | `2` |
| `city` | 2 | SSO heap | `3` (`BIT_CITY_SSO`) |
| `postal_code` | 3 | EXPLICIT presence | `4` |
| `latitude` | 4 | EXPLICIT presence | `5` |

Generated code indexes bits only through `MessageCommonBits` / inherent `MessageCommon` helpers (`is_bit_set` / `set_bit` / `bit_mut`), not by reaching into raw `BitArray` APIs from accessors. `N` is `ceil(bit_count / 8)` for the message's assigned bits.

---

## 11. Constructors & allocator

- **`Task::new_in(alloc)`** — default every field; `_common.bits = BitArray::ZERO`; heap fields via `*_in(alloc.clone())`, with the last heap field taking the original by move. (Building an empty `unmanaged` container does not allocate, so the clone is only used to decompose an empty `Vec`.)
- **`Task::new()`** — `A = Global` only, so `Task::new()` infers. Delegates to `Default`.
- **`Default`** — `A: Clone + Default` → `new_in(A::default())`.

Runtime **`str_to_unmanaged_in(s, alloc)`** — copy bytes into an `UnmanagedString` (`puroro_rt::decode`). The `unsafe` (raw-parts / `deallocate`) is confined to the `puroro-rt` runtime and the generated `Drop`, not to generated accessors.

---

## 12. Message-level wire I/O

### Encode

1. `visit_fields` with [`EncodedLenVisitor`](puroro-rt/src/fields/shared/field_inspect.rs) / [`EncodeRawVisitor`](puroro-rt/src/fields/shared/field_inspect.rs) — each catalog field implements [`FieldEncode`](puroro-rt/src/fields/shared/field_inspect.rs) (`encoded_len` / `encode_raw`; omit rules in [§14](#14-singular-fields)).
2. Append `_common.unknown_fields` verbatim.
3. Message `encoded_len` must match bytes written.

**Field order is not guaranteed.** Identical logical content may produce different wire bytes. Compare with `PartialEq`, not wire equality.

Runtime (`puroro_rt::encode` + [`encode_type`](puroro-rt/src/fields/wire/encode_type.rs)): `encode_field` / `encoded_len_field`, plus low-level `encode_varint_field`, `encode_tag`, `encoded_len_len_field`, …

```rust
fn encoded_len(&self) -> usize {
    let mut v = EncodedLenVisitor::new(&self._common);
    let _ = self.visit_fields(&mut v);
    v.finish() + self._common.unknown_fields.len()
}

fn encode_raw<B: BufMut>(&self, buf: &mut B) {
    let _ = self.visit_fields(&mut EncodeRawVisitor::new(&self._common, buf));
    buf.put_slice(&self._common.unknown_fields);
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
| `Clone` / `CloneIn` | `A: Clone` | Field-wise [`FieldCloneIn::clone_field`](puroro-rt/src/fields/shared/field_inspect.rs); `Clone` clones `MessageCommon.alloc` and delegates |
| `PartialEq` | `A: Clone` | Semantic getter comparison (not wire bytes); float uses Rust `PartialEq` |
| `Debug` | `A: Clone` | Field-name `debug_struct` (oneof shown as `notification` → `case`) |

**Not currently generated:** `Eq`, `Copy`, `Ord`, `Hash`.

**`Global` extras:** `Task::new()` (`Default` for `Task<Global>`). `Message::decode(buf)` when `Self: Default`.

Prefer `Arc<Task<A>>` for shared immutable messages when clone cost matters.

---

## Part IV — Field behaviour

## 14. Singular fields

In this catalog, **singular** means a **non-repeated** field — both `IMPLICIT` (no presence bit; proto3-style / “non-optional”) and `EXPLICIT` / `LEGACY_REQUIRED` (presence-tracked / “optional”). Presence is selected by `P: FieldPresence`; cardinality is selected by using `SingularField` vs a repeated wrapper.

### Unified wrapper (`SingularField<T, P, FIELD>`)

Varint and LEN share [`SingularField`](puroro-rt/src/fields/singular/field.rs), parametrised by type marker `T: SingularType` and layout `L: ValueLayout<T>`. Addressable scalars use `L = Inline` ([`PayloadAccess`](puroro-rt/src/fields/wire/singular_type.rs)); bit-packed [`ProtoBool`](puroro-rt/src/fields/wire/numerical.rs) uses `L = BitPacked<VALUE_BIT>`. There are no wire-family aliases — generated code names `SingularField` directly.

Storage is `ManuallyDrop<P::ValueSlot<L::Slot>>` — `T` / `MaybeUninit<T>` (including ZST `ProtoBool`) depending on presence. Heap LEN payloads need an explicit [`FieldDeallocate::deallocate`](puroro-rt/src/fields/shared/field_deallocate.rs)(`&common`) from message / oneof teardown; copy scalars’ / unit-slot `DeallocateIn` is a no-op.

**Mutation goes through a bound view:** `field.bind_mut(&mut common)` yields [`SingularFieldMut`](puroro-rt/src/fields/singular/field.rs). `value_mut` returns `L::Mut` (e.g. `&mut i32`, `SsoStringMut` / `SsoBytesMut`, heap `StringGuard` / `VecGuard`, or bitvec `BitRef` for `ProtoBool`).
**Read accessors also go through a bound view:** `field.bind(&common)` yields [`SingularFieldRef`](puroro-rt/src/fields/singular/field.rs). Generated getters always bind first — even for `IMPLICIT` `value()` which does not consult `common` — so read and write share one shape.

| | IMPLICIT | EXPLICIT / LEGACY_REQUIRED |
|---|---|---|
| Encode | Omit when empty / type-zero | Omit when bit unset |
| Merge | `bind_mut(&mut common).merge(wire, buf)` | same (sets bit via `SlotInitMut`) |
| Getter | `bind(&common).value()` | `bind(&common).optional()` |
| Mutator | `bind_mut(&mut common).value_mut()` → `L::Mut` | same (sets bit) |
| Clear | `bind_mut(&mut common).clear()` | same |
| Release (LEN) | `deallocate(&common)` from message `Drop` | same |

Encode / `deallocate` / `validate_required` stay as plain field methods that take `&common` directly. Enum fields use `ProtoEnum<E, Open|Closed>`; closed-enum unknown values surface as `DecodeError::UnknownClosedEnum` and are diverted by singular `merge`. `Optional` is a concrete struct (DESIGN.md §3); no `Option<T>` conversion.

### Bit-packed `bool` (`ProtoBool` + `BitPacked`)

Singular / oneof `bool` is the same type marker [`ProtoBool`](puroro-rt/src/fields/wire/numerical.rs) as repeated / map bool. Generated code uses layout [`BitPacked<VALUE_BIT>`](puroro-rt/src/fields/shared/value_layout.rs) (`ValueLayout::Slot = ()`); the logical `bool` lives at `VALUE_BIT` in `_common.bits`. [`Inline`](puroro-rt/src/fields/shared/value_layout.rs) is also valid (`Slot = bool` via [`PayloadAccess`](puroro-rt/src/fields/wire/singular_type.rs), same as other numerics). EXPLICIT / LEGACY_REQUIRED also use `P`'s presence bit (orthogonal). Bound views are the same `SingularFieldRef` / `SingularFieldMut`; packed `value_mut` returns bitvec's `BitRef<'_, Mut, …>` via [`MessageCommon::bit_mut`](puroro-rt/src/fields/shared.rs), inline `value_mut` returns `&mut bool`. Wire encode goes through `EncodeType` / `encode_field`; storage access goes through `ValueLayout`. Implicit omit treats a clear / type-zero value as absent; Explicit can encode an explicit `false`.

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

Mutation uses the bound-view idiom: `field.bind_mut(&mut common)` → [`RepeatedFieldMut`](puroro-rt/src/fields/repeated/field.rs). Markers with [`RepeatedVecMut`](puroro-rt/src/fields/wire/repeated_element.rs) (copy scalars / enums / messages) expose `values_mut()` → `Vec` guard; string / bytes use `container_mut()` → [`RepeatedElementsMut`](puroro-rt/src/fields/repeated/container.rs) (`push` then fill). `clear` / `deallocate` drain and free heap elements first, then free the buffer. Read: `field.bind(&common)` → [`RepeatedFieldRef`](puroro-rt/src/fields/repeated/field.rs) (`as_slice` / `is_empty`). Merge requires `T: RepeatedElementMerge<A>`.

**`repeated message`:** `RepeatedField<ProtoMessage<M>, Expanded, FIELD, A>` with `Element = M` (no per-element `UnmanagedBox`). Each wire occurrence constructs a new `M` via `Message::new_in` and appends — it does **not** merge into an existing list index. Sample: `Task.watchers` (`repeated Address`).

**`repeated bool`:** `RepeatedField<ProtoBool, Packed|Expanded, FIELD, A>` with plain `bool` elements — **no** [`BitPacked`](puroro-rt/src/fields/shared/value_layout.rs) / MessageCommon bit index (see [Bit-packed bool](#bit-packed-bool-protobool)). Sample: `Task.votes`.

> Note: the bound-view idiom covers every field family — `SingularField`, `RepeatedField`, `MapField`, and `OneofSlot` — on both read and write paths. Terminal `deallocate` stays a direct field method (called from `Drop`).

### 15.1 Map fields

**Catalog:** [`MapField<K, V, FIELD, A>`](puroro-rt/src/fields/map/field.rs) with `K: MapKey`, `V: RepeatedElement`. Storage is an allocator-owning `hashbrown::HashMap<K::Element<A>, V::Element<A>, …, A>` (unlike `UnmanagedVec` fields). Wire order is unspecified; only the hash map is kept.

**Marker GATs** (same idea as singular `SingularType::{Ref, Mut}`):

| Trait | Assoc / method | Role |
|---|---|---|
| [`MapKey`](puroro-rt/src/fields/wire/map_element.rs) | `key_from_view` | Materialize stored key from [`RepeatedElement::RefView`](puroro-rt/src/fields/wire/repeated_element.rs) |
| [`RepeatedElement`](puroro-rt/src/fields/wire/repeated_element.rs) | `RefView` / `as_ref_view` | Shared key/value view (`i32`, `str`, …); also for repeated element-wise reads |
| [`RepeatedElementMut`](puroro-rt/src/fields/wire/repeated_element.rs) | `MutTarget` / `ElementMut` | Mutable handle target |

**Wire:** each map occurrence is one LEN field `FIELD` whose payload is a synthetic entry message (`key = 1`, `value = 2`). Encode/decode helpers live in [`map/entry.rs`](puroro-rt/src/fields/map/entry.rs). Element tags use [`encode_field`](puroro-rt/src/fields/wire/encode_type.rs) after [`RepeatedElement::wire_view`](puroro-rt/src/fields/wire/repeated_element.rs). Decode uses `RepeatedElementMerge::{decode_element, default_element}` (singular wire types only; packed rejected inside the entry). Missing key/value → type default. Unknown tags inside the entry are skipped via [`skip_field`](puroro-rt/src/decode.rs) (not preserved).

| | Behaviour |
|---|---|
| Encode | One LEN record per map entry (order unspecified) |
| Merge | Decode one entry → `insert` (last-wins; frees replaced value + discarded key) |
| Empty | Absent on the wire |

**Bound views:** `bind` / `bind_mut` → [`MapFieldRef`](puroro-rt/src/fields/map/field.rs) / [`MapFieldMut`](puroro-rt/src/fields/map/field.rs).

Catalog helpers on `MapFieldMut`:

| Method | Role |
|---|---|
| `get_element_mut` | Lookup → element mut handle (`MapMut::get_mut`) |
| `entry_element_mut_view` | Ensure via `MapKey::key_from_view` + default (`MapMut::entry_mut`) |
| `insert` | Owned key + value elements (catalog / merge) |
| `remove` / `clear` | Free key + value via `deallocate_element` |
| `merge` | One wire occurrence |

User-facing [`MapRef`](src/map.rs) / [`MapMut`](src/map.rs) are **two blanket impls** over `MapFieldRef` / `MapFieldMut` in [`map/field.rs`](puroro-rt/src/fields/map/field.rs) (no K×V macro matrix). Key methods take `impl Borrow<K>`; mutation is `entry_mut` then assign / fill.

**Key collision safety:** `HashMap::insert` would drop a colliding incoming key; `MapFieldMut::insert` keeps the stored key and explicitly releases `(incoming_key, previous_value)` (required for `UnmanagedString` keys).

Sample: `Task.attributes` — `map<string, int32>` → `MapField<ProtoString, ProtoInt32, { FIELD_ATTRIBUTES }, A>` with accessors `attributes()` → `MapRef<str, i32>`, `attributes_mut()` → `MapMut<str, i32, MutTarget = i32>`, `clear_attributes()` via `MapMut::clear`.

---

## 16. Nested messages, oneof, unknown fields

### Nested (`SingularField<ProtoMessage<M>, P, FIELD, A, L>`)

Same wrapper as other singular fields. [`ProtoMessage`](puroro-rt/src/fields/wire/proto_message.rs) `PayloadAccess::Slot` is `M`. Layout chooses the physical slot:

- [`Boxed`](puroro-rt/src/fields/shared/value_layout.rs) + [`Message`](puroro-rt/src/fields/shared/field_presence.rs) → `Option<UnmanagedBox<M, A>>` (typical singular; sample `Task.assignee`)
- [`Boxed`](puroro-rt/src/fields/shared/value_layout.rs) + [`Oneof`](puroro-rt/src/fields/shared/field_presence.rs) → always-present `UnmanagedBox<M, A>` (sample `notification.postal`)
- [`Inline`](puroro-rt/src/fields/shared/value_layout.rs) + [`Explicit<BIT>`](puroro-rt/src/fields/shared/field_presence.rs) → `MaybeUninit<M>` (sample `Task.origin` / `Point`)

Child type bound is `M: Message<Alloc = A>` (empty children via [`Message::new_in`](src/message.rs)). An inlined child still owns its own [`MessageCommon`](#4-shared-infrastructure) (no sharing). Wire merge / encode / clear go through [`ValueLayout`](puroro-rt/src/fields/shared/value_layout.rs) (`Boxed` vs `Inline` + [`PayloadMerge`](puroro-rt/src/fields/wire/singular_type.rs)). Accessors stay `Option<&M>` / `&mut M` (`bind` / `bind_mut` → `get` / `get_mut` / `merge` / `clear`). Recursion limit: enforced (`RECURSION_LIMIT`).

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
| `OneofGroup::Mut` | inline | via [`SingularFieldAccess::Mut`](puroro-rt/src/fields/singular/field.rs) on each variant field alias | opaque on `notification_mut()` RPIT; typed mut via per-variant `_mut` |

**Ref/Mut payloads are not hard-coded in generated aliases.** Ref uses concrete getter types (`&str`, `i32`, …). Mut projects [`SingularFieldAccess::Mut`](puroro-rt/src/fields/singular/field.rs) from each variant's private field type alias (`EmailAddressField<A>`, …), which already carries the marker and [`ValueLayout`](puroro-rt/src/fields/shared/value_layout.rs). Per-variant private field type aliases remain the single source for Storage.

**Variants own field wrappers, not raw storage.** Each variant holds the same field wrapper an ordinary singular field of that kind uses (`SingularField` — including `ProtoBool` + `BitPacked<VALUE_BIT>` for `bool` and `ProtoMessage<M>` for messages), so `value` / `value_mut` / `deallocate` are reused. The wrapper's presence is inert here (`Oneof` / `FieldPresence::Oneof`), so presence-aware omit rules are never consulted; bool still packs its value into `_common.bits`. Markers are allocator-free; unmanaged payloads and field wrappers carry `A` (type only / `PhantomData`); the owned allocator instance stays on the message.

The oneof drives each variant with the **field's own** primitives. Empty construction is [`DefaultIn`](puroro-rt/src/fields/shared.rs) on the variant field wrapper (`SingularField::default_in`). Merging is `slot.bind_mut(common).variant_mut::<FIELD_…>().bind_mut(common).merge(wire, buf)`. Read getters use `slot.bind(common).variant_of::<FIELD_…>().optional()` / `.get()`. [`OneofVariant`](puroro-rt/src/fields/oneof_variant.rs) is keyed by proto field number (no per-variant marker ZSTs).

**The message variant uses `SingularField<ProtoMessage<…>, Oneof, …, Boxed>` — always-present `UnmanagedBox<M, A>`, not `Option`.**

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
| UTF-8 validation | Per-field `utf8_validation` (`ProtoString` / `&str` vs `ProtoStringUnchecked` / `&[u8]`) | — |
| Recursion limit | Enforced (`RECURSION_LIMIT = 100`, `merge_from_with_depth`) | — |
| Repeated wrappers | `RepeatedField` + `RepeatedElement` (message / bool / scalar / LEN) | — |
| Map wrappers | `MapField` + `MapKey` / `RepeatedElement` (sample `attributes`) | — |
| `protoc-gen-puroro` field emission | Eager-path families done (singular / repeated / enum / message / oneof / map / defaults); see [§3](#3-implementation-status) | Typed extensions / services |
| Zero-copy views | — | `TaskView<'buf>` (DESIGN.md §8) |
| `TaskLazy` | DESIGN only | Wire buffer + on-demand decode |
| `Hash` / `serde` | Deferred | Opt-in features |
| Submessage inline | Runtime + codegen: `Boxed` vs `Inline`+presence bit (`(puroro.message_layout)`, SCC, 1..=4 scalar heuristic) | `MessageCommon` sharing ([§17.1](#171-submessage-inline-optimisation)) |
| String / Bytes inline | Singular `string` / `bytes` use SSO (`SsoString` / `SsoBytes` + `InlineOrHeap`); repeated / map stay heap | Repeated / map SSO deferred ([§17.2](#172-string--bytes-inline-optimisation)) |

### 17.1 Submessage inline optimisation

**Status (runtime, no common sharing): done.** Singular nested messages use [`Boxed`](puroro-rt/src/fields/shared/value_layout.rs) (`UnmanagedBox<M, A>`) or [`Inline`](puroro-rt/src/fields/shared/value_layout.rs) (`M` in the parent slot + `Explicit` / `LegacyRequired` presence bit). Public accessors stay `Option<&M>`. The inlined child still owns its own [`MessageCommon`](#4-shared-infrastructure) (allocator clone, unknown fields, bits). Oneof / repeated stay boxed / `Element = M`. Same-SCC singular edges (self / mutual recursion) stay `Boxed`.

**Codegen.** [`(puroro.message_layout)`](proto/puroro/options.proto) (`UNSPECIFIED` / `INLINE` / `BOXED`, field 51402) plus [`plan_message_storage`](protoc-gen-puroro/src/field_kind/storage.rs): graph of singular non-oneof message edges → Tarjan SCCs ineligible for inline → `BOXED` honored, `INLINE` ignored when illegal, unspecified uses the auto-inline heuristic (child has 1..=4 non-repeated Copy scalars / enums). Sample `Task.origin` is the hand-written inlined path; plugin fixtures live in [`puroro-codegen-tests/fixtures/message_layout`](puroro-codegen-tests/fixtures/message_layout/).

**Sharing `MessageCommon` (future).** A smaller inlined child would share the parent's common (bits offset, allocator, unknown buffer) rather than embedding a second `MessageCommon`. That remains future work: generic-over-common message types and scoped `bind` views.

### 17.2 String / Bytes inline optimisation

**Status (string / bytes): done.** Singular `string` and `bytes` (IMPLICIT / EXPLICIT / LEGACY_REQUIRED / oneof) default to the shared 3-word SSO slot ([`SsoBuf`](puroro-rt/src/fields/wire/sso_buf.rs) → [`SsoString`](puroro-rt/src/fields/wire/sso_string.rs) / [`SsoBytes`](puroro-rt/src/fields/wire/sso_bytes.rs)) + [`InlineOrHeap`](puroro-rt/src/fields/shared/value_layout.rs). The puroro field options [`(puroro.string_layout)`](proto/puroro/options.proto) and [`(puroro.bytes_layout)`](proto/puroro/options.proto) (`UNSPECIFIED` / unset) use that generator default; `SSO` pins SSO; `HEAP` selects heap [`UnmanagedString`](unmanaged/src/string.rs) / [`UnmanagedVec`](unmanaged/src/vec.rs) via `ProtoString` / `ProtoBytes` + [`Inline`](puroro-rt/src/fields/shared/value_layout.rs) (`PayloadAccess`). Heap `_mut` is `impl DerefMut<Target = puroro::String<A>>` / `impl DerefMut<Target = Vec<u8, A>>`; SSO `_mut` is `impl StringMut<A>` / `impl BytesMut<A>`. This is independent of C++ `FieldOptions.ctype`. Do not reuse `string_layout` on `bytes` fields.

**Layout.** The slot stays **3 words** (same as [`UnmanagedString`](unmanaged/src/string.rs) / [`UnmanagedVec`](unmanaged/src/vec.rs)):

- **heap arm:** `UnmanagedString` or `UnmanagedVec<u8>` (any length, including short/empty — allowed when packed as heap; not an in-slot tag)
- **inline arm:** `[u8; INLINE_CAP]` + length byte (`INLINE_CAP = 3*usize - 1`, 23 on 64-bit). Length lives in the **last byte** of the slot (`0..=INLINE_CAP`)
- **MessageCommon heap bit** (`BIT_*_SSO` / `InlineOrHeap<HEAP_BIT>`; [`SSO_HEAP`](puroro-rt/src/fields/shared/value_layout.rs) = `true`, [`SSO_INLINE`](puroro-rt/src/fields/shared/value_layout.rs) = `false`) is the **sole** arm discriminant. The slot is an untagged union and does not inspect the heap type's memory layout
- **Arm choice is layout-internal:** crate-private pack helpers / decode `merge` decide inline vs heap. Hot paths (`StringMut::set(&str)`, `BytesMut::set(&[u8])`, short decode) pack without a heap allocation

**Mutator.** Generated `utf8_validation=VERIFY` string `_mut` returns `impl ::puroro::StringMut<A>` (concrete [`SsoStringMut`](puroro-rt/src/fields/wire/sso_string.rs) stays in `puroro-rt`). Methods: `set(&str)` / `set_string(unmanaged::String)` / `clear` / `push_str` / `push` / `truncate`. Generated bytes `_mut` and `utf8_validation=NONE` string `_mut` return `impl ::puroro::BytesMut<A>` ([`SsoBytesMut`](puroro-rt/src/fields/wire/sso_bytes.rs)): `set(&[u8])` / `set_vec(Vec<u8, A>)` / `clear` / `extend_from_slice` / `push` / `truncate`. Both stay inline while the result fits; overflow promotes to heap.

**Still open / deferred.** Repeated / map string and bytes elements stay `UnmanagedString` / `UnmanagedVec`.
