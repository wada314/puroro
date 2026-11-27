# AGENTS.md

Instructions for coding agents working in this repository. Human-oriented docs stay in `DESIGN.md` / `IMPLEMENTATION.md`. This file is the [AGENTS.md](https://agents.md/) entry point.

## Overview

`master` is the working branch of **puroro**, a Protocol Buffers runtime and `protoc` plugin for Rust (editions / proto2 / proto3). Land changes on `master` through a pull request. There is no `scratch-ai` branch on origin.

Draft pull requests #101 and #102 are leftover environment-setup changes from August and October 2026. Do not treat them as the current environment or as the working branch.

| Crate | Role |
|---|---|
| `puroro` (workspace root) | Stable user API (`Message`, `Optional`, errors); optional Cargo feature `tonic` for `PuroroCodec` |
| `puroro-rt` | Generated-code field catalog and wire helpers |
| `protoc-gen-puroro` | `protoc` plugin (`--puroro_out`) |
| `puroro-types` | Generated `google.protobuf` descriptors + WKTs (`descriptors` / `wkt` features); leaf crate, not depended on by `puroro` / `puroro-rt`. Vendored `.proto` pin in `puroro-types/proto/UPSTREAM`; refresh with `puroro-types/scripts/vendor-google-protos.sh`. Rustdoc is on GitHub Pages (`.github/workflows/update-doc.yml`), not docs.rs |
| `sample-generated` | Hand-written reference (`Task` / `Address`) on the plugin's `*Fields` + `Record` shell |
| `puroro-codegen-tests` | Live plugin fixtures (`fixtures/<case>/`) |
| `protobuf-core` | Git submodule — wire primitives |
| `unmanaged` | Git submodule — allocator-less containers that panic on implicit `Drop` |

Library users of generated messages depend on the generated crate. The plugin re-exports the runtime as `<crate>::puroro`. Do not surface `puroro-rt` types in public generated signatures. The root module names `puroro` and `descriptors` are reserved. Accepted and rejected proto names: [`DESIGN.md`](DESIGN.md#allowed-and-disallowed-names).

Lazy decode is **experimental and opt-in**. The plugin emits `NameLazy` only with `--puroro_opt=lazy=true` (also enables `puroro-rt`'s `lazy` feature). `(puroro.child_parse) = LAZY` requires that opt-in. `sample-generated` keeps hand-written lazy aliases with `puroro-rt/lazy` enabled. Do not treat lazy as stable public API.

Lazy `RecordScanner` emits `ScannedRecord` / `ScannedValue`. LEN is a span into `_wire`, not a `protobuf_core::Field` / owned payload. Do not copy complete LEN bodies into the scan record. Numerical apply uses catalog `merge_scanned` (`CopyWirePayload::from_scanned`). Do not re-encode a scanned numeric into a wire body and merge that. Packed LEN uses the scanned `payload_len` (`merge_packed_buf`); do not re-read the length prefix. Do not re-land island-first leftover-as-offset / `push_bytes` ingest: it did not move typical decode and duplicated leftover state.

## Setup

- Nightly is pinned by `rust-toolchain.toml`. `.cargo/config.toml` enables `-Z bindeps` (needed by `puroro-codegen-tests`).
- Initialize submodules before the first build: `git submodule update --init`.
- In this Cloud workspace the sibling checkouts may already exist at `/agent/repos/protobuf-core` and `/agent/repos/unmanaged`; do not rewrite those gitlinks to satisfy lint.

## Build, test, lint

```sh
cargo build
cargo test
cargo test -p puroro-sample-generated
cargo test -p puroro-codegen-tests   # needs `protoc` ≥ 32.1 (or `PROTOC=`); plugin via bindeps
cargo test -p puroro-types           # needs `protoc`; plugin via bindeps; features `descriptors`/`wkt`
cargo doc --no-deps -p puroro-types --all-features  # GitHub Pages publishes this on master
cargo bench -p puroro-benches -- --test
cargo fmt --all
cargo clippy --all-targets --workspace \
  --exclude puroro-codegen-tests --exclude puroro-types --exclude puroro-benches --exclude address-view-spike
RUSTDOCFLAGS='-D warnings' cargo doc --no-deps \
  -p puroro -p puroro-rt -p protoc-gen-puroro -p puroro-sample-generated
# Optional tonic Codec surface:
cargo test -p puroro --features tonic
RUSTDOCFLAGS='-D warnings' cargo doc --no-deps -p puroro --features tonic
# Generated service stubs (cfg-gated in plugin output):
cargo test -p puroro-codegen-tests --features tonic
```

`cargo test` also runs rustdoc examples (doctests). CI (`.github/workflows/rust.yml`) runs fmt, clippy (`-D warnings`, excluding generated/archive/bench crates), tests, codegen fixtures, and `cargo doc` with `RUSTDOCFLAGS=-D warnings` for the first-party packages above. Bench smoke stays in `.github/workflows/benches.yml`. `puroro-types` is excluded from workspace clippy like other protoc-generated surfaces (its `build.rs` needs `protoc` + bindeps).

Criterion benches are the unpublished `puroro-benches` member (`publish = false`). Sources live in `puroro-benches/benches/`. The `puroro` package sets `autobenches = false` so Cargo does not pick those files up as its own targets. Do not move the targets back onto `puroro`. A `puroro` dev-dependency on `puroro-rt` makes `cargo publish -p puroro` wait for `puroro-rt`, which itself depends on `puroro`.

Users install the plugin with `cargo install protoc-gen-puroro` (or `cargo build -p protoc-gen-puroro` plus `--plugin=`). That path does not use `-Z bindeps`. `bindeps` is only the artifact dependency in `puroro-codegen-tests`. 0.14 called `puroro-codegen` in-process from `build.rs`; it did not install this binary.

The plugin writes `lib.rs` and `Cargo.toml` into `--puroro_out` (`[lib] path = "lib.rs"`). `--puroro_opt=crate_name=` sets the crate name: the `name` value under `[package]`. If omitted, that name is the string `generated`. `lib.rs` re-exports the runtime as `puroro`. Application code depends on that crate. A protobuf package `library` and message `Book` is `<crate>::library::Book` from outside. Do not drop the manifest to leave a bare `lib.rs` as the only user-facing output.

Dependency versions in that manifest come from `protoc-gen-puroro/build.rs`, which reads `Cargo.lock` while the plugin is compiled. Do not hardcode `puroro` / `puroro-rt` / `allocator-api2` / `bitvec` / `bytes` versions in `generate.rs`. Those crates stay build-dependencies of the plugin so `cargo install` resolves the same names.

Fix new fmt/clippy issues. Do not silence lints unless this repo already does for that case. Do not edit the `protobuf-core` / `unmanaged` submodules for workspace-only lint policy unless asked.

## Code style

- Imports: `crate::` / `self::` for this crate; every other path starts with `::` (`::std::…`, `::puroro::…`). See `.cursor/rules/rust-name-resolution.mdc`.
- Comments in English.
- New submodules use `foo.rs`, not `foo/mod.rs`.
- In `quote!`, interpolations must be simple `#ident` (bind expressions first).

## Groups

Unknown `SGroup` is preserved, skipped, or rejected through the matching `EGroup`. A known group is `ProtoGroup<M>`: the child reads the caller's buffer from the consumed start tag until that field number's end tag. Do not copy the interior into a side buffer. Encode writes start tag + body + end tag with no length prefix. A lazy child records the group body span (no start/end tags) via `merge_shared`, same island model as LEN. `protoc-gen-puroro` emits `ProtoGroup` for `TYPE_GROUP` and for non-map `message_encoding=DELIMITED`. Synthetic map-entry key/value fields stay length-prefixed even when the file sets `DELIMITED`.

## Unknown-field reject

`(puroro.unknown_fields) = REJECT` fails decode on an unrecognized field number (`DecodeError::UnrecognizedField`). That is outside the protobuf spec. A conforming parser accepts the tag and preserves the bytes; the default preserve store is that behavior. Do not describe reject as conformant, and do not change it to preserve in order to match the spec. Closed-enum unknowns still divert and, with no store, are dropped. Public wording: [`DESIGN.md` §4.9](DESIGN.md#49-unknown-fields).

## SSO inline slot

Singular `string` / `bytes` SSO stores `[MaybeUninit<u8>; INLINE_CAP]` plus a length byte. Only that prefix is initialized. Short decode copies the wire bytes once into the prefix, and the first merge writes an absent slot in place. Do not zero the tail or read it as `[u8]`. Layout and merge rules: [`IMPLEMENTATION.md` §17.2](IMPLEMENTATION.md#172-string--bytes-inline-optimisation).

## Where to read next

- Crate landing page (GitHub repository root and the crates.io page): [`README.md`](README.md). It tells most users to use Google's `google-protobuf` instead.
- Public API: [`DESIGN.md`](DESIGN.md)
- Generated-code internals: [`IMPLEMENTATION.md`](IMPLEMENTATION.md)
- Topic locks (lazy decode, SSO bools, superseded sharing): [`.cursor/plans/`](.cursor/plans/)
- Workflow rules: [`.cursor/rules/`](.cursor/rules/)

There is no `AI_REFERENCES.md` on this branch. Do not revive the `scratch3` `doc/` tree; current decisions live in the files above.
