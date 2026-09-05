//! Fallible read surface for [`Task`](crate::Task).
//!
//! Inherent `_mut` / `clear_*` stay off this trait. Eager [`Task`](crate::Task)
//! uses [`Infallible`](::core::convert::Infallible).

use crate::Address;
use crate::Point;
use crate::Task;
use crate::enums::{Priority, Status};
use crate::task::{Notification, NotificationCase};
use ::allocator_api2::alloc::Allocator;
use ::core::convert::Infallible;
use ::core::ops::Deref;
use ::puroro::{HasDefault, MapRef, OneofView, Optional};

/// Shared `notification()` projection used by [`TaskMessageFallible`].
type NotificationRef<'a, A> = Notification<&'a str, &'a str, i32, &'a Address<A>, bool>;

/// Per-message fallible getters for `Task`.
///
/// Call inherent methods on a concrete [`Task`](crate::Task) when you do not
/// need the generic surface; they stay infallible and do not go through
/// [`Result`].
pub trait TaskMessageFallible {
    type Error;
    type Alloc: Allocator;

    fn title(&self) -> Result<Optional<&str, impl HasDefault<&str>>, Self::Error>;
    fn score(&self) -> Result<i32, Self::Error>;
    fn max_retries(&self) -> Result<Optional<i32, impl HasDefault<i32>>, Self::Error>;
    fn owner_id(&self) -> Result<Optional<&str, impl HasDefault<&str>>, Self::Error>;
    fn payload(&self) -> Result<Optional<&[u8], impl HasDefault<&[u8]>>, Self::Error>;
    fn tag_ids(&self) -> Result<&[i32], Self::Error>;
    fn scores(&self) -> Result<&[i32], Self::Error>;
    fn labels(&self) -> Result<&[impl Deref<Target = str>], Self::Error>;
    fn status(&self) -> Result<Optional<Status, impl HasDefault<Status>>, Self::Error>;
    fn priority(&self) -> Result<Optional<Priority, impl HasDefault<Priority>>, Self::Error>;
    fn assignee(&self) -> Result<Option<&Address<Self::Alloc>>, Self::Error>;
    fn done(&self) -> Result<bool, Self::Error>;
    fn flag(&self) -> Result<Optional<bool, impl HasDefault<bool>>, Self::Error>;
    fn watchers(&self) -> Result<&[Address<Self::Alloc>], Self::Error>;
    fn votes(&self) -> Result<&[bool], Self::Error>;
    fn attributes(&self) -> Result<impl MapRef<str, i32> + '_, Self::Error>;
    fn origin(&self) -> Result<Option<&Point<Self::Alloc>>, Self::Error>;
    fn notification(
        &self,
    ) -> Result<
        impl OneofView<Case = NotificationCase, Ref = NotificationRef<'_, Self::Alloc>> + '_,
        Self::Error,
    >;
    fn email_address(&self) -> Result<Optional<&str, impl HasDefault<&str>>, Self::Error>;
    fn phone_number(&self) -> Result<Optional<&str, impl HasDefault<&str>>, Self::Error>;
    fn webhook_id(&self) -> Result<Optional<i32, impl HasDefault<i32>>, Self::Error>;
    fn postal(&self) -> Result<Option<&Address<Self::Alloc>>, Self::Error>;
    fn urgent(&self) -> Result<Optional<bool, impl HasDefault<bool>>, Self::Error>;
}

impl<A: Allocator> TaskMessageFallible for Task<A> {
    type Error = Infallible;
    type Alloc = A;

    fn title(&self) -> Result<Optional<&str, impl HasDefault<&str>>, Self::Error> {
        Ok(Task::title(self))
    }

    fn score(&self) -> Result<i32, Self::Error> {
        Ok(Task::score(self))
    }

    fn max_retries(&self) -> Result<Optional<i32, impl HasDefault<i32>>, Self::Error> {
        Ok(Task::max_retries(self))
    }

    fn owner_id(&self) -> Result<Optional<&str, impl HasDefault<&str>>, Self::Error> {
        Ok(Task::owner_id(self))
    }

    fn payload(&self) -> Result<Optional<&[u8], impl HasDefault<&[u8]>>, Self::Error> {
        Ok(Task::payload(self))
    }

    fn tag_ids(&self) -> Result<&[i32], Self::Error> {
        Ok(Task::tag_ids(self))
    }

    fn scores(&self) -> Result<&[i32], Self::Error> {
        Ok(Task::scores(self))
    }

    fn labels(&self) -> Result<&[impl Deref<Target = str>], Self::Error> {
        Ok(Task::labels(self))
    }

    fn status(&self) -> Result<Optional<Status, impl HasDefault<Status>>, Self::Error> {
        Ok(Task::status(self))
    }

    fn priority(&self) -> Result<Optional<Priority, impl HasDefault<Priority>>, Self::Error> {
        Ok(Task::priority(self))
    }

    fn assignee(&self) -> Result<Option<&Address<Self::Alloc>>, Self::Error> {
        Ok(Task::assignee(self))
    }

    fn done(&self) -> Result<bool, Self::Error> {
        Ok(Task::done(self))
    }

    fn flag(&self) -> Result<Optional<bool, impl HasDefault<bool>>, Self::Error> {
        Ok(Task::flag(self))
    }

    fn watchers(&self) -> Result<&[Address<Self::Alloc>], Self::Error> {
        Ok(Task::watchers(self))
    }

    fn votes(&self) -> Result<&[bool], Self::Error> {
        Ok(Task::votes(self))
    }

    fn attributes(&self) -> Result<impl MapRef<str, i32> + '_, Self::Error> {
        Ok(Task::attributes(self))
    }

    fn origin(&self) -> Result<Option<&Point<Self::Alloc>>, Self::Error> {
        Ok(Task::origin(self))
    }

    fn notification(
        &self,
    ) -> Result<
        impl OneofView<Case = NotificationCase, Ref = NotificationRef<'_, Self::Alloc>> + '_,
        Self::Error,
    > {
        Ok(Task::notification(self))
    }

    fn email_address(&self) -> Result<Optional<&str, impl HasDefault<&str>>, Self::Error> {
        Ok(Task::email_address(self))
    }

    fn phone_number(&self) -> Result<Optional<&str, impl HasDefault<&str>>, Self::Error> {
        Ok(Task::phone_number(self))
    }

    fn webhook_id(&self) -> Result<Optional<i32, impl HasDefault<i32>>, Self::Error> {
        Ok(Task::webhook_id(self))
    }

    fn postal(&self) -> Result<Option<&Address<Self::Alloc>>, Self::Error> {
        Ok(Task::postal(self))
    }

    fn urgent(&self) -> Result<Optional<bool, impl HasDefault<bool>>, Self::Error> {
        Ok(Task::urgent(self))
    }
}
