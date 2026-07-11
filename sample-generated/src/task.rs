//! Sample of the accessors, encode/decode glue, and presence handling that
//! puroro generates for message `example.Task` (from `example.proto`).
//!
//! Written for human readers: unlike real plugin output it uses short imported
//! names instead of fully-qualified paths (see the crate root docs).

mod defaults;
mod notification;

use ::allocator_api2::alloc::{Allocator, Global};
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;
use ::bytes::{Buf, BufMut};
use ::core::ops::DerefMut;

use ::puroro::{DecodeError, HasDefault, MessageDecode, MessageEncode, Optional};
use ::puroro_rt::{
    Explicit, FieldDeallocate, Implicit, LegacyRequired, MessageCommon, NestedMessageField,
    OneofSlot, PresenceBits, ProtoBool, ProtoBytes, ProtoEnum, ProtoInt32, ProtoString,
    RepeatedExpandedVarintField, RepeatedLenField, RepeatedPackedVarintField, Singular,
    SingularAccess, SingularLenField, SingularVarintField,
};

use defaults::MaxRetriesDefault;

use crate::address::Address;
use crate::enums::{Priority, Status};

use notification::NotificationStorage;
use notification::variant::{EmailAddress, PhoneNumber, Postal, Urgent, WebhookId};
pub use notification::{
    Notification, NotificationCase, NotificationMut, NotificationRef,
};

// ---------------------------------------------------------------------------
// Presence bitfield (presence + bool value bits)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default)]
pub struct TaskPresence(BitArray<[u8; 2], Lsb0>);

impl TaskPresence {
    pub const ZERO: Self = Self(BitArray::ZERO);

    /// Concrete [`BitRef`] for projections that cannot hold `impl Trait`
    /// (e.g. [`NotificationMut`](notification::NotificationMut)).
    #[inline]
    pub(crate) fn bit_ref_mut(
        &mut self,
        bit: usize,
    ) -> ::bitvec::ptr::BitRef<'_, ::bitvec::ptr::Mut, u8, Lsb0> {
        self.0
            .get_mut(bit)
            .expect("bool / presence bit index in range")
    }
}

impl PresenceBits for TaskPresence {
    fn is_set(&self, bit: usize) -> bool {
        self.0[bit]
    }

    fn set(&mut self, bit: usize, present: bool) {
        self.0.set(bit, present);
    }

    fn bit_mut(&mut self, bit: usize) -> impl DerefMut<Target = bool> + '_ {
        self.bit_ref_mut(bit)
    }
}

// ---------------------------------------------------------------------------
// Bit indices — presence (EXPLICIT/LEGACY_REQUIRED) then bool value bits,
// assigned by ascending field number in one pass.
// ---------------------------------------------------------------------------

pub const BIT_TITLE: usize = 0; // title (EXPLICIT presence)
pub const BIT_MAX_RETRIES: usize = 1; // max_retries (EXPLICIT presence)
pub const BIT_OWNER_ID: usize = 2; // owner_id (LEGACY_REQUIRED presence)
pub const BIT_PAYLOAD: usize = 3; // payload (EXPLICIT presence)
pub const BIT_PRIORITY: usize = 4; // priority (EXPLICIT presence)
pub const BIT_DONE_VALUE: usize = 5; // done (IMPLICIT bool value)
pub const BIT_FLAG: usize = 6; // flag (EXPLICIT presence)
pub const BIT_FLAG_VALUE: usize = 7; // flag (EXPLICIT bool value)
pub const BIT_URGENT_VALUE: usize = 8; // notification.urgent (oneof bool value)

// ---------------------------------------------------------------------------
// Proto field numbers
// ---------------------------------------------------------------------------

pub const FIELD_TITLE: u32 = 1; // title
pub const FIELD_SCORE: u32 = 2; // score
pub const FIELD_MAX_RETRIES: u32 = 3; // max_retries
pub const FIELD_OWNER_ID: u32 = 4; // owner_id
pub const FIELD_PAYLOAD: u32 = 5; // payload
pub const FIELD_TAG_IDS: u32 = 6; // tag_ids
pub const FIELD_SCORES: u32 = 7; // scores
pub const FIELD_LABELS: u32 = 8; // labels
pub const FIELD_STATUS: u32 = 9; // status
pub const FIELD_PRIORITY: u32 = 10; // priority
pub const FIELD_ASSIGNEE: u32 = 11; // assignee
pub const FIELD_EMAIL_ADDRESS: u32 = 12; // notification.email_address
pub const FIELD_PHONE_NUMBER: u32 = 13; // notification.phone_number
pub const FIELD_WEBHOOK_ID: u32 = 14; // notification.webhook_id
pub const FIELD_POSTAL: u32 = 15; // notification.postal
pub const FIELD_DONE: u32 = 16; // done (IMPLICIT bool)
pub const FIELD_FLAG: u32 = 17; // flag (EXPLICIT bool)
pub const FIELD_URGENT: u32 = 18; // notification.urgent (oneof bool)
// ---------------------------------------------------------------------------
// Message struct
// ---------------------------------------------------------------------------

/// Reference `Task` message from `DESIGN.md`.
pub struct Task<A: Allocator + Clone = Global> {
    _common: MessageCommon<TaskPresence, A>,
    title: SingularLenField<ProtoString, Explicit<{ BIT_TITLE }>, { FIELD_TITLE }>, // proto: string title = 1;
    score: SingularVarintField<ProtoInt32, Implicit, { FIELD_SCORE }>, // proto: int32 score = 2;
    max_retries: SingularVarintField<
        ProtoInt32,
        Explicit<{ BIT_MAX_RETRIES }>,
        { FIELD_MAX_RETRIES },
        MaxRetriesDefault,
    >, // proto: int32 max_retries = 3;
    owner_id: SingularLenField<ProtoString, LegacyRequired<{ BIT_OWNER_ID }>, { FIELD_OWNER_ID }>, // proto: string owner_id = 4;
    payload: SingularLenField<ProtoBytes, Explicit<{ BIT_PAYLOAD }>, { FIELD_PAYLOAD }>, // proto: bytes payload = 5;
    tag_ids: RepeatedPackedVarintField<ProtoInt32, { FIELD_TAG_IDS }, A>, // proto: repeated int32 tag_ids = 6 [packed];
    scores: RepeatedExpandedVarintField<ProtoInt32, { FIELD_SCORES }, A>, // proto: repeated int32 scores = 7;
    labels: RepeatedLenField<ProtoString, { FIELD_LABELS }, A>, // proto: repeated string labels = 8;
    status: SingularVarintField<ProtoEnum<Status>, Implicit, { FIELD_STATUS }>, // proto: Status status = 9;
    priority:
        SingularVarintField<ProtoEnum<Priority>, Explicit<{ BIT_PRIORITY }>, { FIELD_PRIORITY }>, // proto: Priority priority = 10;
    assignee: NestedMessageField<Address<A>, Singular, { FIELD_ASSIGNEE }, A>, // proto: Address assignee = 11;
    // proto: oneof notification { string email_address=12; string phone_number=13;
    //                             int32 webhook_id=14 [default=-1]; Address postal=15;
    //                             bool urgent=18; }
    notification: OneofSlot<NotificationStorage<A>>,
    done: SingularVarintField<ProtoBool<{ BIT_DONE_VALUE }>, Implicit, { FIELD_DONE }>, // proto: bool done = 16;
    flag: SingularVarintField<
        ProtoBool<{ BIT_FLAG_VALUE }>,
        Explicit<{ BIT_FLAG }>,
        { FIELD_FLAG },
    >, // proto: bool flag = 17;
}

impl<A: Allocator + Clone> Task<A> {
    pub fn new_in(alloc: A) -> Self {
        // Each field initializer gets its own clone of the allocator; the last
        // heap field (`labels`) takes the original by move.
        Self {
            _common: MessageCommon::new_in(TaskPresence::ZERO, alloc.clone()),
            title: SingularLenField::new_in(alloc.clone()),
            score: SingularVarintField::new_in(alloc.clone()),
            max_retries: SingularVarintField::new_in(alloc.clone()),
            owner_id: SingularLenField::new_in(alloc.clone()),
            payload: SingularLenField::new_in(alloc.clone()),
            tag_ids: RepeatedPackedVarintField::new_in(alloc.clone()),
            scores: RepeatedExpandedVarintField::new_in(alloc.clone()),
            labels: RepeatedLenField::new_in(alloc.clone()),
            status: SingularVarintField::new_in(alloc.clone()),
            priority: SingularVarintField::new_in(alloc.clone()),
            assignee: NestedMessageField::new_in(alloc.clone()),
            notification: OneofSlot::new_in(alloc.clone()),
            done: SingularVarintField::new_in(alloc.clone()),
            flag: SingularVarintField::new_in(alloc),
        }
    }

    // -- title (EXPLICIT string, proto field 1) ----------------------------

    pub fn title<'a>(&'a self) -> Optional<&'a str, impl HasDefault<&'a str>> {
        self.title.bind(&self._common).optional()
    }

    pub fn title_mut<'s>(
        &'s mut self,
    ) -> impl ::core::ops::DerefMut<Target = ::unmanaged::String<A>> + 's {
        self.title.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_title(&mut self) {
        self.title.bind_mut(&mut self._common).clear();
    }

    // -- score (IMPLICIT int32, proto field 2) ------------------------------

    pub fn score(&self) -> i32 {
        self.score.bind(&self._common).value()
    }

    pub fn score_mut(&mut self) -> impl ::core::ops::DerefMut<Target = i32> + '_ {
        self.score.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_score(&mut self) {
        self.score.bind_mut(&mut self._common).clear();
    }

    // -- max_retries (EXPLICIT int32, default = 3, proto field 3) ------------

    pub fn max_retries(&self) -> Optional<i32, impl HasDefault<i32>> {
        self.max_retries.bind(&self._common).optional()
    }

    pub fn max_retries_mut(&mut self) -> impl ::core::ops::DerefMut<Target = i32> + '_ {
        self.max_retries.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_max_retries(&mut self) {
        self.max_retries.bind_mut(&mut self._common).clear();
    }

    // -- owner_id (LEGACY_REQUIRED string, proto field 4) --------------------

    pub fn owner_id<'a>(&'a self) -> Optional<&'a str, impl HasDefault<&'a str>> {
        self.owner_id.bind(&self._common).optional()
    }

    pub fn owner_id_mut<'s>(
        &'s mut self,
    ) -> impl ::core::ops::DerefMut<Target = ::unmanaged::String<A>> + 's {
        self.owner_id.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_owner_id(&mut self) {
        self.owner_id.bind_mut(&mut self._common).clear();
    }

    // -- payload (EXPLICIT bytes, proto field 5) -----------------------------

    pub fn payload<'a>(&'a self) -> Optional<&'a [u8], impl HasDefault<&'a [u8]>> {
        self.payload.bind(&self._common).optional()
    }

    pub fn payload_mut<'s>(
        &'s mut self,
    ) -> impl ::core::ops::DerefMut<Target = ::allocator_api2::vec::Vec<u8, A>> + 's {
        self.payload.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_payload(&mut self) {
        self.payload.bind_mut(&mut self._common).clear();
    }

    // -- tag_ids (repeated int32 PACKED, proto field 6) ----------------------

    pub fn tag_ids(&self) -> &[i32] {
        self.tag_ids.bind(&self._common).as_slice()
    }

    pub fn tag_ids_mut<'s>(
        &'s mut self,
    ) -> impl ::core::ops::DerefMut<Target = ::allocator_api2::vec::Vec<i32, A>> + 's {
        self.tag_ids.bind_mut(&mut self._common).values_mut()
    }

    pub fn clear_tag_ids(&mut self) {
        self.tag_ids.bind_mut(&mut self._common).clear();
    }

    // -- scores (repeated int32 EXPANDED, proto field 7) ---------------------

    pub fn scores(&self) -> &[i32] {
        self.scores.bind(&self._common).as_slice()
    }

    pub fn scores_mut<'s>(
        &'s mut self,
    ) -> impl ::core::ops::DerefMut<Target = ::allocator_api2::vec::Vec<i32, A>> + 's {
        self.scores.bind_mut(&mut self._common).values_mut()
    }

    pub fn clear_scores(&mut self) {
        self.scores.bind_mut(&mut self._common).clear();
    }

    // -- labels (repeated string, proto field 8) -----------------------------

    pub fn labels(&self) -> &[::unmanaged::UnmanagedString] {
        self.labels.bind(&self._common).as_slice()
    }

    /// Typed append helper for repeated LEN fields (the `_mut` accessor would
    /// expose allocator-less element storage, which is impractical to build).
    pub fn push_label(&mut self, v: &str) {
        self.labels.bind_mut(&mut self._common).push_in(v).ok();
    }

    pub fn clear_labels(&mut self) {
        self.labels.bind_mut(&mut self._common).clear();
    }

    // -- status (IMPLICIT open enum, proto field 9) -------------------------

    pub fn status(&self) -> Optional<Status, impl HasDefault<Status>> {
        self.status.bind(&self._common).optional()
    }

    pub fn status_mut(&mut self) -> impl ::core::ops::DerefMut<Target = Status> + '_ {
        self.status.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_status(&mut self) {
        self.status.bind_mut(&mut self._common).clear();
    }

    // -- priority (EXPLICIT closed enum, proto field 10) ---------------------

    pub fn priority(&self) -> Optional<Priority, impl HasDefault<Priority>> {
        self.priority.bind(&self._common).optional()
    }

    pub fn priority_mut(&mut self) -> impl ::core::ops::DerefMut<Target = Priority> + '_ {
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

    pub fn done_mut(&mut self) -> impl ::core::ops::DerefMut<Target = bool> + '_ {
        self.done.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_done(&mut self) {
        self.done.bind_mut(&mut self._common).clear();
    }

    // -- flag (EXPLICIT bool, proto field 17) --------------------------------

    pub fn flag(&self) -> Optional<bool, impl HasDefault<bool>> {
        self.flag.bind(&self._common).optional()
    }

    pub fn flag_mut(&mut self) -> impl ::core::ops::DerefMut<Target = bool> + '_ {
        self.flag.bind_mut(&mut self._common).value_mut()
    }

    pub fn clear_flag(&mut self) {
        self.flag.bind_mut(&mut self._common).clear();
    }

    // -- oneof notification (proto fields 12 / 13 / 14 / 15 / 18) ------------

    /// Which variant is set (payload-less; `None` when the group is unset).
    pub fn notification_case(&self) -> Option<NotificationCase> {
        self.notification().case()
    }

    /// Bound shared view of the oneof group (always available, including when unset).
    pub fn notification<'a>(
        &'a self,
    ) -> ::puroro_rt::OneofView<
        'a,
        impl ::puroro_rt::OneofGroup<
            Case = NotificationCase,
            Ref<'a> = NotificationRef<'a, A>,
            Mut<'a> = NotificationMut<'a, A>,
        >,
    > {
        ::puroro_rt::OneofView::<NotificationStorage<A>>::new(&self.notification, &self._common)
    }

    /// Bound mutable view of the oneof group (always available, including when unset).
    ///
    /// RPIT omits `Ref` so `as_view` reborrows stay short-lived; match shared
    /// payloads via [`Self::notification`].
    pub fn notification_mut<'a>(
        &'a mut self,
    ) -> ::puroro_rt::OneofViewMut<
        'a,
        impl ::puroro_rt::OneofGroup<
            Case = NotificationCase,
            Mut<'a> = NotificationMut<'a, A>,
        >,
    > {
        ::puroro_rt::OneofViewMut::<NotificationStorage<A>>::new(
            &mut self.notification,
            &mut self._common,
        )
    }

    pub fn clear_notification(&mut self) {
        self.notification_mut().clear();
    }

    // -- notification.email_address (string, proto field 12) ----------------

    pub fn email_address<'a>(&'a self) -> Optional<&'a str, impl HasDefault<&'a str>> {
        // Unset / other variant → Optional::None → get() is ProtoDefault ("").
        self.notification
            .bind(&self._common)
            .variant_of::<EmailAddress>()
            .optional()
    }

    pub fn email_address_mut(
        &mut self,
    ) -> impl ::core::ops::DerefMut<Target = ::unmanaged::String<A>> + '_ {
        self.notification
            .bind_mut(&mut self._common)
            .variant_mut::<EmailAddress>()
            .value_mut()
    }

    // -- notification.phone_number (string, proto field 13) -----------------

    pub fn phone_number<'a>(&'a self) -> Optional<&'a str, impl HasDefault<&'a str>> {
        self.notification
            .bind(&self._common)
            .variant_of::<PhoneNumber>()
            .optional()
    }

    pub fn phone_number_mut(
        &mut self,
    ) -> impl ::core::ops::DerefMut<Target = ::unmanaged::String<A>> + '_ {
        self.notification
            .bind_mut(&mut self._common)
            .variant_mut::<PhoneNumber>()
            .value_mut()
    }

    // -- notification.webhook_id (int32, default = -1, proto field 14) ------

    /// `[default = -1]`: when this variant is not active, `get()` returns `-1`
    /// and `is_set()` is false (does not select the variant).
    pub fn webhook_id(&self) -> Optional<i32, impl HasDefault<i32>> {
        self.notification
            .bind(&self._common)
            .variant_of::<WebhookId>()
            .optional()
    }

    /// Switches the group to `webhook_id` (freeing any other variant) and returns
    /// a mutable handle to the scalar.
    pub fn webhook_id_mut(&mut self) -> impl ::core::ops::DerefMut<Target = i32> + '_ {
        self.notification
            .bind_mut(&mut self._common)
            .variant_mut::<WebhookId>()
            .value_mut()
    }

    // -- notification.postal (Address message, proto field 15) --------------

    pub fn postal(&self) -> Option<&Address<A>> {
        self.notification
            .bind(&self._common)
            .variant_of::<Postal>()
            .get()
    }

    /// Switches the group to `postal` (freeing any other variant) and returns a
    /// mutable handle to the nested message, creating an empty one if needed.
    pub fn postal_mut(&mut self) -> &mut Address<A> {
        self.notification
            .bind_mut(&mut self._common)
            .variant_mut::<Postal>()
            .value_mut()
    }

    // -- notification.urgent (bool, proto field 18) -------------------------

    pub fn urgent(&self) -> Optional<bool, impl HasDefault<bool>> {
        self.notification
            .bind(&self._common)
            .variant_of::<Urgent>()
            .optional()
    }

    pub fn urgent_mut(&mut self) -> impl ::core::ops::DerefMut<Target = bool> + '_ {
        self.notification
            .bind_mut(&mut self._common)
            .variant_mut::<Urgent>()
            .value_mut()
    }

    // -- message-level ------------------------------------------------------

    pub fn unknown_fields(&self) -> &[u8] {
        &self._common.unknown_fields
    }

    /// Checks `LEGACY_REQUIRED` fields (`owner_id`).
    pub fn validate(&self) -> Result<(), DecodeError> {
        self.owner_id.validate_required(&self._common)
    }

    pub fn decode_strict<B: Buf>(buf: B) -> Result<Self, DecodeError>
    where
        Self: Default,
    {
        let msg = Self::decode(buf)?;
        msg.validate()?;
        Ok(msg)
    }
}

impl Task<::allocator_api2::alloc::Global> {
    pub fn new() -> Self {
        Self::new_in(Global)
    }
}

impl<A: Allocator + Clone + Default> Default for Task<A> {
    fn default() -> Self {
        Self::new_in(A::default())
    }
}

// ---------------------------------------------------------------------------
// Drop — releases every unmanaged field through the single allocator
// ---------------------------------------------------------------------------

impl<A: Allocator + Clone> Drop for Task<A> {
    fn drop(&mut self) {
        // Every direct child — same `deallocate(&common)` shape (FieldDeallocate).
        self.title.deallocate(&self._common);
        self.score.deallocate(&self._common);
        self.max_retries.deallocate(&self._common);
        self.owner_id.deallocate(&self._common);
        self.payload.deallocate(&self._common);
        self.tag_ids.deallocate(&self._common);
        self.scores.deallocate(&self._common);
        self.labels.deallocate(&self._common);
        self.status.deallocate(&self._common);
        self.priority.deallocate(&self._common);
        self.assignee.deallocate(&self._common);
        self.notification.deallocate(&self._common);
        self.done.deallocate(&self._common);
        self.flag.deallocate(&self._common);
        self._common.deallocate();
    }
}

// ---------------------------------------------------------------------------
// MessageEncode / MessageDecode
// ---------------------------------------------------------------------------

impl<A: Allocator + Clone> MessageEncode for Task<A> {
    fn encoded_len(&self) -> usize {
        let c = &self._common;
        let mut n = 0usize;
        n += self.title.encoded_len(c);
        n += self.score.encoded_len(c);
        n += self.max_retries.encoded_len(c);
        n += self.owner_id.encoded_len(c);
        n += self.payload.encoded_len(c);
        n += self.tag_ids.encoded_len(c);
        n += self.scores.encoded_len(c);
        n += self.labels.encoded_len(c);
        n += self.status.encoded_len(c);
        n += self.priority.encoded_len(c);
        n += self.assignee.encoded_len(c);
        n += self.notification.encoded_len(c);
        n += self.done.encoded_len(c);
        n += self.flag.encoded_len(c);
        n + c.unknown_fields.len()
    }

    fn encode_raw<B: BufMut>(&self, buf: &mut B) {
        let c = &self._common;
        self.title.encode_raw(c, buf);
        self.score.encode_raw(c, buf);
        self.max_retries.encode_raw(c, buf);
        self.owner_id.encode_raw(c, buf);
        self.payload.encode_raw(c, buf);
        self.tag_ids.encode_raw(c, buf);
        self.scores.encode_raw(c, buf);
        self.labels.encode_raw(c, buf);
        self.status.encode_raw(c, buf);
        self.priority.encode_raw(c, buf);
        self.assignee.encode_raw(c, buf);
        self.notification.encode_raw(c, buf);
        self.done.encode_raw(c, buf);
        self.flag.encode_raw(c, buf);
        let unknown: &[u8] = &c.unknown_fields;
        buf.put_slice(unknown);
    }
}

impl<A: Allocator + Clone> MessageDecode for Task<A> {
    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError> {
        while buf.has_remaining() {
            let (field_number, wire_type) = ::puroro_rt::decode::decode_tag(buf)?;
            match field_number {
                FIELD_TITLE => {
                    // title = 1, EXPLICIT string
                    self.title
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf)?;
                }
                FIELD_SCORE => {
                    // score = 2, IMPLICIT int32
                    self.score
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf)?;
                }
                FIELD_MAX_RETRIES => {
                    // max_retries = 3, EXPLICIT int32
                    self.max_retries
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf)?;
                }
                FIELD_OWNER_ID => {
                    // owner_id = 4, LEGACY_REQUIRED string
                    self.owner_id
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf)?;
                }
                FIELD_PAYLOAD => {
                    // payload = 5, EXPLICIT bytes
                    self.payload
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf)?;
                }
                FIELD_TAG_IDS => {
                    // tag_ids = 6, repeated int32 PACKED
                    self.tag_ids
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf)?;
                }
                FIELD_SCORES => {
                    // scores = 7, repeated int32 EXPANDED
                    self.scores
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf)?;
                }
                FIELD_LABELS => {
                    // labels = 8, repeated string
                    self.labels
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf)?;
                }
                FIELD_STATUS => {
                    // status = 9, IMPLICIT open enum
                    self.status
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf)?;
                }
                FIELD_PRIORITY => {
                    // priority = 10, EXPLICIT closed enum
                    self.priority.bind_mut(&mut self._common).merge_closed(
                        wire_type,
                        buf,
                        |v| Priority::try_from(v).is_ok(),
                    )?;
                }
                FIELD_ASSIGNEE => {
                    // assignee = 11, nested message
                    self.assignee
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf)?;
                }
                FIELD_EMAIL_ADDRESS => {
                    // notification.email_address = 12, oneof LEN string
                    self.notification
                        .bind_mut(&mut self._common)
                        .variant_mut::<EmailAddress>()
                        .merge(wire_type, buf)?;
                }
                FIELD_PHONE_NUMBER => {
                    // notification.phone_number = 13, oneof LEN string
                    self.notification
                        .bind_mut(&mut self._common)
                        .variant_mut::<PhoneNumber>()
                        .merge(wire_type, buf)?;
                }
                FIELD_WEBHOOK_ID => {
                    // notification.webhook_id = 14, oneof VARINT int32
                    self.notification
                        .bind_mut(&mut self._common)
                        .variant_mut::<WebhookId>()
                        .merge(wire_type, buf)?;
                }
                FIELD_POSTAL => {
                    // notification.postal = 15, oneof nested message
                    self.notification
                        .bind_mut(&mut self._common)
                        .variant_mut::<Postal>()
                        .merge(wire_type, buf)?;
                }
                FIELD_DONE => {
                    // done = 16, IMPLICIT bool
                    self.done
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf)?;
                }
                FIELD_FLAG => {
                    // flag = 17, EXPLICIT bool
                    self.flag
                        .bind_mut(&mut self._common)
                        .merge(wire_type, buf)?;
                }
                FIELD_URGENT => {
                    // notification.urgent = 18, oneof bool
                    self.notification
                        .bind_mut(&mut self._common)
                        .variant_mut::<Urgent>()
                        .merge(wire_type, buf)?;
                }
                _ => {
                    // unknown field — preserve in _common.unknown_fields
                    ::puroro_rt::decode::skip_field_and_save(
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
