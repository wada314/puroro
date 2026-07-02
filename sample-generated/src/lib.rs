//! Hand-written sample of code the puroro `protoc` plugin is expected to emit.
//!
//! Source schema: `DESIGN.md` reference `Task` / `Address` messages (edition 2024).
//! This crate exists to compile-check the runtime [`puroro`] field catalog against
//! realistic generated accessors, encode/decode glue, and presence bit indices.
//!
//! **Naming vs. real generated code.** The production plugin must *fully-qualify*
//! every path it emits (`::puroro::SingularLenField`, `::core::ops::DerefMut`, …)
//! because a `.proto` schema may name things that would collide with unqualified
//! identifiers. This sample deliberately relaxes that: it uses `use` imports and
//! short names for readability. Read the short names here as stand-ins for their
//! fully-qualified forms. See `IMPLEMENTATION.md` § "Path qualification (naming)".
//!
//! These files are maintained by hand as a readable reference; in a real project
//! the equivalent sources would come from the plugin instead. Comments here are
//! free-form and intentionally omit machine markers like `@generated`, which
//! would wrongly imply the files are tool-generated.

#![allow(dead_code)]

pub mod address;
pub mod enums;
pub mod task;

pub use address::Address;
pub use enums::{Priority, Status};
pub use task::Task;
