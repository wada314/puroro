# AGENTS.md

## Cursor Cloud specific instructions

### Overview

This is the `scratch-ai` development branch of **puroro** — a performance-oriented Protocol Buffers runtime for Rust, targeting edition 2024 with nightly Rust.

### Rust toolchain

- `rust-toolchain.toml` pins the workspace to `channel = "nightly"` (rolling latest). Cargo respects this automatically; no manual `rustup default` is needed.
- Ensure the nightly toolchain is installed: `rustup toolchain install nightly`
- Components needed: `rustfmt`, `clippy`. They are installed alongside nightly when the update script runs.

### Git submodules — REQUIRED before building

Two workspace members are private git submodules and **must be initialised** before anything compiles:

| Submodule       | GitHub URL                                   |
|-----------------|----------------------------------------------|
| `protobuf-core` | `git@github.com:wada314/protobuf-core.git`   |
| `unmanaged`     | `git@github.com:wada314/unmanaged.git`       |

Run after SSH access is configured:
```sh
git submodule update --init
```

Without this step, every `cargo` command fails with a missing `Cargo.toml` error.

### Build

```sh
cargo build           # workspace (excludes update-* crates by default)
cargo build -p puroro # specific crate
```

`.cargo/config.toml` enables unstable `bindeps` (required for `puroro-codegen-tests` build script which uses the `protoc-gen-puroro` binary as an artifact dependency).

### Test

```sh
cargo test                             # all default workspace members
cargo test -p puroro-sample-generated  # sample hand-written generated code tests
cargo test -p puroro-codegen-tests     # end-to-end codegen + behavioural tests (needs protoc in PATH or via protoc-bin-vendored)
```

### Lint (run before handing off changes)

Per `.cursor/rules/rust-fmt-clippy.mdc`:

```sh
cargo fmt --all
cargo clippy --all-targets
```

Fix any new warnings introduced; do not silence lints unless the project already does so.

### Code style

See `.cursor/rules/rust-name-resolution.mdc` for import path conventions (`::` prefix for external crates, `crate::` / `self::` for internal).

### SSH key setup

The environment needs an SSH private key with read access to the `wada314` private repos. Set it up before initialising submodules:

```sh
eval "$(ssh-agent -s)"
ssh-add ~/.ssh/id_rsa   # or wherever the key is stored
git submodule update --init
```
