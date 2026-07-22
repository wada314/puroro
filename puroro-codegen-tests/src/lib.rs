//! Runtime tests against code produced by `protoc-gen-puroro` at build time.
//!
//! Each case lives under `fixtures/<case>/` (`*.proto` + `test.rs`). `build.rs`
//! generates modules into `OUT_DIR` and wires the per-case tests as unit tests.

#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[cfg(test)]
mod cases {
    include!(concat!(env!("OUT_DIR"), "/case_tests.rs"));
}
