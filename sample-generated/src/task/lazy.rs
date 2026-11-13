//! Inherent API for [`TaskLazy`](crate::TaskLazy).
//!
//! Numerical getters bind through [`Record::field`](puroro_rt::Record::field).
//! String and bytes getters read spans with [`HasField::field`](puroro_rt::HasField::field).
//! `into_eager` replays `_wire` through [`Record::decode_eager`](puroro_rt::Record::decode_eager).

use ::allocator_api2::alloc::Allocator;
use ::core::ops::Deref;
use ::puroro::{DecodeError, HasDefault, MapRef, Optional};
use ::puroro_rt::decode::LazyMessage;
use ::puroro_rt::{HasField, Lazy, OneofGroup, Record};

use super::notification::NotificationStorage;
use crate::AddressLazy;
use crate::PointLazy;
use crate::Task;
use crate::TaskLazy;
use crate::enums::{Priority, Status};
use crate::task::{
    FIELD_ASSIGNEE, FIELD_ATTRIBUTES, FIELD_DONE, FIELD_EMAIL_ADDRESS, FIELD_FLAG, FIELD_LABELS,
    FIELD_MAX_RETRIES, FIELD_ORIGIN, FIELD_OWNER_ID, FIELD_PAYLOAD, FIELD_PHONE_NUMBER,
    FIELD_POSTAL, FIELD_PRIORITY, FIELD_SCORE, FIELD_SCORES, FIELD_STATUS, FIELD_TAG_IDS,
    FIELD_TITLE, FIELD_URGENT, FIELD_VOTES, FIELD_WATCHERS, FIELD_WEBHOOK_ID, NotificationCase,
};
use crate::task_type::TaskFields;

impl<A: Allocator + Clone> TaskLazy<A> {
    pub fn title(&self) -> Result<Optional<&str, impl HasDefault<&str>>, DecodeError>
    where
        A: Clone,
    {
        LazyMessage::ensure_scanned(self)?;
        <TaskFields<A, Lazy<A>> as HasField<{ FIELD_TITLE }>>::field(&self.record.fields)
            .try_str(&self.record.common, self.record.common.lazy.scan.wire())
    }

    /// Wire presence only; does not UTF-8-check the payload.
    pub fn has_title(&self) -> Result<bool, DecodeError>
    where
        A: Clone,
    {
        LazyMessage::ensure_scanned(self)?;
        Ok(
            <TaskFields<A, Lazy<A>> as HasField<{ FIELD_TITLE }>>::field(&self.record.fields)
                .is_present(&self.record.common),
        )
    }

    pub fn owner_id(&self) -> Result<Optional<&str, impl HasDefault<&str>>, DecodeError>
    where
        A: Clone,
    {
        LazyMessage::ensure_scanned(self)?;
        <TaskFields<A, Lazy<A>> as HasField<{ FIELD_OWNER_ID }>>::field(&self.record.fields)
            .try_str(&self.record.common, self.record.common.lazy.scan.wire())
    }

    /// Wire presence only; does not UTF-8-check the payload.
    pub fn has_owner_id(&self) -> Result<bool, DecodeError>
    where
        A: Clone,
    {
        LazyMessage::ensure_scanned(self)?;
        Ok(
            <TaskFields<A, Lazy<A>> as HasField<{ FIELD_OWNER_ID }>>::field(&self.record.fields)
                .is_present(&self.record.common),
        )
    }

    pub fn payload(&self) -> Result<Optional<&[u8], impl HasDefault<&[u8]>>, DecodeError>
    where
        A: Clone,
    {
        LazyMessage::ensure_scanned(self)?;
        <TaskFields<A, Lazy<A>> as HasField<{ FIELD_PAYLOAD }>>::field(&self.record.fields)
            .try_bytes(&self.record.common, self.record.common.lazy.scan.wire())
    }

    pub fn has_payload(&self) -> Result<bool, DecodeError>
    where
        A: Clone,
    {
        LazyMessage::ensure_scanned(self)?;
        Ok(
            <TaskFields<A, Lazy<A>> as HasField<{ FIELD_PAYLOAD }>>::field(&self.record.fields)
                .is_present(&self.record.common),
        )
    }

    pub fn score(&self) -> Result<i32, DecodeError>
    where
        A: Clone,
    {
        LazyMessage::ensure_scanned(self)?;
        Ok(self.record.field::<{ FIELD_SCORE }>().value())
    }

    pub fn max_retries<'a>(&'a self) -> Result<Optional<i32, impl HasDefault<i32>>, DecodeError>
    where
        A: Clone + 'a,
    {
        LazyMessage::ensure_scanned(self)?;
        Ok(self.record.field::<{ FIELD_MAX_RETRIES }>().optional())
    }

    pub fn tag_ids(&self) -> Result<&[i32], DecodeError>
    where
        A: Clone,
    {
        LazyMessage::ensure_scanned(self)?;
        Ok(self.record.field::<{ FIELD_TAG_IDS }>().as_slice())
    }

    pub fn scores(&self) -> Result<&[i32], DecodeError>
    where
        A: Clone,
    {
        LazyMessage::ensure_scanned(self)?;
        Ok(self.record.field::<{ FIELD_SCORES }>().as_slice())
    }

    pub fn labels(&self) -> Result<&[impl Deref<Target = str>], DecodeError>
    where
        A: Clone,
    {
        LazyMessage::ensure_scanned(self)?;
        Ok(
            <TaskFields<A, Lazy<A>> as HasField<{ FIELD_LABELS }>>::field(&self.record.fields)
                .bind(self.record.common.lazy.scan.wire(), &self.record.common)?
                .as_slice(),
        )
    }

    pub fn status<'a>(&'a self) -> Result<Optional<Status, impl HasDefault<Status>>, DecodeError>
    where
        A: Clone + 'a,
    {
        LazyMessage::ensure_scanned(self)?;
        Ok(self.record.field::<{ FIELD_STATUS }>().optional())
    }

    pub fn priority<'a>(
        &'a self,
    ) -> Result<Optional<Priority, impl HasDefault<Priority>>, DecodeError>
    where
        A: Clone + 'a,
    {
        LazyMessage::ensure_scanned(self)?;
        Ok(self.record.field::<{ FIELD_PRIORITY }>().optional())
    }

    pub fn done(&self) -> Result<bool, DecodeError>
    where
        A: Clone,
    {
        LazyMessage::ensure_scanned(self)?;
        Ok(self.record.field::<{ FIELD_DONE }>().value())
    }

    pub fn flag<'a>(&'a self) -> Result<Optional<bool, impl HasDefault<bool>>, DecodeError>
    where
        A: Clone + 'a,
    {
        LazyMessage::ensure_scanned(self)?;
        Ok(self.record.field::<{ FIELD_FLAG }>().optional())
    }

    pub fn votes(&self) -> Result<&[bool], DecodeError>
    where
        A: Clone,
    {
        LazyMessage::ensure_scanned(self)?;
        Ok(self.record.field::<{ FIELD_VOTES }>().as_slice())
    }

    pub fn assignee(&self) -> Result<Option<&AddressLazy<A>>, DecodeError>
    where
        A: Clone,
    {
        LazyMessage::ensure_scanned(self)?;
        Ok(self.record.field::<{ FIELD_ASSIGNEE }>().get())
    }

    pub fn origin(&self) -> Result<Option<&PointLazy<A>>, DecodeError>
    where
        A: Clone,
    {
        LazyMessage::ensure_scanned(self)?;
        Ok(self.record.field::<{ FIELD_ORIGIN }>().get())
    }

    pub fn watchers(&self) -> Result<&[AddressLazy<A>], DecodeError>
    where
        A: Clone,
    {
        LazyMessage::ensure_scanned(self)?;
        Ok(self.record.field::<{ FIELD_WATCHERS }>().as_slice())
    }

    pub fn notification(&self) -> Result<Option<NotificationCase>, DecodeError>
    where
        A: Clone,
    {
        LazyMessage::ensure_scanned(self)?;
        Ok(self
            .record
            .fields
            .notification
            .as_ref()
            .map(<NotificationStorage<A, Lazy<A>> as OneofGroup>::case))
    }

    pub fn email_address(&self) -> Result<Optional<&str, impl HasDefault<&str>>, DecodeError>
    where
        A: Clone,
    {
        LazyMessage::ensure_scanned(self)?;
        <TaskFields<A, Lazy<A>> as HasField<{ FIELD_EMAIL_ADDRESS }>>::field(&self.record.fields)
            .try_str(&self.record.common, self.record.common.lazy.scan.wire())
    }

    pub fn phone_number(&self) -> Result<Optional<&str, impl HasDefault<&str>>, DecodeError>
    where
        A: Clone,
    {
        LazyMessage::ensure_scanned(self)?;
        <TaskFields<A, Lazy<A>> as HasField<{ FIELD_PHONE_NUMBER }>>::field(&self.record.fields)
            .try_str(&self.record.common, self.record.common.lazy.scan.wire())
    }

    pub fn webhook_id(&self) -> Result<Optional<i32, impl HasDefault<i32>>, DecodeError>
    where
        A: Clone,
    {
        LazyMessage::ensure_scanned(self)?;
        Ok(self.record.field::<{ FIELD_WEBHOOK_ID }>().optional())
    }

    pub fn postal(&self) -> Result<Option<&AddressLazy<A>>, DecodeError>
    where
        A: Clone,
    {
        LazyMessage::ensure_scanned(self)?;
        Ok(self.record.field::<{ FIELD_POSTAL }>().get())
    }

    pub fn urgent(&self) -> Result<Optional<bool, impl HasDefault<bool>>, DecodeError>
    where
        A: Clone,
    {
        LazyMessage::ensure_scanned(self)?;
        Ok(self.record.field::<{ FIELD_URGENT }>().optional())
    }

    pub fn attributes(&self) -> Result<impl MapRef<str, i32> + '_, DecodeError>
    where
        A: Clone,
    {
        LazyMessage::ensure_scanned(self)?;
        <TaskFields<A, Lazy<A>> as HasField<{ FIELD_ATTRIBUTES }>>::field(&self.record.fields)
            .bind(self.record.common.lazy.scan.wire(), &self.record.common)
    }

    /// Decode `_wire` into the eager message.
    pub fn into_eager(self) -> Result<Task<A>, DecodeError> {
        Ok(Task {
            record: Record::<_, A, Lazy<A>, _>::decode_eager(self.record)?,
        })
    }
}
