//! Inherent API for [`AddressLazy`](crate::AddressLazy).
//!
//! String getters read spans out of `_wire` through
//! [`LazyProfile`](::puroro_rt::decode::LazyProfile). Numerical getters bind
//! the slot to message common after [`LazyMessage::ensure_scanned`].

use ::allocator_api2::alloc::{Allocator, Global};
use ::puroro::{DecodeError, HasDefault, Optional};
use ::puroro_rt::decode::{Borrowed, LazyMessage, LazyProfile};
use ::puroro_rt::{DefaultIn, HasField, Lazy, Record};

use crate::Address;
use crate::AddressLazy;
use crate::address::{FIELD_CITY, FIELD_LATITUDE, FIELD_POSTAL_CODE, FIELD_STREET};
use crate::address_type::AddressFields;

impl<A: Allocator + Clone, P: LazyProfile> AddressLazy<A, P> {
    pub fn has_street(&self) -> Result<bool, DecodeError> {
        LazyMessage::ensure_scanned(self)?;
        Ok(
            <AddressFields<A, Lazy<A, P>> as HasField<{ FIELD_STREET }>>::field(
                &self.record.fields,
            )
            .is_present(&self.record.common),
        )
    }

    pub fn has_city(&self) -> Result<bool, DecodeError> {
        LazyMessage::ensure_scanned(self)?;
        Ok(
            <AddressFields<A, Lazy<A, P>> as HasField<{ FIELD_CITY }>>::field(&self.record.fields)
                .is_present(&self.record.common),
        )
    }

    pub fn street(&self) -> Result<Optional<&str, impl HasDefault<&str>>, DecodeError>
    where
        A: Clone,
    {
        LazyMessage::ensure_scanned(self)?;
        <AddressFields<A, Lazy<A, P>> as HasField<{ FIELD_STREET }>>::field(&self.record.fields)
            .profile_str::<_, P>(&self.record.common, self.record.common.lazy.scan.wire_buf())
    }

    pub fn city(&self) -> Result<Optional<&str, impl HasDefault<&str>>, DecodeError>
    where
        A: Clone,
    {
        LazyMessage::ensure_scanned(self)?;
        <AddressFields<A, Lazy<A, P>> as HasField<{ FIELD_CITY }>>::field(&self.record.fields)
            .profile_str::<_, P>(&self.record.common, self.record.common.lazy.scan.wire_buf())
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
            record: Record::<_, A, Lazy<A, P>, _>::decode_eager(self.record)?,
        })
    }
}

impl<'buf, A: Allocator + Clone> AddressLazy<A, Borrowed<'buf>> {
    /// Scan `buf` once without copying it into [`bytes::Bytes`].
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
}

impl<'buf> AddressLazy<Global, Borrowed<'buf>> {
    /// [`parse_in`](Self::parse_in) with [`Global`].
    pub fn parse(buf: &'buf [u8]) -> Result<Self, DecodeError> {
        Self::parse_in(buf, Global)
    }
}
