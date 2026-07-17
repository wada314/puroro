# Plan: Allocator on fields, GATs on markers

## Goal

Markers (`ProtoInt32`, `ProtoBool`, `ProtoString`, …) become **allocator-free** type identity. Physical storage / views are **GATs** parametrised by `A`. The allocator type lives on **field wrappers** (`SingularField` / `RepeatedField`).

## Target spellings

```rust
// Markers (no A)
ProtoInt32, ProtoBool, ProtoString, ProtoBytes
ProtoEnum<Status, Open>, ProtoMessage<Address<A>>  // message type M may still mention A

// Fields
SingularField<ProtoInt32, Implicit, { FIELD_SCORE }, A>
SingularField<ProtoString, Explicit<{ BIT_TITLE }>, { FIELD_TITLE }, A>
SingularField<ProtoBool, Implicit, { FIELD_DONE }, A, BitPacked<{ BIT_DONE_VALUE }>>
SingularField<ProtoInt32, Explicit<{ BIT }>, { FIELD }, A, Inline, MaxRetriesDefault>

RepeatedField<ProtoInt32, Packed, { FIELD_TAG_IDS }, A>
RepeatedField<ProtoString, Expanded, { FIELD_LABELS }, A>
```

**Parameter order (locked):**

```rust
SingularField<T, P, const FIELD, A, L = Inline, D = ProtoDefault>
RepeatedField<T, E, const FIELD, A>
```

`A` sits after `FIELD` so layout/default remain trailing defaults. Custom-default sites pass `Inline` then `D` as today.

## Trait shape

### `ProtoType` (wire + identity)

```rust
pub trait ProtoType: Sized {
    type Slot<A: Allocator + Clone>: AddressableSlot<SlotAlloc = A> + DefaultIn<Alloc = A>;
    type Ref<'a, A: Allocator + Clone>
    where
        Self: 'a,
        A: 'a;
    type Mut<'a, A: Allocator + Clone>: DerefMut
    where
        Self: 'a,
        A: 'a;
    type Written<A: Allocator + Clone>;

    const WIRE_TYPE: WireType;

    fn encoded_len<'a, A: Allocator + Clone>(value: Self::Ref<'a, A>, field: u32) -> usize;
    fn encode<'a, A, B: BufMut>(value: Self::Ref<'a, A>, field: u32, buf: &mut B);
    fn decode<A: Allocator + Clone, B: Buf>(
        wire_type: WireType,
        buf: &mut B,
        alloc: A,
    ) -> Result<Self::Written<A>, DecodeError>;
}
```

Remove associated `Alloc` from `ProtoType`.

### Slot mapping

| Marker | `Slot<A>` | `Ref` / `Mut` / `Written` (sketch) |
|---|---|---|
| `ProtoInt32` (etc.) | `i32` (bare; not the marker) | `i32` / `&mut i32` / `i32` |
| `ProtoEnum<E, K>` | `E` | `E` / `&mut E` / `E` (or thin written) |
| `ProtoBool` | `()` | `bool` / `BitRef` / `bool` |
| `ProtoString` | `UnmanagedString<A>` | `&str` / `StringGuard` / `UnmanagedString<A>` |
| `ProtoBytes` | `UnmanagedVec<u8, A>` | `&[u8]` / `VecGuard` / `UnmanagedVec<u8, A>` |
| `ProtoMessage<M>` | `UnmanagedBox<M, A>` where `M: Message<Alloc = A>` | `&M` / `&mut M` / `UnmanagedBox<M, A>` |

Markers are ZST (or `PhantomData` only for `ProtoEnum` / `ProtoMessage`).

### `PayloadAccess` / `ValueLayout`

Thread `A` through storage APIs:

```rust
trait PayloadAccess: ProtoType {
    fn get<'a, A: Allocator + Clone, Pb>(slot: &'a Self::Slot<A>, common: &'a MessageCommon<Pb, A>) -> Self::Ref<'a, A>;
    // with_mut / write / clear / merge similarly
}

trait ValueLayout<T: ProtoType, A: Allocator + Clone>: Copy { … }

impl<T: PayloadAccess, A: Allocator + Clone> ValueLayout<T, A> for Inline { … }
impl<A: Allocator + Clone, const V: usize> ValueLayout<ProtoBool, A> for BitPacked<V> { … }
```

### `RepeatedItems`

```rust
trait RepeatedItems: ProtoType {
    type Element<A: Allocator + Clone>;
    // encode/decode/merge_occurrence take Element<A> + A
}
```

| Marker | `Element<A>` |
|---|---|
| varint / enum | `i32` / `E` (ignore `A`) |
| string / bytes | `UnmanagedString<A>` / `UnmanagedVec<u8, A>` |

`RepeatedField` stores `UnmanagedVec<T::Element<A>, A>`.

### Helpers

- [`VarintProtoType`](puroro-rt/src/fields/wire/varint.rs) / [`LenProtoType`](puroro-rt/src/fields/wire/len.rs): implement on allocator-free markers; `LenProtoType::Storage` becomes GAT `Storage<A>` or is folded into `Slot`/`Element` and deleted if redundant.
- [`AddressableSlot`](puroro-rt/src/fields/shared/value_slot.rs) / `DefaultIn` / `DeallocateIn` / `ProtoEmpty`: impl for **slot/element types** (`i32`, `()`, `UnmanagedString<A>`, …), not for markers.
- Drop `PhantomData<A>` from varint wrapper macro; markers no longer wrap payload.

## Field wrapper changes

### [`SingularField`](puroro-rt/src/fields/singular/field.rs)

```rust
pub struct SingularField<T, P, const FIELD: u32, A, L = Inline, D = ProtoDefault>
where
    T: ProtoType,
    P: FieldPresence,
    A: Allocator + Clone,
    L: ValueLayout<T, A>,
    P::ValueSlot<T::Slot<A>>: ValueSlot<T::Slot<A>>,
{
    value: ManuallyDrop<P::ValueSlot<T::Slot<A>>>,
    _marker: PhantomData<(T, P, A, L, D)>,
}
```

All former `T::Alloc` / `T::Slot` / `T::Ref` sites become `A` / `T::Slot<A>` / `T::Ref<'_, A>`.

### [`RepeatedField`](puroro-rt/src/fields/repeated/field.rs)

```rust
pub struct RepeatedField<T, E, const FIELD: u32, A>
where
    T: RepeatedItems,
    E: RepeatedEncoding<T, A>,  // or encoding stays generic over Element
    A: Allocator + Clone,
```

Update aliases (`RepeatedPackedVarintField`, …) to take `A` last.

### Oneof / sample

- [`oneof.rs`](puroro-rt/src/fields/oneof.rs) `OneofVariantRef` bounds: `SingularField<T, Oneof, FIELD, A, L, D>`.
- [`task.rs`](sample-generated/src/task.rs) / [`notification.rs`](sample-generated/src/task/notification.rs) / [`address.rs`](sample-generated/src/address.rs): rewrite every field member and `ProtoType::Ref`/`Mut` projection to new GATs / markers.
- `NotificationRef` etc.: `<ProtoString as ProtoType>::Ref<'a, A>` (syntax as required by GAT).

## Docs

Update [`IMPLEMENTATION.md`](IMPLEMENTATION.md) §5–9, §14–15 and [`DESIGN.md`](DESIGN.md) catalog sentences: markers are A-free; fields carry `A`; `Slot`/`Element` are GATs.

## Out of scope

- `repeated bool` / `repeated message` wiring
- Changing public message accessor signatures (`title()`, `tag_ids_mut()`, …)
- Moving `VALUE_BIT` again (already on `BitPacked`)

## Implementation order

1. Introduce GAT `ProtoType` + allocator-free markers; impl `Slot`/`Ref`/`Mut`/`Written` for all current kinds; move `DefaultIn`/etc. onto slot types.
2. Update `PayloadAccess` + `ValueLayout`/`BitPacked`/`Inline` for `(T, A)`.
3. Update `SingularField` (+ oneof helpers) to take `A`.
4. GAT-ify `RepeatedItems` + `RepeatedField` + encoding.
5. Slim/remove obsolete `LenProtoType`/`VarintProtoType` surface if fully superseded; otherwise keep as thin wire helpers on markers.
6. Migrate `sample-generated`.
7. Doc sync; `cargo fmt` + `clippy --all-targets` + `cargo test -p puroro-sample-generated`.

## Success criteria

- No `ProtoInt32<A>` / `ProtoBool<A>` / `ProtoString<A>` marker forms. ✅
- `SingularField` / `RepeatedField` take `A`. ✅
- `Slot = Self` is gone for all markers. ✅
- Sample encode/decode tests pass; clippy clean on touched crates. ✅

## Notes from implementation

- ~~Numerics use `CopySlot<V, A>`~~ → removed: `DefaultIn<A>` / `DeallocateIn<A>` are trait-parameterised, so bare `i32` / `()` work.
- Bool singular slot is `()`; value via `BitPacked`.
- `ProtoMessage<M>` is A-free; `Slot` is `UnmanagedBox<M, M::Alloc>` (call sites unify with field `A`).
- `ProtoType::decode` for messages is a stub; real decode is `PayloadAccess::merge`.
