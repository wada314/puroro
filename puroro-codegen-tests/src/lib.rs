//! Runtime tests against code produced by `protoc-gen-puroro` at build time.
//!
//! `build.rs` emits one module per fixture into `OUT_DIR`; this crate
//! `include!`s them and exposes ordinary Rust types for `#[test]`s. Add more
//! fixtures in `build.rs` — do not spawn `cargo` from tests.

#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/generated.rs"));
