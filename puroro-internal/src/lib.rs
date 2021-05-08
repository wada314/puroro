// Used in helpers/slice_view_update.rs
// TODO: Remove this?
#![feature(str_internals)]

pub mod deser;
pub mod helpers;
pub mod ser;
pub mod tags;
pub mod types;
pub mod variant;

pub use ::puroro::{ErrorKind, PuroroError, Result};
