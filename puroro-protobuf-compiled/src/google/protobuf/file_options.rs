// A generated source code by puroro library
// package .google.protobuf.FileOptions

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Debug, Clone, PartialEq)]
pub enum OptimizeMode {
    Speed,
    CodeSize,
    LiteRuntime,
}

impl ::std::default::Default for OptimizeMode {
    fn default() -> Self {
        OptimizeMode::Speed
    }
}

impl ::std::convert::TryFrom<i32> for OptimizeMode {
    type Error = self::_puroro::PuroroError;
    fn try_from(x: i32) -> ::std::result::Result<Self, Self::Error> {
        #[allow(unused)]
        use ::std::result::Result::{Err, Ok};
        match x {
            1 => Ok(self::OptimizeMode::Speed),
            2 => Ok(self::OptimizeMode::CodeSize),
            3 => Ok(self::OptimizeMode::LiteRuntime),
            e => Err(self::_puroro::ErrorKind::UnknownEnumVariant(e))?,
        }
    }
}

impl ::std::convert::From<OptimizeMode> for i32 {
    fn from(x: OptimizeMode) -> i32 {
        match x {
            self::OptimizeMode::Speed => 1,
            self::OptimizeMode::CodeSize => 2,
            self::OptimizeMode::LiteRuntime => 3,
        }
    }
}
