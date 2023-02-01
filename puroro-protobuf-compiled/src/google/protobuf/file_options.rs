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
/** Generated classes can be optimized for speed or code size.
*/
pub enum OptimizeMode {
    /** Generate complete code for parsing, serialization,
*/
    Speed,
    /** etc.


 Use ReflectionOps to implement these methods.
*/
    CodeSize,
    /** Generate code using MessageLite and the lite runtime.
*/
    LiteRuntime,
}
impl ::std::default::Default for OptimizeMode {
    fn default() -> Self {
        Self::Speed
    }
}
impl ::std::convert::From::<OptimizeMode> for i32 {
    fn from(val: OptimizeMode) -> i32 {
        match val {
            OptimizeMode::Speed => 1i32,
            OptimizeMode::CodeSize => 2i32,
            OptimizeMode::LiteRuntime => 3i32,
        }
    }
}
impl ::std::convert::TryFrom::<i32> for OptimizeMode {
    type Error = self::_puroro::PuroroError;
    fn try_from(val: i32) -> ::std::result::Result<Self, Self::Error> {
        match val {
            1i32 => ::std::result::Result::Ok(self::OptimizeMode::Speed),
            2i32 => ::std::result::Result::Ok(self::OptimizeMode::CodeSize),
            3i32 => ::std::result::Result::Ok(self::OptimizeMode::LiteRuntime),
            _ => {
                ::std::result::Result::Err(
                    self::_puroro::PuroroError::UnknownEnumVariant(val),
                )?
            }
        }
    }
}
