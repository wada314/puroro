#![cfg_attr(not(feature = "std"), no_std)]

//! Experimental allocator-aware utilities for puroro.

pub mod string;
pub mod traits;
pub mod util;

pub use crate::string::{FromUtf8Error, OwnedString, OwnedStringGlobal};
pub use crate::traits::{CloneIn, DefaultIn, ToOwnedIn};
pub use ::allocator_api2::alloc::{AllocError, Allocator, Global, Layout};
