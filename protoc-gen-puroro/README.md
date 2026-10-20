# protoc-gen-puroro

`protoc` plugin that emits Rust for the [puroro](https://github.com/wada314/puroro) runtime.

**This is not an official Google product.** For production Rust protobuf code, use [`google-protobuf`](https://crates.io/crates/google-protobuf).

Install the binary and leave it on `PATH`:

```shell
cargo install protoc-gen-puroro
protoc --puroro_out=gen -I . book.proto
```

`protoc` runs `protoc-gen-puroro` when it sees `--puroro_out`. Application code should not depend on this crate as a library. Depend on the generated crate and `puroro`.

## License

[Apache License 2.0](https://github.com/wada314/puroro/blob/master/LICENSE).
