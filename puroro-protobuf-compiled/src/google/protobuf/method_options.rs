mod _puroro_root {
    #[allow(unused)]
    pub(crate) use super::super::_puroro_root::*;
}
mod _puroro {
    #[allow(unused)]
    pub(crate) use ::puroro::*;
}
#[derive(
    ::std::clone::Clone,
    ::std::marker::Copy,
    ::std::cmp::PartialEq,
    ::std::cmp::Eq,
    ::std::cmp::PartialOrd,
    ::std::cmp::Ord,
    ::std::hash::Hash,
    ::std::fmt::Debug,
)]
pub enum IdempotencyLevel {
    IdempotencyUnknown,
    NoSideEffects,
    Idempotent,
}
impl ::std::default::Default for IdempotencyLevel {
    fn default() -> Self {
        Self::IdempotencyUnknown
    }
}
impl ::std::convert::From::<IdempotencyLevel> for i32 {
    fn from(val: IdempotencyLevel) -> i32 {
        match val {
            IdempotencyLevel::IdempotencyUnknown => 0i32,
            IdempotencyLevel::NoSideEffects => 1i32,
            IdempotencyLevel::Idempotent => 2i32,
        }
    }
}
impl ::std::convert::TryFrom::<i32> for IdempotencyLevel {
    type Error = self::_puroro::PuroroError;
    fn try_from(val: i32) -> ::std::result::Result<Self, Self::Error> {
        match val {
            0i32 => ::std::result::Result::Ok(self::IdempotencyLevel::IdempotencyUnknown),
            1i32 => ::std::result::Result::Ok(self::IdempotencyLevel::NoSideEffects),
            2i32 => ::std::result::Result::Ok(self::IdempotencyLevel::Idempotent),
            _ => {
                ::std::result::Result::Err(
                    self::_puroro::ErrorKind::UnknownEnumVariant(val),
                )?
            }
        }
    }
}
