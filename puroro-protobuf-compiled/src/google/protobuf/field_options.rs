mod _root {
    #[allow(unused)]
    pub(crate) use super::super::_root::*;
}
mod _puroro {
    #[allow(unused)]
    pub(crate) use ::puroro::*;
}
mod _pinternal {
    #[allow(unused)]
    pub(crate) use ::puroro::internal::*;
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
pub enum CType {
    String,
    Cord,
    StringPiece,
}
impl ::std::default::Default for CType {
    fn default() -> Self {
        Self::String
    }
}
impl ::std::convert::From::<CType> for i32 {
    fn from(val: CType) -> i32 {
        match val {
            CType::String => 0i32,
            CType::Cord => 1i32,
            CType::StringPiece => 2i32,
        }
    }
}
impl ::std::convert::TryFrom::<i32> for CType {
    type Error = self::_puroro::PuroroError;
    fn try_from(val: i32) -> ::std::result::Result<Self, Self::Error> {
        match val {
            0i32 => ::std::result::Result::Ok(self::CType::String),
            1i32 => ::std::result::Result::Ok(self::CType::Cord),
            2i32 => ::std::result::Result::Ok(self::CType::StringPiece),
            _ => {
                ::std::result::Result::Err(
                    self::_puroro::PuroroError::UnknownEnumVariant(val),
                )?
            }
        }
    }
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
pub enum JSType {
    JsNormal,
    JsString,
    JsNumber,
}
impl ::std::default::Default for JSType {
    fn default() -> Self {
        Self::JsNormal
    }
}
impl ::std::convert::From::<JSType> for i32 {
    fn from(val: JSType) -> i32 {
        match val {
            JSType::JsNormal => 0i32,
            JSType::JsString => 1i32,
            JSType::JsNumber => 2i32,
        }
    }
}
impl ::std::convert::TryFrom::<i32> for JSType {
    type Error = self::_puroro::PuroroError;
    fn try_from(val: i32) -> ::std::result::Result<Self, Self::Error> {
        match val {
            0i32 => ::std::result::Result::Ok(self::JSType::JsNormal),
            1i32 => ::std::result::Result::Ok(self::JSType::JsString),
            2i32 => ::std::result::Result::Ok(self::JSType::JsNumber),
            _ => {
                ::std::result::Result::Err(
                    self::_puroro::PuroroError::UnknownEnumVariant(val),
                )?
            }
        }
    }
}
