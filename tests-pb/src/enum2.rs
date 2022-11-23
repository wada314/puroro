pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}
pub mod _puroro {
    pub use ::puroro::*;
}
pub struct Msg {
    enum_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        self::_puroro_root::enum2::Enum,
        self::_puroro::tags::Enum2::<self::_puroro_root::enum2::Enum>,
        0usize,
    >,
    enum_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        self::_puroro_root::enum2::Enum,
        self::_puroro::tags::Enum2::<self::_puroro_root::enum2::Enum>,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl Msg {
    pub fn enum_optional(&self) -> self::_puroro_root::enum2::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            self::_puroro_root::enum2::Enum,
            self::_puroro::tags::Enum2::<self::_puroro_root::enum2::Enum>,
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.enum_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn enum_repeated(&self) -> &[self::_puroro_root::enum2::Enum] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            self::_puroro_root::enum2::Enum,
            self::_puroro::tags::Enum2::<self::_puroro_root::enum2::Enum>,
        > as RepeatedFieldType>::get_field(&self.enum_repeated, &self._bitfield)
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
)]
pub enum Enum {
    ValueSeven,
    ValueZero,
    ValueOne,
    ValueFourtyTwo,
}
impl ::std::default::Default for Enum {
    fn default() -> Self {
        Self::ValueSeven
    }
}
impl ::std::convert::From::<Enum> for i32 {
    fn from(val: Enum) -> i32 {
        match val {
            Enum::ValueSeven => 7i32,
            Enum::ValueZero => 0i32,
            Enum::ValueOne => 1i32,
            Enum::ValueFourtyTwo => 42i32,
        }
    }
}
impl ::std::convert::TryFrom::<i32> for Enum {
    type Error = self::_puroro::PuroroError;
    fn try_from(val: i32) -> ::std::result::Result<Self, Self::Error> {
        use ::std::result::Result::{Ok, Err};
        match val {
            7i32 => Ok(self::Enum::ValueSeven),
            0i32 => Ok(self::Enum::ValueZero),
            1i32 => Ok(self::Enum::ValueOne),
            42i32 => Ok(self::Enum::ValueFourtyTwo),
            _ => Err(self::_puroro::ErrorKind::UnknownEnumVariant(e))?,
        }
    }
}
