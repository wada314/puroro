//! Inherent API for [`AddressLazy`](crate::AddressLazy).
//!
//! String getters read spans out of `_wire`. Numerical getters bind the slot
//! to message common after [`LazyMessage::ensure_scanned`]. Owned `Bytes`
//! promotes; borrowed `&[u8]` peeks without promoting.

use ::allocator_api2::alloc::{Allocator, Global};
use ::bytes::Bytes;
use ::puroro::{DecodeError, HasDefault, Optional};
use ::puroro_rt::decode::{LazyMessage, WireBuf};
use ::puroro_rt::{DefaultIn, HasField, Lazy, Record};

use crate::Address;
use crate::AddressLazy;
use crate::address::{FIELD_CITY, FIELD_LATITUDE, FIELD_POSTAL_CODE, FIELD_STREET};
use crate::address_type::AddressFields;

impl<A: Allocator + Clone, B: WireBuf> AddressLazy<A, B> {
    pub fn has_street(&self) -> Result<bool, DecodeError> {
        LazyMessage::ensure_scanned(self)?;
        Ok(
            <AddressFields<A, Lazy<A, B>> as HasField<{ FIELD_STREET }>>::field(
                &self.record.fields,
            )
            .is_present(&self.record.common),
        )
    }

    pub fn has_city(&self) -> Result<bool, DecodeError> {
        LazyMessage::ensure_scanned(self)?;
        Ok(
            <AddressFields<A, Lazy<A, B>> as HasField<{ FIELD_CITY }>>::field(&self.record.fields)
                .is_present(&self.record.common),
        )
    }

    pub fn postal_code<'a>(&'a self) -> Result<Optional<u32, impl HasDefault<u32>>, DecodeError>
    where
        A: 'a,
    {
        LazyMessage::ensure_scanned(self)?;
        Ok(self.record.field::<{ FIELD_POSTAL_CODE }>().optional())
    }

    pub fn latitude<'a>(&'a self) -> Result<Optional<f64, impl HasDefault<f64>>, DecodeError>
    where
        A: 'a,
    {
        LazyMessage::ensure_scanned(self)?;
        Ok(self.record.field::<{ FIELD_LATITUDE }>().optional())
    }

    /// Decode `_wire` into the eager message.
    pub fn into_eager(self) -> Result<Address<A>, DecodeError> {
        Ok(Address {
            record: Record::<_, A, Lazy<A, B>, _>::decode_eager(self.record)?,
        })
    }
}

impl<A: Allocator + Clone> AddressLazy<A, Bytes> {
    pub fn street<'a>(&'a self) -> Result<Optional<&'a str, impl HasDefault<&'a str>>, DecodeError>
    where
        A: 'a,
    {
        LazyMessage::ensure_scanned(self)?;
        <AddressFields<A, Lazy<A, Bytes>> as HasField<{ FIELD_STREET }>>::field(&self.record.fields)
            .try_str(&self.record.common, self.record.common.lazy.scan.wire())
    }

    pub fn city<'a>(&'a self) -> Result<Optional<&'a str, impl HasDefault<&'a str>>, DecodeError>
    where
        A: 'a,
    {
        LazyMessage::ensure_scanned(self)?;
        <AddressFields<A, Lazy<A, Bytes>> as HasField<{ FIELD_CITY }>>::field(&self.record.fields)
            .try_str(&self.record.common, self.record.common.lazy.scan.wire())
    }
}

impl<'buf, A: Allocator + Clone> AddressLazy<A, &'buf [u8]> {
    /// Scan `buf` once without copying it into [`Bytes`].
    pub fn parse_in(buf: &'buf [u8], alloc: A) -> Result<Self, DecodeError> {
        let mut msg = Self {
            record: DefaultIn::default_in(alloc),
        };
        let records = msg.record.common.lazy.scan.open(buf)?;
        for rec in records {
            LazyMessage::apply_record(&mut msg, &rec, 0, 0)?;
        }
        Ok(msg)
    }

    pub fn street(&self) -> Result<Optional<&'buf str, impl HasDefault<&'buf str>>, DecodeError> {
        LazyMessage::ensure_scanned(self)?;
        let wire = *self.record.common.lazy.scan.wire_buf();
        <AddressFields<A, Lazy<A, &'buf [u8]>> as HasField<{ FIELD_STREET }>>::field(
            &self.record.fields,
        )
        .peek_str(&self.record.common, wire)
    }

    pub fn city(&self) -> Result<Optional<&'buf str, impl HasDefault<&'buf str>>, DecodeError> {
        LazyMessage::ensure_scanned(self)?;
        let wire = *self.record.common.lazy.scan.wire_buf();
        <AddressFields<A, Lazy<A, &'buf [u8]>> as HasField<{ FIELD_CITY }>>::field(
            &self.record.fields,
        )
        .peek_str(&self.record.common, wire)
    }
}

impl<'buf> AddressLazy<Global, &'buf [u8]> {
    /// [`parse_in`](Self::parse_in) with [`Global`].
    pub fn parse(buf: &'buf [u8]) -> Result<Self, DecodeError> {
        Self::parse_in(buf, Global)
    }
}
