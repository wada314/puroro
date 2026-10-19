//! Archived zero-copy view of [`Address`](::puroro_sample_generated::Address).
//!
//! This crate is not workspace member code and not plugin output. It is not
//! [`task::MessageFallible`]. `sample-generated` stays the generator reference.
//! One scan records last-wins offsets into the caller's buffer. String and
//! the numeric payloads are not copied. String UTF-8 is checked on each get,
//! so a numeric-only read does not pay for validation, and a repeated get
//! validates again.
//!
//! A known field with the wrong wire type is [`DecodeError::InvalidTag`],
//! matching eager `Address` (the payload is not consumed). Unknown field
//! numbers are skipped and not stored. [`AddressView::to_owned`] re-decodes
//! the original buffer, so preserved unknowns still round-trip there.
//! Groups are [`DecodeError::InvalidTag`].

use ::allocator_api2::alloc::Allocator;
use ::bytes::Buf;
use ::core::fmt;
use ::core::str;
use ::puroro::{DecodeError, Message, WireType};
use ::puroro_rt::decode::{decode_tag, decode_varint};
use ::puroro_sample_generated::Address;
use ::puroro_sample_generated::address::{
    FIELD_CITY, FIELD_LATITUDE, FIELD_POSTAL_CODE, FIELD_STREET,
};

/// `u32::MAX` length means the field was absent. A real protobuf string is at
/// most 2 GiB, so this sentinel does not collide with a present empty string.
const ABSENT: u32 = u32::MAX;

/// Byte range of one length-delimited payload inside the original buffer.
#[derive(Clone, Copy)]
struct LenSlot {
    start: u32,
    len: u32,
}

impl LenSlot {
    fn absent() -> Self {
        Self {
            start: 0,
            len: ABSENT,
        }
    }

    fn is_present(self) -> bool {
        self.len != ABSENT
    }

    fn slice(self, buf: &[u8]) -> &[u8] {
        let start = self.start as usize;
        let end = start + self.len as usize;
        &buf[start..end]
    }
}

/// Read-only `Address` borrowed from `buf`.
///
/// The index is built once. Getters do not rescan the message.
#[derive(Clone, Copy)]
pub struct AddressView<'buf> {
    buf: &'buf [u8],
    street: LenSlot,
    city: LenSlot,
    postal_code: Option<u32>,
    latitude: Option<f64>,
}

impl<'buf> AddressView<'buf> {
    /// Scans `buf` once and records the last value of each known field.
    pub fn parse(buf: &'buf [u8]) -> Result<Self, DecodeError> {
        let mut rest = buf;
        let mut street = LenSlot::absent();
        let mut city = LenSlot::absent();
        let mut postal_code = None;
        let mut latitude = None;

        while rest.has_remaining() {
            let (field_number, wire_type) = decode_tag(&mut rest)?;
            match field_number.as_u32() {
                FIELD_STREET => {
                    if wire_type != WireType::Len {
                        return Err(DecodeError::InvalidTag);
                    }
                    street = take_len(buf, &mut rest)?;
                }
                FIELD_CITY => {
                    if wire_type != WireType::Len {
                        return Err(DecodeError::InvalidTag);
                    }
                    city = take_len(buf, &mut rest)?;
                }
                FIELD_POSTAL_CODE => {
                    if wire_type != WireType::Int32 {
                        return Err(DecodeError::InvalidTag);
                    }
                    postal_code = Some(take_fixed32(&mut rest)?);
                }
                FIELD_LATITUDE => {
                    if wire_type != WireType::Int64 {
                        return Err(DecodeError::InvalidTag);
                    }
                    latitude = Some(take_fixed64(&mut rest)?);
                }
                _ => skip_payload(wire_type, &mut rest)?,
            }
        }

        Ok(Self {
            buf,
            street,
            city,
            postal_code,
            latitude,
        })
    }

    /// Last `street`, or `None` when the field was absent.
    ///
    /// Invalid UTF-8 is reported here, not from [`parse`](Self::parse).
    pub fn street(&self) -> Result<Option<&'buf str>, DecodeError> {
        utf8(self.buf, self.street)
    }

    /// Last `city`, or `None` when the field was absent.
    pub fn city(&self) -> Result<Option<&'buf str>, DecodeError> {
        utf8(self.buf, self.city)
    }

    /// Last `postal_code`. Decoded during [`parse`](Self::parse).
    pub fn postal_code(&self) -> Option<u32> {
        self.postal_code
    }

    /// Last `latitude`. Decoded during [`parse`](Self::parse).
    pub fn latitude(&self) -> Option<f64> {
        self.latitude
    }

    /// Owned `Address` decoded from the same buffer, including unknown fields.
    ///
    /// This is a second full parse. The index is not copied into the owned message.
    pub fn to_owned<A>(&self, alloc: A) -> Result<Address<A>, DecodeError>
    where
        A: Allocator + Clone,
    {
        let mut msg = Address::new_in(alloc);
        let mut buf = self.buf;
        Message::merge_from(&mut msg, &mut buf)?;
        Ok(msg)
    }
}

impl fmt::Debug for AddressView<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AddressView")
            .field("street", &self.street())
            .field("city", &self.city())
            .field("postal_code", &self.postal_code)
            .field("latitude", &self.latitude)
            .finish()
    }
}

fn utf8(buf: &[u8], slot: LenSlot) -> Result<Option<&str>, DecodeError> {
    if !slot.is_present() {
        return Ok(None);
    }
    str::from_utf8(slot.slice(buf))
        .map(Some)
        .map_err(|_| DecodeError::InvalidUtf8)
}

fn take_len(input: &[u8], rest: &mut &[u8]) -> Result<LenSlot, DecodeError> {
    let len = usize::try_from(decode_varint(rest)?).map_err(|_| DecodeError::TruncatedMessage)?;
    if rest.len() < len {
        return Err(DecodeError::TruncatedMessage);
    }
    let start = input.len() - rest.len();
    *rest = &rest[len..];
    Ok(LenSlot {
        start: u32::try_from(start).map_err(|_| DecodeError::TruncatedMessage)?,
        len: u32::try_from(len).map_err(|_| DecodeError::TruncatedMessage)?,
    })
}

fn take_fixed32(rest: &mut &[u8]) -> Result<u32, DecodeError> {
    if rest.len() < 4 {
        return Err(DecodeError::UnexpectedEof);
    }
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&rest[..4]);
    *rest = &rest[4..];
    Ok(u32::from_le_bytes(bytes))
}

fn take_fixed64(rest: &mut &[u8]) -> Result<f64, DecodeError> {
    if rest.len() < 8 {
        return Err(DecodeError::UnexpectedEof);
    }
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&rest[..8]);
    *rest = &rest[8..];
    Ok(f64::from_le_bytes(bytes))
}

/// Skip one unknown payload. Local copy of `puroro_rt::decode::skip_field`,
/// which is crate-private.
fn skip_payload(wire_type: WireType, rest: &mut &[u8]) -> Result<(), DecodeError> {
    match wire_type {
        WireType::Varint => {
            decode_varint(rest)?;
        }
        WireType::Int64 => {
            if rest.len() < 8 {
                return Err(DecodeError::UnexpectedEof);
            }
            *rest = &rest[8..];
        }
        WireType::Len => {
            let len =
                usize::try_from(decode_varint(rest)?).map_err(|_| DecodeError::TruncatedMessage)?;
            if rest.len() < len {
                return Err(DecodeError::TruncatedMessage);
            }
            *rest = &rest[len..];
        }
        WireType::Int32 => {
            if rest.len() < 4 {
                return Err(DecodeError::UnexpectedEof);
            }
            *rest = &rest[4..];
        }
        WireType::SGroup | WireType::EGroup => return Err(DecodeError::InvalidTag),
    }
    Ok(())
}
