//! Sample of the `oneof notification` enum puroro generates for `Task`.

use ::allocator_api2::alloc::Allocator;
use ::unmanaged::UnmanagedString;

/// `oneof notification { string email_address = 12; string phone_number = 13; }`
///
/// Variants hold allocator-less [`UnmanagedString`]; the parent `Task` releases
/// the active variant via [`Notification::deallocate`] in its `Drop`.
pub enum Notification {
    EmailAddress(UnmanagedString),
    PhoneNumber(UnmanagedString),
}

impl Notification {
    /// Drops the active variant's string and frees it through `alloc`.
    ///
    /// # Safety
    ///
    /// `alloc` must be the allocator that owns the variant's buffer.
    pub unsafe fn deallocate<A: Allocator>(self, alloc: A) {
        match self {
            Self::EmailAddress(s) | Self::PhoneNumber(s) => unsafe { s.deallocate(alloc) },
        }
    }
}

impl ::core::fmt::Debug for Notification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Self::EmailAddress(s) => f.debug_tuple("EmailAddress").field(&&**s).finish(),
            Self::PhoneNumber(s) => f.debug_tuple("PhoneNumber").field(&&**s).finish(),
        }
    }
}

impl PartialEq for Notification {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::EmailAddress(a), Self::EmailAddress(b)) => **a == **b,
            (Self::PhoneNumber(a), Self::PhoneNumber(b)) => **a == **b,
            _ => false,
        }
    }
}

impl Eq for Notification {}
