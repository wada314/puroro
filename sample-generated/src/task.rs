//! Sample of the accessors, encode/decode glue, and presence handling that
//! puroro generates for message `example.Task` (from `example.proto`).
//!
//! Written for human readers: unlike real plugin output it uses short imported
//! names instead of fully-qualified paths (see the crate root docs).

mod notification;

use ::allocator_api2::alloc::{Allocator, Global};
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;
use ::bytes::{Buf, BufMut};

use ::puroro::{
    DecodeError, Explicit, HasDefault, Implicit, LegacyRequired, MessageCommon, MessageDecode,
    MessageEncode, NestedMessageField, OneofSlot, Optional, PresenceBits, ProtoBytes, ProtoEnum,
    ProtoInt32, ProtoString, RepeatedExpandedVarintField, RepeatedLenField,
    RepeatedPackedVarintField, SingularLenField, SingularVarintField,
};

use crate::address::Address;
use crate::enums::{Priority, Status};

use notification::NotificationStorage;
pub use notification::{NotificationCase, NotificationMut, NotificationRef};

// ---------------------------------------------------------------------------
// Presence bitfield (5 tracked singular fields)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TaskPresence(BitArray<[u8; 1], Lsb0>);

impl TaskPresence {
    pub const ZERO: Self = Self(BitArray::ZERO);
}

impl PresenceBits for TaskPresence {
    fn is_set(&self, bit: usize) -> bool {
        self.0[bit]
    }

    fn set(&mut self, bit: usize, present: bool) {
        self.0.set(bit, present);
    }
}

// ---------------------------------------------------------------------------
// Message struct
// ---------------------------------------------------------------------------

/// Reference `Task` message from `DESIGN.md`.
pub struct Task<A: Allocator + Clone = Global> {
    _common: MessageCommon<TaskPresence, A>,
    title: SingularLenField<ProtoString, Explicit, A>, // proto: string title = 1;
    score: SingularVarintField<ProtoInt32, Implicit>,  // proto: int32 score = 2;
    max_retries: SingularVarintField<ProtoInt32, Explicit>, // proto: int32 max_retries = 3;
    owner_id: SingularLenField<ProtoString, LegacyRequired, A>, // proto: string owner_id = 4;
    payload: SingularLenField<ProtoBytes, Explicit, A>, // proto: bytes payload = 5;
    tag_ids: RepeatedPackedVarintField<ProtoInt32, A>, // proto: repeated int32 tag_ids = 6 [packed];
    scores: RepeatedExpandedVarintField<ProtoInt32, A>, // proto: repeated int32 scores = 7;
    labels: RepeatedLenField<ProtoString, A>,          // proto: repeated string labels = 8;
    status: SingularVarintField<ProtoEnum, Implicit>,  // proto: Status status = 9;
    priority: SingularVarintField<ProtoEnum, Explicit>, // proto: Priority priority = 10;
    assignee: NestedMessageField<Address<A>, A>,       // proto: Address assignee = 11;
    // proto: oneof notification { string email_address=12; string phone_number=13;
    //                             int32 webhook_id=14; Address postal=15; }
    notification: OneofSlot<NotificationStorage<A>>,
}

// ---------------------------------------------------------------------------
// Field constants (associated with `Task`)
// ---------------------------------------------------------------------------

impl<A: Allocator + Clone> Task<A> {
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
    // oneof notification variant field numbers live in the `notification` module
    // (`FIELD_EMAIL_ADDRESS` = 12, `FIELD_PHONE_NUMBER` = 13, `FIELD_WEBHOOK_ID` = 14,
    // `FIELD_POSTAL` = 15), so they stay usable as `match` patterns despite
    // `NotificationStorage` being generic over `A`.

    pub const BIT_TITLE: usize = 0; // title (EXPLICIT)
    pub const BIT_MAX_RETRIES: usize = 1; // max_retries (EXPLICIT)
    pub const BIT_OWNER_ID: usize = 2; // owner_id (LEGACY_REQUIRED)
    pub const BIT_PAYLOAD: usize = 3; // payload (EXPLICIT)
    pub const BIT_PRIORITY: usize = 4; // priority (EXPLICIT)

    /// Dummy bit index for IMPLICIT / nested / repeated fields (ignored by `Implicit`).
    pub const BIT_UNUSED: usize = 0;
}

impl<A: Allocator + Clone> Task<A> {
    pub fn new_in(alloc: A) -> Self {
        // Each field initializer gets its own clone of the allocator; the last
        // heap field (`labels`) takes the original by move.
        Self {
            _common: MessageCommon::new_in(TaskPresence::ZERO, alloc.clone()),
            title: SingularLenField::new_in(alloc.clone()),
            score: SingularVarintField::new(),
            max_retries: SingularVarintField::new(),
            owner_id: SingularLenField::new_in(alloc.clone()),
            payload: SingularLenField::new_in(alloc.clone()),
            tag_ids: RepeatedPackedVarintField::new_in(alloc.clone()),
            scores: RepeatedExpandedVarintField::new_in(alloc.clone()),
            labels: RepeatedLenField::new_in(alloc),
            status: SingularVarintField::new(),
            priority: SingularVarintField::new(),
            assignee: NestedMessageField::new(),
            notification: OneofSlot::new(),
        }
    }

    // -- title (EXPLICIT string, proto field 1) ----------------------------

    pub fn title<'a>(&'a self) -> Optional<&'a str, impl HasDefault<&'a str>> {
        struct TitleDefault;
        impl<'a> HasDefault<&'a str> for TitleDefault {
            const DEFAULT: &'a str = "";
        }
        self.title.optional(&self._common, Self::BIT_TITLE, TitleDefault)
    }

    pub fn has_title(&self) -> bool {
        self.title.has(&self._common, Self::BIT_TITLE)
    }

    pub fn title_mut<'s>(
        &'s mut self,
    ) -> impl ::core::ops::DerefMut<Target = ::unmanaged::String<A>> + 's {
        self.title.bind(&mut self._common, Self::BIT_TITLE).value_mut()
    }

    pub fn clear_title(&mut self) {
        self.title.bind(&mut self._common, Self::BIT_TITLE).clear();
    }

    // -- score (IMPLICIT int32, proto field 2) ------------------------------

    pub fn score(&self) -> i32 {
        self.score.value()
    }

    pub fn score_mut(&mut self) -> &mut i32 {
        self.score.bind(&mut self._common, Self::BIT_UNUSED).value_mut()
    }

    // -- max_retries (EXPLICIT int32, default = 3, proto field 3) ------------

    pub fn max_retries(&self) -> Optional<i32, impl HasDefault<i32>> {
        struct MaxRetriesDefault;
        impl HasDefault<i32> for MaxRetriesDefault {
            const DEFAULT: i32 = 3;
        }
        self.max_retries
            .optional(&self._common, Self::BIT_MAX_RETRIES, MaxRetriesDefault)
    }

    pub fn has_max_retries(&self) -> bool {
        self.max_retries.has(&self._common, Self::BIT_MAX_RETRIES)
    }

    pub fn max_retries_mut(&mut self) -> &mut i32 {
        self.max_retries
            .bind(&mut self._common, Self::BIT_MAX_RETRIES)
            .value_mut()
    }

    pub fn clear_max_retries(&mut self) {
        self.max_retries
            .bind(&mut self._common, Self::BIT_MAX_RETRIES)
            .clear();
    }

    // -- owner_id (LEGACY_REQUIRED string, proto field 4) --------------------

    pub fn owner_id<'a>(&'a self) -> Optional<&'a str, impl HasDefault<&'a str>> {
        struct OwnerIdDefault;
        impl<'a> HasDefault<&'a str> for OwnerIdDefault {
            const DEFAULT: &'a str = "";
        }
        self.owner_id
            .optional(&self._common, Self::BIT_OWNER_ID, OwnerIdDefault)
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.has(&self._common, Self::BIT_OWNER_ID)
    }

    pub fn owner_id_mut<'s>(
        &'s mut self,
    ) -> impl ::core::ops::DerefMut<Target = ::unmanaged::String<A>> + 's {
        self.owner_id
            .bind(&mut self._common, Self::BIT_OWNER_ID)
            .value_mut()
    }

    pub fn clear_owner_id(&mut self) {
        self.owner_id
            .bind(&mut self._common, Self::BIT_OWNER_ID)
            .clear();
    }

    // -- payload (EXPLICIT bytes, proto field 5) -----------------------------

    pub fn payload<'a>(&'a self) -> Optional<&'a [u8], impl HasDefault<&'a [u8]>> {
        struct PayloadDefault;
        impl<'a> HasDefault<&'a [u8]> for PayloadDefault {
            const DEFAULT: &'a [u8] = &[];
        }
        self.payload
            .optional(&self._common, Self::BIT_PAYLOAD, PayloadDefault)
    }

    pub fn has_payload(&self) -> bool {
        self.payload.has(&self._common, Self::BIT_PAYLOAD)
    }

    pub fn payload_mut<'s>(
        &'s mut self,
    ) -> impl ::core::ops::DerefMut<Target = ::allocator_api2::vec::Vec<u8, A>> + 's {
        self.payload
            .bind(&mut self._common, Self::BIT_PAYLOAD)
            .value_mut()
    }

    pub fn clear_payload(&mut self) {
        self.payload
            .bind(&mut self._common, Self::BIT_PAYLOAD)
            .clear();
    }

    // -- tag_ids (repeated int32 PACKED, proto field 6) ----------------------

    pub fn tag_ids(&self) -> &[i32] {
        self.tag_ids.as_slice()
    }

    pub fn tag_ids_mut<'s>(
        &'s mut self,
    ) -> impl ::core::ops::DerefMut<Target = ::allocator_api2::vec::Vec<i32, A>> + 's {
        self.tag_ids.bind(&mut self._common).values_mut()
    }

    pub fn clear_tag_ids(&mut self) {
        self.tag_ids.bind(&mut self._common).clear();
    }

    // -- scores (repeated int32 EXPANDED, proto field 7) ---------------------

    pub fn scores(&self) -> &[i32] {
        self.scores.as_slice()
    }

    pub fn scores_mut<'s>(
        &'s mut self,
    ) -> impl ::core::ops::DerefMut<Target = ::allocator_api2::vec::Vec<i32, A>> + 's {
        self.scores.bind(&mut self._common).values_mut()
    }

    pub fn clear_scores(&mut self) {
        self.scores.bind(&mut self._common).clear();
    }

    // -- labels (repeated string, proto field 8) -----------------------------

    pub fn labels(&self) -> &[::unmanaged::UnmanagedString] {
        self.labels.as_slice()
    }

    /// Typed append helper for repeated LEN fields (the `_mut` accessor would
    /// expose allocator-less element storage, which is impractical to build).
    pub fn push_label(&mut self, v: &str) {
        self.labels.bind(&mut self._common).push_in(v).ok();
    }

    pub fn clear_labels(&mut self) {
        self.labels.bind(&mut self._common).clear();
    }

    // -- status (IMPLICIT open enum, proto field 9) -------------------------

    pub fn status_raw(&self) -> i32 {
        self.status.value()
    }

    pub fn status_mut(&mut self) -> &mut i32 {
        self.status.bind(&mut self._common, Self::BIT_UNUSED).value_mut()
    }

    pub fn status(&self) -> Result<Status, i32> {
        Status::try_from(self.status.value())
    }

    // -- priority (EXPLICIT closed enum, proto field 10) ---------------------

    pub fn priority(&self) -> Option<Result<Priority, i32>> {
        if !self.priority.has(&self._common, Self::BIT_PRIORITY) {
            return None;
        }
        Some(Priority::try_from(self.priority.value()))
    }

    pub fn priority_mut(&mut self) -> &mut i32 {
        self.priority
            .bind(&mut self._common, Self::BIT_PRIORITY)
            .value_mut()
    }

    pub fn clear_priority(&mut self) {
        self.priority
            .bind(&mut self._common, Self::BIT_PRIORITY)
            .clear();
    }

    // -- assignee (nested message, proto field 11) --------------------------

    pub fn assignee(&self) -> Option<&Address<A>> {
        self.assignee.get()
    }

    pub fn assignee_mut(&mut self) -> &mut Address<A> {
        self.assignee.get_mut(&self._common)
    }

    pub fn clear_assignee(&mut self) {
        self.assignee.clear(self._common.alloc.clone());
    }

    // -- oneof notification (proto fields 12 / 13 / 14 / 15) ----------------

    /// Which variant is set (payload-less; `None` when the group is unset).
    pub fn notification_case(&self) -> Option<NotificationCase> {
        self.notification.get().map(|s| s.case())
    }

    /// Safe borrowed read view of the active variant.
    pub fn notification(&self) -> Option<NotificationRef<'_, A>> {
        self.notification.get().map(|s| s.to_ref())
    }

    /// Safe borrowed mutable view of the *currently active* variant (no switch).
    pub fn notification_mut(&mut self) -> Option<NotificationMut<'_, A>> {
        let alloc = self._common.alloc.clone();
        self.notification.get_mut().map(|s| s.to_mut(alloc))
    }

    pub fn email_address_mut(
        &mut self,
    ) -> impl ::core::ops::DerefMut<Target = ::unmanaged::String<A>> + '_ {
        let alloc = self._common.alloc.clone();
        NotificationStorage::bind_email_address_mut(&mut self.notification, &mut self._common)
            .value_mut(alloc)
    }

    pub fn phone_number_mut(
        &mut self,
    ) -> impl ::core::ops::DerefMut<Target = ::unmanaged::String<A>> + '_ {
        let alloc = self._common.alloc.clone();
        NotificationStorage::bind_phone_number_mut(&mut self.notification, &mut self._common)
            .value_mut(alloc)
    }

    /// Switches the group to `webhook_id` (freeing any other variant) and returns
    /// a mutable handle to the scalar.
    pub fn webhook_id_mut(&mut self) -> &mut i32 {
        NotificationStorage::bind_webhook_id_mut(&mut self.notification, &mut self._common)
            .value_mut()
    }

    /// Switches the group to `postal` (freeing any other variant) and returns a
    /// mutable handle to the nested message, creating an empty one if needed.
    pub fn postal_mut(&mut self) -> &mut Address<A> {
        // The variant invariant guarantees the child is present.
        NotificationStorage::bind_postal_mut(&mut self.notification, &mut self._common)
            .get_present_mut()
            .unwrap()
    }

    pub fn clear_notification(&mut self) {
        self.notification.bind(&mut self._common).clear();
    }

    // -- message-level ------------------------------------------------------

    pub fn unknown_fields(&self) -> &[u8] {
        &self._common.unknown_fields
    }

    /// Checks `LEGACY_REQUIRED` fields (`owner_id`).
    pub fn validate(&self) -> Result<(), DecodeError> {
        self.owner_id
            .validate_required(&self._common, Self::BIT_OWNER_ID, Self::FIELD_OWNER_ID)
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
        self.title.deallocate(self._common.alloc.clone());
        self.owner_id.deallocate(self._common.alloc.clone());
        self.payload.deallocate(self._common.alloc.clone());
        self.tag_ids.deallocate(self._common.alloc.clone());
        self.scores.deallocate(self._common.alloc.clone());
        self.labels.deallocate(self._common.alloc.clone());
        self.assignee.deallocate(self._common.alloc.clone());
        self.notification.bind(&mut self._common).clear();
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
        n += self.title.encoded_len(c, Self::FIELD_TITLE, Self::BIT_TITLE);
        n += self
            .score
            .encoded_len(c, Self::FIELD_SCORE, Self::BIT_UNUSED);
        n += self
            .max_retries
            .encoded_len(c, Self::FIELD_MAX_RETRIES, Self::BIT_MAX_RETRIES);
        n += self
            .owner_id
            .encoded_len(c, Self::FIELD_OWNER_ID, Self::BIT_OWNER_ID);
        n += self
            .payload
            .encoded_len(c, Self::FIELD_PAYLOAD, Self::BIT_PAYLOAD);
        n += self.tag_ids.encoded_len(Self::FIELD_TAG_IDS);
        n += self.scores.encoded_len(Self::FIELD_SCORES);
        n += self.labels.encoded_len(Self::FIELD_LABELS);
        n += self
            .status
            .encoded_len(c, Self::FIELD_STATUS, Self::BIT_UNUSED);
        n += self
            .priority
            .encoded_len(c, Self::FIELD_PRIORITY, Self::BIT_PRIORITY);
        n += self.assignee.encoded_len(Self::FIELD_ASSIGNEE);
        n += NotificationStorage::encoded_len(&self.notification);
        n + c.unknown_fields.len()
    }

    fn encode_raw<B: BufMut>(&self, buf: &mut B) {
        let c = &self._common;
        self.title
            .encode_raw(c, Self::FIELD_TITLE, Self::BIT_TITLE, buf);
        self.score
            .encode_raw(c, Self::FIELD_SCORE, Self::BIT_UNUSED, buf);
        self.max_retries
            .encode_raw(c, Self::FIELD_MAX_RETRIES, Self::BIT_MAX_RETRIES, buf);
        self.owner_id
            .encode_raw(c, Self::FIELD_OWNER_ID, Self::BIT_OWNER_ID, buf);
        self.payload
            .encode_raw(c, Self::FIELD_PAYLOAD, Self::BIT_PAYLOAD, buf);
        self.tag_ids.encode_raw(Self::FIELD_TAG_IDS, buf);
        self.scores.encode_raw(Self::FIELD_SCORES, buf);
        self.labels.encode_raw(Self::FIELD_LABELS, buf);
        self.status
            .encode_raw(c, Self::FIELD_STATUS, Self::BIT_UNUSED, buf);
        self.priority
            .encode_raw(c, Self::FIELD_PRIORITY, Self::BIT_PRIORITY, buf);
        self.assignee.encode_raw(Self::FIELD_ASSIGNEE, buf);
        NotificationStorage::encode(&self.notification, buf);
        let unknown: &[u8] = &c.unknown_fields;
        buf.put_slice(unknown);
    }
}

impl<A: Allocator + Clone> MessageDecode for Task<A> {
    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError> {
        while buf.has_remaining() {
            let (field_number, wire_type) = ::puroro::decode::decode_tag(buf)?;
            match field_number {
                Self::FIELD_TITLE => {
                    // title = 1, EXPLICIT string
                    self.title
                        .bind(&mut self._common, Self::BIT_TITLE)
                        .merge(wire_type, buf)?;
                }
                Self::FIELD_SCORE => {
                    // score = 2, IMPLICIT int32
                    self.score
                        .bind(&mut self._common, Self::BIT_UNUSED)
                        .merge(wire_type, buf)?;
                }
                Self::FIELD_MAX_RETRIES => {
                    // max_retries = 3, EXPLICIT int32
                    self.max_retries
                        .bind(&mut self._common, Self::BIT_MAX_RETRIES)
                        .merge(wire_type, buf)?;
                }
                Self::FIELD_OWNER_ID => {
                    // owner_id = 4, LEGACY_REQUIRED string
                    self.owner_id
                        .bind(&mut self._common, Self::BIT_OWNER_ID)
                        .merge(wire_type, buf)?;
                }
                Self::FIELD_PAYLOAD => {
                    // payload = 5, EXPLICIT bytes
                    self.payload
                        .bind(&mut self._common, Self::BIT_PAYLOAD)
                        .merge(wire_type, buf)?;
                }
                Self::FIELD_TAG_IDS => {
                    // tag_ids = 6, repeated int32 PACKED
                    self.tag_ids.bind(&mut self._common).merge(wire_type, buf)?;
                }
                Self::FIELD_SCORES => {
                    // scores = 7, repeated int32 EXPANDED
                    self.scores.bind(&mut self._common).merge(wire_type, buf)?;
                }
                Self::FIELD_LABELS => {
                    // labels = 8, repeated string
                    self.labels.bind(&mut self._common).merge(wire_type, buf)?;
                }
                Self::FIELD_STATUS => {
                    // status = 9, IMPLICIT open enum
                    self.status
                        .bind(&mut self._common, Self::BIT_UNUSED)
                        .merge(wire_type, buf)?;
                }
                Self::FIELD_PRIORITY => {
                    // priority = 10, EXPLICIT closed enum
                    self.priority
                        .bind(&mut self._common, Self::BIT_PRIORITY)
                        .merge_closed(Self::FIELD_PRIORITY, wire_type, buf, |v| {
                            Priority::try_from(v).is_ok()
                        })?;
                }
                Self::FIELD_ASSIGNEE => {
                    // assignee = 11, nested message
                    self.assignee.merge(&self._common, wire_type, buf)?;
                }
                notification::FIELD_EMAIL_ADDRESS => {
                    // notification.email_address = 12 (LEN variant): select the
                    // variant (freeing any other), then merge via the field's bind.
                    let f = NotificationStorage::bind_email_address_mut(
                        &mut self.notification,
                        &mut self._common,
                    );
                    f.bind_oneof(&mut self._common).merge(wire_type, buf)?;
                }
                notification::FIELD_PHONE_NUMBER => {
                    // notification.phone_number = 13 (LEN variant)
                    let f = NotificationStorage::bind_phone_number_mut(
                        &mut self.notification,
                        &mut self._common,
                    );
                    f.bind_oneof(&mut self._common).merge(wire_type, buf)?;
                }
                notification::FIELD_WEBHOOK_ID => {
                    // notification.webhook_id = 14 (varint variant)
                    let f = NotificationStorage::bind_webhook_id_mut(
                        &mut self.notification,
                        &mut self._common,
                    );
                    f.bind_oneof(&mut self._common).merge(wire_type, buf)?;
                }
                notification::FIELD_POSTAL => {
                    // notification.postal = 15 (message variant): merge into the
                    // (present) child via the nested-message field's own merge.
                    let f = NotificationStorage::bind_postal_mut(
                        &mut self.notification,
                        &mut self._common,
                    );
                    f.merge(&self._common, wire_type, buf)?;
                }
                _ => {
                    // unknown field — preserve in _common.unknown_fields
                    ::puroro::decode::skip_field_and_save(
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
