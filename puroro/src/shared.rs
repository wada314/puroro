//! Shared fields for message implementations.
//!
//! This module provides a wrapper type for fields that are shared across
//! all message fields (presence tracking, boolean values, etc.).

use bitvec::prelude::*;

/// Shared fields for a message implementation.
///
/// This struct contains all fields that are shared across the message,
/// such as presence tracking bits, boolean value bits, etc.
///
/// # Type Parameters
/// - `BYTES`: Number of bytes needed for presence tracking (⌈fields/8⌉)
///
/// # Example
/// ```ignore
/// // For 3 fields: ⌈3/8⌉ = 1 byte
/// pub struct PersonImpl {
///     _shared: SharedFields<1>,
///     name: String,
///     age: i32,
///     email: String,
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct SharedFields<const BYTES: usize> {
    /// Presence tracking bits
    /// Uses BitArray with fixed-size array for stack allocation
    has_bits: BitArray<[u8; BYTES]>,
    // Future fields to be added here (not as function parameters):
    // bool_bits: BitArray<[u8; BOOL_BYTES]>,  // Packed boolean values
    // allocator: A,                            // Custom allocator (stored IN message)
    // _unknown_fields: Vec<u8, A>,            // Unknown fields from newer protos
    //
    // Key design: All shared state is INSIDE this struct.
    // This means field operation functions (set_*, clear_*, etc.) don't need
    // to change their signatures when we add new shared fields.
    // They always receive (&mut SharedFields, index, &mut field, value).
}

impl<const BYTES: usize> SharedFields<BYTES> {
    /// Creates new shared fields with all bits unset.
    #[inline]
    pub fn new() -> Self {
        Self {
            has_bits: BitArray::ZERO,
        }
    }

    /// Gets a mutable reference to the presence tracking bits.
    #[inline]
    pub fn has_bits_mut(&mut self) -> &mut BitSlice<u8, Lsb0> {
        self.has_bits.as_mut_bitslice()
    }

    /// Gets a reference to the presence tracking bits.
    #[inline]
    pub fn has_bits(&self) -> &BitSlice<u8, Lsb0> {
        self.has_bits.as_bitslice()
    }
}

impl<const BYTES: usize> Default for SharedFields<BYTES> {
    fn default() -> Self {
        Self::new()
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

        // 1 byte for ≤8 fields
        assert_eq!(size_of::<SharedFields<1>>(), 1);

        // 2 bytes for 9-16 fields
        assert_eq!(size_of::<SharedFields<2>>(), 2);

        // 13 bytes for 97-104 fields
        assert_eq!(size_of::<SharedFields<13>>(), 13);
    }
}
