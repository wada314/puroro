//! Companion namespace for [`crate::Task`]: `FIELD_*` / `BIT_*`, defaults, and
//! the `notification` oneof submodule.
//!
//! The `Task` struct itself lives in the crate root (the parent module).

pub(crate) mod defaults;
pub mod notification;

pub use notification::{Notification, NotificationCase};

// ---------------------------------------------------------------------------
// Bit indices — presence, string / bytes SSO heap bits (1 = heap / 0 = inline),
// bool value bits, then inlined nested `BIT_COUNT` ranges, assigned by
// ascending field number in one pass.
// ---------------------------------------------------------------------------

pub const BIT_TITLE: usize = 0; // title (EXPLICIT presence)
pub const BIT_TITLE_SSO: usize = 1; // title (SSO: 1 = heap)
pub const BIT_MAX_RETRIES: usize = 2; // max_retries (EXPLICIT presence)
pub const BIT_OWNER_ID: usize = 3; // owner_id (LEGACY_REQUIRED presence)
pub const BIT_OWNER_ID_SSO: usize = 4; // owner_id (SSO: 1 = heap)
pub const BIT_PAYLOAD: usize = 5; // payload (EXPLICIT presence)
pub const BIT_PAYLOAD_SSO: usize = 6; // payload (SSO: 1 = heap)
pub const BIT_PRIORITY: usize = 7; // priority (EXPLICIT presence)
pub const BIT_EMAIL_ADDRESS_SSO: usize = 8; // notification.email_address (SSO: 1 = heap)
pub const BIT_PHONE_NUMBER_SSO: usize = 9; // notification.phone_number (SSO: 1 = heap)
/// Parent bit index of inlined `notification.postal` (`Address`) local bit 0.
pub const BIT_POSTAL_BASE: usize = 10; // notification.postal (inlined Address, 6 bits)
pub const BIT_DONE_VALUE: usize = 16; // done (IMPLICIT bool value)
pub const BIT_FLAG: usize = 17; // flag (EXPLICIT presence)
pub const BIT_FLAG_VALUE: usize = 18; // flag (EXPLICIT bool value)
pub const BIT_URGENT_VALUE: usize = 19; // notification.urgent (oneof bool value)
pub const BIT_ORIGIN: usize = 20; // origin (inlined nested Point, EXPLICIT presence)

// ---------------------------------------------------------------------------
// Proto field numbers
// ---------------------------------------------------------------------------

pub const FIELD_TITLE: u32 = 1; // title
pub const FIELD_SCORE: u32 = 2; // score
pub const FIELD_MAX_RETRIES: u32 = 3; // max_retries
pub const FIELD_OWNER_ID: u32 = 4; // owner_id
pub const FIELD_PAYLOAD: u32 = 5; // payload
pub const FIELD_TAG_IDS: u32 = 6; // tag_ids
pub const FIELD_SCORES: u32 = 7; // scores
pub const FIELD_LABELS: u32 = 8; // labels
pub const FIELD_STATUS: u32 = 9; // status
pub const FIELD_PRIORITY: u32 = 10; // priority
pub const FIELD_ASSIGNEE: u32 = 11; // assignee
pub const FIELD_EMAIL_ADDRESS: u32 = 12; // notification.email_address
pub const FIELD_PHONE_NUMBER: u32 = 13; // notification.phone_number
pub const FIELD_WEBHOOK_ID: u32 = 14; // notification.webhook_id
pub const FIELD_POSTAL: u32 = 15; // notification.postal
pub const FIELD_DONE: u32 = 16; // done (IMPLICIT bool)
pub const FIELD_FLAG: u32 = 17; // flag (EXPLICIT bool)
pub const FIELD_URGENT: u32 = 18; // notification.urgent (oneof bool)
pub const FIELD_WATCHERS: u32 = 19; // watchers (repeated Address)
pub const FIELD_VOTES: u32 = 20; // votes (repeated bool PACKED)
pub const FIELD_ATTRIBUTES: u32 = 21; // attributes (map<string, int32>)
pub const FIELD_ORIGIN: u32 = 22; // origin (inlined Point)
