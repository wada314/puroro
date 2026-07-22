//! Allocation context for a single [`resolve`](super::resolve) call.
//!
//! Own an [`Arena`] on the stack (or in the generate entrypoint), resolve into a
//! [`FileSet`](super::FileSet) borrowing it, then drop both when emit finishes.

use ::bumpalo::Bump;

/// Bump allocator that owns all resolved nodes for one codegen invocation.
#[derive(Default)]
pub struct Arena {
    bump: Bump,
}

impl Arena {
    pub fn new() -> Self {
        Self { bump: Bump::new() }
    }

    /// Allocate `value` in this arena and return a stable reference.
    pub fn alloc<T>(&self, value: T) -> &mut T {
        self.bump.alloc(value)
    }
}
