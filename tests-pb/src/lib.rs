//! "Generated from root package"
#![feature(generic_associated_types)]
/// re-export puroro.
pub use ::puroro;
/// re-export the primitive types in puroro namespace.
/// by using the "*", it can be hidden by the same typename explicitly defined in this file.
pub use ::puroro::*;
pub mod _puroro_root {
    pub use super::*;
}
pub mod _puroro {
    pub use ::puroro::*;
}
