# puroro-rt

Runtime crate that `protoc-gen-puroro` output names directly (`::puroro_rt::…`). A generated crate depends on this package. Application code should not import it; depend on the generated crate and [`puroro`](https://github.com/wada314/puroro).

**This is not an official Google product.** For production Rust protobuf code, use [`google-protobuf`](https://crates.io/crates/google-protobuf).

```toml
[dependencies]
puroro = "0.15"
puroro-rt = "0.1"
```

## Features

| Feature | Default | Purpose |
|---|---|---|
| `lazy` | off | Experimental lazy message aliases (`NameLazy`, island scan, `WireOrSso`, …). Enable via `--puroro_opt=lazy=true` on the plugin (the generated `Cargo.toml` turns this on). API and behaviour may change. |

## License

[Apache License 2.0](https://github.com/wada314/puroro/blob/master/LICENSE).
