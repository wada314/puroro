//! Companion namespace for [`crate::Student`]: `FIELD_*` / `BIT_*`.
//!
//! Sample-only two-hop inline child (`School → Student → Point` / `Address`).
//! Not in DESIGN.md.

use crate::address::BIT_COUNT as ADDRESS_BIT_COUNT;

// ---------------------------------------------------------------------------
// Bit indices — local to `Student` (owned common, or parent window + bit_base).
// ---------------------------------------------------------------------------

pub const BIT_YEAR: usize = 0; // year (EXPLICIT presence)
pub const BIT_LOCATION: usize = 1; // location (inlined Point, EXPLICIT presence)
pub const BIT_HOME: usize = 2; // home (inlined Address, EXPLICIT presence)

/// Local bit index of inlined [`crate::Address`]'s local bit 0 (`street` presence).
pub const BIT_HOME_BASE: usize = 3;

/// Parent bits occupied by an inlined `Student` (local fields + inlined `Address`).
pub const BIT_COUNT: usize = BIT_HOME_BASE + ADDRESS_BIT_COUNT;

// ---------------------------------------------------------------------------
// Proto field numbers
// ---------------------------------------------------------------------------

pub const FIELD_YEAR: u32 = 1; // year
pub const FIELD_LOCATION: u32 = 2; // location (inlined Point)
pub const FIELD_HOME: u32 = 3; // home (inlined Address)
