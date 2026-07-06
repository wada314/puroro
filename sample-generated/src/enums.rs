//! Sample of the enums puroro generates from `example.proto`.
//!
//! Protobuf enums are newtypes over `i32`, not Rust enums: multiple proto
//! value names may share the same integer (`allow_alias`), and the wire
//! carries only the number. Open vs closed is encoded in [`ProtoEnumStorage`].

use ::core::convert::TryFrom;

use ::puroro::{DecodeError, HasDefault};
use ::puroro_rt::{ProtoDefault, ProtoEnumStorage};

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

impl From<i32> for Status {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl TryFrom<Status> for i32 {
    type Error = i32;

    fn try_from(value: Status) -> Result<Self, Self::Error> {
        match value.0 {
            0 | 1 | 2 => Ok(value.0),
            other => Err(other),
        }
    }
}

impl ProtoEnumStorage for Status {
    fn proto_zero() -> Self {
        Self::UNSPECIFIED
    }

    fn to_wire(self) -> i32 {
        self.0
    }

    fn decode_from_wire(wire: i32) -> Result<Self, DecodeError> {
        Ok(Self::from(wire))
    }
}

impl HasDefault<Status> for ProtoDefault {
    const DEFAULT: Status = Status::UNSPECIFIED;
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

impl ProtoEnumStorage for Priority {
    fn proto_zero() -> Self {
        Self::UNSPECIFIED
    }

    fn to_wire(self) -> i32 {
        self.0
    }

    fn decode_from_wire(wire: i32) -> Result<Self, DecodeError> {
        Self::try_from(wire).map_err(|_| DecodeError::InvalidTag)
    }
}

impl HasDefault<Priority> for ProtoDefault {
    const DEFAULT: Priority = Priority::UNSPECIFIED;
}
