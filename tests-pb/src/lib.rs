//! "Generated from root package"
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
pub mod oneofs2;
pub mod oneofs3;
pub mod enum2;
pub mod keywords;
pub mod self_recursive;
pub mod ser_tests2;
pub mod ser_tests3;
pub mod enum3;
pub mod nested;
pub mod full_coverage2;
pub mod full_coverage3;
pub mod official_samples2;
pub mod official_samples3;
