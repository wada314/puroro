//! Generated `Task` message (`example.Task`).

mod notification;

use ::allocator_api2::alloc::{Allocator, Global};
use ::allocator_api2::boxed::Box as ABox;
use ::allocator_api2::vec::Vec as AVec;
use ::bitvec::array::BitArray;
use ::bitvec::order::Lsb0;
use ::bytes::{Buf, BufMut};

use ::puroro::{
    DecodeError, ExplicitBytes, ExplicitEnum, ExplicitInt32, ExplicitString, ExplicitVarintField,
    HasDefault, ImplicitEnum, ImplicitInt32, MessageCommon, MessageDecode, MessageEncode,
    NestedMessageField, OneofSlot, Optional, PresenceBits, ProtoEnum, ProtoInt32,
    VarintProtoType, WireType,
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

const FIELD_TITLE: u32 = 1;
const FIELD_SCORE: u32 = 2;
const FIELD_MAX_RETRIES: u32 = 3;
const FIELD_OWNER_ID: u32 = 4;
const FIELD_PAYLOAD: u32 = 5;
const FIELD_TAG_IDS: u32 = 6;
const FIELD_SCORES: u32 = 7;
const FIELD_LABELS: u32 = 8;
const FIELD_STATUS: u32 = 9;
const FIELD_PRIORITY: u32 = 10;
const FIELD_ASSIGNEE: u32 = 11;
const FIELD_EMAIL: u32 = 12;
const FIELD_PHONE: u32 = 13;

const BIT_TITLE: usize = 0;
const BIT_MAX_RETRIES: usize = 1;
const BIT_OWNER_ID: usize = 2;
const BIT_PAYLOAD: usize = 3;
const BIT_PRIORITY: usize = 4;

// IMPLICIT / nested / repeated fields use a dummy bit index (ignored by `Implicit`).
const BIT_UNUSED: usize = 0;

// ---------------------------------------------------------------------------
// Message struct
// ---------------------------------------------------------------------------

/// Reference `Task` message from `DESIGN.md`.
pub struct Task<A: Allocator = Global> {
    _common: MessageCommon<TaskPresence, A>,
    title: ExplicitString<A>,
    score: ImplicitInt32,
    max_retries: ExplicitInt32,
    owner_id: ExplicitString<A>,
    payload: ExplicitBytes<A>,
    tag_ids: AVec<i32, A>,
    scores: AVec<i32, A>,
    labels: AVec<ABox<str, A>, A>,
    status: ImplicitEnum,
    priority: ExplicitEnum,
    assignee: NestedMessageField<Address<A>, A>,
    notification: OneofSlot<Notification<A>>,
}

impl<A: Allocator + Clone> Task<A> {
    pub fn new_in(alloc: A) -> Self {
        Self {
            _common: MessageCommon::new_in(TaskPresence::ZERO, alloc.clone()),
            title: ExplicitString::new_in(alloc.clone()),
            score: ImplicitInt32::new(),
            max_retries: ExplicitInt32::new(),
            owner_id: ExplicitString::new_in(alloc.clone()),
            payload: ExplicitBytes::new_in(alloc.clone()),
            tag_ids: AVec::new_in(alloc.clone()),
            scores: AVec::new_in(alloc.clone()),
            labels: AVec::new_in(alloc.clone()),
            status: ImplicitEnum::new(),
            priority: ExplicitEnum::new(),
            assignee: NestedMessageField::new(),
            notification: OneofSlot::new(),
        }
    }

    // -- title (EXPLICIT string) --------------------------------------------

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

    // -- score (IMPLICIT int32) ---------------------------------------------

    pub fn score(&self) -> i32 {
        self.score.value()
    }

    pub fn set_score(&mut self, v: i32) {
        self.score.set::<_, _, BIT_UNUSED>(&mut self._common, v);
    }

    // -- max_retries (EXPLICIT int32, default = 3) --------------------------

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

    // -- owner_id (LEGACY_REQUIRED string) ----------------------------------

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

    // -- payload (EXPLICIT bytes) -------------------------------------------

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

    // -- repeated fields (inline until catalog wrappers land) ---------------

    pub fn tag_ids(&self) -> &[i32] {
        &self.tag_ids
    }

    pub fn push_tag_id(&mut self, v: i32) {
        self.tag_ids.push(v);
    }

    pub fn clear_tag_ids(&mut self) {
        self.tag_ids.clear();
    }

    pub fn scores(&self) -> &[i32] {
        &self.scores
    }

    pub fn push_score(&mut self, v: i32) {
        self.scores.push(v);
    }

    pub fn clear_scores(&mut self) {
        self.scores.clear();
    }

    pub fn labels(&self) -> &[ABox<str, A>] {
        &self.labels
    }

    pub fn push_label(&mut self, v: &str) {
        self.labels
            .push(::puroro::decode::str_to_box_in(v, self._common.alloc.clone()));
    }

    pub fn clear_labels(&mut self) {
        self.labels.clear();
    }

    // -- status (IMPLICIT open enum) ----------------------------------------

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

    // -- priority (EXPLICIT closed enum) ------------------------------------

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

    // -- assignee (nested message) ------------------------------------------

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

    // -- oneof notification -------------------------------------------------

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
        if !self.owner_id.has::<_, BIT_OWNER_ID>(&self._common) {
            return Err(DecodeError::MissingRequiredField {
                field_number: FIELD_OWNER_ID,
            });
        }
        Ok(())
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
        n += encoded_len_tag_ids(&self.tag_ids);
        n += encoded_len_scores(&self.scores);
        n += encoded_len_labels(&self.labels);
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
        encode_tag_ids(&self.tag_ids, buf);
        encode_scores(&self.scores, buf);
        encode_labels(&self.labels, buf);
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
                FIELD_TITLE => {
                    self.title
                        .merge::<_, _, BIT_TITLE>(&mut self._common, wire_type, buf)?;
                }
                FIELD_SCORE => {
                    self.score.merge::<_, _, _, BIT_UNUSED>(
                        &mut self._common,
                        wire_type,
                        buf,
                    )?;
                }
                FIELD_MAX_RETRIES => {
                    self.max_retries.merge::<_, _, _, BIT_MAX_RETRIES>(
                        &mut self._common,
                        wire_type,
                        buf,
                    )?;
                }
                FIELD_OWNER_ID => {
                    self.owner_id
                        .merge::<_, _, BIT_OWNER_ID>(&mut self._common, wire_type, buf)?;
                }
                FIELD_PAYLOAD => {
                    self.payload
                        .merge::<_, _, BIT_PAYLOAD>(&mut self._common, wire_type, buf)?;
                }
                FIELD_TAG_IDS => {
                    merge_repeated_i32(&mut self.tag_ids, wire_type, buf)?;
                }
                FIELD_SCORES => {
                    merge_repeated_i32(&mut self.scores, wire_type, buf)?;
                }
                FIELD_LABELS => {
                    merge_repeated_label(&mut self.labels, &self._common, wire_type, buf)?;
                }
                FIELD_STATUS => {
                    self.status.merge::<_, _, _, BIT_UNUSED>(
                        &mut self._common,
                        wire_type,
                        buf,
                    )?;
                }
                FIELD_PRIORITY => {
                    merge_priority_closed(
                        &mut self.priority,
                        &mut self._common,
                        wire_type,
                        buf,
                    )?;
                }
                FIELD_ASSIGNEE => {
                    self.assignee
                        .merge(&mut self._common, wire_type, buf)?;
                }
                FIELD_EMAIL => {
                    merge_notification_email(
                        &mut self.notification,
                        &self._common,
                        wire_type,
                        buf,
                    )?;
                }
                FIELD_PHONE => {
                    merge_notification_phone(
                        &mut self.notification,
                        &self._common,
                        wire_type,
                        buf,
                    )?;
                }
                _ => ::puroro::decode::skip_field_and_save(
                    field_number,
                    wire_type,
                    buf,
                    &mut self._common.unknown_fields,
                )?,
            }
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Repeated / oneof / closed-enum helpers (generated per message today)
// ---------------------------------------------------------------------------

fn encoded_len_tag_ids(tag_ids: &[i32]) -> usize {
    ::puroro::encode::encoded_len_packed_varint_field(
        FIELD_TAG_IDS,
        tag_ids,
        |v| ProtoInt32::encode_wire(*v),
    )
}

fn encode_tag_ids<B: BufMut>(tag_ids: &[i32], buf: &mut B) {
    ::puroro::encode::encode_packed_varint_field(
        FIELD_TAG_IDS,
        tag_ids,
        |v| ProtoInt32::encode_wire(*v),
        buf,
    );
}

fn encoded_len_scores(scores: &[i32]) -> usize {
    scores
        .iter()
        .map(|&v| {
            ::puroro::encode::encoded_len_varint_field(
                FIELD_SCORES,
                ProtoInt32::encode_wire(v),
            )
        })
        .sum()
}

fn encode_scores<B: BufMut>(scores: &[i32], buf: &mut B) {
    for &v in scores {
        ::puroro::encode::encode_varint_field(
            FIELD_SCORES,
            ProtoInt32::encode_wire(v),
            buf,
        );
    }
}

fn encoded_len_labels<A: Allocator>(labels: &[ABox<str, A>]) -> usize {
    labels
        .iter()
        .map(|s| ::puroro::encode::encoded_len_len_field(FIELD_LABELS, s.len()))
        .sum()
}

fn encode_labels<A: Allocator, B: BufMut>(labels: &[ABox<str, A>], buf: &mut B) {
    for s in labels {
        ::puroro::encode::encode_len_field(FIELD_LABELS, s.as_bytes(), buf);
    }
}

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

fn merge_repeated_i32<B: Buf>(vec: &mut AVec<i32, impl Allocator>, wire_type: WireType, buf: &mut B) -> Result<(), DecodeError> {
    match wire_type {
        WireType::Len => {
            let len = ::puroro::decode::decode_varint(buf)? as usize;
            if buf.remaining() < len {
                return Err(DecodeError::TruncatedMessage);
            }
            let mut sub = buf.take(len);
            while sub.has_remaining() {
                let raw = ::puroro::decode::decode_varint(&mut sub)?;
                vec.push(ProtoInt32::decode_wire(raw)?);
            }
        }
        WireType::Varint => {
            let raw = ::puroro::decode::decode_varint(buf)?;
            vec.push(ProtoInt32::decode_wire(raw)?);
        }
        _ => return Err(DecodeError::InvalidTag),
    }
    Ok(())
}

fn merge_repeated_label<A: Allocator + Clone, B: Buf>(
    vec: &mut AVec<ABox<str, A>, A>,
    common: &MessageCommon<TaskPresence, A>,
    wire_type: WireType,
    buf: &mut B,
) -> Result<(), DecodeError> {
    if wire_type != WireType::Len {
        return Err(DecodeError::InvalidTag);
    }
    let s = ::puroro::decode::decode_string_in(buf, common.alloc.clone())?;
    vec.push(s);
    Ok(())
}

fn merge_priority_closed<A: Allocator + Clone, B: Buf>(
    field: &mut ExplicitVarintField<ProtoEnum>,
    common: &mut MessageCommon<TaskPresence, A>,
    wire_type: WireType,
    buf: &mut B,
) -> Result<(), DecodeError> {
    if wire_type != WireType::Varint {
        return Err(DecodeError::InvalidTag);
    }
    let raw = ::puroro::decode::decode_varint(buf)?;
    let value = raw as i32;
    if Priority::try_from(value).is_err() {
        ::puroro::decode::save_unknown_varint_field(
            FIELD_PRIORITY,
            raw,
            &mut common.unknown_fields,
        );
        return Ok(());
    }
    field.set::<_, _, BIT_PRIORITY>(common, value);
    Ok(())
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
