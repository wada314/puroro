//! Shared fields for message implementations.
//!
//! This module provides a wrapper type for fields that are shared across
//! all message fields (presence tracking, boolean values, allocator state, etc.).

use ::allocator_api2::alloc::{Allocator, Global};
use ::allocator_api2::vec::Vec;
use bitvec::prelude::*;

/// Shared fields for a message implementation.
///
/// This struct contains all fields that are shared across the message,
/// such as presence tracking bits, unknown-field storage, and allocator state.
///
/// # Type Parameters
/// - `BYTES`: Number of bytes needed for presence tracking (⌈fields/8⌉)
/// - `A`: Allocator used for unknown fields and derived structures
///
/// # Example
/// ```ignore
/// // For 3 fields: ⌈3/8⌉ = 1 byte
/// pub struct PersonImpl<A: Allocator = Global> {
///     _shared: SharedFields<1, A>,
///     name: StringFieldWrapper<A>,
///     age: i32,
///     email: StringFieldWrapper<A>,
/// }
/// ```
#[derive(Debug)]
pub struct SharedFields<const BYTES: usize, A: Allocator = Global> {
    /// Presence tracking bits
    /// Uses BitArray with fixed-size array for stack allocation
    has_bits: BitArray<[u8; BYTES]>,
    /// Unknown-field storage preserved during parsing
    unknown_fields: Vec<u8, A>,
    /// Allocator used for derived allocations
    allocator: A,
}

impl<const BYTES: usize, A> SharedFields<BYTES, A>
where
    A: Allocator + Clone,
{
    /// Creates new shared fields with all bits unset using the provided allocator.
    #[inline]
    pub fn new_in(alloc: A) -> Self {
        let unknown_fields = Vec::new_in(alloc.clone());
        Self {
            has_bits: BitArray::ZERO,
            unknown_fields,
            allocator: alloc,
        }
    }

    /// Gets a mutable reference to the presence tracking bits.
    ///
    /// This method is internal to the puroro crate and should not be used
    /// by generated code. Use the Field trait methods instead.
    #[inline]
    pub(crate) fn has_bits_mut(&mut self) -> &mut BitSlice<u8, Lsb0> {
        self.has_bits.as_mut_bitslice()
    }

    /// Gets a reference to the presence tracking bits.
    ///
    /// This method is internal to the puroro crate and should not be used
    /// by generated code. Use `is_field_present()` instead.
    #[inline]
    pub(crate) fn has_bits(&self) -> &BitSlice<u8, Lsb0> {
        self.has_bits.as_bitslice()
    }

    /// Checks if a specific field is present (for explicit optional fields).
    #[inline]
    pub fn is_field_present(&self, bit_index: usize) -> bool {
        self.has_bits()[bit_index]
    }

    /// Returns the allocator associated with these shared fields.
    #[inline]
    pub fn allocator(&self) -> &A {
        &self.allocator
    }

    /// Returns a mutable reference to the allocator associated with these shared fields.
    #[inline]
    pub fn allocator_mut(&mut self) -> &mut A {
        &mut self.allocator
    }

    /// Returns a slice of the unknown-field buffer.
    #[inline]
    pub fn unknown_fields(&self) -> &[u8] {
        self.unknown_fields.as_slice()
    }

    /// Returns a mutable reference to the unknown-field buffer.
    #[inline]
    pub fn unknown_fields_mut(&mut self) -> &mut Vec<u8, A> {
        &mut self.unknown_fields
    }

    /// Consumes the shared fields, returning the stored allocator.
    #[inline]
    pub fn into_allocator(self) -> A {
        self.allocator
    }
}

impl<const BYTES: usize> SharedFields<BYTES, Global> {
    /// Creates new shared fields with all bits unset using the global allocator.
    #[inline]
    pub fn new() -> Self {
        Self::new_in(Global)
    }
}

impl<const BYTES: usize, A> Clone for SharedFields<BYTES, A>
where
    A: Allocator + Clone,
{
    fn clone(&self) -> Self {
        Self {
            has_bits: self.has_bits,
            unknown_fields: self.unknown_fields.clone(),
            allocator: self.allocator.clone(),
        }
    }
}

impl<const BYTES: usize, A> PartialEq for SharedFields<BYTES, A>
where
    A: Allocator,
{
    fn eq(&self, other: &Self) -> bool {
        self.has_bits == other.has_bits && self.unknown_fields == other.unknown_fields
    }
}

impl<const BYTES: usize, A> Eq for SharedFields<BYTES, A> where A: Allocator {}

impl<const BYTES: usize, A> Default for SharedFields<BYTES, A>
where
    A: Allocator + Default + Clone,
{
    fn default() -> Self {
        Self::new_in(A::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_fields_small() {
        // 3 fields → 1 byte
        let mut shared = SharedFields::<1>::new();

        // All bits should be unset initially
        assert!(!shared.has_bits()[0]);
        assert!(!shared.has_bits()[1]);
        assert!(!shared.has_bits()[2]);

        // Set a bit
        shared.has_bits_mut().set(1, true);
        assert!(shared.has_bits()[1]);
        assert!(!shared.has_bits()[0]);
    }

    #[test]
    fn test_shared_fields_large() {
        // 100 fields → 13 bytes (⌈100/8⌉ = 13)
        let mut shared = SharedFields::<13>::new();

        // Set bit 99
        shared.has_bits_mut().set(99, true);
        assert!(shared.has_bits()[99]);
        assert!(!shared.has_bits()[0]);
    }

    #[test]
    fn test_memory_size() {
        use std::mem::size_of;

        // Layout now includes allocator state and unknown fields buffer handles.
        assert!(size_of::<SharedFields<1>>() >= size_of::<Vec<u8>>());
        assert!(size_of::<SharedFields<2>>() >= size_of::<Vec<u8>>());
        assert!(size_of::<SharedFields<13>>() >= size_of::<Vec<u8>>());
    }

    #[test]
    fn test_unknown_fields_default() {
        let shared = SharedFields::<5>::new();
        assert!(shared.unknown_fields().is_empty());
    }
}
