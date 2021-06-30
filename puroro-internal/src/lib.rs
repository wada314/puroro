#![allow(unstable_name_collisions)] // For ResultHelper::flatten.

pub mod variant;

// Re-exporting library modules
pub use ::puroro::{bumpalo, hashbrown};

pub use ::puroro::{ErrorKind, PuroroError, Result};
