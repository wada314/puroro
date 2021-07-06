#![allow(unstable_name_collisions)] // For ResultHelper::flatten.

pub mod de;
pub mod impls;

// Re-exporting library modules
pub use ::puroro::{bumpalo, hashbrown};

// Impl symbols
pub use impls::simple::SimpleImpl;

use ::puroro::{ErrorKind, Result, StructInternalTypeGen};

pub use de::from_iter::ScopedIter;

pub trait MessageInternal: ::puroro::Message {
    fn new_with_internal_data(
        internal_data: <Self::ImplTypeTag as StructInternalTypeGen>::Type,
    ) -> Self;
}
