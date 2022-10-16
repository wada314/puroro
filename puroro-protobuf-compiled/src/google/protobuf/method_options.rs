// A generated source code by puroro library
// package .google.protobuf.MethodOptions

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Debug, Clone, PartialEq)]
pub enum IdempotencyLevel {
    IdempotencyUnknown,
    NoSideEffects,
    Idempotent,
}

impl ::std::default::Default for IdempotencyLevel {
    fn default() -> Self {
        IdempotencyLevel::IdempotencyUnknown
    }
}

impl ::std::convert::TryFrom<i32> for IdempotencyLevel {
    type Error = self::_puroro::PuroroError;
    fn try_from(x: i32) -> ::std::result::Result<Self, Self::Error> {
        #[allow(unused)]
        use ::std::result::Result::{Err, Ok};
        match x {
            0 => Ok(self::IdempotencyLevel::IdempotencyUnknown),
            1 => Ok(self::IdempotencyLevel::NoSideEffects),
            2 => Ok(self::IdempotencyLevel::Idempotent),
            e => Err(self::_puroro::ErrorKind::UnknownEnumVariant(e))?,
        }
    }
}

impl ::std::convert::From<IdempotencyLevel> for i32 {
    fn from(x: IdempotencyLevel) -> i32 {
        match x {
            self::IdempotencyLevel::IdempotencyUnknown => 0,
            self::IdempotencyLevel::NoSideEffects => 1,
            self::IdempotencyLevel::Idempotent => 2,
        }
    }
}
