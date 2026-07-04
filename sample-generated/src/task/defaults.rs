//! Compile-time default markers for [`super::Task`] fields with non-zero proto defaults.

use ::puroro::HasDefault;

/// `[default = 3]` on `max_retries`.
pub struct MaxRetriesDefault;
impl HasDefault<i32> for MaxRetriesDefault {
    const DEFAULT: i32 = 3;
}
