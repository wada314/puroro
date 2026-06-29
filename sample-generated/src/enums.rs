//! Generated enums from `example.proto`.

use ::core::convert::TryFrom;

/// Open enum (`enum_type = OPEN`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Status {
    #[default]
    Unspecified = 0,
    Pending = 1,
    Done = 2,
}

impl TryFrom<i32> for Status {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Unspecified),
            1 => Ok(Self::Pending),
            2 => Ok(Self::Done),
            other => Err(other),
        }
    }
}

impl From<Status> for i32 {
    fn from(value: Status) -> Self {
        value as Self
    }
}

/// Closed enum (`enum_type = CLOSED`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Priority {
    #[default]
    Unspecified = 0,
    Low = 1,
    High = 2,
}

impl TryFrom<i32> for Priority {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Unspecified),
            1 => Ok(Self::Low),
            2 => Ok(Self::High),
            other => Err(other),
        }
    }
}

impl From<Priority> for i32 {
    fn from(value: Priority) -> Self {
        value as Self
    }
}
