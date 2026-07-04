//! Sample of the enums puroro generates from `example.proto`.
//!
//! Protobuf enums are newtypes over `i32`, not Rust enums: multiple proto
//! value names may share the same integer (`allow_alias`), and the wire
//! carries only the number.

use ::core::convert::TryFrom;

/// Open enum (`enum_type = OPEN`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Status(i32);

impl Status {
    pub const UNSPECIFIED: Self = Self(0);
    pub const PENDING: Self = Self(1);
    pub const DONE: Self = Self(2);
}

impl Default for Status {
    fn default() -> Self {
        Self::UNSPECIFIED
    }
}

impl TryFrom<i32> for Status {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 | 1 | 2 => Ok(Self(value)),
            other => Err(other),
        }
    }
}

impl From<Status> for i32 {
    fn from(value: Status) -> Self {
        value.0
    }
}

/// Closed enum (`enum_type = CLOSED`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Priority(i32);

impl Priority {
    pub const UNSPECIFIED: Self = Self(0);
    pub const LOW: Self = Self(1);
    pub const HIGH: Self = Self(2);
}

impl Default for Priority {
    fn default() -> Self {
        Self::UNSPECIFIED
    }
}

impl TryFrom<i32> for Priority {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 | 1 | 2 => Ok(Self(value)),
            other => Err(other),
        }
    }
}

impl From<Priority> for i32 {
    fn from(value: Priority) -> Self {
        value.0
    }
}
