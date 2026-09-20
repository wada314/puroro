//! Inherent API for [`TaskLazy`](crate::TaskLazy) (`TaskImpl<A, Lazy<A>>`).
//!
//! Read-oriented: numericals apply during the scan; LEN fields store spans
//! (`WireOrSso` / `RepeatedSpans` / `MapSpans` / lazy children). Getters
//! require `finish`. `into_eager` re-merges `_wire` into [`Task`](crate::Task).

use ::allocator_api2::alloc::{Allocator, Global};
use ::bytes::{Buf, BufMut};
use ::core::ops::Deref;
use ::puroro::{DecodeError, HasDefault, MapRef, Message, Optional};
use ::puroro_rt::decode::{ScannedRecord, merge_scanned_field, scanned_len_span};
use ::puroro_rt::{
    InteriorBitArray, LazyMessageCommon, MapField, OneofGroup, OneofSlot, RepeatedField,
    SingularField,
};

use super::notification_lazy::NotificationLazyStorage;
use crate::AddressLazy;
use crate::PointLazy;
use crate::Task;
use crate::TaskLazy;
use crate::enums::{Priority, Status};
use crate::task::{
    BIT_OWNER_ID, BIT_PAYLOAD, BIT_TITLE, FIELD_ASSIGNEE, FIELD_ATTRIBUTES, FIELD_DONE,
    FIELD_EMAIL_ADDRESS, FIELD_FLAG, FIELD_LABELS, FIELD_MAX_RETRIES, FIELD_ORIGIN, FIELD_OWNER_ID,
    FIELD_PAYLOAD, FIELD_PHONE_NUMBER, FIELD_POSTAL, FIELD_PRIORITY, FIELD_SCORE, FIELD_SCORES,
    FIELD_STATUS, FIELD_TAG_IDS, FIELD_TITLE, FIELD_URGENT, FIELD_VOTES, FIELD_WATCHERS,
    FIELD_WEBHOOK_ID, NotificationCase,
};

impl<A: Allocator + Clone + Default> Default for TaskLazy<A> {
    fn default() -> Self {
        Self::new_in(A::default())
    }
}

impl TaskLazy<Global> {
    pub fn new() -> Self {
        Self::new_in(Global)
    }

    pub fn decode<B: Buf>(mut buf: B) -> Result<Self, DecodeError> {
        let mut msg = Self::new();
        msg.merge_from(&mut buf)?;
        Ok(msg)
    }
}

impl<A: Allocator + Clone> TaskLazy<A> {
    pub fn title(&self) -> Result<Optional<&str, impl HasDefault<&str>>, DecodeError>
    where
        A: Clone,
    {
        self.require_finished()?;
        self.title
            .try_str(&self._common, self._common.lazy.scan.wire())
    }

    /// Wire presence only; does not UTF-8-check the payload.
    pub fn has_title(&self) -> Result<bool, DecodeError> {
        self.require_finished()?;
        Ok(self._common.is_bit_set(BIT_TITLE))
    }

    pub fn owner_id(&self) -> Result<Optional<&str, impl HasDefault<&str>>, DecodeError>
    where
        A: Clone,
    {
        self.require_finished()?;
        self.owner_id
            .try_str(&self._common, self._common.lazy.scan.wire())
    }

    /// Wire presence only; does not UTF-8-check the payload.
    pub fn has_owner_id(&self) -> Result<bool, DecodeError> {
        self.require_finished()?;
        Ok(self._common.is_bit_set(BIT_OWNER_ID))
    }

    pub fn payload(&self) -> Result<Optional<&[u8], impl HasDefault<&[u8]>>, DecodeError>
    where
        A: Clone,
    {
        self.require_finished()?;
        self.payload
            .try_bytes(&self._common, self._common.lazy.scan.wire())
    }

    pub fn has_payload(&self) -> Result<bool, DecodeError> {
        self.require_finished()?;
        Ok(self._common.is_bit_set(BIT_PAYLOAD))
    }

    pub fn score(&self) -> Result<i32, DecodeError> {
        self.require_finished()?;
        Ok(self.score.bind(&self._common).value())
    }

    pub fn max_retries<'a>(&'a self) -> Result<Optional<i32, impl HasDefault<i32>>, DecodeError>
    where
        A: 'a,
    {
        self.require_finished()?;
        Ok(self.max_retries.bind(&self._common).optional())
    }

    pub fn tag_ids(&self) -> Result<&[i32], DecodeError> {
        self.require_finished()?;
        Ok(self.tag_ids.bind(&self._common).as_slice())
    }

    pub fn scores(&self) -> Result<&[i32], DecodeError> {
        self.require_finished()?;
        Ok(self.scores.bind(&self._common).as_slice())
    }

    pub fn labels(&self) -> Result<&[impl Deref<Target = str>], DecodeError>
    where
        A: Clone,
    {
        self.require_finished()?;
        Ok(self
            .labels
            .bind(self._common.lazy.scan.wire(), &self._common)?
            .as_slice())
    }

    pub fn status<'a>(&'a self) -> Result<Optional<Status, impl HasDefault<Status>>, DecodeError>
    where
        A: 'a,
    {
        self.require_finished()?;
        Ok(self.status.bind(&self._common).optional())
    }

    pub fn priority<'a>(
        &'a self,
    ) -> Result<Optional<Priority, impl HasDefault<Priority>>, DecodeError>
    where
        A: 'a,
    {
        self.require_finished()?;
        Ok(self.priority.bind(&self._common).optional())
    }

    pub fn done(&self) -> Result<bool, DecodeError> {
        self.require_finished()?;
        Ok(self.done.bind(&self._common).value())
    }

    pub fn flag<'a>(&'a self) -> Result<Optional<bool, impl HasDefault<bool>>, DecodeError>
    where
        A: 'a,
    {
        self.require_finished()?;
        Ok(self.flag.bind(&self._common).optional())
    }

    pub fn votes(&self) -> Result<&[bool], DecodeError> {
        self.require_finished()?;
        Ok(self.votes.bind(&self._common).as_slice())
    }

    pub fn assignee(&self) -> Result<Option<&AddressLazy<A>>, DecodeError> {
        self.require_finished()?;
        Ok(self.assignee.bind(&self._common).get())
    }

    pub fn origin(&self) -> Result<Option<&PointLazy<A>>, DecodeError> {
        self.require_finished()?;
        Ok(self.origin.bind(&self._common).get())
    }

    pub fn watchers(&self) -> Result<&[AddressLazy<A>], DecodeError> {
        self.require_finished()?;
        Ok(self.watchers.bind(&self._common).as_slice())
    }

    pub fn notification(&self) -> Result<Option<NotificationCase>, DecodeError> {
        self.require_finished()?;
        Ok(self
            .notification
            .as_ref()
            .map(NotificationLazyStorage::case))
    }

    pub fn email_address(&self) -> Result<Optional<&str, impl HasDefault<&str>>, DecodeError>
    where
        A: Clone,
    {
        self.require_finished()?;
        match self
            .notification
            .bind(&self._common)
            .variant_of::<FIELD_EMAIL_ADDRESS>()
            .as_field()
        {
            Some(f) => f.try_str(&self._common, self._common.lazy.scan.wire()),
            None => Ok(Optional::new(None)),
        }
    }

    pub fn phone_number(&self) -> Result<Optional<&str, impl HasDefault<&str>>, DecodeError>
    where
        A: Clone,
    {
        self.require_finished()?;
        match self
            .notification
            .bind(&self._common)
            .variant_of::<FIELD_PHONE_NUMBER>()
            .as_field()
        {
            Some(f) => f.try_str(&self._common, self._common.lazy.scan.wire()),
            None => Ok(Optional::new(None)),
        }
    }

    pub fn webhook_id(&self) -> Result<Optional<i32, impl HasDefault<i32>>, DecodeError> {
        self.require_finished()?;
        Ok(self
            .notification
            .bind(&self._common)
            .variant_of::<FIELD_WEBHOOK_ID>()
            .optional())
    }

    pub fn postal(&self) -> Result<Option<&AddressLazy<A>>, DecodeError> {
        self.require_finished()?;
        Ok(self
            .notification
            .bind(&self._common)
            .variant_of::<FIELD_POSTAL>()
            .get())
    }

    pub fn urgent(&self) -> Result<Optional<bool, impl HasDefault<bool>>, DecodeError> {
        self.require_finished()?;
        Ok(self
            .notification
            .bind(&self._common)
            .variant_of::<FIELD_URGENT>()
            .optional())
    }

    pub fn attributes(&self) -> Result<impl MapRef<str, i32> + '_, DecodeError>
    where
        A: Clone,
    {
        self.require_finished()?;
        self.attributes
            .bind(self._common.lazy.scan.wire(), &self._common)
    }

    /// Length of the gathered `_wire`. Same as a single-buffer ingest.
    pub fn encoded_len(&self) -> Result<usize, DecodeError> {
        self._common.lazy.scan.encoded_len()
    }

    /// Write `_wire` as-is (not a canonical field-by-field encode).
    pub fn encode<B: BufMut>(&self, buf: &mut B) -> Result<(), DecodeError> {
        self._common.lazy.scan.encode(buf)
    }

    pub fn encode_to_vec(&self) -> Result<Vec<u8>, DecodeError> {
        let mut out = Vec::with_capacity(self.encoded_len()?);
        self.encode(&mut out)?;
        Ok(out)
    }

    fn require_finished(&self) -> Result<(), DecodeError> {
        self._common.lazy.scan.require_finished()
    }
}

impl<A: Allocator + Clone> TaskLazy<A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            _common: LazyMessageCommon::new_in(InteriorBitArray::zero(), alloc.clone()),
            title: SingularField::new_in(alloc.clone()),
            owner_id: SingularField::new_in(alloc.clone()),
            payload: SingularField::new_in(alloc.clone()),
            assignee: SingularField::new_in(alloc.clone()),
            origin: SingularField::new_in(alloc.clone()),
            watchers: RepeatedField::new_in(alloc.clone()),
            labels: RepeatedField::new_in(alloc.clone()),
            attributes: MapField::new_in(alloc.clone()),
            score: SingularField::new_in(alloc.clone()),
            max_retries: SingularField::new_in(alloc.clone()),
            tag_ids: RepeatedField::new_in(alloc.clone()),
            scores: RepeatedField::new_in(alloc.clone()),
            status: SingularField::new_in(alloc.clone()),
            priority: SingularField::new_in(alloc.clone()),
            done: SingularField::new_in(alloc.clone()),
            flag: SingularField::new_in(alloc.clone()),
            votes: RepeatedField::new_in(alloc.clone()),
            notification: OneofSlot::new_in(alloc),
        }
    }

    /// Feed one I/O chunk. Complete numerical records are merged immediately;
    /// singular LEN records store a span into `_wire`. Nested message LENs are
    /// merged into a child lazy message. Map entries append a span.
    pub fn push(&mut self, chunk: &[u8]) -> Result<(), DecodeError> {
        let (origin, records) = self._common.lazy.scan.push(chunk)?;
        for rec in records {
            self.apply_record(&rec, origin)?;
        }
        Ok(())
    }

    /// Close the current input stream. Non-empty leftover is truncated input.
    pub fn finish(&mut self) -> Result<(), DecodeError> {
        self._common.lazy.scan.finish()
    }

    /// Merge one complete message body (push remaining bytes, then finish).
    ///
    /// A second call is a real protobuf merge: scalars last-wins, repeated
    /// appends. Distinct from concatenating I/O chunks of a single message.
    pub fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError> {
        while buf.has_remaining() {
            let n = buf.chunk().len();
            if n == 0 {
                let rest = buf.copy_to_bytes(buf.remaining());
                self.push(&rest)?;
                break;
            }
            self.push(buf.chunk())?;
            buf.advance(n);
        }
        self.finish()
    }

    /// Decode every field from `_wire` into an eager [`Task`].
    ///
    /// Fields not yet materialised on the lazy getters are applied here because
    /// the full ingest bytes are replayed.
    pub fn into_eager(self) -> Result<Task<A>, DecodeError> {
        self.require_finished()?;
        let mut eager = Task::new_in(self._common.alloc.clone());
        self._common.lazy.scan.for_each_body(|chunk| {
            let mut buf = chunk;
            Message::merge_from(&mut eager, &mut buf)
        })?;
        Ok(eager)
    }

    fn apply_record(&mut self, rec: &ScannedRecord<A>, origin: usize) -> Result<(), DecodeError> {
        match rec.field_number.as_u32() {
            FIELD_TITLE => {
                let span = scanned_len_span(origin, rec)?;
                self.title.store_len_span(span, &mut self._common);
                Ok(())
            }
            FIELD_OWNER_ID => {
                let span = scanned_len_span(origin, rec)?;
                self.owner_id.store_len_span(span, &mut self._common);
                Ok(())
            }
            FIELD_PAYLOAD => {
                let span = scanned_len_span(origin, rec)?;
                self.payload.store_len_span(span, &mut self._common);
                Ok(())
            }
            FIELD_ASSIGNEE => {
                let span = scanned_len_span(origin, rec)?;
                let root = self._common.lazy.scan.shared().clone_handle();
                self.assignee.merge_shared(&root, span, &mut self._common)
            }
            FIELD_ORIGIN => {
                let span = scanned_len_span(origin, rec)?;
                let root = self._common.lazy.scan.shared().clone_handle();
                self.origin.merge_shared(&root, span, &mut self._common)
            }
            FIELD_WATCHERS => {
                let span = scanned_len_span(origin, rec)?;
                let root = self._common.lazy.scan.shared().clone_handle();
                self.watchers.merge_shared(&root, span, &self._common)
            }
            FIELD_EMAIL_ADDRESS => {
                let span = scanned_len_span(origin, rec)?;
                self.notification
                    .bind_mut(&mut self._common)
                    .variant_mut::<FIELD_EMAIL_ADDRESS>()
                    .store_len_span(span, &mut self._common);
                Ok(())
            }
            FIELD_PHONE_NUMBER => {
                let span = scanned_len_span(origin, rec)?;
                self.notification
                    .bind_mut(&mut self._common)
                    .variant_mut::<FIELD_PHONE_NUMBER>()
                    .store_len_span(span, &mut self._common);
                Ok(())
            }
            FIELD_WEBHOOK_ID => merge_scanned_field(&rec.field, |wire_type, buf| {
                self.notification
                    .bind_mut(&mut self._common)
                    .variant_mut::<FIELD_WEBHOOK_ID>()
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, 0)
            }),
            FIELD_POSTAL => {
                let span = scanned_len_span(origin, rec)?;
                let root = self._common.lazy.scan.shared().clone_handle();
                self.notification
                    .bind_mut(&mut self._common)
                    .variant_mut::<FIELD_POSTAL>()
                    .merge_shared(&root, span, &mut self._common)
            }
            FIELD_URGENT => merge_scanned_field(&rec.field, |wire_type, buf| {
                self.notification
                    .bind_mut(&mut self._common)
                    .variant_mut::<FIELD_URGENT>()
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, 0)
            }),
            FIELD_LABELS => {
                let span = scanned_len_span(origin, rec)?;
                self.labels
                    .store_span(span, self._common.lazy.scan.wire(), &self._common)
            }
            FIELD_ATTRIBUTES => {
                let span = scanned_len_span(origin, rec)?;
                self.attributes
                    .store_span(span, self._common.lazy.scan.wire(), &self._common)
            }
            FIELD_SCORE => merge_scanned_field(&rec.field, |wire_type, buf| {
                self.score
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, 0)
            }),
            FIELD_MAX_RETRIES => merge_scanned_field(&rec.field, |wire_type, buf| {
                self.max_retries
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, 0)
            }),
            FIELD_TAG_IDS => merge_scanned_field(&rec.field, |wire_type, buf| {
                self.tag_ids
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, 0)
            }),
            FIELD_SCORES => merge_scanned_field(&rec.field, |wire_type, buf| {
                self.scores
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, 0)
            }),
            FIELD_STATUS => merge_scanned_field(&rec.field, |wire_type, buf| {
                self.status
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, 0)
            }),
            FIELD_PRIORITY => merge_scanned_field(&rec.field, |wire_type, buf| {
                self.priority
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, 0)
            }),
            FIELD_DONE => merge_scanned_field(&rec.field, |wire_type, buf| {
                self.done
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, 0)
            }),
            FIELD_FLAG => merge_scanned_field(&rec.field, |wire_type, buf| {
                self.flag
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, 0)
            }),
            FIELD_VOTES => merge_scanned_field(&rec.field, |wire_type, buf| {
                self.votes
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, 0)
            }),
            _ => Ok(()),
        }
    }
}
