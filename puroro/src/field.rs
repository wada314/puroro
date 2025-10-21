//! Field operation utilities for generated code.
//!
//! This module provides reusable functions for field operations,
//! separating "shared" state (has_bits, bool_bits, etc.) from
//! "exclusive" field storage (individual field values).
//!
//! All implementations use `bitvec` for presence tracking,
//! which supports any number of fields without artificial limits.

use bitvec::prelude::*;

/// Context for field operations containing shared state.
///
/// This struct groups together all the shared metadata needed
/// for field operations across a message.
#[derive(Debug)]
pub struct FieldContext<'a, T = usize, O = Lsb0>
where
    T: BitStore,
    O: BitOrder,
{
    /// Reference to the has_bits field for presence tracking
    pub has_bits: &'a mut BitSlice<T, O>,
    /// Bit index for this specific field
    pub bit_index: usize,
}

impl<'a, T, O> FieldContext<'a, T, O>
where
    T: BitStore,
    O: BitOrder,
{
    /// Creates a new field context
    #[inline]
    pub fn new(has_bits: &'a mut BitSlice<T, O>, bit_index: usize) -> Self {
        Self {
            has_bits,
            bit_index,
        }
    }

    /// Marks the field as set (present)
    #[inline]
    pub fn mark_set(&mut self) {
        self.has_bits.set(self.bit_index, true);
    }

    /// Marks the field as unset (not present)
    #[inline]
    pub fn mark_unset(&mut self) {
        self.has_bits.set(self.bit_index, false);
    }

    /// Checks if the field is set (present)
    #[inline]
    pub fn is_set(&self) -> bool {
        self.has_bits[self.bit_index]
    }
}

/// Sets a scalar field value.
///
/// For Copy types like i32, i64, f32, f64, bool, etc.
#[inline]
pub fn set_scalar<T, S, O>(mut ctx: FieldContext<T, O>, storage: &mut S, value: S)
where
    T: BitStore,
    O: BitOrder,
    S: Copy,
{
    *storage = value;
    ctx.mark_set();
}

/// Gets a scalar field value.
#[inline]
pub fn get_scalar<T: Copy>(storage: &T) -> T {
    *storage
}

/// Checks if a scalar field has been set.
#[inline]
pub fn has_scalar<T, O>(ctx: &FieldContext<T, O>) -> bool
where
    T: BitStore,
    O: BitOrder,
{
    ctx.is_set()
}

/// Clears a scalar field to its default value.
#[inline]
pub fn clear_scalar<T, S, O>(mut ctx: FieldContext<T, O>, storage: &mut S)
where
    T: BitStore,
    O: BitOrder,
    S: Default,
{
    *storage = S::default();
    ctx.mark_unset();
}

/// Sets a string field value, reusing allocation when possible.
#[inline]
pub fn set_string<T, O>(mut ctx: FieldContext<T, O>, storage: &mut String, value: &str)
where
    T: BitStore,
    O: BitOrder,
{
    value.clone_into(storage); // Reuse allocation
    ctx.mark_set();
}

/// Gets a string field value as &str.
#[inline]
pub fn get_string(storage: &String) -> &str {
    storage.as_str()
}

/// Checks if a string field has been set.
#[inline]
pub fn has_string<T, O>(ctx: &FieldContext<T, O>) -> bool
where
    T: BitStore,
    O: BitOrder,
{
    ctx.is_set()
}

/// Clears a string field.
#[inline]
pub fn clear_string<T, O>(mut ctx: FieldContext<T, O>, storage: &mut String)
where
    T: BitStore,
    O: BitOrder,
{
    storage.clear();
    ctx.mark_unset();
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
pub fn set_bytes<T, O>(mut ctx: FieldContext<T, O>, storage: &mut Vec<u8>, value: &[u8])
where
    T: BitStore,
    O: BitOrder,
{
    storage.clear();
    storage.extend_from_slice(value);
    ctx.mark_set();
}

/// Gets a bytes field value as &[u8].
#[inline]
pub fn get_bytes(storage: &Vec<u8>) -> &[u8] {
    storage.as_slice()
}

/// Checks if a bytes field has been set.
#[inline]
pub fn has_bytes<T, O>(ctx: &FieldContext<T, O>) -> bool
where
    T: BitStore,
    O: BitOrder,
{
    ctx.is_set()
}

/// Clears a bytes field.
#[inline]
pub fn clear_bytes<T, O>(mut ctx: FieldContext<T, O>, storage: &mut Vec<u8>)
where
    T: BitStore,
    O: BitOrder,
{
    storage.clear();
    ctx.mark_unset();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_context() {
        let mut bits = bitvec![0; 100];

        {
            let mut ctx = FieldContext::new(&mut bits, 42);
            assert!(!ctx.is_set());

            ctx.mark_set();
            assert!(ctx.is_set());
        }
        assert!(bits[42]);

        {
            let mut ctx = FieldContext::new(&mut bits, 42);
            ctx.mark_unset();
            assert!(!ctx.is_set());
        }
        assert!(!bits[42]);
    }

    #[test]
    fn test_set_scalar() {
        let mut bits = bitvec![0; 10];
        let mut value: i32 = 0;

        let ctx = FieldContext::new(&mut bits, 0);
        set_scalar(ctx, &mut value, 42);

        assert_eq!(value, 42);
        assert!(bits[0]);
    }

    #[test]
    fn test_set_string() {
        let mut bits = bitvec![0; 10];
        let mut value = String::new();

        let ctx = FieldContext::new(&mut bits, 1);
        set_string(ctx, &mut value, "hello");

        assert_eq!(value, "hello");
        assert!(bits[1]);
    }

    #[test]
    fn test_string_allocation_reuse() {
        let mut bits = bitvec![0; 10];
        let mut value = String::new();

        // First set - allocates
        let ctx = FieldContext::new(&mut bits, 1);
        set_string(
            ctx,
            &mut value,
            "A very long string that requires heap allocation",
        );
        let capacity_after_first = value.capacity();

        // Second set with shorter string - should reuse allocation
        bits.set(1, false); // Reset
        let ctx = FieldContext::new(&mut bits, 1);
        set_string(ctx, &mut value, "Short");
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
        let mut bits = bitvec![1; 10]; // All set
        let mut value: i32 = 42;

        let ctx = FieldContext::new(&mut bits, 2);
        clear_scalar(ctx, &mut value);

        assert_eq!(value, 0);
        assert!(!bits[2]);
    }

    #[test]
    fn test_clear_string() {
        let mut bits = bitvec![1; 10];
        let mut value = String::from("hello");

        let ctx = FieldContext::new(&mut bits, 1);
        clear_string(ctx, &mut value);

        assert_eq!(value, "");
        assert!(!bits[1]);
    }

    #[test]
    fn test_large_message() {
        // Test with a message that has 100 optional fields
        let mut bits = bitvec![0; 100];

        // Set field 99
        FieldContext::new(&mut bits, 99).mark_set();
        assert!(bits[99]);

        // Set field 0
        FieldContext::new(&mut bits, 0).mark_set();
        assert!(bits[0]);
        assert!(bits[99]);
    }
}
