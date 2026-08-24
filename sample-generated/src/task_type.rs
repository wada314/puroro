//! Sample of the accessors, encode/decode glue, and presence handling that
//! puroro generates for message `example.Task` (from `example.proto`).
//!
//! The public type is re-exported at the crate root so it sits beside companion
//! module [`crate::task`]. Written for human readers: unlike real plugin output
//! it uses short imported names instead of fully-qualified paths (see the crate
//! root docs).

use ::allocator_api2::alloc::{Allocator, Global};
use ::allocator_api2::vec::Vec as AllocVec;
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;
use ::bytes::{Buf, BufMut};
use ::core::fmt;
use ::core::mem;
use ::core::ops::ControlFlow;
use ::core::ops::{Deref, DerefMut};

use ::puroro::{
    DecodeBuf, DecodeError, HasDefault, MapMut, MapRef, Message, OneofView, OneofViewMut, Optional,
    RepeatedStringMut,
};
use ::puroro_rt::decode::{decode_tag, skip_field_and_save};
use ::puroro_rt::{
    BitPacked, CloneFieldsVisitor, CloneIn, Closed, DebugStructVisitor, EncodeCtx,
    EncodeRawVisitor, EncodedLenVisitor, Expanded, Explicit, FieldDeallocVisitor, FieldEqVisitor,
    FieldPairVisitor, FieldPairVisitorMut, FieldVisitor, FieldVisitorMut, Implicit, Inline,
    InlineOrHeap, LegacyRequired, MapField, Message as MessagePresence, MessageCommon,
    MessageEncode, MessageMerge, OneofSlot, Open, Packed, ProtoBool, ProtoBytes, ProtoEnum,
    ProtoInt32, ProtoMessage, ProtoString, RepeatedField, SingularField,
};

use crate::Address;
use crate::enums::{Priority, Status};
use crate::task::defaults::MaxRetriesDefault;
use crate::task::notification::NotificationStorage;
use crate::task::{
    BIT_DONE_VALUE, BIT_FLAG, BIT_FLAG_VALUE, BIT_MAX_RETRIES, BIT_OWNER_ID, BIT_OWNER_ID_SSO,
    BIT_PAYLOAD, BIT_PAYLOAD_SSO, BIT_PRIORITY, BIT_TITLE, BIT_TITLE_SSO, FIELD_ASSIGNEE,
    FIELD_ATTRIBUTES, FIELD_DONE, FIELD_EMAIL_ADDRESS, FIELD_FLAG, FIELD_LABELS, FIELD_MAX_RETRIES,
    FIELD_OWNER_ID, FIELD_PAYLOAD, FIELD_PHONE_NUMBER, FIELD_POSTAL, FIELD_PRIORITY, FIELD_SCORE,
    FIELD_SCORES, FIELD_STATUS, FIELD_TAG_IDS, FIELD_TITLE, FIELD_URGENT, FIELD_VOTES,
    FIELD_WATCHERS, FIELD_WEBHOOK_ID, Notification, NotificationCase,
};

// ---------------------------------------------------------------------------
// Message struct
// ---------------------------------------------------------------------------

/// Reference `Task` message from `DESIGN.md`.
pub struct Task<A: Allocator + Clone = Global> {
    _common: MessageCommon<BitArray<[u8; 2], Lsb0>, A>,
    title: SingularField<
        ProtoString,
        Explicit<{ BIT_TITLE }>,
        { FIELD_TITLE },
        A,
        InlineOrHeap<{ BIT_TITLE_SSO }>,
    >, // proto: string title = 1;
    score: SingularField<ProtoInt32, Implicit, { FIELD_SCORE }, A>, // proto: int32 score = 2;
    max_retries: SingularField<
        ProtoInt32,
        Explicit<{ BIT_MAX_RETRIES }>,
        { FIELD_MAX_RETRIES },
        A,
        Inline,
        MaxRetriesDefault,
    >, // proto: int32 max_retries = 3;
    owner_id: SingularField<
        ProtoString,
        LegacyRequired<{ BIT_OWNER_ID }>,
        { FIELD_OWNER_ID },
        A,
        InlineOrHeap<{ BIT_OWNER_ID_SSO }>,
    >, // proto: string owner_id = 4;
    payload: SingularField<
        ProtoBytes,
        Explicit<{ BIT_PAYLOAD }>,
        { FIELD_PAYLOAD },
        A,
        InlineOrHeap<{ BIT_PAYLOAD_SSO }>,
    >, // proto: bytes payload = 5;
    tag_ids: RepeatedField<ProtoInt32, Packed, { FIELD_TAG_IDS }, A>, // proto: repeated int32 tag_ids = 6 [packed];
    scores: RepeatedField<ProtoInt32, Expanded, { FIELD_SCORES }, A>, // proto: repeated int32 scores = 7;
    labels: RepeatedField<ProtoString, Expanded, { FIELD_LABELS }, A>, // proto: repeated string labels = 8;
    status: SingularField<ProtoEnum<Status, Open>, Implicit, { FIELD_STATUS }, A>, // proto: Status status = 9;
    priority: SingularField<
        ProtoEnum<Priority, Closed>,
        Explicit<{ BIT_PRIORITY }>,
        { FIELD_PRIORITY },
        A,
    >, // proto: Priority priority = 10;
    assignee: SingularField<ProtoMessage<Address<A>>, MessagePresence, { FIELD_ASSIGNEE }, A>, // proto: Address assignee = 11;
    // proto: oneof notification { string email_address=12; string phone_number=13;
    //                             int32 webhook_id=14 [default=-1]; Address postal=15;
    //                             bool urgent=18; }
    notification: OneofSlot<NotificationStorage<A>>,
    done: SingularField<ProtoBool, Implicit, { FIELD_DONE }, A, BitPacked<{ BIT_DONE_VALUE }>>, // proto: bool done = 16;
    flag: SingularField<
        ProtoBool,
        Explicit<{ BIT_FLAG }>,
        { FIELD_FLAG },
        A,
        BitPacked<{ BIT_FLAG_VALUE }>,
    >, // proto: bool flag = 17;
    watchers: RepeatedField<ProtoMessage<Address<A>>, Expanded, { FIELD_WATCHERS }, A>, // proto: repeated Address watchers = 19;
    votes: RepeatedField<ProtoBool, Packed, { FIELD_VOTES }, A>, // proto: repeated bool votes = 20;
    attributes: MapField<ProtoString, ProtoInt32, { FIELD_ATTRIBUTES }, A>, // proto: map<string, int32> attributes = 21;
}

impl<A: Allocator + Clone> Task<A> {
    pub fn new_in(alloc: A) -> Self {
        // Each field initializer gets its own clone of the allocator; the last
        // heap field (`attributes`) takes the original by move.
        Self {
            _common: MessageCommon::new_in(BitArray::ZERO, alloc.clone()),
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
            attributes: MapField::new_in(alloc),
        }
    }

    // -- title (EXPLICIT string, proto field 1) ----------------------------

    pub fn title<'a>(&'a self) -> Optional<&'a str, impl HasDefault<&'a str>>
    where
        A: 'a,
    {
        self.title.bind(&self._common).optional()
    }

    pub fn title_mut(&mut self) -> impl ::puroro::StringMut<A> + '_ {
        self.title.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_title(&mut self) {
        self.title.bind_mut(&mut self._common).clear();
    }

    // -- score (IMPLICIT int32, proto field 2) ------------------------------

    pub fn score(&self) -> i32 {
        self.score.bind(&self._common).value()
    }

    pub fn score_mut(&mut self) -> impl DerefMut<Target = i32> + '_ {
        self.score.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_score(&mut self) {
        self.score.bind_mut(&mut self._common).clear();
    }

    // -- max_retries (EXPLICIT int32, default = 3, proto field 3) ------------

    pub fn max_retries<'a>(&'a self) -> Optional<i32, impl HasDefault<i32>>
    where
        A: 'a,
    {
        self.max_retries.bind(&self._common).optional()
    }

    pub fn max_retries_mut(&mut self) -> impl DerefMut<Target = i32> + '_ {
        self.max_retries.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_max_retries(&mut self) {
        self.max_retries.bind_mut(&mut self._common).clear();
    }

    // -- owner_id (LEGACY_REQUIRED string, proto field 4) --------------------

    pub fn owner_id<'a>(&'a self) -> Optional<&'a str, impl HasDefault<&'a str>>
    where
        A: 'a,
    {
        self.owner_id.bind(&self._common).optional()
    }

    pub fn owner_id_mut(&mut self) -> impl ::puroro::StringMut<A> + '_ {
        self.owner_id.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_owner_id(&mut self) {
        self.owner_id.bind_mut(&mut self._common).clear();
    }

    // -- payload (EXPLICIT bytes, proto field 5) -----------------------------

    pub fn payload<'a>(&'a self) -> Optional<&'a [u8], impl HasDefault<&'a [u8]>>
    where
        A: 'a,
    {
        self.payload.bind(&self._common).optional()
    }

    pub fn payload_mut(&mut self) -> impl ::puroro::BytesMut<A> + '_ {
        self.payload.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_payload(&mut self) {
        self.payload.bind_mut(&mut self._common).clear();
    }

    // -- tag_ids (repeated int32 PACKED, proto field 6) ----------------------

    pub fn tag_ids(&self) -> &[i32] {
        self.tag_ids.bind(&self._common).as_slice()
    }

    pub fn tag_ids_mut<'s>(&'s mut self) -> impl DerefMut<Target = AllocVec<i32, A>> + 's {
        self.tag_ids.bind_mut(&mut self._common).values_mut()
    }

    pub fn clear_tag_ids(&mut self) {
        self.tag_ids.bind_mut(&mut self._common).clear();
    }

    // -- scores (repeated int32 EXPANDED, proto field 7) ---------------------

    pub fn scores(&self) -> &[i32] {
        self.scores.bind(&self._common).as_slice()
    }

    pub fn scores_mut<'s>(&'s mut self) -> impl DerefMut<Target = AllocVec<i32, A>> + 's {
        self.scores.bind_mut(&mut self._common).values_mut()
    }

    pub fn clear_scores(&mut self) {
        self.scores.bind_mut(&mut self._common).clear();
    }

    // -- labels (repeated string, proto field 8) -----------------------------

    pub fn labels(&self) -> &[impl Deref<Target = str>] {
        self.labels.bind(&self._common).as_slice()
    }

    /// Container mutator: `push()` appends an empty string and returns a
    /// [`::puroro::String`] handle.
    pub fn labels_mut(&mut self) -> impl RepeatedStringMut<A> + '_ {
        self.labels.bind_mut(&mut self._common).container_mut()
    }

    pub fn clear_labels(&mut self) {
        self.labels.bind_mut(&mut self._common).clear();
    }

    // -- status (IMPLICIT open enum, proto field 9) -------------------------

    pub fn status<'a>(&'a self) -> Optional<Status, impl HasDefault<Status>>
    where
        A: 'a,
    {
        self.status.bind(&self._common).optional()
    }

    pub fn status_mut(&mut self) -> impl DerefMut<Target = Status> + '_ {
        self.status.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_status(&mut self) {
        self.status.bind_mut(&mut self._common).clear();
    }

    // -- priority (EXPLICIT closed enum, proto field 10) ---------------------

    pub fn priority<'a>(&'a self) -> Optional<Priority, impl HasDefault<Priority>>
    where
        A: 'a,
    {
        self.priority.bind(&self._common).optional()
    }

    pub fn priority_mut(&mut self) -> impl DerefMut<Target = Priority> + '_ {
        self.priority.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_priority(&mut self) {
        self.priority.bind_mut(&mut self._common).clear();
    }

    // -- assignee (nested message, proto field 11) --------------------------

    pub fn assignee(&self) -> Option<&Address<A>> {
        self.assignee.bind(&self._common).get()
    }

    pub fn assignee_mut(&mut self) -> &mut Address<A> {
        self.assignee.bind_mut(&mut self._common).get_mut()
    }

    pub fn clear_assignee(&mut self) {
        self.assignee.bind_mut(&mut self._common).clear();
    }

    // -- done (IMPLICIT bool, proto field 16) --------------------------------

    pub fn done(&self) -> bool {
        self.done.bind(&self._common).value()
    }

    pub fn done_mut(&mut self) -> impl DerefMut<Target = bool> + '_ {
        self.done.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_done(&mut self) {
        self.done.bind_mut(&mut self._common).clear();
    }

    // -- flag (EXPLICIT bool, proto field 17) --------------------------------

    pub fn flag<'a>(&'a self) -> Optional<bool, impl HasDefault<bool>>
    where
        A: 'a,
    {
        self.flag.bind(&self._common).optional()
    }

    pub fn flag_mut(&mut self) -> impl DerefMut<Target = bool> + '_ {
        self.flag.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_flag(&mut self) {
        self.flag.bind_mut(&mut self._common).clear();
    }

    // -- watchers (repeated Address, proto field 19) -------------------------

    pub fn watchers(&self) -> &[Address<A>] {
        self.watchers.bind(&self._common).as_slice()
    }

    pub fn watchers_mut<'s>(&'s mut self) -> impl DerefMut<Target = AllocVec<Address<A>, A>> + 's {
        self.watchers.bind_mut(&mut self._common).values_mut()
    }

    pub fn clear_watchers(&mut self) {
        self.watchers.bind_mut(&mut self._common).clear();
    }

    // -- votes (repeated bool PACKED, proto field 20) ------------------------

    pub fn votes(&self) -> &[bool] {
        self.votes.bind(&self._common).as_slice()
    }

    pub fn votes_mut<'s>(&'s mut self) -> impl DerefMut<Target = AllocVec<bool, A>> + 's {
        self.votes.bind_mut(&mut self._common).values_mut()
    }

    pub fn clear_votes(&mut self) {
        self.votes.bind_mut(&mut self._common).clear();
    }

    // -- attributes (map<string, int32>, proto field 21) ---------------------

    pub fn attributes(&self) -> impl MapRef<str, i32> + '_ {
        self.attributes.bind(&self._common)
    }

    pub fn attributes_mut(&mut self) -> impl MapMut<str, i32, MutTarget = i32> + '_ {
        self.attributes.bind_mut(&mut self._common)
    }

    pub fn clear_attributes(&mut self) {
        MapMut::clear(&mut self.attributes_mut());
    }

    // -- oneof notification (proto fields 12 / 13 / 14 / 15 / 18) ------------

    /// Bound shared view of the oneof group (always available, including when unset).
    ///
    /// Discriminant: `notification().case() -> Option<NotificationCase>`.
    pub fn notification<'a>(
        &'a self,
    ) -> impl OneofView<
        Case = NotificationCase,
        Ref = Notification<&'a str, &'a str, i32, &'a Address<A>, bool>,
    > + 'a {
        ::puroro_rt::OneofView::<NotificationStorage<A>>::new(&self.notification, &self._common)
    }

    /// Bound mutable view of the oneof group (always available, including when unset).
    ///
    /// Use [`OneofViewMut::as_view`] to reborrow for `case` / `as_ref` while mutating;
    /// match shared payloads via [`Self::notification`]. Mutable projection from
    /// [`OneofViewMut::as_mut`] is intentionally opaque (`impl` associated type) so
    /// `puroro-rt` mut handles do not appear in this signature — prefer per-variant
    /// `_mut` accessors for typed mutation.
    pub fn notification_mut<'a>(&'a mut self) -> impl OneofViewMut<Case = NotificationCase> + 'a {
        ::puroro_rt::OneofViewMut::<NotificationStorage<A>>::new(
            &mut self.notification,
            &mut self._common,
        )
    }

    pub fn clear_notification(&mut self) {
        self.notification_mut().clear();
    }

    // -- notification.email_address (string, proto field 12) ----------------

    pub fn email_address<'a>(&'a self) -> Optional<&'a str, impl HasDefault<&'a str>>
    where
        A: 'a,
    {
        // Unset / other variant → Optional::None → get() is ProtoDefault ("").
        self.notification
            .bind(&self._common)
            .variant_of::<FIELD_EMAIL_ADDRESS>()
            .optional()
    }

    pub fn email_address_mut(&mut self) -> impl ::puroro::StringMut<A> + '_ {
        self.notification
            .bind_mut(&mut self._common)
            .variant_mut::<FIELD_EMAIL_ADDRESS>()
            .bind_mut(&mut self._common)
            .value_mut()
    }

    // -- notification.phone_number (string, proto field 13) -----------------

    pub fn phone_number<'a>(&'a self) -> Optional<&'a str, impl HasDefault<&'a str>>
    where
        A: 'a,
    {
        self.notification
            .bind(&self._common)
            .variant_of::<FIELD_PHONE_NUMBER>()
            .optional()
    }

    pub fn phone_number_mut(&mut self) -> impl ::puroro::StringMut<A> + '_ {
        self.notification
            .bind_mut(&mut self._common)
            .variant_mut::<FIELD_PHONE_NUMBER>()
            .bind_mut(&mut self._common)
            .value_mut()
    }

    // -- notification.webhook_id (int32, default = -1, proto field 14) ------

    /// `[default = -1]`: when this variant is not active, `get()` returns `-1`
    /// and `is_set()` is false (does not select the variant).
    pub fn webhook_id<'a>(&'a self) -> Optional<i32, impl HasDefault<i32>>
    where
        A: 'a,
    {
        self.notification
            .bind(&self._common)
            .variant_of::<FIELD_WEBHOOK_ID>()
            .optional()
    }

    /// Switches the group to `webhook_id` (freeing any other variant) and returns
    /// a mutable handle to the scalar.
    pub fn webhook_id_mut(&mut self) -> impl DerefMut<Target = i32> + '_ {
        self.notification
            .bind_mut(&mut self._common)
            .variant_mut::<FIELD_WEBHOOK_ID>()
            .bind_mut(&mut self._common)
            .value_mut()
    }

    // -- notification.postal (Address message, proto field 15) --------------

    pub fn postal(&self) -> Option<&Address<A>> {
        self.notification
            .bind(&self._common)
            .variant_of::<FIELD_POSTAL>()
            .get()
    }

    /// Switches the group to `postal` (freeing any other variant) and returns a
    /// mutable handle to the nested message, creating an empty one if needed.
    pub fn postal_mut(&mut self) -> &mut Address<A> {
        self.notification
            .bind_mut(&mut self._common)
            .variant_mut::<FIELD_POSTAL>()
            .bind_mut(&mut self._common)
            .value_mut()
    }

    // -- notification.urgent (bool, proto field 18) -------------------------

    pub fn urgent<'a>(&'a self) -> Optional<bool, impl HasDefault<bool>>
    where
        A: 'a,
    {
        self.notification
            .bind(&self._common)
            .variant_of::<FIELD_URGENT>()
            .optional()
    }

    pub fn urgent_mut(&mut self) -> impl DerefMut<Target = bool> + '_ {
        self.notification
            .bind_mut(&mut self._common)
            .variant_mut::<FIELD_URGENT>()
            .bind_mut(&mut self._common)
            .value_mut()
    }

    // -- field visitors (scalar/pair × shared/mut) --------------------------
    // Visitors capture `MessageCommon` at construction; these methods only
    // enumerate field slots.

    /// Scalar / shared: invoke `v` once per catalog field, in declaration order.
    // Internal field walks for codec / Clone / Eq / Drop — not public API.
    fn visit_fields<V: FieldVisitor<MessageCommon<BitArray<[u8; 2], Lsb0>, A>>>(
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
        ControlFlow::Continue(())
    }

    /// Pair / shared: walk matching fields of `self` and `other`.
    fn visit_field_pairs<V: FieldPairVisitor<MessageCommon<BitArray<[u8; 2], Lsb0>, A>>>(
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
        ControlFlow::Continue(())
    }

    /// Pair / mut: walk `self` fields against mutable `dst` fields.
    ///
    /// For [`CloneIn`], `dst` must start as [`Self::new_in`] so placeholders
    /// match empty common bits; install the cloned [`MessageCommon`] afterwards.
    fn visit_field_pairs_mut<V: FieldPairVisitorMut<MessageCommon<BitArray<[u8; 2], Lsb0>, A>>>(
        &self,
        dst: &mut Self,
        v: &mut V,
    ) -> ControlFlow<V::Break> {
        v.visit("title", &self.title, &mut dst.title)?;
        v.visit("score", &self.score, &mut dst.score)?;
        v.visit("max_retries", &self.max_retries, &mut dst.max_retries)?;
        v.visit("owner_id", &self.owner_id, &mut dst.owner_id)?;
        v.visit("payload", &self.payload, &mut dst.payload)?;
        v.visit("tag_ids", &self.tag_ids, &mut dst.tag_ids)?;
        v.visit("scores", &self.scores, &mut dst.scores)?;
        v.visit("labels", &self.labels, &mut dst.labels)?;
        v.visit("status", &self.status, &mut dst.status)?;
        v.visit("priority", &self.priority, &mut dst.priority)?;
        v.visit("assignee", &self.assignee, &mut dst.assignee)?;
        v.visit("notification", &self.notification, &mut dst.notification)?;
        v.visit("done", &self.done, &mut dst.done)?;
        v.visit("flag", &self.flag, &mut dst.flag)?;
        v.visit("watchers", &self.watchers, &mut dst.watchers)?;
        v.visit("votes", &self.votes, &mut dst.votes)?;
        v.visit("attributes", &self.attributes, &mut dst.attributes)?;
        ControlFlow::Continue(())
    }

    /// Scalar / mut: invoke `v` once per catalog field.
    fn visit_fields_mut<V: FieldVisitorMut<MessageCommon<BitArray<[u8; 2], Lsb0>, A>>>(
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
        ControlFlow::Continue(())
    }
}

impl Task<Global> {
    pub fn new() -> Self {
        <Self as Default>::default()
    }
}

impl<A: Allocator + Clone + Default> Default for Task<A> {
    fn default() -> Self {
        Self::new_in(A::default())
    }
}

// ---------------------------------------------------------------------------
// Clone / PartialEq / Debug
// ---------------------------------------------------------------------------

impl<A: Allocator + Clone> ::puroro_rt::CloneIn<A> for Task<A> {
    fn clone_in(&self, alloc: A) -> Self {
        // Empty placeholders first (presence still zero), then clone fields,
        // then install cloned common (presence + unknown fields).
        let mut dst = Self::new_in(alloc.clone());
        let mut v = CloneFieldsVisitor::new(&self._common, &dst._common);
        let _ = self.visit_field_pairs_mut(&mut dst, &mut v);
        let mut old = mem::replace(&mut dst._common, self._common.clone_in(alloc));
        old.deallocate();
        dst
    }
}

impl<A: Allocator + Clone> Clone for Task<A> {
    #[inline]
    fn clone(&self) -> Self {
        self.clone_in(self._common.alloc.clone())
    }
}

impl<A: Allocator + Clone> PartialEq for Task<A> {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            self.visit_field_pairs(
                other,
                &mut FieldEqVisitor::new(&self._common, &other._common)
            ),
            ControlFlow::Continue(())
        ) && self._common.unknown_fields_eq(&other._common)
    }
}

impl<A: Allocator + Clone> fmt::Debug for Task<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut v = DebugStructVisitor::new(f.debug_struct("Task"), &self._common);
        let _ = self.visit_fields(&mut v);
        v.finish()
    }
}

// ---------------------------------------------------------------------------
// Drop — releases every unmanaged field through the single allocator
// ---------------------------------------------------------------------------

impl<A: Allocator + Clone> Drop for Task<A> {
    fn drop(&mut self) {
        let mut v = FieldDeallocVisitor::new(&self._common);
        let _ = self.visit_fields_mut(&mut v);
        self._common.deallocate();
    }
}

// ---------------------------------------------------------------------------
// DeallocateIn — required for nested `UnmanagedBox` / catalog bounds
// ---------------------------------------------------------------------------

impl<A: Allocator + Clone> ::puroro_rt::DeallocateIn<A> for Task<A> {
    #[inline]
    unsafe fn deallocate_in(self, _alloc: A) {
        // Heap is owned by `self._common.alloc`; parent-passed `alloc` is only
        // needed when freeing an enclosing `UnmanagedBox` slot.
        drop(self);
    }
}

// ---------------------------------------------------------------------------
// Message
// ---------------------------------------------------------------------------

impl<A: Allocator + Clone> MessageEncode for Task<A> {
    fn encoded_len(&self, ctx: &mut EncodeCtx) -> usize {
        let mut v = EncodedLenVisitor::new(&self._common, ctx);
        let _ = self.visit_fields(&mut v);
        v.len + self._common.unknown_fields.len()
    }

    fn encode_raw<B: BufMut>(&self, ctx: &mut EncodeCtx, buf: &mut B) {
        let _ = self.visit_fields(&mut EncodeRawVisitor::new(&self._common, ctx, buf));
        let unknown: &[u8] = &self._common.unknown_fields;
        buf.put_slice(unknown);
    }
}

impl<A: Allocator + Clone> MessageMerge for Task<A> {
    fn merge_from_with_depth<B: DecodeBuf>(
        &mut self,
        buf: &mut B,
        depth: usize,
    ) -> Result<(), DecodeError> {
        if depth >= ::puroro::RECURSION_LIMIT {
            return Err(DecodeError::RecursionLimitExceeded);
        }
        while buf.has_remaining() {
            let (field_number, wire_type) = decode_tag(buf)?;
            match field_number.as_u32() {
                FIELD_TITLE => {
                    // title = 1, EXPLICIT string
                    self.title
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                FIELD_SCORE => {
                    // score = 2, IMPLICIT int32
                    self.score
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                FIELD_MAX_RETRIES => {
                    // max_retries = 3, EXPLICIT int32
                    self.max_retries
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                FIELD_OWNER_ID => {
                    // owner_id = 4, LEGACY_REQUIRED string
                    self.owner_id
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                FIELD_PAYLOAD => {
                    // payload = 5, EXPLICIT bytes
                    self.payload
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                FIELD_TAG_IDS => {
                    // tag_ids = 6, repeated int32 PACKED
                    self.tag_ids
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                FIELD_SCORES => {
                    // scores = 7, repeated int32 EXPANDED
                    self.scores
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                FIELD_LABELS => {
                    // labels = 8, repeated string
                    self.labels
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                FIELD_STATUS => {
                    // status = 9, IMPLICIT open enum
                    self.status
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                FIELD_PRIORITY => {
                    // priority = 10, EXPLICIT closed enum
                    self.priority
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                FIELD_ASSIGNEE => {
                    // assignee = 11, nested message
                    self.assignee
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                FIELD_EMAIL_ADDRESS => {
                    // notification.email_address = 12, oneof LEN string
                    self.notification
                        .bind_mut(&mut self._common)
                        .variant_mut::<FIELD_EMAIL_ADDRESS>()
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                FIELD_PHONE_NUMBER => {
                    // notification.phone_number = 13, oneof LEN string
                    self.notification
                        .bind_mut(&mut self._common)
                        .variant_mut::<FIELD_PHONE_NUMBER>()
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                FIELD_WEBHOOK_ID => {
                    // notification.webhook_id = 14, oneof VARINT int32
                    self.notification
                        .bind_mut(&mut self._common)
                        .variant_mut::<FIELD_WEBHOOK_ID>()
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                FIELD_POSTAL => {
                    // notification.postal = 15, oneof nested message
                    self.notification
                        .bind_mut(&mut self._common)
                        .variant_mut::<FIELD_POSTAL>()
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                FIELD_DONE => {
                    // done = 16, IMPLICIT bool
                    self.done
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                FIELD_FLAG => {
                    // flag = 17, EXPLICIT bool
                    self.flag
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                FIELD_URGENT => {
                    // notification.urgent = 18, oneof bool
                    self.notification
                        .bind_mut(&mut self._common)
                        .variant_mut::<FIELD_URGENT>()
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                FIELD_WATCHERS => {
                    // watchers = 19, repeated Address
                    self.watchers
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                FIELD_VOTES => {
                    // votes = 20, repeated bool PACKED
                    self.votes
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                FIELD_ATTRIBUTES => {
                    // attributes = 21, map<string, int32>
                    self.attributes
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf, depth)?;
                }
                _ => {
                    // unknown field — preserve in _common.unknown_fields
                    skip_field_and_save(
                        field_number,
                        wire_type,
                        buf,
                        &mut self._common.unknown_fields,
                        self._common.alloc.clone(),
                    )?
                }
            }
        }
        Ok(())
    }
}

impl<A: Allocator + Clone> ::puroro_rt::DefaultIn<A> for Task<A> {
    #[inline]
    fn default_in(alloc: A) -> Self {
        Self::new_in(alloc)
    }
}

impl<A: Allocator + Clone> Message for Task<A> {
    type Alloc = A;

    fn new_in(alloc: A) -> Self {
        Self::new_in(alloc)
    }

    fn encode<B: BufMut>(&self, buf: &mut B) {
        ::puroro_rt::encode_message(self, buf)
    }

    fn encode_to_vec(&self) -> Vec<u8> {
        ::puroro_rt::encode_message_to_vec(self)
    }

    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError> {
        ::puroro_rt::merge_message(self, buf)
    }

    fn unknown_fields(&self) -> impl Iterator<Item = ::puroro::UnknownField<'_>> + '_ {
        self._common.iter_unknown_fields()
    }

    fn validate(&self) -> Result<(), DecodeError> {
        self.owner_id.validate_required(&self._common)
    }
}
