//! Hand-written sample of code the puroro `protoc` plugin is expected to emit.
//!
//! Source schema: `DESIGN.md` reference `Task` / `Address` messages (edition 2024).
//! This crate exists to compile-check the runtime [`puroro`] field catalog against
//! realistic generated accessors, encode/decode glue, and presence bit indices.
//!
//! **Do not edit by hand in production** — the plugin will generate equivalent sources.

#![allow(dead_code)]

pub mod address;
pub mod enums;
pub mod task;

pub use address::Address;
pub use enums::{Priority, Status};
pub use task::Task;
