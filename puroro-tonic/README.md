# puroro-tonic

[`tonic`](https://docs.rs/tonic) [`Codec`](https://docs.rs/tonic/latest/tonic/codec/trait.Codec.html) for
[`puroro`](https://crates.io/crates/puroro) messages.

This crate does **not** implement HTTP/2, gRPC framing, or service/stub codegen.
It only maps `puroro::Message` encode / decode onto tonic's codec traits so a
gRPC runtime can carry puroro types.

```rust,ignore
use puroro_tonic::PuroroCodec;

let codec = PuroroCodec::<Request, Response>::default();
```

Service traits and method paths still need a generator (or hand-written stubs)
that names this codec. That work is separate from this adapter.
