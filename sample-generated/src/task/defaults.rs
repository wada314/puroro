//! Compile-time default markers for [`crate::Task`] fields with non-zero proto defaults.

use ::puroro::HasDefault;

/// `[default = 3]` on `max_retries`.
pub struct MaxRetriesDefault;
impl HasDefault<i32> for MaxRetriesDefault {
    const DEFAULT: i32 = 3;
}

/// `[default = -1]` on oneof member `notification.webhook_id`.
///
/// Used only by the immutable getter's [`Optional`](::puroro::Optional) fallback
/// when that variant is not the active case. `_mut` still installs type-default
/// storage (`0`), matching official `mutable_*` semantics.
pub struct WebhookIdDefault;
impl HasDefault<i32> for WebhookIdDefault {
    const DEFAULT: i32 = -1;
}
