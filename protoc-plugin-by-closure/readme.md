# protoc-plugin-by-closure

## Requirements

Nightly channel required.

This crate requires an unstable (as of 2024/06) [cargo feature `bindeps`](https://rust-lang.github.io/rfcs/3028-cargo-binary-dependencies.html).
This crate contains [.cargo/config.toml](.cargo/config.toml) file which enables this feature so you normally don't need to worry about it, but if you include this crate as a part of your [Cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html), then you need to create the `.cargo/config.toml` file with the following contents *under your cargo workspace directory* manually (see [the official document](https://doc.rust-lang.org/cargo/reference/config.html) for the reasons):

```toml
[unstable]
bindeps = true
```

WIP