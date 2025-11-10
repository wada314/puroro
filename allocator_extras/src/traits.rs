//! Traits that mirror standard library behaviour with allocator awareness.

use ::allocator_api2::alloc::Allocator;

/// Allocator-aware counterpart of `Default`.
pub trait DefaultIn<A: Allocator>: Sized {
    /// Creates a default value using the provided allocator.
    fn default_in(alloc: A) -> Self;
}

/// Allocator-aware counterpart of `Clone`.
pub trait CloneIn<A: Allocator>: Sized {
    /// Clones `self`, allocating with the provided allocator.
    fn clone_in(&self, alloc: A) -> Self;
}

/// Allocator-aware counterpart of `ToOwned`.
pub trait ToOwnedIn<A: Allocator> {
    /// The owned type produced when cloning into an allocator.
    type Owned;

    /// Creates an owned value using the provided allocator.
    fn to_owned_in(&self, alloc: A) -> Self::Owned;
}
