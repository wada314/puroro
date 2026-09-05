//! Read-oriented `Task` that applies numericals during a resumable scan.
//!
//! LEN / map / nested / oneof records are skipped. Numerical getters require a
//! finished input stream so last-wins is final.

use ::allocator_api2::alloc::{Allocator, Global};
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;
use ::bytes::Buf;
use ::core::ops::ControlFlow;
use ::puroro::{DecodeError, HasDefault, Optional};
use ::puroro_rt::decode::{RecordScanner, merge_scanned_field};
use ::puroro_rt::{
    BitPacked, Closed, Expanded, Explicit, FieldDeallocVisitor, FieldVisitorMut, Implicit, Inline,
    MessageCommon, Open, Packed, ProtoBool, ProtoEnum, ProtoInt32, RepeatedField, SingularField,
};

use crate::enums::{Priority, Status};
use crate::task::defaults::MaxRetriesDefault;
use crate::task::{
    BIT_DONE_VALUE, BIT_FLAG, BIT_FLAG_VALUE, BIT_MAX_RETRIES, BIT_PRIORITY, FIELD_DONE,
    FIELD_FLAG, FIELD_MAX_RETRIES, FIELD_PRIORITY, FIELD_SCORE, FIELD_SCORES, FIELD_STATUS,
    FIELD_TAG_IDS, FIELD_VOTES,
};

/// Lazy `Task` with numerical catalog slots; LEN fields are not stored yet.
pub struct TaskLazy<A: Allocator = Global> {
    scanner: RecordScanner<A>,
    finished: bool,
    _common: MessageCommon<BitArray<[u8; 2], Lsb0>, A>,
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

    fn require_finished(&self) -> Result<(), DecodeError> {
        if self.finished {
            Ok(())
        } else if self.scanner.needs_more() {
            Err(DecodeError::TruncatedMessage)
        } else {
            Err(DecodeError::UnfinishedMessage)
        }
    }

    fn visit_fields_mut<V: FieldVisitorMut<MessageCommon<BitArray<[u8; 2], Lsb0>, A>>>(
        &mut self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
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
            scanner: RecordScanner::new_in(alloc.clone()),
            finished: true,
            _common: MessageCommon::new_in(BitArray::ZERO, alloc.clone()),
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

    /// Feed one I/O chunk. Complete numerical records are merged immediately.
    pub fn push(&mut self, chunk: &[u8]) -> Result<(), DecodeError> {
        self.finished = false;
        let records = self.scanner.push(chunk)?;
        for field in records {
            self.apply_record(&field)?;
        }
        Ok(())
    }

    /// Close the current input stream. Non-empty leftover is truncated input.
    pub fn finish(&mut self) -> Result<(), DecodeError> {
        self.scanner.finish()?;
        self.finished = true;
        Ok(())
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

    fn apply_record<L: AsRef<[u8]>>(
        &mut self,
        field: &::puroro_rt::Field<L>,
    ) -> Result<(), DecodeError> {
        match field.field_number.as_u32() {
            FIELD_SCORE => merge_scanned_field(field, |wire_type, buf| {
                self.score
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, 0)
            }),
            FIELD_MAX_RETRIES => merge_scanned_field(field, |wire_type, buf| {
                self.max_retries
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, 0)
            }),
            FIELD_TAG_IDS => merge_scanned_field(field, |wire_type, buf| {
                self.tag_ids
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, 0)
            }),
            FIELD_SCORES => merge_scanned_field(field, |wire_type, buf| {
                self.scores
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, 0)
            }),
            FIELD_STATUS => merge_scanned_field(field, |wire_type, buf| {
                self.status
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, 0)
            }),
            FIELD_PRIORITY => merge_scanned_field(field, |wire_type, buf| {
                self.priority
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, 0)
            }),
            FIELD_DONE => merge_scanned_field(field, |wire_type, buf| {
                self.done
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, 0)
            }),
            FIELD_FLAG => merge_scanned_field(field, |wire_type, buf| {
                self.flag
                    .bind_mut(&mut self._common)
                    .merge(wire_type, buf, 0)
            }),
            FIELD_VOTES => merge_scanned_field(field, |wire_type, buf| {
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
        let mut v = FieldDeallocVisitor::new(&self._common);
        let _ = self.visit_fields_mut(&mut v);
        self._common.deallocate();
    }
}
