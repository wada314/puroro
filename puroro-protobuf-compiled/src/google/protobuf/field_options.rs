// A generated source code by puroro library
// package .google.protobuf.FieldOptions

pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}

pub mod _puroro {
    pub use ::puroro::*;
}

#[derive(Debug, Clone, PartialEq)]
pub enum CType {
    String,
    Cord,
    StringPiece,
}

impl ::std::default::Default for CType {
    fn default() -> Self {
        CType::String
    }
}

impl ::std::convert::TryFrom<i32> for CType {
    type Error = self::_puroro::PuroroError;
    fn try_from(x: i32) -> ::std::result::Result<Self, Self::Error> {
        #[allow(unused)]
        use ::std::result::Result::{Err, Ok};
        match x {
            0 => Ok(self::CType::String),
            1 => Ok(self::CType::Cord),
            2 => Ok(self::CType::StringPiece),
            e => Err(self::_puroro::ErrorKind::UnknownEnumVariant(e))?,
        }
    }
}

impl ::std::convert::From<CType> for i32 {
    fn from(x: CType) -> i32 {
        match x {
            self::CType::String => 0,
            self::CType::Cord => 1,
            self::CType::StringPiece => 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JSType {
    JsNormal,
    JsString,
    JsNumber,
}

impl ::std::default::Default for JSType {
    fn default() -> Self {
        JSType::JsNormal
    }
}

impl ::std::convert::TryFrom<i32> for JSType {
    type Error = self::_puroro::PuroroError;
    fn try_from(x: i32) -> ::std::result::Result<Self, Self::Error> {
        #[allow(unused)]
        use ::std::result::Result::{Err, Ok};
        match x {
            0 => Ok(self::JSType::JsNormal),
            1 => Ok(self::JSType::JsString),
            2 => Ok(self::JSType::JsNumber),
            e => Err(self::_puroro::ErrorKind::UnknownEnumVariant(e))?,
        }
    }
}

impl ::std::convert::From<JSType> for i32 {
    fn from(x: JSType) -> i32 {
        match x {
            self::JSType::JsNormal => 0,
            self::JSType::JsString => 1,
            self::JSType::JsNumber => 2,
        }
    }
}
