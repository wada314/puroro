# protoc-gen-puroro

`protoc` plugin that emits Rust for the [puroro](https://github.com/wada314/puroro) runtime.

**This is not an official Google product.** For production Rust protobuf code, use [`google-protobuf`](https://crates.io/crates/google-protobuf).

Install the binary and leave it on `PATH`. `cargo install` builds the `protoc-gen-puroro` executable. It does not use Cargo's unstable `bindeps` feature:

```shell
cargo install protoc-gen-puroro
mkdir -p demo-proto
protoc --puroro_out=demo-proto --puroro_opt=crate_name=demo-proto -I . shelf.proto
```

`protoc` runs `protoc-gen-puroro` from `PATH` when it sees `--puroro_out`. What that writes, and how `package demo` / `message Shelf` becomes `demo_proto::demo::Shelf`, is the diagram in the [puroro README](https://github.com/wada314/puroro/blob/master/README.md#generate-a-crate).

From a git checkout, build the binary and pass the path yourself:

```shell
cargo build -p protoc-gen-puroro
mkdir -p demo-proto
protoc --plugin=protoc-gen-puroro=$PWD/target/debug/protoc-gen-puroro --puroro_out=demo-proto --puroro_opt=crate_name=demo-proto -I . shelf.proto
```

`puroro-codegen-tests` in the puroro workspace does not install this binary. It builds it with an artifact dependency (`artifact = "bin"`, nightly `-Z bindeps`). That is only for those tests. A crates.io dependency cannot take this binary the same way.

Application code should not depend on this crate as a library. Depend on the generated crate and `puroro`.

## License

[Apache License 2.0](https://github.com/wada314/puroro/blob/master/LICENSE).
