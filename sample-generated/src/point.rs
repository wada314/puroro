//! Companion namespace for [`crate::Point`]: `FIELD_*` constants.
//!
//! Sample-only inlined nested-message experiment (not in DESIGN.md yet).

/// Inlined `Point` occupies no parent bits (implicit scalars only).
pub const BIT_COUNT: usize = 0;

pub const FIELD_X: u32 = 1; // x
pub const FIELD_Y: u32 = 2; // y
