//! Companion namespace for [`crate::Student`]: `FIELD_*` / `BIT_*`.
//!
//! Sample-only two-hop inline child (`School → Student → Point` / `Address`).
//! Not in DESIGN.md.

// ---------------------------------------------------------------------------
// Bit indices — local to this message's `MessageCommon`.
// ---------------------------------------------------------------------------

pub const BIT_YEAR: usize = 0; // year (EXPLICIT presence)
pub const BIT_LOCATION: usize = 1; // location (inlined Point, EXPLICIT presence)
pub const BIT_HOME: usize = 2; // home (inlined Address, EXPLICIT presence)

// ---------------------------------------------------------------------------
// Proto field numbers
// ---------------------------------------------------------------------------

pub const FIELD_YEAR: u32 = 1; // year
pub const FIELD_LOCATION: u32 = 2; // location (inlined Point)
pub const FIELD_HOME: u32 = 3; // home (inlined Address)
