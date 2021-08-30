#![allow(incomplete_features)]
#![feature(generic_associated_types)]
pub mod de;
pub mod impls;
pub mod se;

use ::puroro::{ErrorKind, Result};
use ::std::ops::Deref;

// Re-exporting library modules
pub use ::puroro::{bumpalo, hashbrown};

pub struct Derefable<T>(T);
impl<T> Deref for Derefable<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
