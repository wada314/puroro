//! Generated `oneof notification` enum for `Task`.

use ::allocator_api2::alloc::Allocator;
use ::allocator_api2::boxed::Box as ABox;

/// `oneof notification { string email_address = 12; string phone_number = 13; }`
pub enum Notification<A: Allocator> {
    EmailAddress(ABox<str, A>),
    PhoneNumber(ABox<str, A>),
}

impl<A: Allocator> ::core::fmt::Debug for Notification<A> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Self::EmailAddress(s) => f.debug_tuple("EmailAddress").field(&&**s).finish(),
            Self::PhoneNumber(s) => f.debug_tuple("PhoneNumber").field(&&**s).finish(),
        }
    }
}

impl<A: Allocator> PartialEq for Notification<A> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::EmailAddress(a), Self::EmailAddress(b)) => a == b,
            (Self::PhoneNumber(a), Self::PhoneNumber(b)) => a == b,
            _ => false,
        }
    }
}

impl<A: Allocator> Eq for Notification<A> {}
