//! Companion namespace for [`crate::School`]: `FIELD_*` / `BIT_*`.
//!
//! Sample-only parent of an inlined [`crate::Student`] (not in DESIGN.md).

// ---------------------------------------------------------------------------
// Bit indices — School's own bits, then the inlined Student's local 0.
// ---------------------------------------------------------------------------

pub const BIT_NAME: usize = 0; // name (EXPLICIT presence)
pub const BIT_NAME_SSO: usize = 1; // name (SSO: 1 = heap)
pub const BIT_STUDENT: usize = 2; // student (inlined Student, EXPLICIT presence)

/// Parent bit index of inlined [`crate::Student`]'s local bit 0 (`year` presence).
pub const BIT_STUDENT_BASE: usize = 3;

// ---------------------------------------------------------------------------
// Proto field numbers
// ---------------------------------------------------------------------------

pub const FIELD_NAME: u32 = 1; // name
pub const FIELD_STUDENT: u32 = 2; // student (inlined Student)
