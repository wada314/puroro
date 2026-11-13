//! Sample of the accessors the plugin emits for message `example.Task`.
//!
//! Slots live on [`TaskFields`]. [`Task`] / [`TaskLazy`] are newtypes over
//! [`puroro_rt::Record`]. Shell traits (`Clone`, `Message`, `LazyMessage`, …)
//! come from [`puroro_rt::forward_record`]. This file uses short imports; real
//! plugin output qualifies every path.

use ::allocator_api2::alloc::{Allocator, Global};
use ::allocator_api2::vec::Vec as AllocVec;
use ::core::marker::PhantomData;
use ::core::mem;
use ::core::ops::{ControlFlow, Deref, DerefMut};
use ::puroro::{
    DecodeBuf, DecodeError, HasDefault, MapMut, OneofView, OneofViewMut, Optional,
    RepeatedStringMut, WireType,
};
use ::puroro_rt::decode::{MessageScan, ScannedRecord, scanned_len_span};
use ::puroro_rt::{
    BitPacked, Boxed, CloneRecordFields, Closed, ConstructRecordFields, Eager, EagerRecordFields,
    Expanded, Explicit, FieldCloneIn, FieldDeallocate, FieldPairVisitor, FieldVisitor,
    FieldVisitorMut, HasField, Implicit, Inline, Lazy, LazyRecordApply, LegacyRequired, MapField,
    Message as MessagePresence, MessageCommon, OneofFieldMut, OneofFieldRef, OneofSlot, Open,
    Packed, ProtoBool, ProtoBytes, ProtoEnum, ProtoInt32, ProtoMessage, ProtoString, Record,
    RecordFields, RepeatedField, SingularField, UnknownFields,
};

use crate::Address;
use crate::AddressLazy;
use crate::Point;
use crate::enums::{Priority, Status};
use crate::task::defaults::MaxRetriesDefault;
use crate::task::notification::NotificationStorage;
use crate::task::{
    self, BIT_DONE_VALUE, BIT_FLAG, BIT_FLAG_VALUE, BIT_MAX_RETRIES, BIT_ORIGIN, BIT_OWNER_ID,
    BIT_PAYLOAD, BIT_PRIORITY, BIT_TITLE, FIELD_ASSIGNEE, FIELD_ATTRIBUTES, FIELD_DONE,
    FIELD_EMAIL_ADDRESS, FIELD_FLAG, FIELD_LABELS, FIELD_MAX_RETRIES, FIELD_ORIGIN, FIELD_OWNER_ID,
    FIELD_PAYLOAD, FIELD_PHONE_NUMBER, FIELD_POSTAL, FIELD_PRIORITY, FIELD_SCORE, FIELD_SCORES,
    FIELD_STATUS, FIELD_TAG_IDS, FIELD_TITLE, FIELD_URGENT, FIELD_VOTES, FIELD_WATCHERS,
    FIELD_WEBHOOK_ID, Notification, NotificationCase,
};

/// Field slots for `Task`. `L` is [`Eager`] or [`Lazy`].
pub(crate) struct TaskFields<A: Allocator + Clone = Global, L: task::Layout<A> = Eager> {
    title: SingularField<ProtoString, Explicit<{ BIT_TITLE }>, { FIELD_TITLE }, A, L::TitleLen>,
    score: SingularField<ProtoInt32, Implicit, { FIELD_SCORE }, A>,
    max_retries: SingularField<
        ProtoInt32,
        Explicit<{ BIT_MAX_RETRIES }>,
        { FIELD_MAX_RETRIES },
        A,
        Inline,
        MaxRetriesDefault,
    >,
    owner_id: SingularField<
        ProtoString,
        LegacyRequired<{ BIT_OWNER_ID }>,
        { FIELD_OWNER_ID },
        A,
        L::OwnerIdLen,
    >,
    payload:
        SingularField<ProtoBytes, Explicit<{ BIT_PAYLOAD }>, { FIELD_PAYLOAD }, A, L::PayloadLen>,
    tag_ids: RepeatedField<ProtoInt32, Packed, { FIELD_TAG_IDS }, A>,
    scores: RepeatedField<ProtoInt32, Expanded, { FIELD_SCORES }, A>,
    labels: RepeatedField<ProtoString, Expanded, { FIELD_LABELS }, A, L::Repeated>,
    status: SingularField<ProtoEnum<Status, Open>, Implicit, { FIELD_STATUS }, A>,
    priority: SingularField<
        ProtoEnum<Priority, Closed>,
        Explicit<{ BIT_PRIORITY }>,
        { FIELD_PRIORITY },
        A,
    >,
    /// Pinned lazy child (`(puroro.child_parse) = LAZY`), on both aliases.
    assignee:
        SingularField<ProtoMessage<AddressLazy<A>>, MessagePresence, { FIELD_ASSIGNEE }, A, Boxed>,
    /// Crate-visible so `task/lazy.rs` can read the group case. A oneof group
    /// is not one field number, so [`HasField`] does not cover this slot.
    pub(crate) notification: OneofSlot<NotificationStorage<A, L>>,
    done: SingularField<ProtoBool, Implicit, { FIELD_DONE }, A, BitPacked<{ BIT_DONE_VALUE }>>,
    flag: SingularField<
        ProtoBool,
        Explicit<{ BIT_FLAG }>,
        { FIELD_FLAG },
        A,
        BitPacked<{ BIT_FLAG_VALUE }>,
    >,
    watchers: RepeatedField<ProtoMessage<L::WatchersMsg>, Expanded, { FIELD_WATCHERS }, A>,
    votes: RepeatedField<ProtoBool, Packed, { FIELD_VOTES }, A>,
    attributes: MapField<ProtoString, ProtoInt32, { FIELD_ATTRIBUTES }, A, L::Map>,
    origin:
        SingularField<ProtoMessage<L::OriginMsg>, Explicit<{ BIT_ORIGIN }>, { FIELD_ORIGIN }, A>,
    _layout: PhantomData<(A, L)>,
}

macro_rules! has_task_field {
    ($n:expr, $slot:ident, $ty:ty) => {
        impl<A: Allocator + Clone, L: task::Layout<A>> HasField<{ $n }> for TaskFields<A, L> {
            type Ref<'a>
                = &'a $ty
            where
                Self: 'a;
            type Mut<'a>
                = &'a mut $ty
            where
                Self: 'a;

            fn field<'a>(&'a self) -> Self::Ref<'a> {
                &self.$slot
            }

            fn field_mut<'a>(&'a mut self) -> Self::Mut<'a> {
                &mut self.$slot
            }
        }
    };
}

has_task_field!(
    FIELD_TITLE,
    title,
    SingularField<ProtoString, Explicit<{ BIT_TITLE }>, { FIELD_TITLE }, A, L::TitleLen>
);
has_task_field!(FIELD_SCORE, score, SingularField<ProtoInt32, Implicit, { FIELD_SCORE }, A>);
has_task_field!(
    FIELD_MAX_RETRIES,
    max_retries,
    SingularField<
        ProtoInt32,
        Explicit<{ BIT_MAX_RETRIES }>,
        { FIELD_MAX_RETRIES },
        A,
        Inline,
        MaxRetriesDefault,
    >
);
has_task_field!(
    FIELD_OWNER_ID,
    owner_id,
    SingularField<ProtoString, LegacyRequired<{ BIT_OWNER_ID }>, { FIELD_OWNER_ID }, A, L::OwnerIdLen>
);
has_task_field!(
    FIELD_PAYLOAD,
    payload,
    SingularField<ProtoBytes, Explicit<{ BIT_PAYLOAD }>, { FIELD_PAYLOAD }, A, L::PayloadLen>
);
has_task_field!(
    FIELD_TAG_IDS,
    tag_ids,
    RepeatedField<ProtoInt32, Packed, { FIELD_TAG_IDS }, A>
);
has_task_field!(
    FIELD_SCORES,
    scores,
    RepeatedField<ProtoInt32, Expanded, { FIELD_SCORES }, A>
);
has_task_field!(
    FIELD_LABELS,
    labels,
    RepeatedField<ProtoString, Expanded, { FIELD_LABELS }, A, L::Repeated>
);
has_task_field!(
    FIELD_STATUS,
    status,
    SingularField<ProtoEnum<Status, Open>, Implicit, { FIELD_STATUS }, A>
);
has_task_field!(
    FIELD_PRIORITY,
    priority,
    SingularField<ProtoEnum<Priority, Closed>, Explicit<{ BIT_PRIORITY }>, { FIELD_PRIORITY }, A>
);
has_task_field!(
    FIELD_ASSIGNEE,
    assignee,
    SingularField<ProtoMessage<AddressLazy<A>>, MessagePresence, { FIELD_ASSIGNEE }, A, Boxed>
);
has_task_field!(
    FIELD_DONE,
    done,
    SingularField<ProtoBool, Implicit, { FIELD_DONE }, A, BitPacked<{ BIT_DONE_VALUE }>>
);
has_task_field!(
    FIELD_FLAG,
    flag,
    SingularField<ProtoBool, Explicit<{ BIT_FLAG }>, { FIELD_FLAG }, A, BitPacked<{ BIT_FLAG_VALUE }>>
);
has_task_field!(
    FIELD_WATCHERS,
    watchers,
    RepeatedField<ProtoMessage<L::WatchersMsg>, Expanded, { FIELD_WATCHERS }, A>
);
has_task_field!(FIELD_VOTES, votes, RepeatedField<ProtoBool, Packed, { FIELD_VOTES }, A>);
has_task_field!(
    FIELD_ATTRIBUTES,
    attributes,
    MapField<ProtoString, ProtoInt32, { FIELD_ATTRIBUTES }, A, L::Map>
);
has_task_field!(
    FIELD_ORIGIN,
    origin,
    SingularField<ProtoMessage<L::OriginMsg>, Explicit<{ BIT_ORIGIN }>, { FIELD_ORIGIN }, A>
);

macro_rules! has_task_oneof {
    ($n:expr) => {
        impl<A: Allocator + Clone, L: task::Layout<A>> HasField<{ $n }> for TaskFields<A, L> {
            type Ref<'a>
                = OneofFieldRef<'a, NotificationStorage<A, L>, { $n }>
            where
                Self: 'a;
            type Mut<'a>
                = OneofFieldMut<'a, NotificationStorage<A, L>, { $n }>
            where
                Self: 'a;

            fn field<'a>(&'a self) -> Self::Ref<'a> {
                OneofFieldRef::new(&self.notification)
            }

            fn field_mut<'a>(&'a mut self) -> Self::Mut<'a> {
                OneofFieldMut::new(&mut self.notification)
            }
        }
    };
}

has_task_oneof!(FIELD_EMAIL_ADDRESS);
has_task_oneof!(FIELD_PHONE_NUMBER);
has_task_oneof!(FIELD_WEBHOOK_ID);
has_task_oneof!(FIELD_POSTAL);
has_task_oneof!(FIELD_URGENT);

/// Eager `Task`.
pub struct Task<A: Allocator + Clone = Global> {
    pub(crate) record: Record<TaskFields<A, Eager>, A, Eager, UnknownFields<A>>,
}

/// Lazy `Task`.
pub struct TaskLazy<A: Allocator + Clone = Global> {
    pub(crate) record: Record<TaskFields<A, Lazy<A>>, A, Lazy<A>, UnknownFields<A>>,
}

::puroro_rt::forward_record!(eager Task);
::puroro_rt::forward_record!(lazy TaskLazy);

impl<A: Allocator + Clone, L: task::Layout<A> + MessageScan<A>> RecordFields<A, L>
    for TaskFields<A, L>
{
    type Bits = <L as task::Layout<A>>::Bits;
    type Unknown = UnknownFields<A>;
    type EagerMessage = Task<A>;

    const EAGER_NAME: &'static str = "Task";
    const LAZY_NAME: &'static str = "TaskLazy";
    const CHECKS_REQUIRED: bool = true;

    fn empty_bits() -> Self::Bits {
        <L as task::Layout<A>>::empty_bits()
    }

    fn visit_mut<V: FieldVisitorMut<MessageCommon<Self::Bits, A, Self::Unknown, L>>>(
        &mut self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("title", &mut self.title)?;
        v.visit("score", &mut self.score)?;
        v.visit("max_retries", &mut self.max_retries)?;
        v.visit("owner_id", &mut self.owner_id)?;
        v.visit("payload", &mut self.payload)?;
        v.visit("tag_ids", &mut self.tag_ids)?;
        v.visit("scores", &mut self.scores)?;
        v.visit("labels", &mut self.labels)?;
        v.visit("status", &mut self.status)?;
        v.visit("priority", &mut self.priority)?;
        v.visit("assignee", &mut self.assignee)?;
        v.visit("notification", &mut self.notification)?;
        v.visit("done", &mut self.done)?;
        v.visit("flag", &mut self.flag)?;
        v.visit("watchers", &mut self.watchers)?;
        v.visit("votes", &mut self.votes)?;
        v.visit("attributes", &mut self.attributes)?;
        v.visit("origin", &mut self.origin)?;
        ControlFlow::Continue(())
    }

    fn validate(
        &self,
        common: &MessageCommon<Self::Bits, A, Self::Unknown, L>,
    ) -> Result<(), DecodeError> {
        <Self as HasField<{ FIELD_OWNER_ID }>>::field(self).validate_required(common)?;
        Ok(())
    }
}

macro_rules! clone_task_slot {
    ($self:ident, $dst:ident, $src:ident, $dst_common:ident, $alloc:ident, $name:ident) => {{
        let new = FieldCloneIn::clone_field(&$self.$name, $src, $alloc.clone());
        let mut old = mem::replace(&mut $dst.$name, new);
        FieldDeallocate::deallocate(&mut old, $dst_common);
    }};
}

macro_rules! impl_task_build {
    ($layout:ty) => {
        impl<A: Allocator + Clone> ConstructRecordFields<A, $layout> for TaskFields<A, $layout> {
            fn new(alloc: A) -> Self {
                Self {
                    title: SingularField::new_in(alloc.clone()),
                    score: SingularField::new_in(alloc.clone()),
                    max_retries: SingularField::new_in(alloc.clone()),
                    owner_id: SingularField::new_in(alloc.clone()),
                    payload: SingularField::new_in(alloc.clone()),
                    tag_ids: RepeatedField::new_in(alloc.clone()),
                    scores: RepeatedField::new_in(alloc.clone()),
                    labels: RepeatedField::new_in(alloc.clone()),
                    status: SingularField::new_in(alloc.clone()),
                    priority: SingularField::new_in(alloc.clone()),
                    assignee: SingularField::new_in(alloc.clone()),
                    notification: OneofSlot::new_in(alloc.clone()),
                    done: SingularField::new_in(alloc.clone()),
                    flag: SingularField::new_in(alloc.clone()),
                    watchers: RepeatedField::new_in(alloc.clone()),
                    votes: RepeatedField::new_in(alloc.clone()),
                    attributes: MapField::new_in(alloc.clone()),
                    origin: SingularField::new_in(alloc),
                    _layout: PhantomData,
                }
            }
        }

        impl<A: Allocator + Clone> CloneRecordFields<A, $layout> for TaskFields<A, $layout> {
            fn clone_fields(
                &self,
                dst: &mut Self,
                src_common: &MessageCommon<Self::Bits, A, Self::Unknown, $layout>,
                dst_common: &MessageCommon<Self::Bits, A, Self::Unknown, $layout>,
                alloc: A,
            ) {
                clone_task_slot!(self, dst, src_common, dst_common, alloc, title);
                clone_task_slot!(self, dst, src_common, dst_common, alloc, score);
                clone_task_slot!(self, dst, src_common, dst_common, alloc, max_retries);
                clone_task_slot!(self, dst, src_common, dst_common, alloc, owner_id);
                clone_task_slot!(self, dst, src_common, dst_common, alloc, payload);
                clone_task_slot!(self, dst, src_common, dst_common, alloc, tag_ids);
                clone_task_slot!(self, dst, src_common, dst_common, alloc, scores);
                clone_task_slot!(self, dst, src_common, dst_common, alloc, labels);
                clone_task_slot!(self, dst, src_common, dst_common, alloc, status);
                clone_task_slot!(self, dst, src_common, dst_common, alloc, priority);
                clone_task_slot!(self, dst, src_common, dst_common, alloc, assignee);
                clone_task_slot!(self, dst, src_common, dst_common, alloc, notification);
                clone_task_slot!(self, dst, src_common, dst_common, alloc, done);
                clone_task_slot!(self, dst, src_common, dst_common, alloc, flag);
                clone_task_slot!(self, dst, src_common, dst_common, alloc, watchers);
                clone_task_slot!(self, dst, src_common, dst_common, alloc, votes);
                clone_task_slot!(self, dst, src_common, dst_common, alloc, attributes);
                clone_task_slot!(self, dst, src_common, dst_common, alloc, origin);
            }
        }
    };
}

impl_task_build!(Eager);
impl_task_build!(Lazy<A>);

macro_rules! merge_known_arm {
    ($self:ident, $common:ident, $wire_type:ident, $buf:ident, $depth:ident, $n:expr) => {{
        <Self as HasField<{ $n }>>::field_mut($self)
            .bind_mut($common)
            .merge($wire_type, $buf, $depth)?;
        Ok(true)
    }};
}

impl<A: Allocator + Clone> EagerRecordFields<A> for TaskFields<A, Eager> {
    fn visit_fields<V: FieldVisitor<MessageCommon<Self::Bits, A, Self::Unknown, Eager>>>(
        &self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("title", &self.title)?;
        v.visit("score", &self.score)?;
        v.visit("max_retries", &self.max_retries)?;
        v.visit("owner_id", &self.owner_id)?;
        v.visit("payload", &self.payload)?;
        v.visit("tag_ids", &self.tag_ids)?;
        v.visit("scores", &self.scores)?;
        v.visit("labels", &self.labels)?;
        v.visit("status", &self.status)?;
        v.visit("priority", &self.priority)?;
        v.visit("assignee", &self.assignee)?;
        v.visit("notification", &self.notification)?;
        v.visit("done", &self.done)?;
        v.visit("flag", &self.flag)?;
        v.visit("watchers", &self.watchers)?;
        v.visit("votes", &self.votes)?;
        v.visit("attributes", &self.attributes)?;
        v.visit("origin", &self.origin)?;
        ControlFlow::Continue(())
    }

    fn visit_pairs<V: FieldPairVisitor<MessageCommon<Self::Bits, A, Self::Unknown, Eager>>>(
        &self,
        other: &Self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("title", &self.title, &other.title)?;
        v.visit("score", &self.score, &other.score)?;
        v.visit("max_retries", &self.max_retries, &other.max_retries)?;
        v.visit("owner_id", &self.owner_id, &other.owner_id)?;
        v.visit("payload", &self.payload, &other.payload)?;
        v.visit("tag_ids", &self.tag_ids, &other.tag_ids)?;
        v.visit("scores", &self.scores, &other.scores)?;
        v.visit("labels", &self.labels, &other.labels)?;
        v.visit("status", &self.status, &other.status)?;
        v.visit("priority", &self.priority, &other.priority)?;
        v.visit("assignee", &self.assignee, &other.assignee)?;
        v.visit("notification", &self.notification, &other.notification)?;
        v.visit("done", &self.done, &other.done)?;
        v.visit("flag", &self.flag, &other.flag)?;
        v.visit("watchers", &self.watchers, &other.watchers)?;
        v.visit("votes", &self.votes, &other.votes)?;
        v.visit("attributes", &self.attributes, &other.attributes)?;
        v.visit("origin", &self.origin, &other.origin)?;
        ControlFlow::Continue(())
    }

    fn merge_known<B: DecodeBuf>(
        &mut self,
        common: &mut MessageCommon<Self::Bits, A, Self::Unknown, Eager>,
        field_number: u32,
        wire_type: WireType,
        buf: &mut B,
        depth: usize,
    ) -> Result<bool, DecodeError> {
        match field_number {
            FIELD_TITLE => merge_known_arm!(self, common, wire_type, buf, depth, FIELD_TITLE),
            FIELD_SCORE => merge_known_arm!(self, common, wire_type, buf, depth, FIELD_SCORE),
            FIELD_MAX_RETRIES => {
                merge_known_arm!(self, common, wire_type, buf, depth, FIELD_MAX_RETRIES)
            }
            FIELD_OWNER_ID => merge_known_arm!(self, common, wire_type, buf, depth, FIELD_OWNER_ID),
            FIELD_PAYLOAD => merge_known_arm!(self, common, wire_type, buf, depth, FIELD_PAYLOAD),
            FIELD_TAG_IDS => merge_known_arm!(self, common, wire_type, buf, depth, FIELD_TAG_IDS),
            FIELD_SCORES => merge_known_arm!(self, common, wire_type, buf, depth, FIELD_SCORES),
            FIELD_LABELS => merge_known_arm!(self, common, wire_type, buf, depth, FIELD_LABELS),
            FIELD_STATUS => merge_known_arm!(self, common, wire_type, buf, depth, FIELD_STATUS),
            FIELD_PRIORITY => merge_known_arm!(self, common, wire_type, buf, depth, FIELD_PRIORITY),
            FIELD_ASSIGNEE => merge_known_arm!(self, common, wire_type, buf, depth, FIELD_ASSIGNEE),
            FIELD_EMAIL_ADDRESS => {
                merge_known_arm!(self, common, wire_type, buf, depth, FIELD_EMAIL_ADDRESS)
            }
            FIELD_PHONE_NUMBER => {
                merge_known_arm!(self, common, wire_type, buf, depth, FIELD_PHONE_NUMBER)
            }
            FIELD_WEBHOOK_ID => {
                merge_known_arm!(self, common, wire_type, buf, depth, FIELD_WEBHOOK_ID)
            }
            FIELD_POSTAL => merge_known_arm!(self, common, wire_type, buf, depth, FIELD_POSTAL),
            FIELD_DONE => merge_known_arm!(self, common, wire_type, buf, depth, FIELD_DONE),
            FIELD_FLAG => merge_known_arm!(self, common, wire_type, buf, depth, FIELD_FLAG),
            FIELD_URGENT => merge_known_arm!(self, common, wire_type, buf, depth, FIELD_URGENT),
            FIELD_WATCHERS => merge_known_arm!(self, common, wire_type, buf, depth, FIELD_WATCHERS),
            FIELD_VOTES => merge_known_arm!(self, common, wire_type, buf, depth, FIELD_VOTES),
            FIELD_ATTRIBUTES => {
                merge_known_arm!(self, common, wire_type, buf, depth, FIELD_ATTRIBUTES)
            }
            FIELD_ORIGIN => merge_known_arm!(self, common, wire_type, buf, depth, FIELD_ORIGIN),
            _ => Ok(false),
        }
    }
}

impl<A: Allocator + Clone> LazyRecordApply<A> for TaskFields<A, Lazy<A>> {
    fn apply_record(
        &mut self,
        common: &mut MessageCommon<Self::Bits, A, Self::Unknown, Lazy<A>>,
        rec: &ScannedRecord,
        origin: usize,
    ) -> Result<(), DecodeError> {
        match rec.field_number.as_u32() {
            FIELD_TITLE => {
                let span = scanned_len_span(origin, rec)?;
                <Self as HasField<{ FIELD_TITLE }>>::field_mut(self).store_len_span(span, common);
                Ok(())
            }
            FIELD_OWNER_ID => {
                let span = scanned_len_span(origin, rec)?;
                <Self as HasField<{ FIELD_OWNER_ID }>>::field_mut(self)
                    .store_len_span(span, common);
                Ok(())
            }
            FIELD_PAYLOAD => {
                let span = scanned_len_span(origin, rec)?;
                <Self as HasField<{ FIELD_PAYLOAD }>>::field_mut(self).store_len_span(span, common);
                Ok(())
            }
            FIELD_ASSIGNEE => {
                let span = scanned_len_span(origin, rec)?;
                let root = common.lazy.scan.shared().clone_handle();
                <Self as HasField<{ FIELD_ASSIGNEE }>>::field_mut(self)
                    .merge_shared(&root, span, common)
            }
            FIELD_ORIGIN => {
                let span = scanned_len_span(origin, rec)?;
                let root = common.lazy.scan.shared().clone_handle();
                <Self as HasField<{ FIELD_ORIGIN }>>::field_mut(self)
                    .merge_shared(&root, span, common)
            }
            FIELD_WATCHERS => {
                let span = scanned_len_span(origin, rec)?;
                let root = common.lazy.scan.shared().clone_handle();
                <Self as HasField<{ FIELD_WATCHERS }>>::field_mut(self)
                    .merge_shared(&root, span, common)
            }
            FIELD_EMAIL_ADDRESS => {
                let span = scanned_len_span(origin, rec)?;
                <Self as HasField<{ FIELD_EMAIL_ADDRESS }>>::field_mut(self)
                    .variant(common)
                    .store_len_span(span, common);
                Ok(())
            }
            FIELD_PHONE_NUMBER => {
                let span = scanned_len_span(origin, rec)?;
                <Self as HasField<{ FIELD_PHONE_NUMBER }>>::field_mut(self)
                    .variant(common)
                    .store_len_span(span, common);
                Ok(())
            }
            FIELD_WEBHOOK_ID => <Self as HasField<{ FIELD_WEBHOOK_ID }>>::field_mut(self)
                .variant(common)
                .bind_mut(common)
                .merge_scanned(&rec.value),
            FIELD_POSTAL => {
                let span = scanned_len_span(origin, rec)?;
                let root = common.lazy.scan.shared().clone_handle();
                <Self as HasField<{ FIELD_POSTAL }>>::field_mut(self)
                    .variant(common)
                    .merge_shared(&root, span, common)
            }
            FIELD_URGENT => <Self as HasField<{ FIELD_URGENT }>>::field_mut(self)
                .variant(common)
                .bind_mut(common)
                .merge_scanned(&rec.value),
            FIELD_LABELS => {
                let span = scanned_len_span(origin, rec)?;
                <Self as HasField<{ FIELD_LABELS }>>::field_mut(self).store_span(
                    span,
                    common.lazy.scan.wire(),
                    common,
                )
            }
            FIELD_ATTRIBUTES => {
                let span = scanned_len_span(origin, rec)?;
                <Self as HasField<{ FIELD_ATTRIBUTES }>>::field_mut(self).store_span(
                    span,
                    common.lazy.scan.wire(),
                    common,
                )
            }
            FIELD_SCORE => <Self as HasField<{ FIELD_SCORE }>>::field_mut(self)
                .bind_mut(common)
                .merge_scanned(&rec.value),
            FIELD_MAX_RETRIES => <Self as HasField<{ FIELD_MAX_RETRIES }>>::field_mut(self)
                .bind_mut(common)
                .merge_scanned(&rec.value),
            FIELD_TAG_IDS => {
                let wire = common.lazy.scan.shared().clone_handle();
                <Self as HasField<{ FIELD_TAG_IDS }>>::field_mut(self)
                    .bind_mut(common)
                    .merge_scanned(rec, wire.as_bytes(), origin)
            }
            FIELD_SCORES => {
                let wire = common.lazy.scan.shared().clone_handle();
                <Self as HasField<{ FIELD_SCORES }>>::field_mut(self)
                    .bind_mut(common)
                    .merge_scanned(rec, wire.as_bytes(), origin)
            }
            FIELD_STATUS => <Self as HasField<{ FIELD_STATUS }>>::field_mut(self)
                .bind_mut(common)
                .merge_scanned(&rec.value),
            FIELD_PRIORITY => <Self as HasField<{ FIELD_PRIORITY }>>::field_mut(self)
                .bind_mut(common)
                .merge_scanned(&rec.value),
            FIELD_DONE => <Self as HasField<{ FIELD_DONE }>>::field_mut(self)
                .bind_mut(common)
                .merge_scanned(&rec.value),
            FIELD_FLAG => <Self as HasField<{ FIELD_FLAG }>>::field_mut(self)
                .bind_mut(common)
                .merge_scanned(&rec.value),
            FIELD_VOTES => {
                let wire = common.lazy.scan.shared().clone_handle();
                <Self as HasField<{ FIELD_VOTES }>>::field_mut(self)
                    .bind_mut(common)
                    .merge_scanned(rec, wire.as_bytes(), origin)
            }
            _ => common.on_unrecognized_tag(rec.field_number.as_u32()),
        }
    }
}

impl<A: Allocator + Clone> Task<A> {
    pub fn title<'a>(&'a self) -> Optional<&'a str, impl HasDefault<&'a str>>
    where
        A: 'a,
    {
        self.record.field::<{ FIELD_TITLE }>().optional()
    }

    pub fn score(&self) -> i32 {
        self.record.field::<{ FIELD_SCORE }>().value()
    }

    pub fn max_retries<'a>(&'a self) -> Optional<i32, impl HasDefault<i32>>
    where
        A: 'a,
    {
        self.record.field::<{ FIELD_MAX_RETRIES }>().optional()
    }

    pub fn owner_id<'a>(&'a self) -> Optional<&'a str, impl HasDefault<&'a str>>
    where
        A: 'a,
    {
        self.record.field::<{ FIELD_OWNER_ID }>().optional()
    }

    pub fn payload<'a>(&'a self) -> Optional<&'a [u8], impl HasDefault<&'a [u8]>>
    where
        A: 'a,
    {
        self.record.field::<{ FIELD_PAYLOAD }>().optional()
    }

    pub fn tag_ids(&self) -> &[i32] {
        self.record.field::<{ FIELD_TAG_IDS }>().as_slice()
    }

    pub fn scores(&self) -> &[i32] {
        self.record.field::<{ FIELD_SCORES }>().as_slice()
    }

    pub fn labels(&self) -> &[impl Deref<Target = str>] {
        self.record.field::<{ FIELD_LABELS }>().as_slice()
    }

    pub fn status<'a>(&'a self) -> Optional<Status, impl HasDefault<Status>>
    where
        A: 'a,
    {
        self.record.field::<{ FIELD_STATUS }>().optional()
    }

    pub fn priority<'a>(&'a self) -> Optional<Priority, impl HasDefault<Priority>>
    where
        A: 'a,
    {
        self.record.field::<{ FIELD_PRIORITY }>().optional()
    }

    pub fn assignee(&self) -> Option<&AddressLazy<A>> {
        self.record.field::<{ FIELD_ASSIGNEE }>().get()
    }

    pub fn done(&self) -> bool {
        self.record.field::<{ FIELD_DONE }>().value()
    }

    pub fn flag<'a>(&'a self) -> Optional<bool, impl HasDefault<bool>>
    where
        A: 'a,
    {
        self.record.field::<{ FIELD_FLAG }>().optional()
    }

    pub fn watchers(&self) -> &[Address<A>] {
        self.record.field::<{ FIELD_WATCHERS }>().as_slice()
    }

    pub fn votes(&self) -> &[bool] {
        self.record.field::<{ FIELD_VOTES }>().as_slice()
    }

    pub fn attributes(&self) -> impl ::puroro::MapRef<str, i32> + '_ {
        self.record.field::<{ FIELD_ATTRIBUTES }>()
    }

    pub fn origin(&self) -> Option<&Point<A>> {
        self.record.field::<{ FIELD_ORIGIN }>().get()
    }

    /// Bound shared view of the oneof group (always available, including when unset).
    pub fn notification<'a>(
        &'a self,
    ) -> impl OneofView<
        Case = NotificationCase,
        Ref = Notification<&'a str, &'a str, i32, &'a Address<A>, bool>,
    > + 'a {
        ::puroro_rt::OneofView::<NotificationStorage<A, Eager>>::new(
            &self.record.fields.notification,
            &self.record.common,
        )
    }

    pub fn email_address<'a>(&'a self) -> Optional<&'a str, impl HasDefault<&'a str>>
    where
        A: 'a,
    {
        self.record.field::<{ FIELD_EMAIL_ADDRESS }>().optional()
    }

    pub fn phone_number<'a>(&'a self) -> Optional<&'a str, impl HasDefault<&'a str>>
    where
        A: 'a,
    {
        self.record.field::<{ FIELD_PHONE_NUMBER }>().optional()
    }

    pub fn webhook_id<'a>(&'a self) -> Optional<i32, impl HasDefault<i32>>
    where
        A: 'a,
    {
        self.record.field::<{ FIELD_WEBHOOK_ID }>().optional()
    }

    pub fn postal(&self) -> Option<&Address<A>> {
        self.record.field::<{ FIELD_POSTAL }>().get()
    }

    pub fn urgent<'a>(&'a self) -> Optional<bool, impl HasDefault<bool>>
    where
        A: 'a,
    {
        self.record.field::<{ FIELD_URGENT }>().optional()
    }

    pub fn title_mut(&mut self) -> impl ::puroro::StringMut<A> + '_ {
        self.record.field_mut::<{ FIELD_TITLE }>().value_mut()
    }

    pub fn clear_title(&mut self) {
        self.record.field_mut::<{ FIELD_TITLE }>().clear();
    }

    pub fn score_mut(&mut self) -> impl DerefMut<Target = i32> + '_ {
        self.record.field_mut::<{ FIELD_SCORE }>().value_mut()
    }

    pub fn clear_score(&mut self) {
        self.record.field_mut::<{ FIELD_SCORE }>().clear();
    }

    pub fn max_retries_mut(&mut self) -> impl DerefMut<Target = i32> + '_ {
        self.record.field_mut::<{ FIELD_MAX_RETRIES }>().value_mut()
    }

    pub fn clear_max_retries(&mut self) {
        self.record.field_mut::<{ FIELD_MAX_RETRIES }>().clear();
    }

    pub fn owner_id_mut(&mut self) -> impl ::puroro::StringMut<A> + '_ {
        self.record.field_mut::<{ FIELD_OWNER_ID }>().value_mut()
    }

    pub fn clear_owner_id(&mut self) {
        self.record.field_mut::<{ FIELD_OWNER_ID }>().clear();
    }

    pub fn payload_mut(&mut self) -> impl ::puroro::BytesMut<A> + '_ {
        self.record.field_mut::<{ FIELD_PAYLOAD }>().value_mut()
    }

    pub fn clear_payload(&mut self) {
        self.record.field_mut::<{ FIELD_PAYLOAD }>().clear();
    }

    pub fn tag_ids_mut<'s>(&'s mut self) -> impl DerefMut<Target = AllocVec<i32, A>> + 's {
        self.record.field_mut::<{ FIELD_TAG_IDS }>().values_mut()
    }

    pub fn clear_tag_ids(&mut self) {
        self.record.field_mut::<{ FIELD_TAG_IDS }>().clear();
    }

    pub fn scores_mut<'s>(&'s mut self) -> impl DerefMut<Target = AllocVec<i32, A>> + 's {
        self.record.field_mut::<{ FIELD_SCORES }>().values_mut()
    }

    pub fn clear_scores(&mut self) {
        self.record.field_mut::<{ FIELD_SCORES }>().clear();
    }

    /// Container mutator: `push()` appends an empty string.
    pub fn labels_mut(&mut self) -> impl RepeatedStringMut<A> + '_ {
        self.record.field_mut::<{ FIELD_LABELS }>().container_mut()
    }

    pub fn clear_labels(&mut self) {
        self.record.field_mut::<{ FIELD_LABELS }>().clear();
    }

    pub fn status_mut(&mut self) -> impl DerefMut<Target = Status> + '_ {
        self.record.field_mut::<{ FIELD_STATUS }>().value_mut()
    }

    pub fn clear_status(&mut self) {
        self.record.field_mut::<{ FIELD_STATUS }>().clear();
    }

    pub fn priority_mut(&mut self) -> impl DerefMut<Target = Priority> + '_ {
        self.record.field_mut::<{ FIELD_PRIORITY }>().value_mut()
    }

    pub fn clear_priority(&mut self) {
        self.record.field_mut::<{ FIELD_PRIORITY }>().clear();
    }

    pub fn assignee_mut(&mut self) -> &mut AddressLazy<A> {
        self.record.field_mut::<{ FIELD_ASSIGNEE }>().get_mut()
    }

    pub fn set_assignee(&mut self, src: Address<A>) {
        *self.assignee_mut() = src.into_lazy();
    }

    pub fn clear_assignee(&mut self) {
        self.record.field_mut::<{ FIELD_ASSIGNEE }>().clear();
    }

    pub fn done_mut(&mut self) -> impl DerefMut<Target = bool> + '_ {
        self.record.field_mut::<{ FIELD_DONE }>().value_mut()
    }

    pub fn clear_done(&mut self) {
        self.record.field_mut::<{ FIELD_DONE }>().clear();
    }

    pub fn flag_mut(&mut self) -> impl DerefMut<Target = bool> + '_ {
        self.record.field_mut::<{ FIELD_FLAG }>().value_mut()
    }

    pub fn clear_flag(&mut self) {
        self.record.field_mut::<{ FIELD_FLAG }>().clear();
    }

    pub fn watchers_mut(&mut self) -> impl DerefMut<Target = AllocVec<Address<A>, A>> + '_ {
        self.record.field_mut::<{ FIELD_WATCHERS }>().values_mut()
    }

    pub fn clear_watchers(&mut self) {
        self.record.field_mut::<{ FIELD_WATCHERS }>().clear();
    }

    pub fn votes_mut<'s>(&'s mut self) -> impl DerefMut<Target = AllocVec<bool, A>> + 's {
        self.record.field_mut::<{ FIELD_VOTES }>().values_mut()
    }

    pub fn clear_votes(&mut self) {
        self.record.field_mut::<{ FIELD_VOTES }>().clear();
    }

    pub fn attributes_mut(&mut self) -> impl MapMut<str, i32, MutTarget = i32> + '_ {
        self.record.field_mut::<{ FIELD_ATTRIBUTES }>()
    }

    pub fn clear_attributes(&mut self) {
        self.record.field_mut::<{ FIELD_ATTRIBUTES }>().clear();
    }

    pub fn origin_mut(&mut self) -> &mut Point<A> {
        self.record.field_mut::<{ FIELD_ORIGIN }>().get_mut()
    }

    pub fn set_origin(&mut self, src: Point<A>) {
        *self.origin_mut() = src;
    }

    pub fn clear_origin(&mut self) {
        self.record.field_mut::<{ FIELD_ORIGIN }>().clear();
    }

    /// Bound mutable view of the oneof group (always available, including when unset).
    pub fn notification_mut<'a>(&'a mut self) -> impl OneofViewMut<Case = NotificationCase> + 'a {
        ::puroro_rt::OneofViewMut::<NotificationStorage<A, Eager>>::new(
            &mut self.record.fields.notification,
            &mut self.record.common,
        )
    }

    pub fn clear_notification(&mut self) {
        self.notification_mut().clear();
    }

    pub fn email_address_mut(&mut self) -> impl ::puroro::StringMut<A> + '_ {
        self.record
            .field_mut::<{ FIELD_EMAIL_ADDRESS }>()
            .value_mut()
    }

    pub fn phone_number_mut(&mut self) -> impl ::puroro::StringMut<A> + '_ {
        self.record
            .field_mut::<{ FIELD_PHONE_NUMBER }>()
            .value_mut()
    }

    /// `[default = -1]`: when this variant is not active, `get()` returns `-1`.
    pub fn webhook_id_mut(&mut self) -> impl DerefMut<Target = i32> + '_ {
        self.record.field_mut::<{ FIELD_WEBHOOK_ID }>().value_mut()
    }

    /// Switches the group to `postal` and returns the nested message.
    pub fn postal_mut(&mut self) -> &mut Address<A> {
        self.record.field_mut::<{ FIELD_POSTAL }>().value_mut()
    }

    pub fn urgent_mut(&mut self) -> impl DerefMut<Target = bool> + '_ {
        self.record.field_mut::<{ FIELD_URGENT }>().value_mut()
    }

    /// Encodes `self` and rebuilds it as the lazy alias.
    pub fn into_lazy(self) -> TaskLazy<A> {
        TaskLazy {
            record: Record::encode_lazy(self.record),
        }
    }
}
