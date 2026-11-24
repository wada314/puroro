# LazyProfile (manifest) parameter

**Status: Phase B landed (runtime + sample + plugin).** Replace `TaskLazy<A, B: WireBuf>` with
`TaskLazy<A, P: LazyProfile = Owned>`. One profile bundles the island buffer
and the singular string/bytes slot layout.

## Public shape

```rust
pub trait LazyProfile: Clone {
    type Buf: WireBuf;

    /// `KIND` is the WireOrSso kind-bit index; SpanOnly ignores it.
    type StringLen<A: Allocator + Clone, const KIND: usize>: LazyStringLen<A>;
    type BytesLen<A: Allocator + Clone, const KIND: usize>: LazyBytesLen<A>;
}

pub struct Owned;   // Buf = Bytes, StringLen/BytesLen = WireOrSso<KIND>
pub struct Borrowed<'buf>(PhantomData<&'buf [u8]>); // Buf = &[u8], … = SpanOnly

pub struct TaskLazy<A = Global, P: LazyProfile = Owned> {
    record: Record<TaskFields<A, Lazy<A, P>>, A, Lazy<A, P>, UnknownFields<A>>,
}
```

`TaskLazy<A>` stays the owned name. Zero-copy is `TaskLazy<A, Borrowed<'buf>>`.

Public string/bytes getters return `&'msg str` / `&'msg [u8]` tied to
`&self` via layout `profile_str` / `profile_bytes` (`LazyStringGet` /
`LazyBytesGet`). Oneof install uses `StringSlotNew` / `BytesSlotNew`.
`LazyStringLen` / `LazyBytesLen` require `Slot: DefaultIn` so proto3
`Implicit` can construct through abstract `P::StringLen`.

`LazyScan` keeps `B: WireBuf`. `Lazy<A, P>` stores `LazyScan<A, P::Buf>`.
`Message` / `push` require `P::Buf: WireAppend`.

## Phases

1. **Phase A:** Profile replaces `B` on `*Lazy` / `Lazy` / plugin; unified
   getters via `P::get_str`. Both profiles still used `WireOrSso`.
2. **Phase B (this):** `StringLen` / `BytesLen` on the profile. `Owned` keeps
   `WireOrSso`; `Borrowed` uses `SpanOnly` (no Inline/Heap/Failed). Kind bit
   allocations stay for v1 (`SpanOnly` ignores `KIND`).

## Follow-ups

- [ ] Bit-planner shrink: drop the two kind bits per singular LEN when the
  profile is span-only (`Borrowed` / future `NoCache`), so `Borrowed` messages
  do not permanently pay unused bits.

## Non-goals

- GAT-free `&str` in fully generic signatures
- Rope / `&Vec<u8>` as `Buf`
- Public `as_borrowed` dual getters
