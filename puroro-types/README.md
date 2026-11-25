# puroro-types

Generated `google.protobuf` message types for **puroro**: descriptor messages
and common well-known types (WKTs).

**This is not an official Google product.** For production Rust protobuf code,
prefer [`google-protobuf`](https://crates.io/crates/google-protobuf).

## Features

| Feature | Default | Contents |
|---|---|---|
| `descriptors` | yes | `FileDescriptorSet`, `FileDescriptorProto`, `DescriptorProto`, … |
| `wkt` | yes | `Any`, `Duration`, `Empty`, `FieldMask`, `Struct` / `Value`, `Timestamp`, wrappers |

Disable what you do not need to shrink the dependency graph of generated code
pulled into your binary (`default-features = false`, then enable one feature).

## Layout

Types live under `google::protobuf::…`, matching the protobuf package. The
crate re-exports the runtime as `puroro` (same as other plugin output).

Sources under `proto/google/protobuf/` are vendored from
[protocolbuffers/protobuf](https://github.com/protocolbuffers/protobuf)
(BSD-style license; see `proto/LICENSE` and each file header). The pin lives in
[`proto/UPSTREAM`](proto/UPSTREAM) (currently **v24.4** /
`7789b3ac85248ad75631a1919071fa268e466210`). Refresh with:

```sh
./puroro-types/scripts/vendor-google-protos.sh          # same tag as UPSTREAM
./puroro-types/scripts/vendor-google-protos.sh v32.1    # bump the pin
```

Do not edit the `.proto` files by hand. After a bump, run `cargo test -p puroro-types`
(needs `protoc` on `PATH`, or `PROTOC=`). `build.rs` regenerates Rust via
`protoc-gen-puroro`.

## Dependency note

This crate depends on `puroro` and `puroro-rt`. Those crates do **not** depend
on `puroro-types`, so there is no cycle. Reflection / `DynamicMessage` that
consume these types should live in a higher crate later, not in `puroro`.
