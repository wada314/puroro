//! Borrowing view of [`Point`](crate::Point).
//!
//! Both fields are `int32`, so the view stores the values and does not keep a
//! buffer pointer. The lifetime is on [`TaskView`](crate::TaskView) only when
//! a parent holds this value beside spans.

use ::puroro::DecodeError;

use crate::point::{FIELD_X, FIELD_Y};
use crate::view_scan::{for_each_record, varint_i32};

/// Read-only `Point`. `x` and `y` are implicit and default to `0`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PointView {
    x: i32,
    y: i32,
}

impl PointView {
    pub(crate) fn blank() -> Self {
        Self { x: 0, y: 0 }
    }

    /// Scans `buf` once.
    pub fn parse(buf: &[u8]) -> Result<Self, DecodeError> {
        let mut view = Self::blank();
        view.merge_body(buf)?;
        Ok(view)
    }

    pub(crate) fn merge_body(&mut self, body: &[u8]) -> Result<(), DecodeError> {
        for_each_record(body, |rec| {
            match rec.field_number.as_u32() {
                FIELD_X => self.x = varint_i32(&rec.value)?,
                FIELD_Y => self.y = varint_i32(&rec.value)?,
                _ => {}
            }
            Ok(())
        })
    }

    /// Last `x`, or `0` when absent.
    pub fn x(&self) -> i32 {
        self.x
    }

    /// Last `y`, or `0` when absent.
    pub fn y(&self) -> i32 {
        self.y
    }
}
