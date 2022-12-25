/// re-export puroro.
pub use ::puroro;
/// re-export the primitive types in puroro namespace.
/// by using the "*", it can be hidden by the same typename explicitly defined in this file.
pub use ::puroro::*;
mod _puroro_root {
    #[allow(unused)]
    pub(crate) use super::*;
}
mod _puroro {
    #[allow(unused)]
    pub(crate) use ::puroro::*;
}
pub mod library;
