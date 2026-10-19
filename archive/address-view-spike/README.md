# Archived `AddressView` spike

This crate is a workspace member so the spike keeps compiling. It is not a
sample of `protoc-gen-puroro` output. [`sample-generated`](../../sample-generated/)
is that reference. The generator does not emit `AddressView`.

The spike scans one `Address` once, stores last-wins offsets into the caller's
buffer, and checks string UTF-8 on each get. It was measured and left here so
the comparison with eager `Address` and `AddressLazy` can be re-read later.
It is not scheduled generator work.

From the repository root:

```sh
cargo test -p address-view-spike
cargo bench -p address-view-spike --bench view
```

Criterion medians from one cloud VM (20 samples, 1 second each). Short is
about 30 bytes. Long is about 2 KiB (two 1024-byte ASCII strings).

| | short | long |
|---|---:|---:|
| eager decode | 89 ns | 1.58 µs |
| lazy decode | 201 ns | 352 ns |
| view parse | 34 ns | 33 ns |
| eager decode + street | 92 ns | 1.41 µs |
| lazy decode + street | 276 ns | 1.11 µs |
| view parse + street | 49 ns | 82 ns |
| view street, already built | 11 ns | 57 ns |
| eager street, already built | 2.2 ns | 1.3 ns |
