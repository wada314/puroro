//! Inherent API for [`AddressLazy`](crate::AddressLazy).
//!
//! String getters read spans out of `_wire`. Numerical getters bind the slot
//! to message common after [`LazyMessage::ensure_scanned`].

use ::allocator_api2::alloc::Allocator;
use ::puroro::{DecodeError, HasDefault, Optional};
use ::puroro_rt::decode::LazyMessage;
use ::puroro_rt::{HasField, Lazy, Record};

use crate::Address;
use crate::AddressLazy;
use crate::address::{FIELD_CITY, FIELD_LATITUDE, FIELD_POSTAL_CODE, FIELD_STREET};
use crate::address_type::AddressFields;

impl<A: Allocator + Clone> AddressLazy<A> {
    pub fn street<'a>(&'a self) -> Result<Optional<&'a str, impl HasDefault<&'a str>>, DecodeError>
    where
        A: 'a,
    {
        LazyMessage::ensure_scanned(self)?;
        <AddressFields<A, Lazy<A>> as HasField<{ FIELD_STREET }>>::field(&self.record.fields)
            .try_str(&self.record.common, self.record.common.lazy.scan.wire())
    }

    pub fn has_street(&self) -> Result<bool, DecodeError> {
        LazyMessage::ensure_scanned(self)?;
        Ok(
            <AddressFields<A, Lazy<A>> as HasField<{ FIELD_STREET }>>::field(&self.record.fields)
                .is_present(&self.record.common),
        )
    }

    pub fn city<'a>(&'a self) -> Result<Optional<&'a str, impl HasDefault<&'a str>>, DecodeError>
    where
        A: 'a,
    {
        LazyMessage::ensure_scanned(self)?;
        <AddressFields<A, Lazy<A>> as HasField<{ FIELD_CITY }>>::field(&self.record.fields)
            .try_str(&self.record.common, self.record.common.lazy.scan.wire())
    }

    pub fn has_city(&self) -> Result<bool, DecodeError> {
        LazyMessage::ensure_scanned(self)?;
        Ok(
            <AddressFields<A, Lazy<A>> as HasField<{ FIELD_CITY }>>::field(&self.record.fields)
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
            record: Record::<_, A, Lazy<A>, _>::decode_eager(self.record)?,
        })
    }
}
