# AGENTS.md

Instructions for coding agents working in this repository. Human-oriented docs stay in `DESIGN.md` / `IMPLEMENTATION.md`. This file is the [AGENTS.md](https://agents.md/) entry point.

## Overview

`scratch-ai` is the working branch of **puroro**, a Protocol Buffers runtime and `protoc` plugin for Rust (editions / proto2 / proto3). Do not land work on `master`.

| Crate | Role |
|---|---|
| `puroro` (workspace root) | Stable user API (`Message`, `Optional`, errors) |
| `puroro-rt` | Generated-code field catalog and wire helpers |
| `protoc-gen-puroro` | `protoc` plugin (`--puroro_out`) |
| `sample-generated` | Hand-written reference output (`Task` / `Address`) |
| `puroro-codegen-tests` | Live plugin fixtures (`fixtures/<case>/`) |
| `protobuf-core` | Git submodule — wire primitives |
| `unmanaged` | Git submodule — allocator-less containers that panic on implicit `Drop` |

Library users of generated messages depend on the generated crate and `puroro` only. Do not surface `puroro-rt` types in public generated signatures.

Lazy `RecordScanner` emits `ScannedRecord` / `ScannedValue`. LEN is a span into `_wire`, not a `protobuf_core::Field` / owned payload. Do not copy complete LEN bodies into the scan record. Numerical apply uses catalog `merge_scanned` (`CopyWirePayload::from_scanned`). Do not rematch scalars through `merge_scanned_field`. Packed LEN uses the scanned `payload_len` (`merge_packed_buf`); do not re-read the length prefix. Do not re-land island-first leftover-as-offset / `push_bytes` ingest: it did not move typical decode and duplicated leftover state.

## Setup

- Nightly is pinned by `rust-toolchain.toml`. `.cargo/config.toml` enables `-Z bindeps` (needed by `puroro-codegen-tests`).
- Initialize submodules before the first build: `git submodule update --init`.
- In this Cloud workspace the sibling checkouts may already exist at `/agent/repos/protobuf-core` and `/agent/repos/unmanaged`; do not rewrite those gitlinks to satisfy lint.

## Build, test, lint

```sh
cargo build
cargo test
cargo test -p puroro-sample-generated
cargo test -p puroro-codegen-tests   # needs `protoc` (or `PROTOC=`); plugin via bindeps
cargo bench -p puroro-benches -- --test
cargo fmt --all
cargo clippy --all-targets
```

Criterion benches are the unpublished `puroro-benches` member (`publish = false`). Sources live in `puroro-benches/benches/`. The `puroro` package sets `autobenches = false` so Cargo does not pick those files up as its own targets. Do not move the targets back onto `puroro`. A `puroro` dev-dependency on `puroro-rt` makes `cargo publish -p puroro` wait for `puroro-rt`, which itself depends on `puroro`.

Users install the plugin with `cargo install protoc-gen-puroro` (or `cargo build -p protoc-gen-puroro` plus `--plugin=`). That path does not use `-Z bindeps`. `bindeps` is only the artifact dependency in `puroro-codegen-tests`. 0.14 called `puroro-codegen` in-process from `build.rs`; it did not install this binary.

The plugin writes `lib.rs` and `Cargo.toml` into `--puroro_out` (`[lib] path = "lib.rs"`). `--puroro_opt=crate_name=` sets the package name (default `generated`). Application code depends on that package and `puroro`; a proto package `library` and message `Book` is `<crate>::library::Book` from outside. Do not drop the manifest to leave a bare `lib.rs` as the only user-facing output.

Fix new fmt/clippy issues. Do not silence lints unless this repo already does for that case. Do not edit the `protobuf-core` / `unmanaged` submodules for workspace-only lint policy unless asked.

## Code style

- Imports: `crate::` / `self::` for this crate; every other path starts with `::` (`::std::…`, `::puroro::…`). See `.cursor/rules/rust-name-resolution.mdc`.
- Comments in English.
- New submodules use `foo.rs`, not `foo/mod.rs`.
- In `quote!`, interpolations must be simple `#ident` (bind expressions first).

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
