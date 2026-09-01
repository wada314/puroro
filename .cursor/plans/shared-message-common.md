# Shared `MessageCommon` + nested unknown store

Discussion lock (2026-08-30). Inline storage without sharing is already shipped
([IMPLEMENTATION.md §17.1](../../IMPLEMENTATION.md#171-submessage-inline-optimisation)).
This note records what we agreed for **sharing** the parent common, and the
**unknown-field container** that should land first as its own task.

## Why sharing

An inlined child is a full `M` today, so it embeds a second
`MessageCommon` (bits + empty `UnmanagedVec` unknown blob ≈ 3 words + `A`).
For `Global`, the empty unknown vec dominates. Auto-inline of a tiny
`Point` can make the parent **larger** than a boxed pointer. Sharing is
how inlined payload (`x`/`y`) can be just the fields.

Sharing **allocator + bits only** (child keeps its own unknown vec) is a
weak win on `Global`. The size motivation is **not storing a per-child
unknown blob until that child actually has unknowns**.

## Agreed product shape

### Types

- Split **body** (field wrappers only) from **binding** (body + common).
- Preferred spelling: one type `Student<C = OwnedCommon>` with a single
  accessor `impl` (no one-liner delegates). Not a strong preference over
  `Student` + `StudentBound`.
- Inline **slot** is the body (ZST / no common). It is not a `Student`.
- Getter return is a **by-value view**: parent `Window` + `&` / `&mut` body.
  Not `&Student`. A ZST cannot be overwritten with a parent reference;
  layouts differ.

### Public API

- Field accessors stay **inherent** on `Student<C>` (name clashes with
  `Message` stay UFCS).
- Per-message traits `StudentMessage` / `StudentMessageMut` are the
  generic face (already reserved in DESIGN.md §4 / §8). Users write
  `&impl StudentMessage`, not `Student<Ref<Window<…>>>`.
- There is **no clean named type** for “the impl of this trait” as a
  struct field. Owned `Student` is the easy stored type. A view in a
  struct field needs a lifetime or a clone-to-owned. A macro / TAT that
  yields `Student<Owned>` is fine; it cannot yield a parent-independent
  view type.
- `let s: &Student = school.student()` does **not** work for an inlined
  (or unified-view) getter. Default type params only omit `C` on the
  **owned** alias.

### Getter stability

`(puroro.message_layout)` is a hint; auto-inline may change. Therefore
**every** singular message getter (boxed and inlined) returns
`Option<impl StudentMessage + '_>` (mut analog for `_mut`). The concrete
view may change; the trait, the lifetime (`&self` / `&mut self`), and
auto-trait bounds must not.

Boxed view: `Window` points at the **child’s own** `MessageCommon`.
Inlined view: `Window` points at the parent with a bit-base offset.

### What each kind can do

| | Owned `Student` | Shared view (`&`) | Mut view (`&mut`) |
|---|---|---|---|
| Field getters | yes | yes | yes |
| `new_in` / `decode` (construct `Self`) | yes | no | no |
| `merge_from` | yes | no | **yes** |
| `encode` / `encoded_len` (as child LEN) | yes | yes | yes |
| Deep `Clone` / `Drop` | yes | no (shallow copy of two pointers) | no |

`StudentMessage` is fields (+ optional encode). `merge_from` lives on
`StudentMessageMut` or inherent mut methods. Full `Message` stays on owned.

### Bits window

Child field types keep **local** bit indices (`Explicit<0>`, …).
`Window` implements `MessageCommonBits` and adds a const/runtime
**bit base** into the parent array. Nested inline composes offsets.
Standalone `Student` uses a short bit array; `StudentBody` is the same
type everywhere.

### Catalog (Task 2 — sample `Task.origin`)

`ProtoMessage` today: `Slot = View = Mut = M`. After sharing:

| GAT | Value |
|---|---|
| `Slot` | `StudentBody` (fields only) |
| `View` | `{ common: Window<&Parent>, body: &Body }` |
| `Mut` | `{ common: Window<&mut Parent>, body: &mut Body }` |

`get` / `get_mut` / `merge` / `clear` / `deallocate` bind through the
**window**, never a child’s private `_common`. Owned `Student` is the
same catalog with a window onto its own common. Parent `Drop` deallocates
embedded bodies with the parent alloc; it does not run a child
`MessageCommon::deallocate`.

Leave an unknown hook on `Window` so the nested store can be swapped in
without a second catalog rewrite.

### Borrows

`student_mut` **does** use Rust field-split inside the getter
(`&mut self._common` and `&mut self.student` are disjoint).

The signature `fn(&mut self) -> View<'_>` still locks the whole parent
for the caller — same as today’s `origin_mut() -> &mut Point`. A mut view
holds `&mut` parent common, so two mut getters cannot coexist. Two
shared views can: both hold `&` common.

### Oneof (later)

Inlined variant stores body only. Active variant uses a parent window.
Switching cases must deallocate the body, **clear that field’s unknown
subtree**, and drop that child’s bit range in the parent array.

### Out of scope for sharing

- Repeated / map elements stay `Element = M` with their own common.
- Same-SCC edges stay boxed.
- `MessageCommon` **sharing of bits/alloc** is blocked on the unknown
  store below (or we accept almost no size win on `Global`).

---

## Task 1 (now): nested unknown store

Independent of inline codegen. Replace the always-present 3-word
`UnmanagedVec<u8>` in `MessageCommon` with a compact tree that **can**
hold per–field-number child blobs. Generated messages keep using the
“self” blob only until sharing lands. Public `Message::unknown_fields()`
is unchanged (this message’s unknowns only).

### Requirements

1. **Isolation.** Unknowns seen while merging an inlined child must
   round-trip **inside that child’s LEN**, not as parent-level unknowns.
2. **Nested path.** `School → Student → Point` is a chain of field
   numbers, not a flat map.
3. **Empty is cheap.** No unknowns anywhere ⇒ about **one word** in
   `MessageCommon` (not three). This helps every message, not only
   inlined ones.
4. **Public iterator** still parses a contiguous self-blob
   (`UnknownField` / `UnknownPayload` unchanged). No `as_bytes()` on the
   trait. Child subtrees are **not** listed on the parent iterator.
5. Preserve / discard / custom policies in DESIGN.md §4.9 stay; this
   store is the Preserve implementation. Groups still not preserved.

### Layout

```text
UnknownFields<A>          // inline in MessageCommon: 1 word
  └─ None                 // empty
  └─ Some(Box<Node>)
        self_blob: UnmanagedVec<u8, A>     // this message’s wire trailer
        children:  UnmanagedVec<(u32, UnknownFields<A>), A>
                   // key = proto field number of an inlined child
                   // missing key = that child has no unknowns
```

`UnknownFields` is recursive (`children` values are the same type).
A node is allocated on the **first** self-append or first `child_mut`.
The empty `children` vec lives on that node (already on the heap); do
not add a second lazy layer unless measurements say so.

Not used as keys: repeated / map message elements (they keep their own
`MessageCommon`). Keys are singular inlined fields and inlined oneof
message variants (field number of the variant).

### Operations

| Method | Role |
|---|---|
| `append` / `skip_field_and_save` / `save_unknown_varint_field` | Write **self** blob (lazy-alloc node) |
| `self_blob() -> &[u8]` | Encode trailer (same as today’s `unknown_fields` slice) |
| `iter_self()` | `Message::unknown_fields()` |
| `child(n) -> Option<&UnknownFields>` | View window (shared) |
| `child_mut(n) -> &mut UnknownFields` | Merge / mut view window; creates empty child node if needed |
| `remove_child(n)` | `clear_*` / oneof switch / Drop of that inline slot |
| `clone_in` / `deallocate` / byte-eq of self + children by field number | `Clone` / `Drop` / `PartialEq` |

`Window` (later) holds `&mut UnknownFields` for the current path
(`parent.child_mut(FIELD_STUDENT)`). Catalog merge of an inlined child
calls `skip_field_and_save` on **that** sink.

### Encode / decode (once inline uses children)

- **Parent encode:** known inlined child LEN = child’s known fields +
  that child’s `self_blob` (same shape as today’s child encode).
- **Parent decode of a known message field:** merge into the child;
  unknown tags inside the LEN go to `child_mut(field)`, not parent self.
- **Parent decode of an unknown tag:** parent `append` (today).
- **Closed enum** on a child: divert into the child’s sink.

### First-task implementation slice — **done**

1. [`UnknownFields<A>`](../../puroro-rt/src/unknown_fields.rs) (node + child map).
2. `MessageCommon::unknown_fields` is that store; decode/encode/iter/Drop adapted.
3. Unit tests: empty size, append, child isolation, `remove_child`, nest/clone/eq.
4. Generated accessors unchanged (`Deref` to the self blob; child keys unused).

### Non-goals for task 1

- `Window` / `StudentBody` / `ProtoMessage` GAT split.
- Discard-policy codegen.
- Changing `UnknownField` / the public iterator item type.

---

## Task 2 (this slice): catalog GAT + `Window`

Runtime proof of the dual catalog, hand-written `Point` / `Task.origin` only.

**Done.**

- [`Window`](../../puroro-rt/src/fields/shared/window.rs) / `WindowMut` (`bit_base`, resolved child unknowns).
- [`MessageBinding`](../../puroro-rt/src/fields/shared.rs) / `InlinedMessageParent`; `SingularField::bind` and `PayloadAccess` take that context.
- [`SharedMessage<B, FIELD>`](../../puroro-rt/src/fields/wire/shared_message.rs): `Slot = B`, `View`/`Mut` from `SharedMessageBody`. `ProtoMessage<M>` unchanged.
- Sample `PointBody` + `PointView` / `PointMut`; `Task.origin` is `SharedMessage<PointBody<A>, FIELD_ORIGIN>`. Assignment is `set_origin` / `copy_from`.
- Tests: origin round-trip / merge / clear / unset omit, unknown isolation inside origin LEN, `Task<Padded>` size ≈ parent common + map (no extra `A` in the slot), `Window` `bit_base` unit test.

**Not this slice:** codegen / fixtures, `Address` Body split, boxed `assignee` as a view, oneof subtree clear, `Student<C>`, `Window::nest`.

---

## Task 3: `Address` Body + boxed `assignee` view

**Done.**

- Sample `AddressBody` + `AddressView` / `AddressMut`. Owned `Address` still has its own `MessageCommon`.
- Storage stays `ProtoMessage<Address>` (boxed slot is the full message).
- `Task.assignee` / `assignee_mut` return the view (`Window` onto the **child** common, `bit_base = 0`). `set_assignee` moves into the box; `copy_from` copies fields.
- `watchers` / `postal` still `&Address` / `&mut Address`.

**Still later:** oneof subtree clear, repeated / map as views, codegen, `Student<C>`.

---

## Task 4: unify singular catalog (`SharedMessage<M>`)

**Done.**

- [`NestedMessage`](../../puroro-rt/src/fields/wire/shared_message.rs) on owned `Point` / `Address`.
- [`SharedMessage<M, FIELD>`](../../puroro-rt/src/fields/wire/shared_message.rs): inline slot = `M::Body`, boxed slot = `UnmanagedBox<M>`, `View`/`Mut` always window + body.
- `Task.origin` and `Task.assignee` use the same marker. `ProtoMessage` remains for fixtures / `watchers` / `postal`.
- Mut-view `merge_from`, `FieldDeallocate` on catalog wrappers (`MessageBindingMut`), [`Window::nest`](../../puroro-rt/src/fields/shared/window.rs).
- Inlined Body teardown/clone: [`DeallocateBound`](../../puroro-rt/src/fields/shared/slot_bound.rs) / [`CloneBound`](../../puroro-rt/src/fields/shared/slot_bound.rs) on the extracted Body after `child_window` (not `NestedMessage` methods). After clear, `NestedMessage::BIT_COUNT` bits from `BIT_BASE` are zeroed. SSO does not impl those traits (`InlineOrHeap` + `HEAP_BIT`).
- Sample `School → Student → Point` / inlined `Address.home`: Student has local bits + inlined Point / Address; School inlines Student at `BIT_STUDENT_BASE`. Tests: two-hop round-trip, year bit does not collide with School `name`, location unknowns stay inside nested LEN, heap street clone/clear/`set_student`.

---

## Task 5: two-hop inline (`School → Student → Point`)

**Done.** See Task 4 notes.
