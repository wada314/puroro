//! Inherent API for [`PointLazy`](crate::PointLazy).
//!
//! Both fields are implicit int32 numericals. Getters call
//! [`LazyMessage::ensure_scanned`](::puroro_rt::decode::LazyMessage::ensure_scanned).
//! `into_eager` replays `_wire` into [`Point`](crate::Point).

use ::allocator_api2::alloc::Allocator;
use ::puroro::DecodeError;
use ::puroro_rt::decode::LazyMessage;
use ::puroro_rt::{Lazy, Record};

use crate::Point;
use crate::PointLazy;
use crate::point::{FIELD_X, FIELD_Y};

impl<A: Allocator + Clone> PointLazy<A> {
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
            record: Record::<_, A, Lazy<A>, _>::decode_eager(self.record)?,
        })
    }
}
