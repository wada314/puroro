# protoc-gen-puroro

`protoc` plugin that emits Rust for the [puroro](https://github.com/wada314/puroro) runtime.

**This is not an official Google product.** For production Rust protobuf code, use [`google-protobuf`](https://crates.io/crates/google-protobuf).

Install the binary and leave it on `PATH`. `cargo install` builds the `protoc-gen-puroro` executable. It does not use Cargo's unstable `bindeps` feature:

```shell
cargo install protoc-gen-puroro
mkdir -p library-proto
protoc --puroro_out=library-proto --puroro_opt=crate_name=library-proto -I . book.proto
```

`protoc` runs `protoc-gen-puroro` from `PATH` when it sees `--puroro_out`. The output directory is a Cargo package: the plugin writes `lib.rs` and `Cargo.toml` (`[lib] path = "lib.rs"`). `--puroro_opt=crate_name=library-proto` sets the package name (the default is `generated`). `protoc` overwrites both files on every run. Another crate depends on that package and on `puroro`; a `package library` message `Book` is `library_proto::library::Book` from outside. See the [puroro README](https://github.com/wada314/puroro/blob/master/README.md#use-the-generated-crate-from-another-crate).

From a git checkout, build the binary and pass the path yourself:

```shell
cargo build -p protoc-gen-puroro
mkdir -p library-proto
protoc --plugin=protoc-gen-puroro=$PWD/target/debug/protoc-gen-puroro --puroro_out=library-proto --puroro_opt=crate_name=library-proto -I . book.proto
```

`puroro-codegen-tests` in the puroro workspace does not install this binary. It builds it with an artifact dependency (`artifact = "bin"`, nightly `-Z bindeps`). That is only for those tests. A crates.io dependency cannot take this binary the same way.

Application code should not depend on this crate as a library. Depend on the generated crate and `puroro`.

## License

[Apache License 2.0](https://github.com/wada314/puroro/blob/master/LICENSE).
