//! User-facing mutator API for singular `bytes` fields.

use ::allocator_api2::alloc::Allocator;
use ::allocator_api2::vec::Vec as AllocVec;
use ::core::ops::Deref;

/// Mutator API for a singular `bytes` field (`payload_mut()`, …).
///
/// Generated accessors return a concrete mutator that implements this trait.
/// Short values may be stored inline; methods promote to the heap when the
/// result no longer fits.
pub trait BytesMut<A: Allocator>: Deref<Target = [u8]> {
    /// Replaces the contents by copying from `bytes`.
    fn set(&mut self, bytes: &[u8]);

    /// Replaces the contents by taking ownership of `v`.
    ///
    /// Long values keep `v`'s heap buffer; short values may be stored inline
    /// (in which case `v`'s buffer is released).
    fn set_vec(&mut self, v: AllocVec<u8, A>);

    /// Clears to empty bytes.
    fn clear(&mut self);

    /// Appends `bytes`.
    fn extend_from_slice(&mut self, bytes: &[u8]);

    /// Appends a single byte.
    fn push(&mut self, b: u8);

    /// Shortens to `new_len` bytes.
    fn truncate(&mut self, new_len: usize);
}
