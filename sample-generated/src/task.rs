//! @generated from example.proto — do not edit
//! Message `example.Task`

mod notification;

use ::allocator_api2::alloc::{Allocator, Global};
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;
use ::bytes::{Buf, BufMut};

use ::puroro::{
    DecodeError, Explicit, HasDefault, Implicit, LegacyRequired, MessageCommon, MessageDecode,
    MessageEncode, NestedMessageField, OneofSlot, Optional, PresenceBits, ProtoBytes, ProtoEnum,
    ProtoInt32, ProtoString, RepeatedExpandedVarintField, RepeatedLenField,
    RepeatedPackedVarintField, SingularLenField, SingularVarintField, WireType,
};

use crate::address::Address;
use crate::enums::{Priority, Status};

pub use notification::Notification;

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
    notification: OneofSlot<Notification>,             // proto: oneof notification { ... }
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
    pub const FIELD_EMAIL: u32 = 12; // notification.email_address
    pub const FIELD_PHONE: u32 = 13; // notification.phone_number

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
    ) -> impl ::core::ops::DerefMut<Target = ::unmanaged::String<&'s A>> + 's {
        self._common.set_presence(Self::BIT_TITLE, true);
        self.title.value_mut(&self._common.alloc)
    }

    pub fn clear_title(&mut self) {
        self.title.clear(&mut self._common, Self::BIT_TITLE);
    }

    // -- score (IMPLICIT int32, proto field 2) ------------------------------

    pub fn score(&self) -> i32 {
        self.score.value()
    }

    pub fn score_mut(&mut self) -> &mut i32 {
        self.score.value_mut(&mut self._common, Self::BIT_UNUSED)
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
            .value_mut(&mut self._common, Self::BIT_MAX_RETRIES)
    }

    pub fn clear_max_retries(&mut self) {
        self.max_retries
            .clear(&mut self._common, Self::BIT_MAX_RETRIES);
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
    ) -> impl ::core::ops::DerefMut<Target = ::unmanaged::String<&'s A>> + 's {
        self._common.set_presence(Self::BIT_OWNER_ID, true);
        self.owner_id.value_mut(&self._common.alloc)
    }

    pub fn clear_owner_id(&mut self) {
        self.owner_id.clear(&mut self._common, Self::BIT_OWNER_ID);
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
    ) -> impl ::core::ops::DerefMut<Target = ::allocator_api2::vec::Vec<u8, &'s A>> + 's {
        self._common.set_presence(Self::BIT_PAYLOAD, true);
        self.payload.value_mut(&self._common.alloc)
    }

    pub fn clear_payload(&mut self) {
        self.payload.clear(&mut self._common, Self::BIT_PAYLOAD);
    }

    // -- tag_ids (repeated int32 PACKED, proto field 6) ----------------------

    pub fn tag_ids(&self) -> &[i32] {
        self.tag_ids.as_slice()
    }

    pub fn tag_ids_mut<'s>(
        &'s mut self,
    ) -> impl ::core::ops::DerefMut<Target = ::allocator_api2::vec::Vec<i32, &'s A>> + 's {
        self.tag_ids.values_mut(&self._common.alloc)
    }

    pub fn clear_tag_ids(&mut self) {
        self.tag_ids.clear(&self._common.alloc);
    }

    // -- scores (repeated int32 EXPANDED, proto field 7) ---------------------

    pub fn scores(&self) -> &[i32] {
        self.scores.as_slice()
    }

    pub fn scores_mut<'s>(
        &'s mut self,
    ) -> impl ::core::ops::DerefMut<Target = ::allocator_api2::vec::Vec<i32, &'s A>> + 's {
        self.scores.values_mut(&self._common.alloc)
    }

    pub fn clear_scores(&mut self) {
        self.scores.clear(&self._common.alloc);
    }

    // -- labels (repeated string, proto field 8) -----------------------------

    pub fn labels(&self) -> &[::unmanaged::UnmanagedString] {
        self.labels.as_slice()
    }

    /// Typed append helper for repeated LEN fields (the `_mut` accessor would
    /// expose allocator-less element storage, which is impractical to build).
    pub fn push_label(&mut self, v: &str) {
        self.labels.push_in(&self._common.alloc, v).ok();
    }

    pub fn clear_labels(&mut self) {
        self.labels.clear(&self._common.alloc);
    }

    // -- status (IMPLICIT open enum, proto field 9) -------------------------

    pub fn status_raw(&self) -> i32 {
        self.status.value()
    }

    pub fn status_mut(&mut self) -> &mut i32 {
        self.status.value_mut(&mut self._common, Self::BIT_UNUSED)
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
            .value_mut(&mut self._common, Self::BIT_PRIORITY)
    }

    pub fn clear_priority(&mut self) {
        self.priority.clear(&mut self._common, Self::BIT_PRIORITY);
    }

    // -- assignee (nested message, proto field 11) --------------------------

    pub fn assignee(&self) -> Option<&Address<A>> {
        self.assignee.get()
    }

    pub fn assignee_mut(&mut self) -> &mut Address<A> {
        self.assignee.get_mut(&self._common)
    }

    pub fn clear_assignee(&mut self) {
        self.assignee.clear(&self._common.alloc);
    }

    // -- oneof notification (proto fields 12 / 13) --------------------------

    pub fn notification(&self) -> Option<&Notification> {
        self.notification.get()
    }

    pub fn set_email_address(&mut self, v: &str) {
        if let Some(old) = self.notification.take() {
            // SAFETY: the message allocator owns the previous variant's buffer.
            unsafe { old.deallocate(&self._common.alloc) };
        }
        let s = ::puroro::decode::str_to_unmanaged_in(v, &self._common.alloc);
        self.notification.set(Some(Notification::EmailAddress(s)));
    }

    pub fn set_phone_number(&mut self, v: &str) {
        if let Some(old) = self.notification.take() {
            // SAFETY: the message allocator owns the previous variant's buffer.
            unsafe { old.deallocate(&self._common.alloc) };
        }
        let s = ::puroro::decode::str_to_unmanaged_in(v, &self._common.alloc);
        self.notification.set(Some(Notification::PhoneNumber(s)));
    }

    pub fn clear_notification(&mut self) {
        if let Some(old) = self.notification.take() {
            // SAFETY: the message allocator owns the active variant's buffer.
            unsafe { old.deallocate(&self._common.alloc) };
        }
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
        self.title.deallocate(&self._common.alloc);
        self.owner_id.deallocate(&self._common.alloc);
        self.payload.deallocate(&self._common.alloc);
        self.tag_ids.deallocate(&self._common.alloc);
        self.scores.deallocate(&self._common.alloc);
        self.labels.deallocate(&self._common.alloc);
        self.assignee.deallocate(&self._common.alloc);
        if let Some(n) = self.notification.take() {
            // SAFETY: the message allocator owns the active variant's buffer.
            unsafe { n.deallocate(&self._common.alloc) };
        }
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
        n += encoded_len_notification::<A>(&self.notification);
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
        encode_notification::<A, B>(&self.notification, buf);
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
                        .merge(&mut self._common, Self::BIT_TITLE, wire_type, buf)?;
                }
                Self::FIELD_SCORE => {
                    // score = 2, IMPLICIT int32
                    self.score
                        .merge(&mut self._common, Self::BIT_UNUSED, wire_type, buf)?;
                }
                Self::FIELD_MAX_RETRIES => {
                    // max_retries = 3, EXPLICIT int32
                    self.max_retries.merge(
                        &mut self._common,
                        Self::BIT_MAX_RETRIES,
                        wire_type,
                        buf,
                    )?;
                }
                Self::FIELD_OWNER_ID => {
                    // owner_id = 4, LEGACY_REQUIRED string
                    self.owner_id
                        .merge(&mut self._common, Self::BIT_OWNER_ID, wire_type, buf)?;
                }
                Self::FIELD_PAYLOAD => {
                    // payload = 5, EXPLICIT bytes
                    self.payload
                        .merge(&mut self._common, Self::BIT_PAYLOAD, wire_type, buf)?;
                }
                Self::FIELD_TAG_IDS => {
                    // tag_ids = 6, repeated int32 PACKED
                    self.tag_ids.merge(&self._common.alloc, wire_type, buf)?;
                }
                Self::FIELD_SCORES => {
                    // scores = 7, repeated int32 EXPANDED
                    self.scores.merge(&self._common.alloc, wire_type, buf)?;
                }
                Self::FIELD_LABELS => {
                    // labels = 8, repeated string
                    self.labels.merge(&self._common.alloc, wire_type, buf)?;
                }
                Self::FIELD_STATUS => {
                    // status = 9, IMPLICIT open enum
                    self.status
                        .merge(&mut self._common, Self::BIT_UNUSED, wire_type, buf)?;
                }
                Self::FIELD_PRIORITY => {
                    // priority = 10, EXPLICIT closed enum
                    self.priority.merge_closed(
                        &mut self._common,
                        Self::FIELD_PRIORITY,
                        Self::BIT_PRIORITY,
                        wire_type,
                        buf,
                        |v| Priority::try_from(v).is_ok(),
                    )?;
                }
                Self::FIELD_ASSIGNEE => {
                    // assignee = 11, nested message
                    self.assignee.merge(&self._common, wire_type, buf)?;
                }
                Self::FIELD_EMAIL => {
                    // notification.email_address = 12
                    merge_notification_email(&mut self.notification, &self._common, wire_type, buf)?;
                }
                Self::FIELD_PHONE => {
                    // notification.phone_number = 13
                    merge_notification_phone(&mut self.notification, &self._common, wire_type, buf)?;
                }
                _ => {
                    // unknown field — preserve in _common.unknown_fields
                    ::puroro::decode::skip_field_and_save(
                        field_number,
                        wire_type,
                        buf,
                        &mut self._common.unknown_fields,
                        &self._common.alloc,
                    )?
                }
            }
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Oneof helpers (generated per message today)
// ---------------------------------------------------------------------------

fn encoded_len_notification<A: Allocator + Clone>(slot: &OneofSlot<Notification>) -> usize {
    match slot.get() {
        Some(Notification::EmailAddress(s)) => {
            ::puroro::encode::encoded_len_len_field(Task::<A>::FIELD_EMAIL, s.len())
        }
        Some(Notification::PhoneNumber(s)) => {
            ::puroro::encode::encoded_len_len_field(Task::<A>::FIELD_PHONE, s.len())
        }
        None => 0,
    }
}

fn encode_notification<A: Allocator + Clone, B: BufMut>(
    slot: &OneofSlot<Notification>,
    buf: &mut B,
) {
    match slot.get() {
        Some(Notification::EmailAddress(s)) => {
            ::puroro::encode::encode_len_field(Task::<A>::FIELD_EMAIL, s.as_bytes(), buf);
        }
        Some(Notification::PhoneNumber(s)) => {
            ::puroro::encode::encode_len_field(Task::<A>::FIELD_PHONE, s.as_bytes(), buf);
        }
        None => {}
    }
}

fn merge_notification_email<A: Allocator + Clone, B: Buf>(
    slot: &mut OneofSlot<Notification>,
    common: &MessageCommon<TaskPresence, A>,
    wire_type: WireType,
    buf: &mut B,
) -> Result<(), DecodeError> {
    if wire_type != WireType::Len {
        return Err(DecodeError::InvalidTag);
    }
    let s = ::puroro::decode::decode_string_in(buf, &common.alloc)?;
    if let Some(old) = slot.take() {
        // SAFETY: `common.alloc` owns the previous variant's buffer.
        unsafe { old.deallocate(&common.alloc) };
    }
    slot.set(Some(Notification::EmailAddress(s)));
    Ok(())
}

fn merge_notification_phone<A: Allocator + Clone, B: Buf>(
    slot: &mut OneofSlot<Notification>,
    common: &MessageCommon<TaskPresence, A>,
    wire_type: WireType,
    buf: &mut B,
) -> Result<(), DecodeError> {
    if wire_type != WireType::Len {
        return Err(DecodeError::InvalidTag);
    }
    let s = ::puroro::decode::decode_string_in(buf, &common.alloc)?;
    if let Some(old) = slot.take() {
        // SAFETY: `common.alloc` owns the previous variant's buffer.
        unsafe { old.deallocate(&common.alloc) };
    }
    slot.set(Some(Notification::PhoneNumber(s)));
    Ok(())
}
