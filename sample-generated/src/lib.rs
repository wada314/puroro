//! Hand-written sample of code the puroro `protoc` plugin is expected to emit.
//!
//! Source schema: `DESIGN.md` reference `Task` / `Address` messages (edition 2024),
//! plus a sample-only `School` → `Student` → `Point` chain for two-hop inline.
//! This crate exists to compile-check the [`puroro`] user API and the
//! [`puroro_rt`] field catalog against realistic generated accessors,
//! encode/decode glue, and presence bit indices.
//!
//! **Naming vs. real generated code.** The production plugin qualifies paths as
//! follows: external crates with leading `::` (`::puroro_rt::SingularField`,
//! `::puroro::Message`, `::core::ops::DerefMut`, …); names inside the generated
//! module forest as `self::_root::…`. A `.proto` schema may introduce identifiers
//! that would collide with short names in the same scope. This sample
//! deliberately relaxes that: it uses `use` imports and short names for
//! readability. Read the short names here as stand-ins for their production
//! forms. See `IMPLEMENTATION.md` § "Path qualification (naming)" and
//! `DESIGN.md` § "Path qualification".
//!
//! **Module layout.** Message structs are public at this crate root (`Address`,
//! `Task`, `School`, `Student`, `Point`) — the parent of their snake_case
//! companion modules (`address`, `task`, `school`, `student`, `point`) which
//! hold `FIELD_*` / `BIT_*`, defaults, and oneofs. Production
//! output follows the same rule under the protobuf `package` module. This sample
//! stays flat (no `example::v1` prefix) for readability. The `*_type` files are
//! a hand-written split so the crate root stays small; they are not a public
//! module path.
//!
//! These files are maintained by hand as a readable reference; in a real project
//! the equivalent sources would come from the plugin instead. Comments here are
//! free-form and intentionally omit machine markers like `@generated`, which
//! would wrongly imply the files are tool-generated.

#![allow(dead_code)]

pub mod address;
pub mod enums;
pub mod point;
pub mod school;
pub mod student;
pub mod task;

mod address_type;
mod point_type;
mod school_type;
mod student_type;
mod task_type;

pub use address_type::{
    Address, AddressBody, AddressBound, AddressMessage, AddressMessageMut, AddressMut,
    AddressOwnedCommon, AddressView,
};
pub use enums::{Priority, Status};
pub use point_type::{
    Point, PointBody, PointBound, PointMessage, PointMessageMut, PointMut, PointOwnedCommon,
    PointView,
};
pub use school_type::School;
pub use student_type::{
    Student, StudentBody, StudentBound, StudentMessage, StudentMessageMut, StudentMut,
    StudentOwnedCommon, StudentView,
};
pub use task_type::Task;

use ::allocator_api2::alloc::Global;
use ::puroro_rt::RepeatedMessagesMut;

/// Mutator handle for a `repeated Address` field (sample `Task.watchers`).
///
/// Concrete so `watchers_mut().push()` returns [`AddressMut`] with a short
/// reborrow (RPIT + GAT pin would force the container lifetime and break
/// `push().copy_from(...)` chains).
pub type AddressListMut<'a, A = Global> = RepeatedMessagesMut<'a, Address<A>, A>;
