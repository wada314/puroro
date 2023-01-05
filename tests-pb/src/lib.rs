/// re-export puroro.
pub use ::puroro;
/// re-export the primitive types in puroro namespace.
/// by using the "*", it can be hidden by the same typename explicitly defined in this file.
pub use ::puroro::*;
mod _root {
    #[allow(unused)]
    pub(crate) use super::*;
}
mod _puroro {
    #[allow(unused)]
    pub(crate) use ::puroro::*;
}
mod _pinternal {
    #[allow(unused)]
    pub(crate) use ::puroro::internal::*;
}
pub mod full_coverage2;
pub mod full_coverage3;
pub mod keywords;
pub mod name_conflict_case;
pub mod name_conflict_fields;
pub mod nested;
pub mod self_recursive;
