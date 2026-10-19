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

Lazy `RecordScanner` emits `ScannedRecord` / `ScannedValue`. LEN is a span into `_wire`, not a `protobuf_core::Field` / owned payload. Do not copy complete LEN bodies into the scan record. Numerical apply uses catalog `merge_scanned` (`CopyWirePayload::from_scanned`). Do not rematch scalars through `merge_scanned_field`.

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
cargo fmt --all
cargo clippy --all-targets
```

Fix new fmt/clippy issues. Do not silence lints unless this repo already does for that case. Do not edit the `protobuf-core` / `unmanaged` submodules for workspace-only lint policy unless asked.

## Code style

- Imports: `crate::` / `self::` for this crate; every other path starts with `::` (`::std::…`, `::puroro::…`). See `.cursor/rules/rust-name-resolution.mdc`.
- Comments in English.
- New submodules use `foo.rs`, not `foo/mod.rs`.
- In `quote!`, interpolations must be simple `#ident` (bind expressions first).

## Where to read next

- Public API: [`DESIGN.md`](DESIGN.md)
- Generated-code internals: [`IMPLEMENTATION.md`](IMPLEMENTATION.md)
- Topic locks (lazy decode, SSO bools, superseded sharing): [`.cursor/plans/`](.cursor/plans/)
- Workflow rules: [`.cursor/rules/`](.cursor/rules/)

There is no `AI_REFERENCES.md` on this branch. Do not revive the `scratch3` `doc/` tree; current decisions live in the files above.
