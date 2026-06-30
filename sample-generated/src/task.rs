//! @generated from example.proto — do not edit
//! Message `example.Task`

mod notification;

use ::allocator_api2::alloc::{Allocator, Global};
use ::allocator_api2::boxed::Box as ABox;
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
// Field constants
// ---------------------------------------------------------------------------

const FIELD_TITLE: u32 = 1;       // title
const FIELD_SCORE: u32 = 2;       // score
const FIELD_MAX_RETRIES: u32 = 3; // max_retries
const FIELD_OWNER_ID: u32 = 4;    // owner_id
const FIELD_PAYLOAD: u32 = 5;     // payload
const FIELD_TAG_IDS: u32 = 6;     // tag_ids
const FIELD_SCORES: u32 = 7;      // scores
const FIELD_LABELS: u32 = 8;      // labels
const FIELD_STATUS: u32 = 9;      // status
const FIELD_PRIORITY: u32 = 10;   // priority
const FIELD_ASSIGNEE: u32 = 11;    // assignee
const FIELD_EMAIL: u32 = 12;      // notification.email_address
const FIELD_PHONE: u32 = 13;      // notification.phone_number

const BIT_TITLE: usize = 0;       // title (EXPLICIT)
const BIT_MAX_RETRIES: usize = 1; // max_retries (EXPLICIT)
const BIT_OWNER_ID: usize = 2;    // owner_id (LEGACY_REQUIRED)
const BIT_PAYLOAD: usize = 3;     // payload (EXPLICIT)
const BIT_PRIORITY: usize = 4;    // priority (EXPLICIT)

// IMPLICIT / nested / repeated fields use a dummy bit index (ignored by `Implicit`).
const BIT_UNUSED: usize = 0;

// ---------------------------------------------------------------------------
// Message struct
// ---------------------------------------------------------------------------

/// Reference `Task` message from `DESIGN.md`.
pub struct Task<A: Allocator = Global> {
    _common: MessageCommon<TaskPresence, A>,
    title: SingularLenField<ProtoString, Explicit, A>,              // proto: string title = 1;
    score: SingularVarintField<ProtoInt32, Implicit>,               // proto: int32 score = 2;
    max_retries: SingularVarintField<ProtoInt32, Explicit>,         // proto: int32 max_retries = 3;
    owner_id: SingularLenField<ProtoString, LegacyRequired, A>,     // proto: string owner_id = 4;
    payload: SingularLenField<ProtoBytes, Explicit, A>,             // proto: bytes payload = 5;
    tag_ids: RepeatedPackedVarintField<ProtoInt32, A>,              // proto: repeated int32 tag_ids = 6 [packed];
    scores: RepeatedExpandedVarintField<ProtoInt32, A>,             // proto: repeated int32 scores = 7;
    labels: RepeatedLenField<ProtoString, A>,                       // proto: repeated string labels = 8;
    status: SingularVarintField<ProtoEnum, Implicit>,               // proto: Status status = 9;
    priority: SingularVarintField<ProtoEnum, Explicit>,            // proto: Priority priority = 10;
    assignee: NestedMessageField<Address<A>, A>,                    // proto: Address assignee = 11;
    notification: OneofSlot<Notification<A>>,                     // proto: oneof notification { ... }
}

impl<A: Allocator + Clone> Task<A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            _common: MessageCommon::new_in(TaskPresence::ZERO, alloc.clone()),
            title: SingularLenField::new_in(alloc.clone()),
            score: SingularVarintField::new(),
            max_retries: SingularVarintField::new(),
            owner_id: SingularLenField::new_in(alloc.clone()),
            payload: SingularLenField::new_in(alloc.clone()),
            tag_ids: RepeatedPackedVarintField::new_in(alloc.clone()),
            scores: RepeatedExpandedVarintField::new_in(alloc.clone()),
            labels: RepeatedLenField::new_in(alloc.clone()),
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
        self.title.optional::<_, _, BIT_TITLE>(&self._common, TitleDefault)
    }

    pub fn has_title(&self) -> bool {
        self.title.has::<_, BIT_TITLE>(&self._common)
    }

    pub fn set_title(&mut self, v: &str) {
        self.title.set_str::<_, BIT_TITLE>(&mut self._common, v);
    }

    pub fn clear_title(&mut self) {
        self.title.clear::<_, BIT_TITLE>(&mut self._common);
    }

    // -- score (IMPLICIT int32, proto field 2) ------------------------------

    pub fn score(&self) -> i32 {
        self.score.value()
    }

    pub fn set_score(&mut self, v: i32) {
        self.score.set::<_, _, BIT_UNUSED>(&mut self._common, v);
    }

    // -- max_retries (EXPLICIT int32, default = 3, proto field 3) ------------

    pub fn max_retries(&self) -> Optional<i32, impl HasDefault<i32>> {
        struct MaxRetriesDefault;
        impl HasDefault<i32> for MaxRetriesDefault {
            const DEFAULT: i32 = 3;
        }
        self.max_retries
            .optional::<_, _, MaxRetriesDefault, BIT_MAX_RETRIES>(&self._common, MaxRetriesDefault)
    }

    pub fn has_max_retries(&self) -> bool {
        self.max_retries.has::<_, _, BIT_MAX_RETRIES>(&self._common)
    }

    pub fn set_max_retries(&mut self, v: i32) {
        self.max_retries
            .set::<_, _, BIT_MAX_RETRIES>(&mut self._common, v);
    }

    pub fn clear_max_retries(&mut self) {
        self.max_retries
            .clear::<_, _, BIT_MAX_RETRIES>(&mut self._common);
    }

    // -- owner_id (LEGACY_REQUIRED string, proto field 4) --------------------

    pub fn owner_id<'a>(&'a self) -> Optional<&'a str, impl HasDefault<&'a str>> {
        struct OwnerIdDefault;
        impl<'a> HasDefault<&'a str> for OwnerIdDefault {
            const DEFAULT: &'a str = "";
        }
        self.owner_id
            .optional::<_, _, BIT_OWNER_ID>(&self._common, OwnerIdDefault)
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.has::<_, BIT_OWNER_ID>(&self._common)
    }

    pub fn set_owner_id(&mut self, v: &str) {
        self.owner_id.set_str::<_, BIT_OWNER_ID>(&mut self._common, v);
    }

    pub fn clear_owner_id(&mut self) {
        self.owner_id.clear::<_, BIT_OWNER_ID>(&mut self._common);
    }

    // -- payload (EXPLICIT bytes, proto field 5) -----------------------------

    pub fn payload<'a>(&'a self) -> Optional<&'a [u8], impl HasDefault<&'a [u8]>> {
        struct PayloadDefault;
        impl<'a> HasDefault<&'a [u8]> for PayloadDefault {
            const DEFAULT: &'a [u8] = &[];
        }
        self.payload
            .optional::<_, _, BIT_PAYLOAD>(&self._common, PayloadDefault)
    }

    pub fn has_payload(&self) -> bool {
        self.payload.has::<_, BIT_PAYLOAD>(&self._common)
    }

    pub fn set_payload(&mut self, v: &[u8]) -> Result<(), DecodeError> {
        self.payload
            .set_from_slice::<_, BIT_PAYLOAD>(&mut self._common, v)
    }

    pub fn clear_payload(&mut self) {
        self.payload.clear::<_, BIT_PAYLOAD>(&mut self._common);
    }

    // -- tag_ids (repeated int32 PACKED, proto field 6) ----------------------

    pub fn tag_ids(&self) -> &[i32] {
        self.tag_ids.as_slice()
    }

    pub fn push_tag_id(&mut self, v: i32) {
        self.tag_ids.push(v);
    }

    pub fn clear_tag_ids(&mut self) {
        self.tag_ids.clear();
    }

    // -- scores (repeated int32 EXPANDED, proto field 7) ---------------------

    pub fn scores(&self) -> &[i32] {
        self.scores.as_slice()
    }

    pub fn push_score(&mut self, v: i32) {
        self.scores.push(v);
    }

    pub fn clear_scores(&mut self) {
        self.scores.clear();
    }

    // -- labels (repeated string, proto field 8) -----------------------------

    pub fn labels(&self) -> &[ABox<str, A>] {
        self.labels.as_slice()
    }

    pub fn push_label(&mut self, v: &str) {
        self.labels.push_str(&self._common, v);
    }

    pub fn clear_labels(&mut self) {
        self.labels.clear();
    }

    // -- status (IMPLICIT open enum, proto field 9) -------------------------

    pub fn status_raw(&self) -> i32 {
        self.status.value()
    }

    pub fn set_status_raw(&mut self, v: i32) {
        self.status.set::<_, _, BIT_UNUSED>(&mut self._common, v);
    }

    pub fn status(&self) -> Result<Status, i32> {
        Status::try_from(self.status.value())
    }

    pub fn set_status(&mut self, v: Status) {
        self.set_status_raw(v.into());
    }

    // -- priority (EXPLICIT closed enum, proto field 10) ---------------------

    pub fn priority(&self) -> Option<Result<Priority, i32>> {
        if !self.priority.has::<_, _, BIT_PRIORITY>(&self._common) {
            return None;
        }
        Some(Priority::try_from(self.priority.value()))
    }

    pub fn set_priority(&mut self, v: Priority) {
        self.priority
            .set::<_, _, BIT_PRIORITY>(&mut self._common, v.into());
    }

    pub fn clear_priority(&mut self) {
        self.priority
            .clear::<_, _, BIT_PRIORITY>(&mut self._common);
    }

    // -- assignee (nested message, proto field 11) --------------------------

    pub fn assignee(&self) -> Option<&Address<A>> {
        self.assignee.get()
    }

    pub fn assignee_mut(&mut self) -> &mut Address<A> {
        self.assignee.get_mut(&self._common)
    }

    pub fn set_assignee(&mut self, v: Address<A>) {
        self.assignee.set_child(&self._common, v);
    }

    pub fn clear_assignee(&mut self) {
        self.assignee.clear();
    }

    // -- oneof notification (proto fields 12 / 13) --------------------------

    pub fn notification(&self) -> Option<&Notification<A>> {
        self.notification.get()
    }

    pub fn notification_mut(&mut self) -> Option<&mut Notification<A>> {
        self.notification.get_mut()
    }

    pub fn set_notification(&mut self, v: Option<Notification<A>>) {
        self.notification.set(v);
    }

    pub fn set_email_address(&mut self, v: &str) {
        self.notification.set(Some(Notification::EmailAddress(
            ::puroro::decode::str_to_box_in(v, self._common.alloc.clone()),
        )));
    }

    pub fn set_phone_number(&mut self, v: &str) {
        self.notification.set(Some(Notification::PhoneNumber(
            ::puroro::decode::str_to_box_in(v, self._common.alloc.clone()),
        )));
    }

    pub fn clear_notification(&mut self) {
        self.notification.clear();
    }

    // -- message-level ------------------------------------------------------

    pub fn unknown_fields(&self) -> &[u8] {
        &self._common.unknown_fields
    }

    /// Checks `LEGACY_REQUIRED` fields (`owner_id`).
    pub fn validate(&self) -> Result<(), DecodeError> {
        self.owner_id
            .validate_required::<_, BIT_OWNER_ID>(&self._common, FIELD_OWNER_ID)
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
// MessageEncode / MessageDecode
// ---------------------------------------------------------------------------

impl<A: Allocator + Clone> MessageEncode for Task<A> {
    fn encoded_len(&self) -> usize {
        let c = &self._common;
        let mut n = 0usize;
        n += self.title.encoded_len::<_, FIELD_TITLE, BIT_TITLE>(c);
        n += self
            .score
            .encoded_len::<_, _, FIELD_SCORE, BIT_UNUSED>(c);
        n += self
            .max_retries
            .encoded_len::<_, _, FIELD_MAX_RETRIES, BIT_MAX_RETRIES>(c);
        n += self
            .owner_id
            .encoded_len::<_, FIELD_OWNER_ID, BIT_OWNER_ID>(c);
        n += self
            .payload
            .encoded_len::<_, FIELD_PAYLOAD, BIT_PAYLOAD>(c);
        n += self.tag_ids.encoded_len::<FIELD_TAG_IDS>();
        n += self.scores.encoded_len::<FIELD_SCORES>();
        n += self.labels.encoded_len::<FIELD_LABELS>();
        n += self
            .status
            .encoded_len::<_, _, FIELD_STATUS, BIT_UNUSED>(c);
        n += self
            .priority
            .encoded_len::<_, _, FIELD_PRIORITY, BIT_PRIORITY>(c);
        n += self.assignee.encoded_len::<FIELD_ASSIGNEE>();
        n += encoded_len_notification(&self.notification);
        n + c.unknown_fields.len()
    }

    fn encode_raw<B: BufMut>(&self, buf: &mut B) {
        let c = &self._common;
        self.title
            .encode_raw::<_, _, FIELD_TITLE, BIT_TITLE>(c, buf);
        self.score
            .encode_raw::<_, _, _, FIELD_SCORE, BIT_UNUSED>(c, buf);
        self.max_retries
            .encode_raw::<_, _, _, FIELD_MAX_RETRIES, BIT_MAX_RETRIES>(c, buf);
        self.owner_id
            .encode_raw::<_, _, FIELD_OWNER_ID, BIT_OWNER_ID>(c, buf);
        self.payload
            .encode_raw::<_, _, FIELD_PAYLOAD, BIT_PAYLOAD>(c, buf);
        self.tag_ids.encode_raw::<_, FIELD_TAG_IDS>(buf);
        self.scores.encode_raw::<_, FIELD_SCORES>(buf);
        self.labels.encode_raw::<_, FIELD_LABELS>(buf);
        self.status
            .encode_raw::<_, _, _, FIELD_STATUS, BIT_UNUSED>(c, buf);
        self.priority
            .encode_raw::<_, _, _, FIELD_PRIORITY, BIT_PRIORITY>(c, buf);
        self.assignee.encode_raw::<_, FIELD_ASSIGNEE>(buf);
        encode_notification(&self.notification, buf);
        buf.put_slice(&c.unknown_fields);
    }
}

impl<A: Allocator + Clone> MessageDecode for Task<A> {
    fn merge_from<B: Buf>(&mut self, buf: &mut B) -> Result<(), DecodeError> {
        while buf.has_remaining() {
            let (field_number, wire_type) = ::puroro::decode::decode_tag(buf)?;
            match field_number {
                FIELD_TITLE => { // title = 1, EXPLICIT string
                    self.title
                        .merge::<_, _, BIT_TITLE>(&mut self._common, wire_type, buf)?;
                }
                FIELD_SCORE => { // score = 2, IMPLICIT int32
                    self.score.merge::<_, _, _, BIT_UNUSED>(
                        &mut self._common,
                        wire_type,
                        buf,
                    )?;
                }
                FIELD_MAX_RETRIES => { // max_retries = 3, EXPLICIT int32
                    self.max_retries.merge::<_, _, _, BIT_MAX_RETRIES>(
                        &mut self._common,
                        wire_type,
                        buf,
                    )?;
                }
                FIELD_OWNER_ID => { // owner_id = 4, LEGACY_REQUIRED string
                    self.owner_id
                        .merge::<_, _, BIT_OWNER_ID>(&mut self._common, wire_type, buf)?;
                }
                FIELD_PAYLOAD => { // payload = 5, EXPLICIT bytes
                    self.payload
                        .merge::<_, _, BIT_PAYLOAD>(&mut self._common, wire_type, buf)?;
                }
                FIELD_TAG_IDS => { // tag_ids = 6, repeated int32 PACKED
                    self.tag_ids.merge(wire_type, buf)?;
                }
                FIELD_SCORES => { // scores = 7, repeated int32 EXPANDED
                    self.scores.merge(wire_type, buf)?;
                }
                FIELD_LABELS => { // labels = 8, repeated string
                    self.labels
                        .merge(&self._common, wire_type, buf)?;
                }
                FIELD_STATUS => { // status = 9, IMPLICIT open enum
                    self.status.merge::<_, _, _, BIT_UNUSED>(
                        &mut self._common,
                        wire_type,
                        buf,
                    )?;
                }
                FIELD_PRIORITY => { // priority = 10, EXPLICIT closed enum
                    self.priority.merge_closed::<_, _, _, FIELD_PRIORITY, BIT_PRIORITY>(
                        &mut self._common,
                        wire_type,
                        buf,
                        |v| Priority::try_from(v).is_ok(),
                    )?;
                }
                FIELD_ASSIGNEE => { // assignee = 11, nested message
                    self.assignee
                        .merge(&mut self._common, wire_type, buf)?;
                }
                FIELD_EMAIL => { // notification.email_address = 12
                    merge_notification_email(
                        &mut self.notification,
                        &self._common,
                        wire_type,
                        buf,
                    )?;
                }
                FIELD_PHONE => { // notification.phone_number = 13
                    merge_notification_phone(
                        &mut self.notification,
                        &self._common,
                        wire_type,
                        buf,
                    )?;
                }
                _ => { // unknown field — preserve in _common.unknown_fields
                    ::puroro::decode::skip_field_and_save(
                        field_number,
                        wire_type,
                        buf,
                        &mut self._common.unknown_fields,
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

fn encoded_len_notification<A: Allocator>(slot: &OneofSlot<Notification<A>>) -> usize {
    match slot.get() {
        Some(Notification::EmailAddress(s)) => {
            ::puroro::encode::encoded_len_len_field(FIELD_EMAIL, s.len())
        }
        Some(Notification::PhoneNumber(s)) => {
            ::puroro::encode::encoded_len_len_field(FIELD_PHONE, s.len())
        }
        None => 0,
    }
}

fn encode_notification<A: Allocator, B: BufMut>(
    slot: &OneofSlot<Notification<A>>,
    buf: &mut B,
) {
    match slot.get() {
        Some(Notification::EmailAddress(s)) => {
            ::puroro::encode::encode_len_field(FIELD_EMAIL, s.as_bytes(), buf);
        }
        Some(Notification::PhoneNumber(s)) => {
            ::puroro::encode::encode_len_field(FIELD_PHONE, s.as_bytes(), buf);
        }
        None => {}
    }
}

fn merge_notification_email<A: Allocator + Clone, B: Buf>(
    slot: &mut OneofSlot<Notification<A>>,
    common: &MessageCommon<TaskPresence, A>,
    wire_type: WireType,
    buf: &mut B,
) -> Result<(), DecodeError> {
    if wire_type != WireType::Len {
        return Err(DecodeError::InvalidTag);
    }
    let s = ::puroro::decode::decode_string_in(buf, common.alloc.clone())?;
    slot.set(Some(Notification::EmailAddress(s)));
    Ok(())
}

fn merge_notification_phone<A: Allocator + Clone, B: Buf>(
    slot: &mut OneofSlot<Notification<A>>,
    common: &MessageCommon<TaskPresence, A>,
    wire_type: WireType,
    buf: &mut B,
) -> Result<(), DecodeError> {
    if wire_type != WireType::Len {
        return Err(DecodeError::InvalidTag);
    }
    let s = ::puroro::decode::decode_string_in(buf, common.alloc.clone())?;
    slot.set(Some(Notification::PhoneNumber(s)));
    Ok(())
}
