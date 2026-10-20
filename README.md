# puroro

**This crate is AI slop.** A model wrote it, another model approved it, and the diff still called that a review.

**Do not use this crate in most cases.**

This is **not an official Google product**. Google already publishes a production-quality Protocol Buffers implementation for Rust: [`google-protobuf`](https://crates.io/crates/google-protobuf). Its documentation is the [Rust reference](https://protobuf.dev/reference/rust/), and the source lives in [protocolbuffers/protobuf](https://github.com/protocolbuffers/protobuf).

puroro is an independent experiment. It is pre-1.0 and the API changes. Use `google-protobuf` unless you are working on puroro itself.

## What this repository is

`puroro` is the user-facing runtime for that experiment: [`Message`](https://github.com/wada314/puroro/blob/master/src/message.rs) (encode, decode, merge, validate, unknown fields), [`Optional`](https://github.com/wada314/puroro/blob/master/src/optional.rs) for explicit-presence fields, and the mutator traits generated code returns.

`.proto` files are compiled with the `protoc-gen-puroro` plugin (`protoc --puroro_out=…`). Install that crate's binary and leave it on `PATH`. Application code that only calls generated messages should depend on the generated crate and `puroro`. It should not import `puroro-rt`.

The interface specification is [DESIGN.md](https://github.com/wada314/puroro/blob/master/DESIGN.md). Generator internals are [IMPLEMENTATION.md](https://github.com/wada314/puroro/blob/master/IMPLEMENTATION.md).

## This tree is 0.15, not the 0.14 crate page

[crates.io `puroro` 0.14.1](https://crates.io/crates/puroro) is the previous line. Its packaged README describes `puroro-codegen`, a `build.rs` that writes `.rs` files into `src/`, and a `Foo` / `FooView` split. None of that exists here.

This repository is **0.15.0**. It is not published yet. `puroro = "0.14"` does not build code from this tree, and `puroro = "0.15"` will not resolve until a 0.15 release exists. Until then, depend on a checkout (see below).

[docs.rs/puroro](https://docs.rs/puroro/) likewise documents 0.14.1 until 0.15 is published.

## Status

- Nightly is pinned by [`rust-toolchain.toml`](https://github.com/wada314/puroro/blob/master/rust-toolchain.toml) because the workspace enables Cargo `-Z bindeps`. The `puroro` library itself has no `#![feature]` flags. Edition is 2024.
- Every generated message is generic over one allocator `A: Allocator + Clone` from `allocator-api2`. `Global` is the ordinary choice (`Book::new()`).
- proto2, proto3, and editions share one accessor shape. Presence, packing, and enum open/closed follow the field's features.
- The default unknown-field policy preserves unrecognized fields and writes them back. `(puroro.unknown_fields) = REJECT` fails decode with `DecodeError::UnrecognizedField`. That option is outside the protobuf spec. A conforming parser preserves the bytes.
- There is no API compatibility with `prost`, `google-protobuf`, or `quick-protobuf`. Bytes on the wire still follow the protobuf encoding.

## Generate code

`protoc` must be on `PATH`. Install the plugin:

```shell
cargo install protoc-gen-puroro
```

```protobuf
syntax = "proto3";
package library;

message Book {
  optional string title = 1;
  uint32 pages = 2;
}
```

```shell
mkdir -p gen
protoc --puroro_out=gen -I . book.proto
```

`protoc` runs `protoc-gen-puroro` from `PATH` when it sees `--puroro_out`.

The plugin writes a single `gen/lib.rs`. The `.proto` file name is not a Rust module name. `package library` becomes `library`, and `message Book` becomes `library::Book`.

For a proto2 file, string fields default to unchecked bytes (`&[u8]`). Pass `--puroro_opt=proto2_utf8=verify` when those fields should be `&str`.

Generated `lib.rs` also depends on `puroro-rt`. That crate is not published. Point it at a checkout of this repository; a crates.io version constraint cannot replace the path dependency.

```toml
[package]
name = "library-proto"
edition = "2024"

[dependencies]
allocator-api2 = "0.2"
bitvec = "1"
bytes = "1"
puroro = { path = "../puroro" }
puroro-rt = { path = "../puroro/puroro-rt" }
```

`lib.rs` names `allocator-api2`, `bitvec`, and `bytes` directly, so they have to be direct dependencies even though `puroro` also uses them.

## Use a generated message

Field access is inherent methods on the concrete type. Codec methods live on `puroro::Message`, so a proto field named `encode` does not hide them; call those with the trait in scope.

`optional string` is explicit presence: `title()` returns `Optional<&str, …>`, `title().get()` is the value or the proto default, and `title().is_set()` reports presence. Eager messages do not generate `has_title()`. A plain proto3 `uint32` is implicit: `pages()` returns `u32`.

```rust
use library::Book;
use ::puroro::{Message, StringMut};

let mut book = Book::new();
book.title_mut().push_str("Shore");
*book.pages_mut() = 320;
assert!(book.title().is_set());
assert_eq!(book.title().get(), "Shore");
assert_eq!(book.pages(), 320);

let bytes = book.encode_to_vec();
let decoded = Book::decode(&bytes[..]).unwrap();
assert_eq!(decoded.title().get(), "Shore");
```

This snippet is code inside the generated crate. `StringMut` has to be in scope for `push_str`. The same pattern is covered by the in-tree [`scalars` fixture](https://github.com/wada314/puroro/blob/master/puroro-codegen-tests/fixtures/scalars/test.rs).

Each message also has a lazy alias (`BookLazy`) with `Result` getters and no mutators. The full accessor table is [DESIGN.md §4](https://github.com/wada314/puroro/blob/master/DESIGN.md#4-generated-code-specification).

## Out of scope

- Extensions (`extend` / proto2 extension fields).
- Services and RPC.
- Well-known types such as `google.protobuf.Any`, `Timestamp`, and `Duration`, unless you compile those `.proto` files yourself.
- A builder type separate from the message. Mutation is `let mut` plus `_mut` / `clear_*`.
- Treating `(puroro.unknown_fields) = REJECT` as spec-compliant behavior. The default preserve store is the spec behavior. `DISCARD` accepts the message and drops the bytes.

## License

[Apache License 2.0](https://github.com/wada314/puroro/blob/master/LICENSE).
