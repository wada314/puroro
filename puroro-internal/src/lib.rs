#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]
pub mod de;
pub mod impls;
pub mod se;

use ::puroro::{ErrorKind, Result};
use ::std::ops::Deref;

// Re-exporting library modules
pub use ::puroro::{bumpalo, hashbrown};

pub struct Derefable<T>(T);
impl<T> Derefable<T> {
    pub fn new(v: T) -> Self {
        Self(v)
    }
}
impl<T> Deref for Derefable<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
