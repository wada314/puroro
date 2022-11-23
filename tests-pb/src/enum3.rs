pub mod _puroro_root {
    pub use super::super::_puroro_root::*;
}
pub mod _puroro {
    pub use ::puroro::*;
}
pub struct Msg {
    enum_optional: self::_puroro::internal::field_type::OptionalNumericalField::<
        self::_puroro_root::enum3::Enum,
        self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
        0usize,
    >,
    enum_unlabeled: self::_puroro::internal::field_type::SingularNumericalField::<
        self::_puroro_root::enum3::Enum,
        self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
    >,
    enum_repeated: self::_puroro::internal::field_type::RepeatedNumericalField::<
        self::_puroro_root::enum3::Enum,
        self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
    >,
    _bitfield: self::_puroro::bitvec::BitArray<1usize>,
}
impl Msg {
    pub fn enum_optional(&self) -> self::_puroro_root::enum3::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::OptionalNumericalField::<
            self::_puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
            0usize,
        > as NonRepeatedFieldType>::get_field(
            &self.enum_optional,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn enum_unlabeled(&self) -> self::_puroro_root::enum3::Enum {
        use self::_puroro::internal::field_type::NonRepeatedFieldType;
        <self::_puroro::internal::field_type::SingularNumericalField::<
            self::_puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
        > as NonRepeatedFieldType>::get_field(
            &self.enum_unlabeled,
            &self._bitfield,
            ::std::default::Default::default,
        )
    }
    pub fn enum_repeated(&self) -> &[self::_puroro_root::enum3::Enum] {
        use self::_puroro::internal::field_type::RepeatedFieldType;
        <self::_puroro::internal::field_type::RepeatedNumericalField::<
            self::_puroro_root::enum3::Enum,
            self::_puroro::tags::Enum3::<self::_puroro_root::enum3::Enum>,
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
    ValueZero,
    ValueSeven,
    ValueOne,
    ValueFourtyTwo,
    _None(i32),
}
impl ::std::default::Default for Enum {
    fn default() -> Self {
        Self::ValueZero
    }
}
