# puroro

**This crate is AI slop.** A model wrote it, another model approved it, and the diff still called that a review.

**Do not use this crate in most cases.**

This is **not an official Google product**. Google already publishes a production-quality Protocol Buffers implementation for Rust: [`google-protobuf`](https://crates.io/crates/google-protobuf). Its documentation is the [Rust reference](https://protobuf.dev/reference/rust/), and the source lives in [protocolbuffers/protobuf](https://github.com/protocolbuffers/protobuf).

puroro is an independent experiment. It is pre-1.0 and the API changes. Use `google-protobuf` unless you are working on puroro itself.

## What this repository is

`puroro` is the user-facing runtime for that experiment: [`Message`](https://github.com/wada314/puroro/blob/master/src/message.rs) (encode, decode, merge, validate, unknown fields), [`Optional`](https://github.com/wada314/puroro/blob/master/src/optional.rs) for explicit-presence fields (like `Option`, with a `[default = "foo"]` proto default), and the mutator traits generated code returns.

`.proto` files are compiled with the `protoc-gen-puroro` plugin (`protoc --puroro_out=…`). Install that crate's binary and leave it on `PATH`. Application code that only calls generated messages should depend on the generated crate and `puroro`. It should not import `puroro-rt`.

The interface specification is [DESIGN.md](https://github.com/wada314/puroro/blob/master/DESIGN.md). Generator internals are [IMPLEMENTATION.md](https://github.com/wada314/puroro/blob/master/IMPLEMENTATION.md).

## Status

- Nightly is pinned by [`rust-toolchain.toml`](https://github.com/wada314/puroro/blob/master/rust-toolchain.toml) because `puroro-codegen-tests` builds the plugin as a Cargo artifact dependency (`-Z bindeps`). Installing or running that binary does not use `bindeps`. The `puroro` library itself has no `#![feature]` flags. Edition is 2024.
- Every generated message is generic over one allocator `A: Allocator + Clone` from `allocator-api2`. `new()` uses `Global`.
- proto2, proto3, and editions share one accessor shape. Presence, packing, and enum open/closed follow the field's features.
- The default unknown-field policy preserves unrecognized fields and writes them back. `(puroro.unknown_fields) = REJECT` fails decode with `DecodeError::UnrecognizedField`. That option is outside the protobuf spec. A conforming parser preserves the bytes.
- There is no API compatibility with `prost`, `google-protobuf`, or `quick-protobuf`. Bytes on the wire still follow the protobuf encoding.

## Generate a crate

`protoc` must be on `PATH`. Install the plugin binary with stable Cargo (edition 2024, Rust 1.85 or newer). This does not use the unstable `bindeps` feature:

```shell
cargo install protoc-gen-puroro
```

`cargo install` builds the `protoc-gen-puroro` executable and copies it onto `PATH`. `protoc` runs that name when it sees `--puroro_out`.

The example message is `Shelf`. It is an editions message with one string, `title`. Editions allow `[default = "foo"]`, so an unset `title` still has the value `"foo"`.

```protobuf
edition = "2023";

package demo;

message Shelf {
  string title = 1 [default = "foo"];
}
```

```shell
mkdir -p demo-proto
protoc --puroro_out=demo-proto --puroro_opt=crate_name=demo-proto -I . shelf.proto
```

```text
shelf.proto
  edition = "2023"
  package demo
  message Shelf
    string title = 1 [default = "foo"]
            |
            |  protoc --puroro_out=demo-proto
            |         --puroro_opt=crate_name=demo-proto
            v
demo-proto/                         Cargo package name: demo-proto
├── Cargo.toml                      Rust crate name:    demo_proto
│     [dependencies]
│       puroro
│       puroro-rt
│       allocator-api2
│       bitvec
│       bytes
└── lib.rs
      mod demo                      package demo
        Shelf                       message Shelf
        ShelfLazy                   lazy alias of Shelf

from another crate:     demo_proto::demo::Shelf
from inside demo-proto: demo::Shelf

shelf.proto is the input file. It is not a Rust module.
```

Omit `crate_name` and the package name is `generated`. `protoc` overwrites `lib.rs` and `Cargo.toml` on the next run.

The `[dependencies]` in that `Cargo.toml` are the crates the generated `lib.rs` calls. The plugin writes the version number on each line. An application does not copy that list. It depends on `demo-proto` and `puroro` (next section). It does not depend on `puroro-rt`.

A proto2 `string` is `&[u8]` unless you pass `--puroro_opt=proto2_utf8=verify` (combine options as `crate_name=demo-proto,proto2_utf8=verify`). The `Shelf` field above is an editions string, so it is `&str`. Proto3 rejects `[default = …]`.

From a checkout of this repository, build the same binary and pass it to `protoc` directly:

```shell
cargo build -p protoc-gen-puroro
mkdir -p demo-proto
protoc --plugin=protoc-gen-puroro=$PWD/target/debug/protoc-gen-puroro --puroro_out=demo-proto --puroro_opt=crate_name=demo-proto -I . shelf.proto
```

The codegen tests in this repository do not install the binary. [`puroro-codegen-tests`](https://github.com/wada314/puroro/blob/master/puroro-codegen-tests/Cargo.toml) lists `protoc-gen-puroro` as an artifact build-dependency (`artifact = "bin"`). Cargo then sets `CARGO_BIN_FILE_PROTOC_GEN_PURORO`, and the test `build.rs` passes that path as `--plugin`. That still needs nightly `-Z bindeps`, enabled by [`.cargo/config.toml`](https://github.com/wada314/puroro/blob/master/.cargo/config.toml). It is how this workspace compiles the plugin for its own tests. A crates.io dependency cannot use the same artifact edge: Cargo does not set `CARGO_BIN_FILE_*` for a registry package.

## Use the generated crate from another crate

An application `Cargo.toml` names the generated package and `puroro`:

```toml
[package]
name = "app"
edition = "2024"

[dependencies]
demo-proto = { path = "../demo-proto" }
puroro = "0.15"
```

`Shelf` is the message in the diagram. `title()` returns [`Optional`](https://github.com/wada314/puroro/blob/master/src/optional.rs)`<&str, …>`. That type follows `std::option::Option`: `is_set()` reports whether the field was set, and `get()` returns that value. When `title` is absent, `get()` returns the editions default `"foo"`. `Optional` does not convert to `Option`. Field methods are inherent. Codec methods live on `puroro::Message`, so a proto field named `encode` does not hide them.

```rust
use demo_proto::demo::Shelf;
use ::puroro::{Message, StringMut};

let mut shelf = Shelf::new();
assert!(!shelf.title().is_set());
assert_eq!(shelf.title().get(), "foo");
shelf.title_mut().set("Shore");
assert!(shelf.title().is_set());
assert_eq!(shelf.title().get(), "Shore");

let bytes = shelf.encode_to_vec();
let decoded = Shelf::decode(&bytes[..]).unwrap();
assert_eq!(decoded.title().get(), "Shore");
```

`Shelf::new()` and `Shelf::decode` use the `Global` allocator. `StringMut` has to be in scope for `set`. Eager messages do not generate `has_title()`. `ShelfLazy` in the diagram has `Result` getters and no mutators. From this application that type is `demo_proto::demo::ShelfLazy`. The full accessor table is [DESIGN.md §4](https://github.com/wada314/puroro/blob/master/DESIGN.md#4-generated-code-specification).

## Out of scope

- Extensions (`extend` / proto2 extension fields).
- Services and RPC.
- Well-known types such as `google.protobuf.Any`, `Timestamp`, and `Duration`, unless you compile those `.proto` files yourself.
- A builder type separate from the message. Mutation is `let mut` plus `_mut` / `clear_*`.
- Treating `(puroro.unknown_fields) = REJECT` as spec-compliant behavior. The default preserve store is the spec behavior. `DISCARD` accepts the message and drops the bytes.

## License

[Apache License 2.0](https://github.com/wada314/puroro/blob/master/LICENSE).
