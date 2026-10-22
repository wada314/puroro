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
