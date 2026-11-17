//! Borrowing view of [`Task`](crate::Task).
//!
//! This is a third type, not a mode of [`Task`](crate::Task) or
//! [`TaskLazy`](crate::TaskLazy). [`parse`](TaskView::parse) scans the buffer
//! once and stores scalars plus spans. Getters do not rescan. String and bytes
//! payloads stay in the caller's buffer. Repeated numericals are copied out
//! during that scan because the scanner already visited every value.
//!
//! The index for repeated strings, messages, and map entries is a `Vec` of
//! spans. That is the index, not a copy of the field bytes. Unknown field
//! numbers, including unknown groups, are skipped. [`TaskView::to_owned`]
//! decodes the same buffer with eager [`Task`], so those unknowns still
//! round-trip there.
//!
//! [`task::MessageFallible`](crate::task::MessageFallible) is not implemented.
//! Its child types are `Address` / `AddressLazy`, and this view's children are
//! [`AddressView`](crate::AddressView).

use ::allocator_api2::alloc::Allocator;
use ::core::str;
use ::puroro::{DecodeError, Message};
use ::puroro_rt::decode::{ScannedRecord, ScannedValue, WireSpan};
use ::std::vec::Vec;

use crate::AddressView;
use crate::PointView;
use crate::Task;
use crate::enums::{Priority, Status};
use crate::task::{
    FIELD_ASSIGNEE, FIELD_ATTRIBUTES, FIELD_DONE, FIELD_EMAIL_ADDRESS, FIELD_FLAG, FIELD_LABELS,
    FIELD_MAX_RETRIES, FIELD_ORIGIN, FIELD_OWNER_ID, FIELD_PAYLOAD, FIELD_PHONE_NUMBER,
    FIELD_POSTAL, FIELD_PRIORITY, FIELD_SCORE, FIELD_SCORES, FIELD_STATUS, FIELD_TAG_IDS,
    FIELD_TITLE, FIELD_URGENT, FIELD_VOTES, FIELD_WATCHERS, FIELD_WEBHOOK_ID, NotificationCase,
};
use crate::view_scan::{
    for_each_record, len_span, push_bool, push_i32, utf8, varint_bool, varint_i32,
};

/// Read-only `Task` borrowed from `buf`.
#[derive(Clone)]
pub struct TaskView<'buf> {
    buf: &'buf [u8],
    title: Option<WireSpan>,
    score: i32,
    max_retries: Option<i32>,
    owner_id: Option<WireSpan>,
    payload: Option<WireSpan>,
    tag_ids: Vec<i32>,
    scores: Vec<i32>,
    labels: Vec<WireSpan>,
    status: Status,
    priority: Option<Priority>,
    assignee: Option<AddressView<'buf>>,
    done: bool,
    flag: Option<bool>,
    watchers: Vec<AddressView<'buf>>,
    votes: Vec<bool>,
    attributes: Vec<(WireSpan, i32)>,
    origin: Option<PointView>,
    note: Option<Note<'buf>>,
}

#[derive(Clone, Copy)]
enum Note<'buf> {
    Email(WireSpan),
    Phone(WireSpan),
    Webhook(i32),
    Postal(AddressView<'buf>),
    Urgent(bool),
}

impl<'buf> TaskView<'buf> {
    /// Scans `buf` once.
    pub fn parse(buf: &'buf [u8]) -> Result<Self, DecodeError> {
        let mut view = Self {
            buf,
            title: None,
            score: 0,
            max_retries: None,
            owner_id: None,
            payload: None,
            tag_ids: Vec::new(),
            scores: Vec::new(),
            labels: Vec::new(),
            status: Status::UNSPECIFIED,
            priority: None,
            assignee: None,
            done: false,
            flag: None,
            watchers: Vec::new(),
            votes: Vec::new(),
            attributes: Vec::new(),
            origin: None,
            note: None,
        };
        view.merge_at(0, buf)?;
        Ok(view)
    }

    fn merge_at(&mut self, origin: usize, body: &[u8]) -> Result<(), DecodeError> {
        for_each_record(body, |rec| self.apply(origin, rec))
    }

    fn apply(&mut self, origin: usize, rec: &ScannedRecord) -> Result<(), DecodeError> {
        match rec.field_number.as_u32() {
            FIELD_TITLE => self.title = Some(expect_len(origin, rec)?),
            FIELD_SCORE => self.score = varint_i32(&rec.value)?,
            FIELD_MAX_RETRIES => self.max_retries = Some(varint_i32(&rec.value)?),
            FIELD_OWNER_ID => self.owner_id = Some(expect_len(origin, rec)?),
            FIELD_PAYLOAD => self.payload = Some(expect_len(origin, rec)?),
            FIELD_TAG_IDS => push_i32(self.buf, origin, rec, &mut self.tag_ids)?,
            FIELD_SCORES => push_i32(self.buf, origin, rec, &mut self.scores)?,
            FIELD_LABELS => self.labels.push(expect_len(origin, rec)?),
            FIELD_STATUS => self.status = Status::from(varint_i32(&rec.value)?),
            FIELD_PRIORITY => {
                if let Some(priority) = known_priority(&rec.value)? {
                    self.priority = Some(priority);
                }
            }
            FIELD_ASSIGNEE => self.merge_assignee(origin, rec)?,
            FIELD_DONE => self.done = varint_bool(&rec.value)?,
            FIELD_FLAG => self.flag = Some(varint_bool(&rec.value)?),
            FIELD_WATCHERS => self.push_watcher(origin, rec)?,
            FIELD_VOTES => push_bool(self.buf, origin, rec, &mut self.votes)?,
            FIELD_ATTRIBUTES => self.push_attribute(origin, rec)?,
            FIELD_ORIGIN => self.merge_origin(origin, rec)?,
            FIELD_EMAIL_ADDRESS => self.note = Some(Note::Email(expect_len(origin, rec)?)),
            FIELD_PHONE_NUMBER => self.note = Some(Note::Phone(expect_len(origin, rec)?)),
            FIELD_WEBHOOK_ID => self.note = Some(Note::Webhook(varint_i32(&rec.value)?)),
            FIELD_POSTAL => self.merge_postal(origin, rec)?,
            FIELD_URGENT => self.note = Some(Note::Urgent(varint_bool(&rec.value)?)),
            _ => {}
        }
        Ok(())
    }

    fn merge_assignee(&mut self, origin: usize, rec: &ScannedRecord) -> Result<(), DecodeError> {
        let span = expect_len(origin, rec)?;
        let body = span.slice(self.buf)?;
        if let Some(child) = self.assignee.as_mut() {
            child.merge_at(span.offset, body)?;
        } else {
            let mut child = AddressView::blank(self.buf);
            child.merge_at(span.offset, body)?;
            self.assignee = Some(child);
        }
        Ok(())
    }

    fn push_watcher(&mut self, origin: usize, rec: &ScannedRecord) -> Result<(), DecodeError> {
        let span = expect_len(origin, rec)?;
        let mut child = AddressView::blank(self.buf);
        child.merge_at(span.offset, span.slice(self.buf)?)?;
        self.watchers.push(child);
        Ok(())
    }

    fn push_attribute(&mut self, origin: usize, rec: &ScannedRecord) -> Result<(), DecodeError> {
        let span = expect_len(origin, rec)?;
        self.attributes.push(map_entry(self.buf, span)?);
        Ok(())
    }

    fn merge_origin(&mut self, origin: usize, rec: &ScannedRecord) -> Result<(), DecodeError> {
        let span = expect_len(origin, rec)?;
        let body = span.slice(self.buf)?;
        if let Some(child) = self.origin.as_mut() {
            child.merge_body(body)?;
        } else {
            self.origin = Some(PointView::parse(body)?);
        }
        Ok(())
    }

    fn merge_postal(&mut self, origin: usize, rec: &ScannedRecord) -> Result<(), DecodeError> {
        let span = expect_len(origin, rec)?;
        let body = span.slice(self.buf)?;
        if let Some(Note::Postal(child)) = self.note.as_mut() {
            child.merge_at(span.offset, body)?;
        } else {
            let mut child = AddressView::blank(self.buf);
            child.merge_at(span.offset, body)?;
            self.note = Some(Note::Postal(child));
        }
        Ok(())
    }

    /// Original buffer this view indexes.
    pub fn as_bytes(&self) -> &'buf [u8] {
        self.buf
    }

    /// Eager `Task` decoded from the same buffer, including unknown fields.
    ///
    /// This is a second full parse. The view's index is not copied across.
    pub fn to_owned<A>(&self, alloc: A) -> Result<Task<A>, DecodeError>
    where
        A: Allocator + Clone,
    {
        let mut msg = Task::new_in(alloc);
        let mut buf = self.buf;
        Message::merge_from(&mut msg, &mut buf)?;
        Ok(msg)
    }

    /// Last `title`. Invalid UTF-8 is reported here.
    pub fn title(&self) -> Result<Option<&'buf str>, DecodeError> {
        utf8(self.buf, self.title)
    }

    /// Wire presence of `title`. Does not check UTF-8.
    pub fn has_title(&self) -> bool {
        self.title.is_some()
    }

    /// Last `score`, or `0` when absent.
    pub fn score(&self) -> i32 {
        self.score
    }

    /// Last `max_retries`, or `None` when absent.
    ///
    /// The proto default `3` is not filled in. [`Task::max_retries`] does that
    /// on the owned message.
    pub fn max_retries(&self) -> Option<i32> {
        self.max_retries
    }

    /// Last `owner_id`. Invalid UTF-8 is reported here.
    pub fn owner_id(&self) -> Result<Option<&'buf str>, DecodeError> {
        utf8(self.buf, self.owner_id)
    }

    /// Wire presence of `owner_id`. Does not check UTF-8.
    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    /// Last `payload`, or `None` when absent.
    pub fn payload(&self) -> Result<Option<&'buf [u8]>, DecodeError> {
        Ok(match self.payload {
            Some(span) => Some(span.slice(self.buf)?),
            None => None,
        })
    }

    /// Wire presence of `payload`.
    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    /// `tag_ids` in wire order. Packed and unpacked records are both appended.
    pub fn tag_ids(&self) -> &[i32] {
        &self.tag_ids
    }

    /// `scores` in wire order. Packed and unpacked records are both appended.
    pub fn scores(&self) -> &[i32] {
        &self.scores
    }

    /// `labels` in wire order.
    ///
    /// Each item is checked when the iterator yields it. Stopping early does
    /// not read the remaining payloads. A bad element is `Err` for that item.
    pub fn labels(&self) -> impl Iterator<Item = Result<&'buf str, DecodeError>> + '_ {
        self.labels
            .iter()
            .map(|span| str::from_utf8(span.slice(self.buf)?).map_err(|_| DecodeError::InvalidUtf8))
    }

    /// Last `status`, or [`Status::UNSPECIFIED`] when absent.
    ///
    /// An unknown number is kept. `status` is an open enum.
    pub fn status(&self) -> Status {
        self.status
    }

    /// Last known `priority`, or `None` when no known value was present.
    ///
    /// A number outside the closed set is left in the buffer for
    /// [`to_owned`](Self::to_owned) and does not replace a value already stored.
    pub fn priority(&self) -> Option<Priority> {
        self.priority
    }

    /// Merged `assignee`, or `None` when the field was absent.
    pub fn assignee(&self) -> Option<&AddressView<'buf>> {
        self.assignee.as_ref()
    }

    /// Last `done`, or `false` when absent.
    pub fn done(&self) -> bool {
        self.done
    }

    /// Last `flag`, or `None` when absent.
    pub fn flag(&self) -> Option<bool> {
        self.flag
    }

    /// One `watchers` element per LEN, in wire order.
    pub fn watchers(&self) -> &[AddressView<'buf>] {
        &self.watchers
    }

    /// `votes` in wire order. Packed and unpacked records are both appended.
    pub fn votes(&self) -> &[bool] {
        &self.votes
    }

    /// Map entries in wire order, including repeated keys.
    ///
    /// A key that is not UTF-8 is `Err` for that entry. [`attribute`](Self::attribute)
    /// resolves last-wins for one key.
    pub fn attributes(&self) -> impl Iterator<Item = Result<(&'buf str, i32), DecodeError>> + '_ {
        self.attributes.iter().map(|(span, value)| {
            let key =
                str::from_utf8(span.slice(self.buf)?).map_err(|_| DecodeError::InvalidUtf8)?;
            Ok((key, *value))
        })
    }

    /// Last value stored for `key`, comparing key bytes.
    ///
    /// Entries whose key bytes differ are skipped, including keys that are not
    /// UTF-8. A matching key is valid UTF-8 because `key` is a `str`.
    pub fn attribute(&self, key: &str) -> Result<Option<i32>, DecodeError> {
        let mut found = None;
        for (span, value) in &self.attributes {
            if span.slice(self.buf)? == key.as_bytes() {
                found = Some(*value);
            }
        }
        Ok(found)
    }

    /// Merged `origin`, or `None` when the field was absent.
    pub fn origin(&self) -> Option<PointView> {
        self.origin
    }

    /// Which `notification` variant was last, or `None` when the oneof is unset.
    pub fn notification(&self) -> Option<NotificationCase> {
        Some(match self.note.as_ref()? {
            Note::Email(_) => NotificationCase::EmailAddress,
            Note::Phone(_) => NotificationCase::PhoneNumber,
            Note::Webhook(_) => NotificationCase::WebhookId,
            Note::Postal(_) => NotificationCase::Postal,
            Note::Urgent(_) => NotificationCase::Urgent,
        })
    }

    /// `email_address` when that variant is active.
    pub fn email_address(&self) -> Result<Option<&'buf str>, DecodeError> {
        match self.note {
            Some(Note::Email(span)) => utf8(self.buf, Some(span)),
            _ => Ok(None),
        }
    }

    /// `phone_number` when that variant is active.
    pub fn phone_number(&self) -> Result<Option<&'buf str>, DecodeError> {
        match self.note {
            Some(Note::Phone(span)) => utf8(self.buf, Some(span)),
            _ => Ok(None),
        }
    }

    /// `webhook_id` when that variant is active.
    ///
    /// Unset is `None`, not the proto default `-1`.
    pub fn webhook_id(&self) -> Option<i32> {
        match self.note {
            Some(Note::Webhook(value)) => Some(value),
            _ => None,
        }
    }

    /// `postal` when that variant is active.
    pub fn postal(&self) -> Option<&AddressView<'buf>> {
        match &self.note {
            Some(Note::Postal(view)) => Some(view),
            _ => None,
        }
    }

    /// `urgent` when that variant is active.
    pub fn urgent(&self) -> Option<bool> {
        match self.note {
            Some(Note::Urgent(value)) => Some(value),
            _ => None,
        }
    }
}

fn expect_len(origin: usize, rec: &ScannedRecord) -> Result<WireSpan, DecodeError> {
    if !matches!(rec.value, ScannedValue::Len { .. }) {
        return Err(DecodeError::InvalidTag);
    }
    len_span(origin, rec)
}

/// `None` means the value is outside the closed set. The caller keeps the previous slot.
fn known_priority(value: &ScannedValue) -> Result<Option<Priority>, DecodeError> {
    match Priority::try_from(varint_i32(value)?) {
        Ok(priority) => Ok(Some(priority)),
        Err(_) => Ok(None),
    }
}

/// One map entry. A missing key is `""` and a missing value is `0`.
fn map_entry(root: &[u8], span: WireSpan) -> Result<(WireSpan, i32), DecodeError> {
    let body = span.slice(root)?;
    let mut key = None;
    let mut value = 0i32;
    for_each_record(body, |rec| {
        match rec.field_number.as_u32() {
            1 => key = Some(expect_len(span.offset, rec)?),
            2 => value = varint_i32(&rec.value)?,
            _ => {}
        }
        Ok(())
    })?;
    Ok((key.unwrap_or(WireSpan { offset: 0, len: 0 }), value))
}
