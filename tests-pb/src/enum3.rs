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
pub enum Enum {
    ValueZero,
    ValueSeven,
    ValueOne,
    ValueFourtyTwo,
    _None(i32),
}
