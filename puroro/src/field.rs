//! Field operation utilities for generated code.
//!
//! This module provides reusable functions for field operations,
//! separating "shared" state (has_bits, bool_bits, etc.) from
//! "exclusive" field storage (individual field values).

/// Context for field operations containing shared state.
///
/// This struct groups together all the shared metadata needed
/// for field operations across a message.
#[derive(Debug)]
pub struct FieldContext<'a> {
    /// Reference to the has_bits field for presence tracking
    pub has_bits: &'a mut u32,
    /// Bit mask for this specific field
    pub has_bit_mask: u32,
}

impl<'a> FieldContext<'a> {
    /// Creates a new field context
    #[inline]
    pub fn new(has_bits: &'a mut u32, has_bit_mask: u32) -> Self {
        Self {
            has_bits,
            has_bit_mask,
        }
    }

    /// Marks the field as set (present)
    #[inline]
    pub fn mark_set(&mut self) {
        *self.has_bits |= self.has_bit_mask;
    }

    /// Marks the field as unset (not present)
    #[inline]
    pub fn mark_unset(&mut self) {
        *self.has_bits &= !self.has_bit_mask;
    }

    /// Checks if the field is set (present)
    #[inline]
    pub fn is_set(&self) -> bool {
        (*self.has_bits & self.has_bit_mask) != 0
    }
}

/// Sets a scalar field value.
///
/// For Copy types like i32, i64, f32, f64, bool, etc.
#[inline]
pub fn set_scalar<T: Copy>(mut ctx: FieldContext, storage: &mut T, value: T) {
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
pub fn has_scalar(ctx: &FieldContext) -> bool {
    ctx.is_set()
}

/// Clears a scalar field to its default value.
#[inline]
pub fn clear_scalar<T: Default>(mut ctx: FieldContext, storage: &mut T) {
    *storage = T::default();
    ctx.mark_unset();
}

/// Sets a string field value, reusing allocation when possible.
#[inline]
pub fn set_string(mut ctx: FieldContext, storage: &mut String, value: &str) {
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
pub fn has_string(ctx: &FieldContext) -> bool {
    ctx.is_set()
}

/// Clears a string field.
#[inline]
pub fn clear_string(mut ctx: FieldContext, storage: &mut String) {
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
/// * `has_mask` - Bit mask for presence tracking
/// * `bool_mask` - Bit mask for boolean value
/// * `value` - The boolean value to set
#[inline]
pub fn set_bool_packed(
    has_bits: &mut u32,
    bool_bits: &mut u32,
    has_mask: u32,
    bool_mask: u32,
    value: bool,
) {
    *has_bits |= has_mask; // Mark as set
    if value {
        *bool_bits |= bool_mask;
    } else {
        *bool_bits &= !bool_mask;
    }
}

/// Gets a boolean field value from a bit array.
#[inline]
pub fn get_bool_packed(bool_bits: u32, bool_mask: u32) -> bool {
    (bool_bits & bool_mask) != 0
}

/// Checks if a boolean field has been set.
#[inline]
pub fn has_bool_packed(has_bits: u32, has_mask: u32) -> bool {
    (has_bits & has_mask) != 0
}

/// Clears a boolean field stored in a bit array.
#[inline]
pub fn clear_bool_packed(has_bits: &mut u32, bool_bits: &mut u32, has_mask: u32, bool_mask: u32) {
    *has_bits &= !has_mask; // Mark as unset
    *bool_bits &= !bool_mask; // Clear value to false
}

/// Sets a bytes field value, reusing allocation when possible.
#[inline]
pub fn set_bytes(mut ctx: FieldContext, storage: &mut Vec<u8>, value: &[u8]) {
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
pub fn has_bytes(ctx: &FieldContext) -> bool {
    ctx.is_set()
}

/// Clears a bytes field.
#[inline]
pub fn clear_bytes(mut ctx: FieldContext, storage: &mut Vec<u8>) {
    storage.clear();
    ctx.mark_unset();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_context() {
        let mut has_bits: u32 = 0;
        let mask = 1 << 3;

        {
            let mut ctx = FieldContext::new(&mut has_bits, mask);
            assert!(!ctx.is_set());

            ctx.mark_set();
            assert!(ctx.is_set());
        }
        assert_eq!(has_bits, mask);

        {
            let mut ctx = FieldContext::new(&mut has_bits, mask);
            ctx.mark_unset();
            assert!(!ctx.is_set());
        }
        assert_eq!(has_bits, 0);
    }

    #[test]
    fn test_set_scalar() {
        let mut has_bits: u32 = 0;
        let mut value: i32 = 0;

        let ctx = FieldContext::new(&mut has_bits, 1 << 0);
        set_scalar(ctx, &mut value, 42);

        assert_eq!(value, 42);
        assert_eq!(has_bits, 1 << 0);
    }

    #[test]
    fn test_set_string() {
        let mut has_bits: u32 = 0;
        let mut value = String::new();

        let ctx = FieldContext::new(&mut has_bits, 1 << 1);
        set_string(ctx, &mut value, "hello");

        assert_eq!(value, "hello");
        assert_eq!(has_bits, 1 << 1);
    }

    #[test]
    fn test_string_allocation_reuse() {
        let mut has_bits: u32 = 0;
        let mut value = String::new();

        // First set - allocates
        let ctx = FieldContext::new(&mut has_bits, 1 << 1);
        set_string(
            ctx,
            &mut value,
            "A very long string that requires heap allocation",
        );
        let capacity_after_first = value.capacity();

        // Second set with shorter string - should reuse allocation
        has_bits = 0; // Reset
        let ctx = FieldContext::new(&mut has_bits, 1 << 1);
        set_string(ctx, &mut value, "Short");
        let capacity_after_second = value.capacity();

        assert_eq!(capacity_after_first, capacity_after_second);
        assert_eq!(value, "Short");
    }

    #[test]
    fn test_bool_packed() {
        let mut has_bits: u32 = 0;
        let mut bool_bits: u32 = 0;
        let has_mask = 1 << 0;
        let bool_mask = 1 << 0;

        // Set to true
        set_bool_packed(&mut has_bits, &mut bool_bits, has_mask, bool_mask, true);
        assert!(has_bool_packed(has_bits, has_mask));
        assert!(get_bool_packed(bool_bits, bool_mask));

        // Set to false
        set_bool_packed(&mut has_bits, &mut bool_bits, has_mask, bool_mask, false);
        assert!(has_bool_packed(has_bits, has_mask));
        assert!(!get_bool_packed(bool_bits, bool_mask));

        // Clear
        clear_bool_packed(&mut has_bits, &mut bool_bits, has_mask, bool_mask);
        assert!(!has_bool_packed(has_bits, has_mask));
        assert!(!get_bool_packed(bool_bits, bool_mask));
    }

    #[test]
    fn test_clear_scalar() {
        let mut has_bits: u32 = 0b1111;
        let mut value: i32 = 42;

        let ctx = FieldContext::new(&mut has_bits, 1 << 2);
        clear_scalar(ctx, &mut value);

        assert_eq!(value, 0);
        assert_eq!(has_bits, 0b1011); // Bit 2 cleared
    }

    #[test]
    fn test_clear_string() {
        let mut has_bits: u32 = 0b1111;
        let mut value = String::from("hello");

        let ctx = FieldContext::new(&mut has_bits, 1 << 1);
        clear_string(ctx, &mut value);

        assert_eq!(value, "");
        assert_eq!(has_bits, 0b1101); // Bit 1 cleared
    }
}
