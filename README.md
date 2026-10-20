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

[crates.io `puroro` 0.14.1](https://crates.io/crates/puroro) is the previous line. Its packaged README describes `puroro-codegen`, a `build.rs` that writes `.rs` files into `src/`, and a `Foo` / `FooView` split. None of that exists here. 0.14 called the generator in-process from that `build.rs`. It did not install a `protoc` plugin, and it did not use Cargo `-Z bindeps` to obtain a binary. The `bindeps` line in the old workspace config was unused after [`protoc-plugin-by-closure`](https://github.com/wada314/protoc-plugin-by-closure) moved to its own repository.

This repository is **0.15.0**, with `puroro-rt` **0.1.0**. `puroro = "0.14"` does not build code from this tree. `puroro = "0.15"` and `puroro-rt = "0.1"` resolve after those releases are uploaded.

[docs.rs/puroro](https://docs.rs/puroro/) likewise documents 0.14.1 until 0.15 is published.

## Status

- Nightly is pinned by [`rust-toolchain.toml`](https://github.com/wada314/puroro/blob/master/rust-toolchain.toml) because `puroro-codegen-tests` builds the plugin as a Cargo artifact dependency (`-Z bindeps`). Installing or running that binary does not use `bindeps`. The `puroro` library itself has no `#![feature]` flags. Edition is 2024.
- Every generated message is generic over one allocator `A: Allocator + Clone` from `allocator-api2`. `Global` is the ordinary choice (`Book::new()`).
- proto2, proto3, and editions share one accessor shape. Presence, packing, and enum open/closed follow the field's features.
- The default unknown-field policy preserves unrecognized fields and writes them back. `(puroro.unknown_fields) = REJECT` fails decode with `DecodeError::UnrecognizedField`. That option is outside the protobuf spec. A conforming parser preserves the bytes.
- There is no API compatibility with `prost`, `google-protobuf`, or `quick-protobuf`. Bytes on the wire still follow the protobuf encoding.

## Generate a crate

`protoc` must be on `PATH`. Install the plugin binary with stable Cargo (edition 2024, Rust 1.85 or newer). This does not use the unstable `bindeps` feature:

```shell
cargo install protoc-gen-puroro
```

`cargo install` builds the `protoc-gen-puroro` executable and copies it onto `PATH`. `protoc` runs that name when it sees `--puroro_out`.

```protobuf
syntax = "proto3";
package library;

message Book {
  optional string title = 1;
  uint32 pages = 2;
}
```

```shell
mkdir -p library-proto
protoc --puroro_out=library-proto --puroro_opt=crate_name=library-proto -I . book.proto
```

The output directory is a Cargo package. The plugin writes `library-proto/lib.rs` and `library-proto/Cargo.toml`, and `protoc` overwrites both on the next run. `[lib] path = "lib.rs"` makes that file the crate root. Omit `crate_name` and the package name is `generated`.

`library-proto` is the Cargo package name. Hyphens become underscores in Rust, so other crates import it as `library_proto`. `package library` is a module inside that crate, and `message Book` is `library_proto::library::Book`. The `.proto` file name is not a module name. Code that is compiled as part of `library-proto` itself writes `library::Book`.

For a proto2 file, string fields default to unchecked bytes (`&[u8]`). Pass `--puroro_opt=proto2_utf8=verify` (combine with other options as `crate_name=library-proto,proto2_utf8=verify`) when those fields should be `&str`.

The generated `Cargo.toml` depends on `puroro` 0.15, `puroro-rt` 0.1, `allocator-api2`, `bitvec`, and `bytes`. `lib.rs` names those crates directly. Those versions resolve from crates.io once `puroro` 0.15.0 and `puroro-rt` 0.1.0 are uploaded. Until then, [crates.io `puroro` 0.14.1](https://crates.io/crates/puroro) is the previous API.

From a checkout of this repository, build the same binary and pass it to `protoc` directly:

```shell
cargo build -p protoc-gen-puroro
mkdir -p library-proto
protoc --plugin=protoc-gen-puroro=$PWD/target/debug/protoc-gen-puroro --puroro_out=library-proto --puroro_opt=crate_name=library-proto -I . book.proto
```

The codegen tests in this repository do not install the binary. [`puroro-codegen-tests`](https://github.com/wada314/puroro/blob/master/puroro-codegen-tests/Cargo.toml) lists `protoc-gen-puroro` as an artifact build-dependency (`artifact = "bin"`). Cargo then sets `CARGO_BIN_FILE_PROTOC_GEN_PURORO`, and the test `build.rs` passes that path as `--plugin`. That still needs nightly `-Z bindeps`, enabled by [`.cargo/config.toml`](https://github.com/wada314/puroro/blob/master/.cargo/config.toml). It is how this workspace compiles the plugin for its own tests. A crates.io dependency cannot use the same artifact edge: Cargo does not set `CARGO_BIN_FILE_*` for a registry package.

## Use the generated crate from another crate

Application code depends on the generated package and on `puroro`. It does not depend on `puroro-rt`, and it does not repeat `allocator-api2`, `bitvec`, or `bytes` unless it names those crates itself.

```toml
[package]
name = "app"
edition = "2024"

[dependencies]
library-proto = { path = "../library-proto" }
puroro = "0.15"
```

Field access is inherent methods on the concrete type. Codec methods live on `puroro::Message`, so a proto field named `encode` does not hide them; call those with the trait in scope.

`optional string` is explicit presence: `title()` returns `Optional<&str, …>`, `title().get()` is the value or the proto default, and `title().is_set()` reports presence. Eager messages do not generate `has_title()`. A plain proto3 `uint32` is implicit: `pages()` returns `u32`.

```rust
use library_proto::library::Book;
use ::puroro::{Message, StringMut};

let mut book = Book::new();
book.title_mut().push_str("Shore");
*book.pages_mut() = 320;
assert!(book.title().is_set());
assert_eq!(book.title().get(), "Shore");
assert_eq!(book.pages(), 320);

let bytes = book.encode_to_vec();
let decoded: Book = Book::decode(&bytes[..]).unwrap();
assert_eq!(decoded.title().get(), "Shore");
```

The `Book` annotation on `decode` selects the default allocator (`Global`). `StringMut` has to be in scope for `push_str`. The same pattern is covered by the in-tree [`scalars` fixture](https://github.com/wada314/puroro/blob/master/puroro-codegen-tests/fixtures/scalars/test.rs).

Each message also has a lazy alias (`BookLazy`) with `Result` getters and no mutators. From this application that type is `library_proto::library::BookLazy`. The full accessor table is [DESIGN.md §4](https://github.com/wada314/puroro/blob/master/DESIGN.md#4-generated-code-specification).

## Out of scope

- Extensions (`extend` / proto2 extension fields).
- Services and RPC.
- Well-known types such as `google.protobuf.Any`, `Timestamp`, and `Duration`, unless you compile those `.proto` files yourself.
- A builder type separate from the message. Mutation is `let mut` plus `_mut` / `clear_*`.
- Treating `(puroro.unknown_fields) = REJECT` as spec-compliant behavior. The default preserve store is the spec behavior. `DISCARD` accepts the message and drops the bytes.

## License

[Apache License 2.0](https://github.com/wada314/puroro/blob/master/LICENSE).
