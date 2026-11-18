# Wire buffer parameter

**Status: design.** Not implemented. There is no `*View` message type.
`TaskLazy<A>` stays the owned lazy message. Zero-copy is the same struct with
a second type parameter.

## Public type

```rust
pub struct TaskLazy<A: Allocator + Clone = Global, B: WireBuf = Bytes> {
    record: Record<TaskFields<A, Lazy<A, B>>, A, Lazy<A, B>, UnknownFields<A>>,
}
```

`TaskLazy<A>` means `TaskLazy<A, Bytes>`. Call sites that write `TaskLazy<A>`
do not change. Zero-copy is `TaskLazy<A, &'buf [u8]>`.

`Message::decode` stays on `TaskLazy<A, Bytes>`. It copies the input into
`Bytes`. `TaskLazy<A, &'buf [u8]>` is built with `parse`:

```rust
impl<'buf, A: Allocator + Clone> TaskLazy<A, &'buf [u8]> {
    pub fn parse(buf: &'buf [u8]) -> Result<Self, DecodeError>;
}
```

`parse` scans once. It does not copy `buf` into `Bytes`.

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

A child message holds a **clone of the same `B`** plus one or more
`WireSpan`s (the existing `regions` list). It does not store `B::Slice`.
One singular child can contain several LEN occurrences, and those ranges are
not one contiguous slice. `Clone` on `&'buf [u8]` copies the fat pointer.
`Clone` on `Bytes` bumps the refcount. Both keep the parent's offsets valid.
For the two buffers shipped here, `Slice = Self`, so the child type written
as `AddressLazy<A, B>` is the same buffer type as the parent.

`subslice` is for one contiguous range: a string, a bytes field, or a caller
that wants a single span as its own handle. A rope whose `Slice` is `Bytes`
can implement the same trait later. This design does not add that rope.

`SharedWire<A>`'s `Bytes` operations move onto `WireBuf` / `WireAppend`.
`clone_handle` is `Clone`. `adopt` stores that clone. The wrapper goes away
once `LazyScan` stores `B` directly. `A` on `SharedWire` is not the allocation
of the bytes; the message already has `A`.

## Getters

`TaskLazy<A, Bytes>` keeps today's getters. `title()` returns
`Optional<&str>` borrowed from `&self`, through `try_str` and `WireOrSso`.

`TaskLazy<A, &'buf [u8]>` is a second impl. `title()` returns
`Optional<&'buf str>`. The body copies the stored `&'buf [u8]` out of the
scan, `subslice`s the span, and checks UTF-8. That `&str` does not borrow
`&self`. This path does not promote into `WireOrSso`: the bytes stay in the
caller's buffer.

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
`finish`. The scanner copies an incomplete tail into its leftover `Vec`.
A successful `finish` leaves that leftover empty, so a complete buffer is
not copied.

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
It has no `new_in`: an empty value would need a buffer, and `&'buf [u8]`
cannot invent one. `TaskLazy<A, Bytes>::new_in` uses `WireAppend::empty`.

## Steps

1. Add `WireBuf` / `WireAppend` and the `&[u8]` and `Bytes` impls, with tests
   for `'buf` on `subslice` and for COW `append`.
2. Store `B` on `LazyScan<A, B>`. Default `B = Bytes` keeps `TaskLazy<A>`
   behavior, including `push` and `Message::decode`.
3. Add `open` and `TaskLazy<A, &'buf [u8]>::parse`, plus the borrowed getters
   on the handwritten sample.
4. Add zero-copy tests on `parse`. There is no `*View` message type.
