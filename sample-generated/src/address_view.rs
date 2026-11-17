//! Borrowing view of [`Address`](crate::Address).
//!
//! One scan records last-wins offsets into the caller's buffer. String bytes
//! stay there. UTF-8 is checked on each get.

use ::puroro::DecodeError;
use ::puroro_rt::decode::{ScannedRecord, ScannedValue, WireSpan};

use crate::address::{FIELD_CITY, FIELD_LATITUDE, FIELD_POSTAL_CODE, FIELD_STREET};
use crate::view_scan::{fixed32, fixed64_f64, for_each_record, len_span, utf8};

/// Read-only `Address` borrowed from a buffer.
///
/// `buf` is the root the spans index. A nested view keeps the parent buffer
/// so later occurrences of the same child still land in one slice.
#[derive(Clone, Copy)]
pub struct AddressView<'buf> {
    buf: &'buf [u8],
    street: Option<WireSpan>,
    city: Option<WireSpan>,
    postal_code: Option<u32>,
    latitude: Option<f64>,
}

impl<'buf> AddressView<'buf> {
    pub(crate) fn blank(buf: &'buf [u8]) -> Self {
        Self {
            buf,
            street: None,
            city: None,
            postal_code: None,
            latitude: None,
        }
    }

    /// Scans `buf` once.
    pub fn parse(buf: &'buf [u8]) -> Result<Self, DecodeError> {
        let mut view = Self::blank(buf);
        view.merge_at(0, buf)?;
        Ok(view)
    }

    /// Merges one LEN body. `origin` is that body's offset in [`Self::buf`].
    pub(crate) fn merge_at(&mut self, origin: usize, body: &[u8]) -> Result<(), DecodeError> {
        for_each_record(body, |rec| {
            match rec.field_number.as_u32() {
                FIELD_STREET => self.street = Some(expect_len(origin, rec)?),
                FIELD_CITY => self.city = Some(expect_len(origin, rec)?),
                FIELD_POSTAL_CODE => self.postal_code = Some(fixed32(&rec.value)?),
                FIELD_LATITUDE => self.latitude = Some(fixed64_f64(&rec.value)?),
                _ => {}
            }
            Ok(())
        })
    }

    /// Original buffer this view indexes.
    pub fn as_bytes(&self) -> &'buf [u8] {
        self.buf
    }

    /// Last `street`. Invalid UTF-8 is reported here.
    pub fn street(&self) -> Result<Option<&'buf str>, DecodeError> {
        utf8(self.buf, self.street)
    }

    /// Wire presence of `street`. Does not check UTF-8.
    pub fn has_street(&self) -> bool {
        self.street.is_some()
    }

    /// Last `city`. Invalid UTF-8 is reported here.
    pub fn city(&self) -> Result<Option<&'buf str>, DecodeError> {
        utf8(self.buf, self.city)
    }

    /// Wire presence of `city`. Does not check UTF-8.
    pub fn has_city(&self) -> bool {
        self.city.is_some()
    }

    /// Last `postal_code`, or `None` when the field was absent.
    pub fn postal_code(&self) -> Option<u32> {
        self.postal_code
    }

    /// Last `latitude`, or `None` when the field was absent.
    pub fn latitude(&self) -> Option<f64> {
        self.latitude
    }
}

fn expect_len(origin: usize, rec: &ScannedRecord) -> Result<WireSpan, DecodeError> {
    if !matches!(rec.value, ScannedValue::Len { .. }) {
        return Err(DecodeError::InvalidTag);
    }
    len_span(origin, rec)
}
