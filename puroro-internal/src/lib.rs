#![allow(unstable_name_collisions)] // For ResultHelper::flatten.

pub mod de;
pub mod impls;

// Re-exporting library modules
pub use ::puroro::{bumpalo, hashbrown};
pub use ::puroro::{ErrorKind, PuroroError, Result};
pub use impls::simple::SimpleImpl;

pub use de::from_iter::ScopedIter;

