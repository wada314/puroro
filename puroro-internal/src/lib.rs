#![allow(unstable_name_collisions)] // For ResultHelper::flatten.
#![cfg_attr(feature = "puroro-nightly", feature(generic_associated_types))]
#![cfg_attr(feature = "puroro-nightly", feature(min_type_alias_impl_trait))]
#![allow(incomplete_features)]

pub mod variant;

// Re-exporting library modules
pub use ::puroro::{bumpalo, hashbrown};

pub use ::puroro::{ErrorKind, PuroroError, Result};
