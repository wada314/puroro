# Wire buffer parameter

**Status: design + steps 1–2.** `WireBuf` / `WireAppend` live in
`puroro-rt/src/decode/wire_buf.rs`. `LazyScan<A, B = Bytes>` / `Lazy<A, B>` /
`LazyMessage<A, B>` store `B` directly. Default `B = Bytes` keeps
`TaskLazy<A>` call sites. `SharedWire` remains for its unit tests; lazy ingest
no longer uses it. Steps 3–4 (`parse_in`, borrowed getters) are not done.
There is no `*View` message type.

## Public type

```rust
pub struct TaskLazy<A: Allocator + Clone = Global, B: WireBuf = Bytes> {
    record: Record<TaskFields<A, Lazy<A, B>>, A, Lazy<A, B>, UnknownFields<A>>,
}
```

`TaskLazy<A>` means `TaskLazy<A, Bytes>`. Call sites that write `TaskLazy<A>`
do not change. Zero-copy is `TaskLazy<A, &'buf [u8]>`. The allocator stays the
first parameter so the owned name does not grow a lifetime. That argument
order is an accepted cost of one struct name.

`Message::decode` stays on `TaskLazy<A, Bytes>`. It copies the input into
`Bytes`. `Message` and the lazy arm of `forward_record!` require `new_in` and
`merge_from`, so they are implemented only for `B: WireAppend`. The borrowed
message does not implement `Message`. Its surface is `parse_in` / `parse` and
`LazyMessage`.

```rust
impl<'buf, A: Allocator + Clone> TaskLazy<A, &'buf [u8]> {
    pub fn parse_in(buf: &'buf [u8], alloc: A) -> Result<Self, DecodeError>;
}

impl<'buf> TaskLazy<Global, &'buf [u8]> {
    pub fn parse(buf: &'buf [u8]) -> Result<Self, DecodeError> {
        Self::parse_in(buf, Global)
    }
}
```

`parse` is a separate impl for `A = Global`. Putting it on `impl<'buf, A>`
does not type-check, because `Global` is not an arbitrary `A`.

`parse_in` scans once. It does not copy `buf` into `Bytes`. Slots and the
scan `Vec`s use `alloc`. Nested apply still builds a child with `DefaultIn`
and then `merge_shared`.

## Trait

Lives in `puroro-rt/src/decode/wire_buf.rs`. Re-exported from `decode`.

```rust
pub trait WireBuf: Clone {
    /// One contiguous subrange. Does not borrow `self`.
    ///
    /// `Slice::Slice` is `Slice`, so a nested subrange stops changing type.
    type Slice: WireBuf<Slice = Self::Slice>;

    fn as_bytes(&self) -> &[u8];

    fn subslice(&self, span: WireSpan) -> Result<Self::Slice, DecodeError>;
}

/// Buffers that can grow. `&[u8]` does not implement this.
pub trait WireAppend: WireBuf {
    fn empty() -> Self;
    fn append(&mut self, chunk: &[u8]);
}
```

`subslice` returns an owned handle. For `&'buf [u8]` the implementation
passes `*self` into `WireSpan::slice`, so the result is `&'buf [u8]`, not a
borrow of the method receiver. For `Bytes` it is `Bytes::slice`.

```rust
impl<'buf> WireBuf for &'buf [u8] {
    type Slice = &'buf [u8];

    fn as_bytes(&self) -> &[u8] {
        self
    }

    fn subslice(&self, span: WireSpan) -> Result<&'buf [u8], DecodeError> {
        span.slice(*self)
    }
}

impl WireBuf for Bytes {
    type Slice = Bytes;

    fn as_bytes(&self) -> &[u8] {
        self
    }

    fn subslice(&self, span: WireSpan) -> Result<Bytes, DecodeError> {
        let end = span.offset.checked_add(span.len).ok_or(DecodeError::TruncatedMessage)?;
        Ok(self.slice(span.offset..end))
    }
}

impl WireAppend for Bytes {
    fn empty() -> Self {
        Bytes::new()
    }

    fn append(&mut self, chunk: &[u8]) {
        // Same COW as SharedWire::append: BytesMut::from(take) + extend + freeze.
    }
}
```

`A` is not a parameter of the buffer. Field vecs and `MessageCommon` already
carry it. `Bytes` stays on the global allocator, as `SharedWire` does today.

## Where the buffer sits

`LazyScan<A, B: WireBuf>` stores `wire: B`. Regions, policy spans, and the
scanner leftover stay `AllocVec<_, A>`. Offsets in `WireSpan` stay indexes
into that `B`.

```rust
pub struct Lazy<A: Allocator = Global, B: WireBuf = Bytes> {
    pub scan: LazyScan<A, B>,
}
```

`impl<A> Layout<A> for Lazy<A>` becomes `impl<A, B: WireBuf> Layout<A> for Lazy<A, B>`.
With the default `B = Bytes`, the owned layout is the current one.

Empty `regions` means the whole `wire` is the message body, as
`LazyScan::body_len` does today. A root from `open` keeps `regions` empty:
`wire` is that message. A child clones the parent's full `B` and records one
or more absolute `WireSpan`s. Those `regions` are non-empty. An adopted child
whose `regions` stayed empty would encode the entire parent buffer.

The child does not store `B::Slice`. One singular child can contain several
LEN occurrences, and those ranges are not one contiguous slice. `Clone` on
`&'buf [u8]` copies the fat pointer. `Clone` on `Bytes` bumps the refcount.
Both keep the parent's offsets valid. `SharedWire::append_after_clone_does_not_mutate_peer`
is the owned case of that rule: a later append on the parent copies, and the
child keeps the previous bytes. For the two buffers shipped here,
`Slice = Self`, so the child type written as `AddressLazy<A, B>` is the same
buffer type as the parent.

`type Slice` stays on `WireBuf`. v1 does not add a rope, and both impls use
`Slice = Self`. The associated type is what lets a later rope return `Bytes`
from `subslice` without changing the trait. Children still do not store it.

`subslice` is for one contiguous range: a string, a bytes field, or a caller
that wants a single span as its own handle. A rope whose `Slice` is `Bytes`
can implement the same trait later. This design does not add that rope.

`SharedWire<A>`'s `Bytes` operations move onto `WireBuf` / `WireAppend`.
`clone_handle` is `Clone`. `adopt` stores that clone. The wrapper goes away
once `LazyScan` stores `B` directly. `A` on `SharedWire` is not the allocation
of the bytes; the message already has `A`.

## Getters

Two inherent impls exist only for singular and oneof string / bytes, because
the return lifetime differs. Numericals, enums, bools, and child messages stay
on one `impl<A, B: WireBuf>`. Repeated string / bytes and maps stay span lists
and materialize on get. Zero-copy on the borrowed message is singular and
oneof string / bytes only.

`TaskLazy<A, Bytes>` keeps today's getters. `title()` returns
`Optional<&str>` borrowed from `&self`, through `try_str` and `WireOrSso`.
`WireOrSso::get_str` promotes and sticks `Failed`. A later get on that arm
returns `InvalidUtf8` and does not read the wire again.

`TaskLazy<A, &'buf [u8]>` is the second impl for those fields. `title()`
returns `Optional<&'buf str>`. The body copies the stored `&'buf [u8]` out of
the scan, slices the span, and checks UTF-8 on every get. That `&str` does
not borrow `&self`. This path does not promote into `WireOrSso` and does not
store a sticky `Failed` arm: the bytes stay in the caller's buffer, so each
get validates them again.

`into_eager`, `PartialEq`, and `Debug` stay on the current lazy rules. When
building the eager message fails, including `InvalidUtf8`, equality compares
ingested body bytes. One success and one failure is unequal. `Debug` prints
the eager message, or the non-exhaustive stub when that build fails.

```rust
impl<'buf, A: Allocator + Clone> TaskLazy<A, &'buf [u8]> {
    pub fn title(&self) -> Result<Optional<&'buf str, impl HasDefault<&'buf str>>, DecodeError> {
        // ensure_scanned, then WireSpan::slice on the copied &'buf [u8]
    }
}
```

Bytes getters follow the same split. `payload()` on the borrowed message
returns `&'buf [u8]`.

The sample writes both impls by hand. The plugin emits both once that sample
matches the current owned tests and the zero-copy `parse` tests.

## Ingest

```rust
impl<A: Allocator + Clone, B: WireBuf> LazyScan<A, B> {
    /// Scan a finished buffer. Does not append and does not copy `wire`.
    pub fn open(&mut self, wire: B) -> Result<AllocVec<ScannedRecord, A>, DecodeError>;
}
```

`open` stores `wire`, runs `RecordScanner::push` on `as_bytes()`, then
`finish`. It does not call `LazyScan::push`. That method appends the chunk
after a successful scan, which a borrowed buffer cannot do. The scanner
copies an incomplete tail into its leftover `Vec`. A successful `finish`
leaves that leftover empty, so a complete buffer is not copied. `open` on a
root leaves `regions` empty.

Groups stay out of scope for a borrowed `B`. The plugin still rejects
`TYPE_GROUP` and `message_encoding = DELIMITED`.

`push`, `ingest_buf`, and `Message::merge_from` need `WireAppend`. They stay
on `TaskLazy<A, Bytes>` only.

```rust
pub trait LazyMessage<A: Allocator + Clone, B: WireBuf = Bytes> {
    fn scan(&self) -> &LazyScan<A, B>;
    fn scan_mut(&mut self) -> &mut LazyScan<A, B>;
    // ensure_scanned, encode, finish. No push.
}

pub trait LazyAppend<A: Allocator + Clone, B: WireAppend>: LazyMessage<A, B> {
    fn push(&mut self, chunk: &[u8]) -> Result<(), DecodeError>;
    fn ingest_buf<T: Buf>(&mut self, buf: &mut T) -> Result<(), DecodeError>;
}
```

`TaskLazy<A, &'buf [u8]>` implements `LazyMessage` and not `LazyAppend`.
It has no public `new_in`. `TaskLazy<A, Bytes>::new_in` uses
`WireAppend::empty`.

A nested slot still needs a placeholder before `adopt`. `DefaultIn` is a
public trait, and nested construction uses `DefaultIn::default_in`. For
`B = &'buf [u8]` that value has `wire = &[]`, which coerces to `'buf`. It is
the slot placeholder, not a root API. A caller who wants a message from a
buffer uses `parse_in`. `adopt` then replaces that wire with a clone of the
parent buffer and records a non-empty region.

## Steps

1. Add `WireBuf` / `WireAppend` and the `&[u8]` and `Bytes` impls, with tests
   for `'buf` on `subslice` and for COW `append`.
2. Store `B` on `LazyScan<A, B>`. Default `B = Bytes` keeps `TaskLazy<A>`
   behavior, including `push` and `Message::decode`.
3. Add `open` and `TaskLazy<A, &'buf [u8]>::parse_in`, plus the borrowed
   getters on the handwritten sample.
4. Add zero-copy tests on `parse`. There is no `*View` message type.

## Which document wins

This file is the design for the unshipped buffer parameter. `DESIGN.md` §7
still says there is no zero-copy decode, and §8 still names `TaskView<'buf>`
as not part of the eager / lazy pair. `IMPLEMENTATION.md` still describes the
shipped lazy alias. Those two stay the description of what is implemented.
The implementation PR updates them when `TaskLazy<A, &'buf [u8]>` exists.
Until then, a reviewer of this design reads this file.
