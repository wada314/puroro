//! Field operation utilities for generated code.
//!
//! This module provides reusable functions for field operations,
//! separating "shared" state (has_bits, bool_bits, etc.) from
//! "exclusive" field storage (individual field values).
//!
//! All implementations use `bitvec` for presence tracking,
//! which supports any number of fields without artificial limits.

use crate::shared::SharedFields;
use bitvec::prelude::*;

/// Sets a scalar field value.
///
/// For Copy types like i32, i64, f32, f64, bool, etc.
#[inline]
pub fn set_scalar<const BYTES: usize, T: Copy>(
    shared: &mut SharedFields<BYTES>,
    bit_index: usize,
    storage: &mut T,
    value: T,
) {
    *storage = value;
    shared.has_bits_mut().set(bit_index, true);
}

/// Gets a scalar field value.
#[inline]
pub fn get_scalar<T: Copy>(storage: &T) -> T {
    *storage
}

/// Clears a scalar field to its default value.
#[inline]
pub fn clear_scalar<const BYTES: usize, T: Default>(
    shared: &mut SharedFields<BYTES>,
    bit_index: usize,
    storage: &mut T,
) {
    *storage = T::default();
    shared.has_bits_mut().set(bit_index, false);
}

/// Sets a string field value, reusing allocation when possible.
#[inline]
pub fn set_string<const BYTES: usize>(
    shared: &mut SharedFields<BYTES>,
    bit_index: usize,
    storage: &mut String,
    value: &str,
) {
    value.clone_into(storage); // Reuse allocation
    shared.has_bits_mut().set(bit_index, true);
}

/// Gets a string field value as &str.
#[inline]
pub fn get_string(storage: &String) -> &str {
    storage.as_str()
}

/// Clears a string field.
#[inline]
pub fn clear_string<const BYTES: usize>(
    shared: &mut SharedFields<BYTES>,
    bit_index: usize,
    storage: &mut String,
) {
    storage.clear();
    shared.has_bits_mut().set(bit_index, false);
}

/// Sets a boolean field value stored in a bit array.
///
/// Boolean fields are stored in a separate bit array to save memory.
/// This function handles both the presence bit and the value bit.
///
/// # Arguments
/// * `has_bits` - Presence tracking bits
/// * `bool_bits` - Boolean value storage bits
/// * `has_index` - Bit index for presence tracking
/// * `bool_index` - Bit index for boolean value
/// * `value` - The boolean value to set
#[inline]
pub fn set_bool_packed(
    has_bits: &mut BitSlice,
    bool_bits: &mut BitSlice,
    has_index: usize,
    bool_index: usize,
    value: bool,
) {
    has_bits.set(has_index, true); // Mark as set
    bool_bits.set(bool_index, value);
}

/// Gets a boolean field value from a bit array.
#[inline]
pub fn get_bool_packed(bool_bits: &BitSlice, bool_index: usize) -> bool {
    bool_bits[bool_index]
}

/// Checks if a boolean field has been set.
#[inline]
pub fn has_bool_packed(has_bits: &BitSlice, has_index: usize) -> bool {
    has_bits[has_index]
}

/// Clears a boolean field stored in a bit array.
#[inline]
pub fn clear_bool_packed(
    has_bits: &mut BitSlice,
    bool_bits: &mut BitSlice,
    has_index: usize,
    bool_index: usize,
) {
    has_bits.set(has_index, false); // Mark as unset
    bool_bits.set(bool_index, false); // Clear value to false
}

/// Sets a bytes field value, reusing allocation when possible.
#[inline]
pub fn set_bytes<const BYTES: usize>(
    shared: &mut SharedFields<BYTES>,
    bit_index: usize,
    storage: &mut Vec<u8>,
    value: &[u8],
) {
    storage.clear();
    storage.extend_from_slice(value);
    shared.has_bits_mut().set(bit_index, true);
}

/// Gets a bytes field value as &[u8].
#[inline]
pub fn get_bytes(storage: &Vec<u8>) -> &[u8] {
    storage.as_slice()
}

/// Clears a bytes field.
#[inline]
pub fn clear_bytes<const BYTES: usize>(
    shared: &mut SharedFields<BYTES>,
    bit_index: usize,
    storage: &mut Vec<u8>,
) {
    storage.clear();
    shared.has_bits_mut().set(bit_index, false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_scalar() {
        let mut shared = SharedFields::<2>::new();
        let mut value: i32 = 0;

        set_scalar(&mut shared, 0, &mut value, 42);

        assert_eq!(value, 42);
        assert!(shared.has_bits()[0]);
    }

    #[test]
    fn test_set_string() {
        let mut shared = SharedFields::<2>::new();
        let mut value = String::new();

        set_string(&mut shared, 1, &mut value, "hello");

        assert_eq!(value, "hello");
        assert!(shared.has_bits()[1]);
    }

    #[test]
    fn test_string_allocation_reuse() {
        let mut shared = SharedFields::<2>::new();
        let mut value = String::new();

        // First set - allocates
        set_string(
            &mut shared,
            1,
            &mut value,
            "A very long string that requires heap allocation",
        );
        let capacity_after_first = value.capacity();

        // Second set with shorter string - should reuse allocation
        shared.has_bits_mut().set(1, false); // Reset
        set_string(&mut shared, 1, &mut value, "Short");
        let capacity_after_second = value.capacity();

        assert_eq!(capacity_after_first, capacity_after_second);
        assert_eq!(value, "Short");
    }

    #[test]
    fn test_bool_packed() {
        let mut has_bits = bitvec![0; 10];
        let mut bool_bits = bitvec![0; 10];

        // Set to true
        set_bool_packed(&mut has_bits, &mut bool_bits, 0, 0, true);
        assert!(has_bool_packed(&has_bits, 0));
        assert!(get_bool_packed(&bool_bits, 0));

        // Set to false
        set_bool_packed(&mut has_bits, &mut bool_bits, 0, 0, false);
        assert!(has_bool_packed(&has_bits, 0));
        assert!(!get_bool_packed(&bool_bits, 0));

        // Clear
        clear_bool_packed(&mut has_bits, &mut bool_bits, 0, 0);
        assert!(!has_bool_packed(&has_bits, 0));
        assert!(!get_bool_packed(&bool_bits, 0));
    }

    #[test]
    fn test_clear_scalar() {
        let mut shared = SharedFields::<2>::new();
        shared.has_bits_mut().set(2, true); // Mark as set
        let mut value: i32 = 42;

        clear_scalar(&mut shared, 2, &mut value);

        assert_eq!(value, 0);
        assert!(!shared.has_bits()[2]);
    }

    #[test]
    fn test_clear_string() {
        let mut shared = SharedFields::<2>::new();
        shared.has_bits_mut().set(1, true); // Mark as set
        let mut value = String::from("hello");

        clear_string(&mut shared, 1, &mut value);

        assert_eq!(value, "");
        assert!(!shared.has_bits()[1]);
    }

    #[test]
    fn test_large_message() {
        // Test with a message that has 100 optional fields
        // ⌈100/8⌉ = 13 bytes
        let mut shared = SharedFields::<13>::new();
        let mut value1: i32 = 0;
        let mut value2: i32 = 0;

        // Set field 99
        set_scalar(&mut shared, 99, &mut value1, 999);
        assert!(shared.has_bits()[99]);

        // Set field 0
        set_scalar(&mut shared, 0, &mut value2, 123);
        assert!(shared.has_bits()[0]);
        assert!(shared.has_bits()[99]);
        assert_eq!(value1, 999);
        assert_eq!(value2, 123);
    }
}
