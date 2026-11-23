# LazyProfile (manifest) parameter

**Status: Phase A landed (runtime + sample + plugin).** Replace `TaskLazy<A, B: WireBuf>` with
`TaskLazy<A, P: LazyProfile = Owned>`. One profile bundles the island buffer
and the singular string/bytes read policy.

## Public shape

```rust
pub trait LazyProfile: Clone {
    type Buf: WireBuf;

    /// String view for a `&'msg` borrow of the message.
    /// `Owned`: `&'msg str`. `Borrowed<'buf>`: `&'buf str` (`'msg` unused).
    fn get_str<'msg>(...) -> Result<&'msg str, DecodeError>;
    fn get_bytes<'msg>(...) -> Result<&'msg [u8], DecodeError>;
}

pub struct Owned;
pub struct Borrowed<'buf>(PhantomData<&'buf [u8]>);

pub struct TaskLazy<A = Global, P: LazyProfile = Owned> {
    record: Record<TaskFields<A, Lazy<A, P>>, A, Lazy<A, P>, UnknownFields<A>>,
}
```

`TaskLazy<A>` stays the owned name. Zero-copy is `TaskLazy<A, Borrowed<'buf>>` (alias optional later).

Public string/bytes getters return `&'msg str` / `&'msg [u8]` tied to
`&self` (Borrowed peeks, then shortens to the message borrow).

`LazyScan` keeps `B: WireBuf`. `Lazy<A, P>` stores `LazyScan<A, P::Buf>`.
`Message` / `push` require `P::Buf: WireAppend`.

## Phases

1. **This PR:** Profile replaces `B` on `*Lazy` / `Lazy` / plugin; unified
   `title()` / etc. via `P::read_str`. Both profiles still use `WireOrSso`
   slots (`Owned` → `try_*`, `Borrowed` → `peek_*`).
2. **Follow-up:** `type StrStore` on the profile so `Borrowed` (or a
   `NoCache` profile) uses span-only slots with no Inline/Heap/Failed bits.

## Non-goals

- GAT-free `&str` in fully generic signatures
- Rope / `&Vec<u8>` as `Buf`
- Public `as_borrowed` dual getters
