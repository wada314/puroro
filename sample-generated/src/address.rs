//! Companion namespace for [`crate::Address`]: `FIELD_*` / `BIT_*` and defaults.
//!
//! The `Address` struct itself lives in the crate root (the parent module).

pub(crate) mod defaults;

// ---------------------------------------------------------------------------
// Bit indices — presence then string SSO heap bits (1 = heap / 0 = inline),
// by ascending field number.
// ---------------------------------------------------------------------------

pub const BIT_STREET: usize = 0; // street (EXPLICIT presence)
pub const BIT_STREET_SSO: usize = 1; // street (SSO: 1 = heap)
pub const BIT_CITY: usize = 2; // city (EXPLICIT presence)
pub const BIT_CITY_SSO: usize = 3; // city (SSO: 1 = heap)
pub const BIT_POSTAL_CODE: usize = 4; // postal_code (EXPLICIT fixed32)
pub const BIT_LATITUDE: usize = 5; // latitude (EXPLICIT double)

// ---------------------------------------------------------------------------
// Proto field numbers
// ---------------------------------------------------------------------------

pub const FIELD_STREET: u32 = 1; // street
pub const FIELD_CITY: u32 = 2; // city
pub const FIELD_POSTAL_CODE: u32 = 3; // postal_code
pub const FIELD_LATITUDE: u32 = 4; // latitude
