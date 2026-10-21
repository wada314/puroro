# puroro

**This crate is AI slop.** A model wrote it, another model approved it, and the diff still called that a review.

**Do not use this crate in most cases.**

This is **not an official Google product**. Google already publishes a production-quality Protocol Buffers implementation for Rust: [`google-protobuf`](https://crates.io/crates/google-protobuf). Its documentation is the [Rust reference](https://protobuf.dev/reference/rust/), and the source lives in [protocolbuffers/protobuf](https://github.com/protocolbuffers/protobuf).

puroro is an independent experiment. It is pre-1.0 and the API changes. Use `google-protobuf` unless you are working on puroro itself.

## What this repository is

`puroro` is the user-facing runtime for that experiment: [`Message`](https://github.com/wada314/puroro/blob/master/src/message.rs) (encode, decode, merge, validate, unknown fields), [`Optional`](https://github.com/wada314/puroro/blob/master/src/optional.rs) for explicit-presence fields (like `Option`, with a `[default = "foo"]` proto default), and the mutator traits generated code returns.

`.proto` files are compiled with the `protoc-gen-puroro` plugin (`protoc --puroro_out=…`). Install that crate's binary and leave it on `PATH`. Application code depends on the generated crate. Traits such as `Message` and `StringMut` are in that crate's `puroro` module. It should not import `puroro-rt`.

The interface specification is [DESIGN.md](https://github.com/wada314/puroro/blob/master/DESIGN.md). Generator internals are [IMPLEMENTATION.md](https://github.com/wada314/puroro/blob/master/IMPLEMENTATION.md).

## Status

- The `puroro` library has no `#![feature]` flags. Edition is 2024. The toolchain for this checkout is [`rust-toolchain.toml`](https://github.com/wada314/puroro/blob/master/rust-toolchain.toml).
- Every generated message is generic over one allocator `A: Allocator + Clone` from `allocator-api2`. `new()` uses `Global`.
- proto2, proto3, and editions share one accessor shape. Presence, packing, and enum open/closed follow the field's features.
- The default unknown-field policy preserves unrecognized fields and writes them back. `(puroro.unknown_fields) = REJECT` fails decode with `DecodeError::UnrecognizedField`. That option is outside the protobuf spec. A conforming parser preserves the bytes.
- There is no API compatibility with `prost`, `google-protobuf`, or `quick-protobuf`. Bytes on the wire still follow the protobuf encoding.

## Generate a crate

`protoc` must be on `PATH`. Install the plugin binary with stable Cargo (edition 2024, Rust 1.85 or newer):

```shell
cargo install protoc-gen-puroro
```

`cargo install` builds the `protoc-gen-puroro` executable and copies it onto `PATH`. `protoc` runs that program when it sees `--puroro_out`.

The example message is `Shelf`. It is an editions message with one string, `title`. Editions allow `[default = "foo"]`, so an unset `title` still has the value `"foo"`.

```protobuf
edition = "2023";

package demo;

message Shelf {
  string title = 1 [default = "foo"];
}
```

`bookstore` in the command below is the directory and the Cargo package name for this example. `--puroro_out` takes whatever directory you pass. `--puroro_opt=crate_name=` sets the package name. Leave `crate_name` out and that name is `generated`. A hyphen in `crate_name` becomes an underscore in Rust (`demo-proto` becomes `demo_proto`).

```shell
mkdir -p bookstore
protoc --puroro_out=bookstore --puroro_opt=crate_name=bookstore -I . shelf.proto
```

```text
shelf.proto
  edition = "2023"
  package demo
  message Shelf
    string title = 1 [default = "foo"]
            |
            |  protoc --puroro_out=bookstore
            |         --puroro_opt=crate_name=bookstore
            v
bookstore/                          directory from --puroro_out
├── Cargo.toml                      package name from crate_name
│     [dependencies]
│       puroro
│       puroro-rt
│       allocator-api2
│       bitvec
│       bytes
└── lib.rs
      puroro                        re-export of the puroro crate
      mod demo                      package demo
        Shelf                       message Shelf
        ShelfLazy                   lazy alias of Shelf

from another crate:       bookstore::demo::Shelf
                          bookstore::puroro::Message
from code in bookstore:   demo::Shelf

shelf.proto is the input file. It is not a Rust module.
```

`protoc` overwrites `lib.rs` and `Cargo.toml` on the next run. The `[dependencies]` in that `Cargo.toml` are the crates `lib.rs` calls. The plugin writes the version on each line. The root module `puroro` is that runtime crate. A schema that would place a module or type named `puroro` at that root is rejected. `package shop.puroro` is a different module (`shop::puroro`). Which other proto names are accepted is [DESIGN.md](DESIGN.md#allowed-and-disallowed-names).

A proto2 `string` is `&[u8]` unless you pass `--puroro_opt=proto2_utf8=verify`. `title` above is an editions string, so it is `&str`. Proto3 rejects `[default = …]`.

## Use the generated crate from another crate

The application crate adds the generated package. The path is the `--puroro_out` directory. It does not add `puroro`.

```toml
[dependencies]
bookstore = { path = "../bookstore" }
```

`Shelf::new`, `title`, `get`, and `is_set` are methods on `Shelf`. `encode_to_vec`, `decode`, and `set` are methods of traits in `bookstore::puroro`. `title()` returns [`Optional<&str, …>`](https://github.com/wada314/puroro/blob/master/src/optional.rs). That type follows `std::option::Option`: `is_set()` reports whether the field was set, and `get()` returns that value. When `title` is absent, `get()` returns the editions default `"foo"`. `Optional` does not convert to `Option`, because that conversion drops the default. `None` would only record that the field is absent, and `"foo"` would be gone. If the `.proto` file has a field `encode`, call the codec method as `Message::encode(&shelf)`. Another generated crate re-exports the same `puroro` when Cargo resolves one `puroro` version. Import `Message` from one of those crates.

```rust
use bookstore::demo::Shelf;
use bookstore::puroro::{Message, StringMut};

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

`Shelf::new()` and `Shelf::decode` use the `Global` allocator. `StringMut` has to be in scope for `set`. Eager messages do not generate `has_title()`. `ShelfLazy` in the diagram has `Result` getters and no mutators. From this application that type is `bookstore::demo::ShelfLazy`. The full accessor table is [DESIGN.md §4](https://github.com/wada314/puroro/blob/master/DESIGN.md#4-generated-code-specification).

## Build the plugin from a checkout

This is for people changing puroro. The install command above is enough for other users.

```shell
cargo build -p protoc-gen-puroro
mkdir -p bookstore
protoc --plugin=protoc-gen-puroro=$PWD/target/debug/protoc-gen-puroro --puroro_out=bookstore --puroro_opt=crate_name=bookstore -I . shelf.proto
```

## Out of scope

- Extensions (`extend` / proto2 extension fields).
- Services and RPC.
- Well-known types such as `google.protobuf.Any`, `Timestamp`, and `Duration`, unless you compile those `.proto` files yourself.
- A builder type separate from the message. Mutation is `let mut` plus `_mut` / `clear_*`.
- Treating `(puroro.unknown_fields) = REJECT` as spec-compliant behavior. The default preserve store is the spec behavior. `DISCARD` accepts the message and drops the bytes.

## License

[Apache License 2.0](https://github.com/wada314/puroro/blob/master/LICENSE).
