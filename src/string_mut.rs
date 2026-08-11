//! User-facing mutator API for singular `string` fields.

use ::allocator_api2::alloc::Allocator;
use ::core::ops::Deref;
use ::unmanaged::String as AllocString;

/// Mutator API for a singular `string` field (`title_mut()`, …).
///
/// Generated accessors return a concrete mutator that implements this trait.
/// Short values may be stored inline; methods promote to the heap when the
/// result no longer fits.
pub trait StringMut<A: Allocator>: Deref<Target = str> {
    /// Replaces the contents by copying from `s`.
    fn set(&mut self, s: &str);

    /// Replaces the contents by taking ownership of `s`.
    ///
    /// Long values keep `s`'s heap buffer; short values may be stored inline
    /// (in which case `s`'s buffer is released).
    fn set_string(&mut self, s: AllocString<A>);

    /// Clears to an empty string.
    fn clear(&mut self);

    /// Appends `s`.
    fn push_str(&mut self, s: &str);

    /// Appends a Unicode scalar value.
    fn push(&mut self, ch: char);

    /// Shortens to `new_len` bytes (must be on a char boundary).
    fn truncate(&mut self, new_len: usize);
}
