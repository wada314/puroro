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
pub mod full_coverage2;
pub mod full_coverage3;
pub mod keywords;
pub mod nested;
pub mod proto2_defaults;
pub mod self_recursive;
