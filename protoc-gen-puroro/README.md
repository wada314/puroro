# protoc-gen-puroro

`protoc` plugin that emits Rust for the [puroro](https://github.com/wada314/puroro) runtime.

**This is not an official Google product.** For production Rust protobuf code, use [`google-protobuf`](https://crates.io/crates/google-protobuf).

Install the binary and leave it on `PATH`. `cargo install` builds the `protoc-gen-puroro` executable:

```shell
cargo install protoc-gen-puroro
mkdir -p out
protoc --puroro_out=out -I . shelf.proto
```

`protoc` runs `protoc-gen-puroro` from `PATH` when it sees `--puroro_out`. The crate name in that `Cargo.toml`, the `name` value under `[package]`, is the string `generated` unless you pass `crate_name`. The diagram in the [puroro README](https://github.com/wada314/puroro/blob/master/README.md#generate-a-crate) shows protobuf `package demo` / `message Shelf` as `generated::demo::Shelf`.

### Options (`--puroro_opt`)

Comma-separated `key=value` pairs (a bare `lazy` also works):

| Option | Default | Meaning |
|---|---|---|
| `crate_name` | `generated` | `[package] name` in the generated `Cargo.toml` |
| `proto2_utf8` | `none` | proto2 string views: `none` → `&[u8]`, `verify` → `&str` |
| `lazy` | off | Experimental: emit `NameLazy` and enable `puroro-rt/lazy` |
| `tonic` | on when services exist | Emit cfg-gated tonic client/server stubs; `false` suppresses |
| `tonic_client` | on | Emit `{service}_client` when `tonic` is on |
| `tonic_server` | on | Emit `{service}_server` when `tonic` is on |

The generate → depend → implement → call walkthrough for those stubs lives in the
[puroro README — gRPC stubs (tonic)](https://github.com/wada314/puroro/blob/master/README.md#grpc-stubs-tonic)
(and on docs.rs under [`puroro::tonic`](https://docs.rs/puroro/latest/puroro/tonic/index.html)).

## Build the plugin from a checkout

This is for people changing puroro. Other users stop at `cargo install`.

```shell
cargo build -p protoc-gen-puroro
mkdir -p out
protoc --plugin=protoc-gen-puroro=$PWD/target/debug/protoc-gen-puroro --puroro_out=out -I . shelf.proto
```

Application code should not depend on this crate as a library. Depend on the generated crate. Runtime traits are that crate's `puroro` module.

## License

[Apache License 2.0](https://github.com/wada314/puroro/blob/master/LICENSE).
