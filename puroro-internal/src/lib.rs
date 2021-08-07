#![allow(incomplete_features)]
#![feature(generic_associated_types)]
pub mod de;
pub mod impls;
pub mod se;

use ::puroro::{ErrorKind, Result};

// Re-exporting library modules
pub use ::puroro::{bumpalo, hashbrown};
