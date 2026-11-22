//! Inherent API for [`PointLazy`](crate::PointLazy).
//!
//! Both fields are numerical; one impl covers every [`WireBuf`].
//! Getters call
//! [`LazyMessage::ensure_scanned`](::puroro_rt::decode::LazyMessage::ensure_scanned).

use ::allocator_api2::alloc::{Allocator, Global};
use ::puroro::DecodeError;
use ::puroro_rt::decode::{LazyMessage, WireBuf};
use ::puroro_rt::{DefaultIn, Lazy, Record};

use crate::Point;
use crate::PointLazy;
use crate::point::{FIELD_X, FIELD_Y};

impl<A: Allocator + Clone, B: WireBuf> PointLazy<A, B> {
    pub fn x(&self) -> Result<i32, DecodeError> {
        LazyMessage::ensure_scanned(self)?;
        Ok(self.record.field::<{ FIELD_X }>().value())
    }

    pub fn y(&self) -> Result<i32, DecodeError> {
        LazyMessage::ensure_scanned(self)?;
        Ok(self.record.field::<{ FIELD_Y }>().value())
    }

    /// Decode `_wire` into the eager message.
    pub fn into_eager(self) -> Result<Point<A>, DecodeError> {
        Ok(Point {
            record: Record::<_, A, Lazy<A, B>, _>::decode_eager(self.record)?,
        })
    }
}

impl<'buf, A: Allocator + Clone> PointLazy<A, &'buf [u8]> {
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

impl<'buf> PointLazy<Global, &'buf [u8]> {
    /// [`parse_in`](Self::parse_in) with [`Global`].
    pub fn parse(buf: &'buf [u8]) -> Result<Self, DecodeError> {
        Self::parse_in(buf, Global)
    }
}
