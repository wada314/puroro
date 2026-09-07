//! Read-oriented `Task` that applies numericals during a resumable scan.
//!
//! Singular string / bytes use [`SingularField`] + [`WireOrSso`]: Wire
//! spans promote to Inline / Heap on first get. Failed UTF-8 is sticky.
//! Nested `assignee` / `origin` merge each complete LEN into a child that
//! shares the island-root buffer. Repeated `watchers` appends one child per
//! occurrence. `attributes` stores map-entry spans and materialises on first
//! get. Repeated `labels` stores element spans and materialises on first get.
//! Oneof is still skipped. Getters require a finished
//! input stream so last-wins is final. `into_eager` re-merges `_wire` into
//! [`Task`]; encode writes `_wire` as-is.

use ::allocator_api2::alloc::{Allocator, Global};
use ::allocator_api2::vec::Vec as AllocVec;
use ::bytes::{Buf, BufMut};
use ::core::ops::{ControlFlow, Deref};
use ::puroro::{DecodeError, HasDefault, MapRef, Message, Optional};
use ::puroro_rt::decode::{LazyScan, ScannedRecord, merge_scanned_field, scanned_len_span};
use ::puroro_rt::{
    BitPacked, Closed, Expanded, Explicit, FieldDeallocVisitor, FieldVisitorMut, Implicit, Inline,
    InteriorBitArray, LazyMapField, LazyRepeatedField, LegacyRequired, MessageCommon, Open, Packed,
    ProtoBool, ProtoBytes, ProtoEnum, ProtoInt32, ProtoString, RepeatedField, SingularField,
    WireOrSso,
};

use crate::Task;
use crate::address::AddressLazy;
use crate::enums::{Priority, Status};
use crate::point::PointLazy;
use crate::task::defaults::MaxRetriesDefault;
use crate::task::{
    BIT_DONE_VALUE, BIT_FLAG, BIT_FLAG_VALUE, BIT_MAX_RETRIES, BIT_OWNER_ID,
    BIT_OWNER_ID_LAZY_KIND, BIT_PAYLOAD, BIT_PAYLOAD_LAZY_KIND, BIT_PRIORITY, BIT_TITLE,
    BIT_TITLE_LAZY_KIND, FIELD_ASSIGNEE, FIELD_ATTRIBUTES, FIELD_DONE, FIELD_FLAG, FIELD_LABELS,
    FIELD_MAX_RETRIES, FIELD_ORIGIN, FIELD_OWNER_ID, FIELD_PAYLOAD, FIELD_PRIORITY, FIELD_SCORE,
    FIELD_SCORES, FIELD_STATUS, FIELD_TAG_IDS, FIELD_TITLE, FIELD_VOTES, FIELD_WATCHERS,
};

/// Lazy `Task` with numerical catalog slots and singular `WireOrSso` LEN.
pub struct TaskLazy<A: Allocator = Global> {
    scan: LazyScan<A>,
    _common: MessageCommon<InteriorBitArray<3>, A>,
    title: SingularField<
        ProtoString,
        Explicit<{ BIT_TITLE }>,
        { FIELD_TITLE },
        A,
        WireOrSso<{ BIT_TITLE_LAZY_KIND }>,
    >,
    owner_id: SingularField<
        ProtoString,
        LegacyRequired<{ BIT_OWNER_ID }>,
        { FIELD_OWNER_ID },
        A,
        WireOrSso<{ BIT_OWNER_ID_LAZY_KIND }>,
    >,
    payload: SingularField<
        ProtoBytes,
        Explicit<{ BIT_PAYLOAD }>,
        { FIELD_PAYLOAD },
        A,
        WireOrSso<{ BIT_PAYLOAD_LAZY_KIND }>,
    >,
    assignee: Option<AddressLazy<A>>,
    origin: Option<PointLazy<A>>,
    watchers: AllocVec<AddressLazy<A>, A>,
    labels: LazyRepeatedField<ProtoString, Expanded, { FIELD_LABELS }, A>,
    attributes: LazyMapField<ProtoString, ProtoInt32, { FIELD_ATTRIBUTES }, A>,
    score: SingularField<ProtoInt32, Implicit, { FIELD_SCORE }, A>,
    max_retries: SingularField<
        ProtoInt32,
        Explicit<{ BIT_MAX_RETRIES }>,
        { FIELD_MAX_RETRIES },
        A,
        Inline,
        MaxRetriesDefault,
    >,
    tag_ids: RepeatedField<ProtoInt32, Packed, { FIELD_TAG_IDS }, A>,
    scores: RepeatedField<ProtoInt32, Expanded, { FIELD_SCORES }, A>,
    status: SingularField<ProtoEnum<Status, Open>, Implicit, { FIELD_STATUS }, A>,
    priority: SingularField<
        ProtoEnum<Priority, Closed>,
        Explicit<{ BIT_PRIORITY }>,
        { FIELD_PRIORITY },
        A,
    >,
    done: SingularField<ProtoBool, Implicit, { FIELD_DONE }, A, BitPacked<{ BIT_DONE_VALUE }>>,
    flag: SingularField<
        ProtoBool,
        Explicit<{ BIT_FLAG }>,
        { FIELD_FLAG },
        A,
        BitPacked<{ BIT_FLAG_VALUE }>,
    >,
    votes: RepeatedField<ProtoBool, Packed, { FIELD_VOTES }, A>,
}

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

impl<A: Allocator> TaskLazy<A> {
    pub fn title(&self) -> Result<Optional<&str, impl HasDefault<&str>>, DecodeError>
    where
        A: Clone,
    {
        self.require_finished()?;
        self.title.try_str(&self._common, self.scan.wire())
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
        self.owner_id.try_str(&self._common, self.scan.wire())
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
        self.payload.try_bytes(&self._common, self.scan.wire())
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
            .bind(self.scan.wire(), &self._common)?
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
        Ok(self.assignee.as_ref())
    }

    pub fn origin(&self) -> Result<Option<&PointLazy<A>>, DecodeError> {
        self.require_finished()?;
        Ok(self.origin.as_ref())
    }

    pub fn watchers(&self) -> Result<&[AddressLazy<A>], DecodeError> {
        self.require_finished()?;
        Ok(&self.watchers)
    }

    pub fn attributes(&self) -> Result<impl MapRef<str, i32> + '_, DecodeError>
    where
        A: Clone,
    {
        self.require_finished()?;
        self.attributes.bind(self.scan.wire(), &self._common)
    }

    /// Length of the gathered `_wire`. Same as a single-buffer ingest.
    pub fn encoded_len(&self) -> Result<usize, DecodeError> {
        self.scan.encoded_len()
    }

    /// Write `_wire` as-is (not a canonical field-by-field encode).
    pub fn encode<B: BufMut>(&self, buf: &mut B) -> Result<(), DecodeError> {
        self.scan.encode(buf)
    }

    pub fn encode_to_vec(&self) -> Result<Vec<u8>, DecodeError> {
        let mut out = Vec::with_capacity(self.encoded_len()?);
        self.encode(&mut out)?;
        Ok(out)
    }

    fn require_finished(&self) -> Result<(), DecodeError> {
        self.scan.require_finished()
    }

    fn visit_fields_mut<V: FieldVisitorMut<MessageCommon<InteriorBitArray<3>, A>>>(
        &mut self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("title", &mut self.title)?;
        v.visit("owner_id", &mut self.owner_id)?;
        v.visit("payload", &mut self.payload)?;
        v.visit("score", &mut self.score)?;
        v.visit("max_retries", &mut self.max_retries)?;
        v.visit("tag_ids", &mut self.tag_ids)?;
        v.visit("scores", &mut self.scores)?;
        v.visit("status", &mut self.status)?;
        v.visit("priority", &mut self.priority)?;
        v.visit("done", &mut self.done)?;
        v.visit("flag", &mut self.flag)?;
        v.visit("votes", &mut self.votes)?;
        ControlFlow::Continue(())
    }
}

impl<A: Allocator + Clone> TaskLazy<A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            scan: LazyScan::new_in(alloc.clone()),
            _common: MessageCommon::new_in(InteriorBitArray::zero(), alloc.clone()),
            title: SingularField::new_in(alloc.clone()),
            owner_id: SingularField::new_in(alloc.clone()),
            payload: SingularField::new_in(alloc.clone()),
            assignee: None,
            origin: None,
            watchers: AllocVec::new_in(alloc.clone()),
            labels: LazyRepeatedField::new_in(alloc.clone()),
            attributes: LazyMapField::new_in(alloc.clone()),
            score: SingularField::new_in(alloc.clone()),
            max_retries: SingularField::new_in(alloc.clone()),
            tag_ids: RepeatedField::new_in(alloc.clone()),
            scores: RepeatedField::new_in(alloc.clone()),
            status: SingularField::new_in(alloc.clone()),
            priority: SingularField::new_in(alloc.clone()),
            done: SingularField::new_in(alloc.clone()),
            flag: SingularField::new_in(alloc.clone()),
            votes: RepeatedField::new_in(alloc),
        }
    }

    /// Feed one I/O chunk. Complete numerical records are merged immediately;
    /// singular LEN records store a span into `_wire`. Nested message LENs are
    /// merged into a child lazy message. Map entries append a span.
    pub fn push(&mut self, chunk: &[u8]) -> Result<(), DecodeError> {
        let (origin, records) = self.scan.push(chunk)?;
        for rec in records {
            self.apply_record(&rec, origin)?;
        }
        Ok(())
    }

    /// Close the current input stream. Non-empty leftover is truncated input.
    pub fn finish(&mut self) -> Result<(), DecodeError> {
        self.scan.finish()
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
    /// Fields the lazy getters still skip (oneof) are applied here because the
    /// full ingest bytes are replayed.
    pub fn into_eager(self) -> Result<Task<A>, DecodeError> {
        self.require_finished()?;
        let mut eager = Task::new_in(self._common.alloc.clone());
        self.scan.for_each_body(|chunk| {
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
                let child = self
                    .assignee
                    .get_or_insert_with(|| AddressLazy::new_in(self._common.alloc.clone()));
                child.merge_shared(self.scan.shared(), span)
            }
            FIELD_ORIGIN => {
                let span = scanned_len_span(origin, rec)?;
                let child = self
                    .origin
                    .get_or_insert_with(|| PointLazy::new_in(self._common.alloc.clone()));
                child.merge_shared(self.scan.shared(), span)
            }
            FIELD_WATCHERS => {
                let span = scanned_len_span(origin, rec)?;
                let mut child = AddressLazy::new_in(self._common.alloc.clone());
                child.merge_shared(self.scan.shared(), span)?;
                self.watchers.push(child);
                Ok(())
            }
            FIELD_LABELS => {
                let span = scanned_len_span(origin, rec)?;
                self.labels
                    .store_span(span, self.scan.wire(), &self._common)
            }
            FIELD_ATTRIBUTES => {
                let span = scanned_len_span(origin, rec)?;
                self.attributes
                    .store_span(span, self.scan.wire(), &self._common)
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

impl<A: Allocator> Drop for TaskLazy<A> {
    fn drop(&mut self) {
        self.labels.deallocate(&self._common);
        self.attributes.deallocate(&self._common);
        let mut v = FieldDeallocVisitor::new(&self._common);
        let _ = self.visit_fields_mut(&mut v);
        self._common.deallocate();
    }
}
